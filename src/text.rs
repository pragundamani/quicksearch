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

#[cfg(test)]
mod tests {
    use super::*;

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
