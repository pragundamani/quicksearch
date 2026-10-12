// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::process::Command;
use std::sync::LazyLock;

use regex::Regex;

use crate::model::{Hit, Outcome};
use crate::providers::fallback;
use crate::text::{encode_query, strip_overstrike};

pub fn open_page(page: &str) -> Option<Outcome> {
    open_page_request(&ManHit {
        name: page.to_string(),
        section: String::new(),
        description: String::new(),
    })
}

fn open_page_request(request: &ManHit) -> Option<Outcome> {
    let mut command = Command::new("man");
    command.args(locator_args(request));
    let output = command
        .env("MANPAGER", "cat")
        .env("PAGER", "cat")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let section = if request.section.is_empty() {
        let listed = String::from_utf8_lossy(&output.stdout);
        let path = listed.lines().next()?.trim();
        section_from_path(path).unwrap_or_else(|| "1".into())
    } else {
        request.section.clone()
    };
    let hit = ManHit {
        name: request.name.clone(),
        section: section.clone(),
        description: String::new(),
    };
    let mut outcome = Outcome::new("man", &request.name);
    outcome.body = page_excerpt(&hit);
    outcome.hits.push(Hit::new(
        format!("{} ({section})", request.name),
        format!("man://{section}/{}", request.name),
    ));
    Some(outcome)
}

fn locator_args(request: &ManHit) -> Vec<&str> {
    let mut args = vec!["-w"];
    if !request.section.is_empty() {
        args.extend(["-s", request.section.as_str()]);
    }
    args.push("--");
    args.push(request.name.as_str());
    args
}

fn section_from_path(path: &str) -> Option<String> {
    static SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.([0-9]+[A-Za-z]*)").unwrap());
    SECTION
        .captures_iter(path)
        .last()
        .and_then(|caps| caps.get(1).map(|section| section.as_str().to_string()))
}

pub fn search(query: &str, limit: usize) -> Result<Outcome, String> {
    if query.trim_start().starts_with('-') {
        return Ok(online_search(
            query,
            "manual names cannot start with an option prefix",
        ));
    }
    if let Some(request) = parse_explicit_page(query) {
        return Ok(open_page_request(&request)
            .map(|mut outcome| {
                outcome.query = query.to_string();
                outcome
            })
            .unwrap_or_else(|| {
                online_search(
                    query,
                    &format!(
                        "no local manual entry for {} in section {}",
                        request.name, request.section
                    ),
                )
            }));
    }

    let output = Command::new("man")
        .args(["-k", "--", &escape_apropos(query)])
        .env("MANPAGER", "cat")
        .env("PAGER", "cat")
        .output();
    let output = match output {
        Ok(output) => output,
        Err(err) => return Ok(online_search(query, &format!("man: {err}"))),
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut hits: Vec<ManHit> = stdout.lines().filter_map(parse_apropos_line).collect();
    if hits.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        let reason = if detail.is_empty() {
            format!("no local manual entries for {query}")
        } else {
            format!("no local manual entries for {query}: {detail}")
        };
        return Ok(online_search(query, &reason));
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

fn online_search(query: &str, reason: &str) -> Outcome {
    let exact = query.split_whitespace().next().unwrap_or(query);
    let (title, url, snippet) = match exact {
        "gofmt" => (
            "gofmt command documentation".to_string(),
            "https://pkg.go.dev/cmd/gofmt".to_string(),
            Some("Format Go source code; use -w to rewrite files or -d to display a diff."),
        ),
        "clang-format" => (
            "ClangFormat documentation".to_string(),
            "https://clang.llvm.org/docs/ClangFormat.html".to_string(),
            Some("Format supported source files; use -i for in-place edits and -style to select formatting rules."),
        ),
        _ => (
            format!("Search online manuals for {query}"),
            format!(
                "https://manpages.debian.org/cgi-bin/search.py?q={}",
                encode_query(query)
            ),
            None,
        ),
    };
    let mut outcome = fallback::search("man", query, title, url, reason);
    if let Some(snippet) = snippet {
        if let Some(hit) = outcome.hits.first_mut() {
            hit.snippet = snippet.into();
        }
    }
    outcome
}

#[derive(Clone, Debug)]
struct ManHit {
    name: String,
    section: String,
    description: String,
}

fn parse_explicit_page(query: &str) -> Option<ManHit> {
    let mut words = query.split_whitespace();
    let section = words.next()?;
    let name = words.next()?;
    if words.next().is_some() || !is_section(section) {
        return None;
    }
    Some(ManHit {
        name: name.to_string(),
        section: section.to_string(),
        description: String::new(),
    })
}

fn is_section(value: &str) -> bool {
    let digit_count = value.chars().take_while(|ch| ch.is_ascii_digit()).count();
    digit_count > 0
        && value
            .chars()
            .skip(digit_count)
            .all(|ch| ch.is_ascii_alphabetic())
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
        .args(["-s", &page.section, "--", &page.name])
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
    fn official_command_fallbacks_include_immediate_usage() {
        let gofmt = online_search("gofmt", "not installed");
        assert!(gofmt.hits[0].snippet.contains("-w"));
        let clang = online_search("clang-format", "not installed");
        assert!(clang.hits[0].snippet.contains("-i"));
    }

    #[test]
    fn parses_explicit_numeric_sections() {
        let page = parse_explicit_page("3 getaddrinfo").unwrap();
        assert_eq!(page.section, "3");
        assert_eq!(page.name, "getaddrinfo");

        let posix = parse_explicit_page("  3p   pthread_create ").unwrap();
        assert_eq!(posix.section, "3p");
        assert_eq!(posix.name, "pthread_create");

        let ssl = parse_explicit_page("1ssl openssl").unwrap();
        assert_eq!(ssl.section, "1ssl");
        assert_eq!(ssl.name, "openssl");
    }

    #[test]
    fn leaves_normal_and_ambiguous_queries_for_apropos() {
        assert!(parse_explicit_page("printf").is_none());
        assert!(parse_explicit_page("network programming").is_none());
        assert!(parse_explicit_page("three printf").is_none());
        assert!(parse_explicit_page("3 name extra").is_none());
        assert!(parse_explicit_page("3-p printf").is_none());
        assert!(parse_explicit_page("3p2 printf").is_none());
    }

    #[test]
    fn builds_section_specific_man_arguments() {
        let page = parse_explicit_page("5 resolv.conf").unwrap();
        assert_eq!(locator_args(&page), ["-w", "-s", "5", "--", "resolv.conf"]);

        let unsectioned = ManHit {
            name: "printf".into(),
            section: String::new(),
            description: String::new(),
        };
        assert_eq!(locator_args(&unsectioned), ["-w", "--", "printf"]);
    }

    #[test]
    fn reads_numeric_section_suffixes_from_paths() {
        assert_eq!(
            section_from_path("/usr/share/man/man3/getaddrinfo.3.gz").as_deref(),
            Some("3")
        );
        assert_eq!(
            section_from_path("/usr/share/man/man3p/pthread_create.3p.gz").as_deref(),
            Some("3p")
        );
    }

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

    #[test]
    fn missing_local_pages_have_an_online_search() {
        let got = online_search("clang-format", "not installed");
        assert_eq!(got.provider, "man");
        assert!(got.hits[0].url.contains("clang.llvm.org/docs/ClangFormat"));
    }
}
