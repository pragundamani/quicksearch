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

use std::sync::LazyLock;

use regex::Regex;

pub fn collapse_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn truncate(text: &str, max_chars: usize) -> String {
    let count = text.chars().count();
    if count <= max_chars {
        return text.to_string();
    }
    let keep = max_chars.saturating_sub(1);
    let mut out: String = text.chars().take(keep).collect();
    out.push('…');
    out
}

/// Returns at most `max_bytes` from the start of `text` without splitting a UTF-8 character.
pub fn byte_prefix(text: &str, max_bytes: usize) -> &str {
    let mut end = max_bytes.min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

pub fn strip_tags(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for ch in text.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

pub fn html_unescape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        match decode_entity(rest) {
            Some((ch, len)) => {
                out.push(ch);
                rest = &rest[len..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn decode_entity(input: &str) -> Option<(char, usize)> {
    let end = input.find(';')?;
    if end > 12 {
        return None;
    }
    let body = &input[1..end];
    let ch = if let Some(hex) = body.strip_prefix("#x").or_else(|| body.strip_prefix("#X")) {
        char::from_u32(u32::from_str_radix(hex, 16).ok()?)?
    } else if let Some(dec) = body.strip_prefix('#') {
        char::from_u32(dec.parse().ok()?)?
    } else {
        match body {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "quot" => '"',
            "apos" => '\'',
            "nbsp" => ' ',
            _ => return None,
        }
    };
    Some((ch, end + 1))
}

pub fn strip_ansi(text: &str) -> String {
    static ANSI: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\x1b\[[0-9;]*m").unwrap());
    ANSI.replace_all(text, "").into_owned()
}

pub fn strip_overstrike(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(chars.len());
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '\u{8}' {
            out.push(chars[i + 2]);
            i += 3;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

pub fn query_terms(query: &str) -> Vec<String> {
    static WORDS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z0-9_./:-]+").unwrap());
    let mut terms: Vec<String> = WORDS
        .find_iter(query)
        .map(|m| m.as_str().to_string())
        .filter(|term| term.chars().count() > 1)
        .collect();
    terms.sort_by_key(|term| std::cmp::Reverse(term.chars().count()));
    terms
}

pub fn compact_count(n: u64) -> String {
    const UNITS: [(u64, &str); 3] = [(1_000_000_000, "B"), (1_000_000, "M"), (1_000, "k")];
    for (div, suffix) in UNITS {
        if n >= div {
            let value = n as f64 / div as f64;
            if value >= 100.0 || (value * 10.0).round() % 10.0 == 0.0 {
                return format!("{value:.0}{suffix}");
            }
            return format!("{value:.1}{suffix}");
        }
    }
    n.to_string()
}

pub fn encode_query(query: &str) -> String {
    urlencoding::encode(query).into_owned()
}

fn unsafe_url_text(text: &str) -> bool {
    text.chars().any(|ch| {
        ch.is_control()
            || matches!(
                ch,
                '\u{061c}'
                    | '\u{200e}'
                    | '\u{200f}'
                    | '\u{202a}'..='\u{202e}'
                    | '\u{2066}'..='\u{2069}'
            )
    })
}

fn unsafe_after_percent_decoding(raw: &str) -> bool {
    let mut current = raw.to_string();
    for _ in 0..64 {
        if unsafe_url_text(&current) {
            return true;
        }
        let Ok(decoded) = urlencoding::decode(&current) else {
            return true;
        };
        if decoded == current {
            return false;
        }
        current = decoded.into_owned();
    }
    // Excessive encoding depth is not useful for a lookup URL and can hide a
    // dangerous character beyond a validator's decoding budget.
    true
}

pub fn normalize_http_url(raw: &str) -> Option<String> {
    if raw.len() > 8 * 1024 || unsafe_after_percent_decoding(raw) {
        return None;
    }
    let rest = raw
        .strip_prefix("https://")
        .or_else(|| raw.strip_prefix("http://"))?;
    if rest.is_empty() || rest.starts_with('/') {
        return None;
    }
    let parsed = url::Url::parse(raw).ok()?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str()?.is_empty() {
        return None;
    }
    Some(parsed.into())
}

pub fn sanitize_terminal_line(text: &str) -> String {
    let normalized: String = text
        .chars()
        .map(|ch| {
            if ch.is_control()
                || matches!(
                    ch,
                    '\u{061c}'
                        | '\u{200e}'
                        | '\u{200f}'
                        | '\u{202a}'..='\u{202e}'
                        | '\u{2066}'..='\u{2069}'
                )
            {
                ' '
            } else {
                ch
            }
        })
        .collect();
    normalized.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_absolute_http_urls_without_spoofing_controls() {
        assert_eq!(
            normalize_http_url("https://example.test/path").as_deref(),
            Some("https://example.test/path")
        );
        assert!(normalize_http_url("http:///test").is_none());
        assert!(normalize_http_url("https://example.test/\u{202e}test").is_none());
        assert!(normalize_http_url("https://example.test/%E2%80%AEtest").is_none());
        assert!(normalize_http_url("https://example.test/%25E2%2580%25AEtest").is_none());
        assert!(normalize_http_url("https://example.test/%00").is_none());
        assert!(normalize_http_url("https://example.test/%0D%0Aheader").is_none());
        assert!(normalize_http_url("https://example.test/%250A").is_none());
        assert!(normalize_http_url("javascript:alert(1)").is_none());
        assert!(normalize_http_url("https://example.test/a%20b/%E2%9C%93").is_some());
    }

    #[test]
    fn terminal_line_removes_controls_and_bidi_overrides() {
        assert_eq!(
            sanitize_terminal_line("Dev\x1b]8;;evil\x07Docs\u{202e}spoof\nnext"),
            "Dev ]8;;evil Docs spoof next"
        );
    }

    #[test]
    fn byte_prefix_stops_at_unicode_boundaries() {
        assert_eq!(byte_prefix("ab“cd", 4), "ab");
        assert_eq!(byte_prefix("ab“cd", 5), "ab“");
        assert_eq!(byte_prefix("café ☕", 5), "café");
        assert_eq!(byte_prefix("🙂ok", 4), "🙂");
        assert_eq!(byte_prefix("🙂ok", usize::MAX), "🙂ok");
    }

    #[test]
    fn unescapes_common_entities() {
        assert_eq!(html_unescape("a &amp; b &#x27;c&#39;"), "a & b 'c'");
        assert_eq!(html_unescape("&lt;Vec&gt;"), "<Vec>");
    }

    #[test]
    fn strips_tags_and_collapses_space() {
        assert_eq!(
            collapse_ws(&strip_tags("<span class=\"x\">Rust</span>  lang")),
            "Rust lang"
        );
    }

    #[test]
    fn overstrike_keeps_the_printed_char() {
        assert_eq!(strip_overstrike("g\u{8}gr\u{8}re\u{8}ep\u{8}p"), "grep");
        assert_eq!(strip_overstrike("_\u{8}g"), "g");
    }

    #[test]
    fn compact_counts_round_cleanly() {
        assert_eq!(compact_count(1_471_000_000), "1.5B");
        assert_eq!(compact_count(337_000_000), "337M");
        assert_eq!(compact_count(1_500), "1.5k");
        assert_eq!(compact_count(2_000), "2k");
        assert_eq!(compact_count(42), "42");
    }
}
