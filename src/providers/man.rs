use std::process::Command;
use std::sync::LazyLock;

use regex::Regex;

use crate::model::{Hit, Outcome};
use crate::text::strip_overstrike;

pub fn open_page(page: &str) -> Option<Outcome> {
    let output = Command::new("man")
        .args(["-w", page])
        .env("MANPAGER", "cat")
        .env("PAGER", "cat")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let listed = String::from_utf8_lossy(&output.stdout);
    let path = listed.lines().next()?.trim();
    let section = section_from_path(path).unwrap_or_else(|| "1".into());
    let hit = ManHit {
        name: page.to_string(),
        section: section.clone(),
        description: String::new(),
    };
    let mut outcome = Outcome::new("man", page);
    outcome.body = page_excerpt(&hit);
    outcome.hits.push(Hit::new(
        format!("{page} ({section})"),
        format!("man://{section}/{page}"),
    ));
    if outcome.body.is_none() && outcome.hits.is_empty() {
        None
    } else {
        Some(outcome)
    }
}

fn section_from_path(path: &str) -> Option<String> {
    static SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.([0-9]+)").unwrap());
    SECTION
        .captures_iter(path)
        .last()
        .and_then(|caps| caps.get(1).map(|section| section.as_str().to_string()))
}

pub fn search(query: &str, limit: usize) -> Result<Outcome, String> {
    let output = Command::new("man")
        .args(["-k", &escape_apropos(query)])
        .env("MANPAGER", "cat")
        .env("PAGER", "cat")
        .output()
        .map_err(|err| format!("man: {err}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut hits: Vec<ManHit> = stdout.lines().filter_map(parse_apropos_line).collect();
    if hits.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        if detail.is_empty() {
            return Err(format!("no manual entries for {query}"));
        }
        return Err(format!("no manual entries for {query}: {detail}"));
    }

    hits.sort_by_key(|hit| {
        (
            u8::from(!is_exact(query, hit)),
            u8::from(hit.section != "1"),
        )
    });
    let exact = pick_exact(query, &hits);
    hits.truncate(limit);

    let mut outcome = Outcome::new("man", query);
    if let Some(page) = exact {
        if let Some(body) = page_excerpt(&page) {
            outcome.body = Some(body);
        }
    }
    outcome.hits = hits
        .into_iter()
        .map(|hit| {
            Hit::new(
                format!("{} ({})", hit.name, hit.section),
                format!("man://{}/{}", hit.section, hit.name),
            )
            .snippet(hit.description)
        })
        .collect();
    Ok(outcome)
}

#[derive(Clone, Debug)]
struct ManHit {
    name: String,
    section: String,
    description: String,
}

fn parse_apropos_line(line: &str) -> Option<ManHit> {
    static LINE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\S+)\s+\(([^)]+)\)\s+-\s+(.*)$").unwrap());
    let caps = LINE_RE.captures(line)?;
    Some(ManHit {
        name: caps.get(1)?.as_str().to_string(),
        section: caps.get(2)?.as_str().to_string(),
        description: caps.get(3)?.as_str().trim().to_string(),
    })
}

fn is_exact(query: &str, hit: &ManHit) -> bool {
    let dashed = query.trim().replace(' ', "-");
    hit.name.eq_ignore_ascii_case(query.trim()) || hit.name.eq_ignore_ascii_case(&dashed)
}

fn pick_exact(query: &str, hits: &[ManHit]) -> Option<ManHit> {
    hits.iter()
        .filter(|hit| is_exact(query, hit))
        .min_by_key(|hit| u8::from(hit.section != "1"))
        .cloned()
}

fn page_excerpt(page: &ManHit) -> Option<String> {
    let output = Command::new("man")
        .args(["-s", &page.section, &page.name])
        .env("MANPAGER", "cat")
        .env("PAGER", "cat")
        .env("MANWIDTH", "88")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let plain = strip_overstrike(&text).replace('\u{c}', "");
    let body = excerpt(&plain);
    if body.trim().is_empty() {
        None
    } else {
        Some(body)
    }
}

pub fn excerpt(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.trim() == "NAME")
        .unwrap_or(0);
    let mut end = (start + 40).min(lines.len());
    if let Some(offset) = lines
        .iter()
        .skip(start)
        .position(|line| line.trim() == "DESCRIPTION")
    {
        let description = start + offset;
        end = lines
            .iter()
            .enumerate()
            .skip(description + 1)
            .find(|(_, line)| is_section_header(line))
            .map(|(index, _)| index)
            .unwrap_or_else(|| (description + 16).min(lines.len()));
    }
    let end = end.min(start + 48).max(start);
    lines[start..end].join("\n").trim_end().to_string()
}

fn is_section_header(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty()
        && !line.starts_with([' ', '\t'])
        && trimmed
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_whitespace())
}

fn escape_apropos(query: &str) -> String {
    let mut out = String::new();
    for ch in query.chars() {
        if "\\.*+?[](){}|^$".contains(ch) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_apropos_and_keeps_the_description() {
        let hit =
            parse_apropos_line("grep (1)             - print lines that match patterns").unwrap();
        assert_eq!(hit.name, "grep");
        assert_eq!(hit.section, "1");
        assert_eq!(hit.description, "print lines that match patterns");
    }

    #[test]
    fn excerpt_starts_at_name_and_includes_description() {
        let text = "\
GREP(1)                     User Commands\n\
\n\
NAME\n\
       grep - print lines\n\
\n\
SYNOPSIS\n\
       grep [OPTION...] PATTERNS [FILE...]\n\
\n\
DESCRIPTION\n\
       grep searches for PATTERNS.\n\
\n\
OPTIONS\n\
       -E\n\
";
        let body = excerpt(text);
        assert!(body.starts_with("NAME"));
        assert!(body.contains("SYNOPSIS"));
        assert!(body.contains("grep searches"));
        assert!(!body.contains("GREP(1)"));
        assert!(!body.contains("OPTIONS"));
    }
}
