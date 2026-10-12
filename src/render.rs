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

use std::io::Write;
use std::process::{Command, Stdio};

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
        let safe = sanitize_terminal_text(url);
        let label = self.paint(&safe, BLUE_UNDER);
        if self.links && safe_hyperlink_url(&safe) {
            format!("\x1b]8;;{safe}\x1b\\{label}\x1b]8;;\x1b\\")
        } else {
            label
        }
    }
}

pub fn print_outcome(outcome: &Outcome, view: &View) {
    let terms = query_terms(&outcome.query);
    println!(
        "{}  {}",
        view.paint(&sanitize_terminal_text(&outcome.provider), BOLD_CYAN),
        highlight(view, &outcome.query, &terms)
    );
    if let Some(url) = &outcome.source_url {
        println!("{}", view.link(url));
    }

    if let Some(body) = &outcome.body {
        println!();
        let text = sanitize_terminal_body(&crate::text::strip_ansi(body));
        println!("{text}");
    }

    if let Some(summary) = outcome.summary.as_ref().filter(|s| !s.trim().is_empty()) {
        println!();
        if let Some(title) = &outcome.summary_title {
            println!("{}", view.paint(&sanitize_terminal_text(title), BOLD));
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
                let line = sanitize_terminal_body(line);
                let code = is_source_code(&line);
                let shown = if code { tighten_indent(&line) } else { line };
                if view.color && code {
                    let highlighted = highlight_code(&shown, &bat_language(&outcome.provider));
                    for code_line in highlighted.lines() {
                        println!("   {code_line}");
                    }
                } else {
                    for code_line in shown.lines() {
                        println!("   {}", view.paint(code_line, DIM));
                    }
                }
            }
            println!();
        }
    }
}

pub fn print_sources(text: &str, color: bool) {
    let view = View {
        color,
        links: false,
    };
    for line in text.lines() {
        if line.trim().is_empty() {
            println!();
            continue;
        }
        if !line.starts_with(' ') {
            println!("{}", view.paint(line, BOLD_CYAN));
            continue;
        }
        if !color {
            println!("{line}");
            continue;
        }
        if let Some((name, description)) = split_source_row(line) {
            let pad = line.len() - line.trim_start().len();
            let gap = line.len() - pad - name.len() - description.len();
            println!(
                "{:pad$}{}{:gap$}{}",
                "",
                view.paint(name, BOLD_CYAN),
                "",
                view.paint(description, DIM),
                pad = pad,
                gap = gap,
            );
        } else {
            println!("{line}");
        }
    }
}

fn split_source_row(line: &str) -> Option<(&str, &str)> {
    let rest = line.trim_start();
    let split = rest.find("  ")?;
    let name = rest[..split].trim();
    let description = rest[split..].trim();
    if name.is_empty() || description.is_empty() {
        None
    } else {
        Some((name, description))
    }
}

fn tighten_indent(text: &str) -> String {
    let widths: Vec<usize> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(leading_spaces)
        .filter(|width| *width > 0)
        .collect();
    let Some(unit) = widths.into_iter().reduce(gcd) else {
        return text.to_string();
    };
    if unit <= 2 {
        return text.to_string();
    }
    text.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                let width = leading_spaces(line);
                let kept = (width / unit) * 2 + width % unit;
                format!("{}{}", " ".repeat(kept), line.trim_start_matches(' '))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|ch| *ch == ' ').count()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a
}

fn is_source_code(text: &str) -> bool {
    let lines = text.lines().filter(|line| !line.trim().is_empty()).count();
    if lines < 3 {
        return false;
    }
    [
        "#include",
        "class ",
        "template<",
        "fn ",
        "def ",
        "void ",
        "func ",
        "impl ",
        "pub ",
        "let ",
        "std::",
    ]
    .iter()
    .any(|marker| text.contains(marker))
}

