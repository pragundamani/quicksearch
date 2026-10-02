use crate::model::Outcome;
use crate::text::query_terms;

const RESET: &str = "\x1b[0m";
const BOLD_CYAN: &str = "\x1b[1m\x1b[38;5;81m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m\x1b[38;5;246m";
const ORANGE: &str = "\x1b[38;5;215m";
const YELLOW_UNDER: &str = "\x1b[4m\x1b[38;5;221m";
const BLUE_UNDER: &str = "\x1b[4m\x1b[38;5;75m";

pub struct View {
    pub color: bool,
    pub links: bool,
}

impl View {
    fn paint(&self, text: &str, code: &str) -> String {
        if !self.color || text.is_empty() {
            text.to_string()
        } else {
            format!("{code}{text}{RESET}")
        }
    }

    fn link(&self, url: &str) -> String {
        let label = self.paint(url, BLUE_UNDER);
        if self.links {
            format!("\x1b]8;;{url}\x1b\\{label}\x1b]8;;\x1b\\")
        } else {
            label
        }
    }
}

pub fn print_outcome(outcome: &Outcome, view: &View) {
    let terms = query_terms(&outcome.query);
    println!(
        "{}  {}",
        view.paint(&outcome.provider, BOLD_CYAN),
        highlight(view, &outcome.query, &terms)
    );
    if let Some(url) = &outcome.source_url {
        println!("{}", view.link(url));
    }

    if let Some(body) = &outcome.body {
        println!();
        let text = if view.color {
            body.clone()
        } else {
            crate::text::strip_ansi(body)
        };
        println!("{text}");
    }

    if let Some(summary) = outcome.summary.as_ref().filter(|s| !s.trim().is_empty()) {
        println!();
        if let Some(title) = &outcome.summary_title {
            println!("{}", view.paint(title, BOLD));
        }
        println!("{}", highlight(view, summary, &terms));
        if let Some(url) = &outcome.summary_url {
            println!("{}", view.link(url));
        }
    }

    if !outcome.hits.is_empty() {
        println!();
        for (index, hit) in outcome.hits.iter().enumerate() {
            let n = index + 1;
            println!(
                "{} {}",
                view.paint(&format!("{n}."), ORANGE),
                highlight(view, &hit.title, &terms)
            );
            if hit.url.starts_with("http") {
                println!("   {}", view.link(&hit.url));
            }
            if !hit.snippet.is_empty() {
                println!("   {}", highlight(view, &hit.snippet, &terms));
            }
            for line in &hit.meta {
                println!("   {}", view.paint(line, DIM));
            }
            println!();
        }
    }
}

fn highlight(view: &View, text: &str, terms: &[String]) -> String {
    if !view.color || text.is_empty() || terms.is_empty() {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let folded: Vec<char> = chars
        .iter()
        .map(|ch| ch.to_lowercase().next().unwrap_or(*ch))
        .collect();
    let mut ranges = Vec::new();
    for term in terms {
        let needle: Vec<char> = term
            .chars()
            .map(|ch| ch.to_lowercase().next().unwrap_or(ch))
            .collect();
        if needle.is_empty() || needle.len() > folded.len() {
            continue;
        }
        for start in 0..=folded.len() - needle.len() {
            if folded[start..start + needle.len()] != needle[..] {
                continue;
            }
            let end = start + needle.len();
            let before_ok = start == 0 || !chars[start - 1].is_alphanumeric();
            let after_ok = end == chars.len() || !chars[end].is_alphanumeric();
            if before_ok && after_ok {
                ranges.push((start, end));
            }
        }
    }
    if ranges.is_empty() {
        return text.to_string();
    }
    ranges.sort_unstable();
    let mut merged = Vec::new();
    for (start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= *last_end {
                *last_end = (*last_end).max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    let mut out = String::new();
    let mut cursor = 0;
    for (start, end) in merged {
        out.extend(chars[cursor..start].iter());
        let piece: String = chars[start..end].iter().collect();
        out.push_str(&view.paint(&piece, YELLOW_UNDER));
        cursor = end;
    }
    out.extend(chars[cursor..].iter());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_whole_terms_only() {
        let view = View {
            color: true,
            links: false,
        };
        let painted = highlight(&view, "use rustc and rust", &["rust".into()]);
        assert!(painted.contains(&format!("{YELLOW_UNDER}rust{RESET}")));
        assert!(painted.contains("rustc"));
        assert!(!painted.contains(&format!("{YELLOW_UNDER}rustc")));
    }
}