fn bat_language(provider: &str) -> String {
    let name = provider
        .split_whitespace()
        .next()
        .unwrap_or(provider)
        .to_ascii_lowercase();
    match name.as_str() {
        "c++" => "cpp",
        "javascript" => "js",
        "typescript" => "ts",
        "golang" => "go",
        "python" => "py",
        "ruby" => "rb",
        "rust" => "rs",
        "haskell" => "hs",
        "ocaml" => "ml",
        "shell" | "sh" => "bash",
        "openjdk" => "java",
        other => other,
    }
    .to_string()
}

fn highlight_code(code: &str, language: &str) -> String {
    let mut child = match Command::new("bat")
        .args([
            "--plain",
            "--paging=never",
            "--color=always",
            "--language",
            language,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return code.to_string(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(code.as_bytes());
        if !code.ends_with('\n') {
            let _ = stdin.write_all(b"\n");
        }
    }
    match child.wait_with_output() {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string())
        }
        _ => code.to_string(),
    }
}

fn is_terminal_spoofing_control(ch: char) -> bool {
    matches!(
        ch,
        '\u{061c}'
            | '\u{200e}'
            | '\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}'
    )
}

fn sanitize_terminal_text(text: &str) -> String {
    text.chars()
        .filter(|ch| !ch.is_control() && *ch != '\u{7f}' && !is_terminal_spoofing_control(*ch))
        .collect()
}

fn sanitize_terminal_body(text: &str) -> String {
    text.chars()
        .filter(|ch| {
            matches!(ch, '\n' | '\t')
                || (!ch.is_control() && *ch != '\u{7f}' && !is_terminal_spoofing_control(*ch))
        })
        .collect()
}

fn safe_hyperlink_url(url: &str) -> bool {
    if url.starts_with("man://") {
        return !url
            .chars()
            .any(|ch| ch.is_control() || is_terminal_spoofing_control(ch));
    }
    crate::text::normalize_http_url(url).is_some()
}

fn highlight(view: &View, text: &str, terms: &[String]) -> String {
    let text = sanitize_terminal_text(text);
    if !view.color || text.is_empty() || terms.is_empty() {
        return text;
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
    fn strips_terminal_controls_from_remote_text_and_links() {
        assert_eq!(
            sanitize_terminal_text("safe\x1b]8;;evil\x07text"),
            "safe]8;;eviltext"
        );
        assert!(!safe_hyperlink_url("https://example.test/\nattack"));
        assert!(!safe_hyperlink_url("javascript:alert(1)"));
        assert_eq!(sanitize_terminal_text("safe\u{202e}spoof"), "safespoof");
        assert!(!safe_hyperlink_url("https://example.test/\u{2066}spoof"));

        let view = View {
            color: false,
            links: true,
        };
        let rendered = view.link("https://example.test/\x1b]8;;evil\x07");
        assert!(!rendered.contains("\x1b]8;;evil"));
        assert!(!rendered.contains('\x07'));
    }

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

    #[test]
    fn code_blocks_are_separate_from_source_labels() {
        let sample = "#include <semaphore>\nclass counting_semaphore;\nvoid acquire();\n";
        assert!(is_source_code(sample));
        assert!(!is_source_code("C++  ·  Standard headers"));
        assert_eq!(bat_language("C++"), "cpp");
        assert_eq!(bat_language("Python 3.14"), "py");
        let (name, description) = split_source_row("  web            DuckDuckGo results").unwrap();
        assert_eq!(name, "web");
        assert_eq!(description, "DuckDuckGo results");
    }

    #[test]
    fn four_space_code_indents_display_as_two() {
        let sample = "void ThreadProc()\n{\n    smph.acquire();\n        nested();\n}\n";
        let tightened = tighten_indent(sample);
        assert!(tightened.contains("\n  smph.acquire();"));
        assert!(tightened.contains("\n    nested();"));
        assert!(!tightened.contains("        nested"));
        let synopsis =
            "namespace std {\n  class counting_semaphore {\n    void acquire();\n  };\n}\n";
        assert_eq!(tighten_indent(synopsis), synopsis);
    }
}
