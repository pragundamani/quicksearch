// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Strict parsing for the model response used by `qs --example`.

use std::collections::HashSet;
use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::Deserialize;

pub const MAX_RESPONSE_BYTES: usize = 64 * 1024;
pub const MAX_CODE_BYTES: usize = 12 * 1024;
pub const MAX_CODE_LINES: usize = 240;
pub const MAX_EXPLANATION_BYTES: usize = 2 * 1024;
pub const MAX_CITATIONS: usize = 3;
const MAX_JSON_DEPTH: usize = 8;

/// The only accepted model response shape.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ExampleResponse {
    pub schema: u8,
    /// A canonical language id returned by [`normalize_language`].
    pub language: String,
    pub code: String,
    pub explanation: String,
    /// One-based indexes into the grounding items supplied to the model.
    pub citations: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseError(String);

impl ResponseError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ResponseError {}

/// Parse one complete JSON value and enforce all response limits.
///
/// `evidence_count` is the number of grounding items in the prompt. Citation
/// indexes are one-based, so every index must be in `1..=evidence_count`.
pub fn parse_response(
    input: &str,
    evidence_count: usize,
) -> Result<ExampleResponse, ResponseError> {
    if input.len() > MAX_RESPONSE_BYTES {
        return Err(ResponseError::new("model response exceeds 64 KiB"));
    }
    if input.contains("```") {
        return Err(ResponseError::new("Markdown fences are not allowed"));
    }
    check_json_envelope(input)?;

    let mut deserializer = serde_json::Deserializer::from_str(input);
    let raw = RawResponse::deserialize(&mut deserializer)
        .map_err(|error| ResponseError::new(format!("invalid response JSON: {error}")))?;
    deserializer
        .end()
        .map_err(|error| ResponseError::new(format!("trailing JSON data: {error}")))?;

    if raw.schema != 1 {
        return Err(ResponseError::new("unsupported response schema"));
    }
    check_text("language", &raw.language, false)?;
    let language = normalize_language(&raw.language)
        .ok_or_else(|| ResponseError::new("unsupported language id"))?;
    check_text("code", &raw.code, true)?;
    check_text("explanation", &raw.explanation, true)?;
    if raw.code.trim().is_empty() {
        return Err(ResponseError::new("code must not be empty"));
    }
    if raw.explanation.trim().is_empty() {
        return Err(ResponseError::new("explanation must not be empty"));
    }

    if raw.code.len() > MAX_CODE_BYTES {
        return Err(ResponseError::new("code exceeds 12 KiB"));
    }
    let line_count = if raw.code.is_empty() {
        0
    } else {
        raw.code.bytes().filter(|byte| *byte == b'\n').count() + 1
    };
    if line_count > MAX_CODE_LINES {
        return Err(ResponseError::new("code exceeds 240 lines"));
    }
    if raw.explanation.len() > MAX_EXPLANATION_BYTES {
        return Err(ResponseError::new("explanation exceeds 2 KiB"));
    }
    if raw.citations.len() > MAX_CITATIONS {
        return Err(ResponseError::new("more than three citations"));
    }
    let mut citations = HashSet::with_capacity(raw.citations.len());
    for &index in &raw.citations {
        if index == 0 || index > evidence_count {
            return Err(ResponseError::new("citation index is out of range"));
        }
        if !citations.insert(index) {
            return Err(ResponseError::new("duplicate citation index"));
        }
    }

    Ok(ExampleResponse {
        schema: 1,
        language: language.to_string(),
        code: raw.code,
        explanation: raw.explanation,
        citations: raw.citations,
    })
}

/// Normalize a supported language name without guessing arbitrary commands.
pub fn normalize_language(input: &str) -> Option<&'static str> {
    match input.trim().to_ascii_lowercase().as_str() {
        "bash" | "sh" | "shell" => Some("bash"),
        "c" | "c17" => Some("c"),
        "c++" | "cpp" | "cxx" | "cc" => Some("cpp"),
        "c#" | "csharp" | "cs" => Some("csharp"),
        "clojure" | "clj" => Some("clojure"),
        "css" => Some("css"),
        "dart" => Some("dart"),
        "elixir" | "ex" => Some("elixir"),
        "erlang" | "erl" => Some("erlang"),
        "go" | "golang" => Some("go"),
        "haskell" | "hs" => Some("haskell"),
        "html" | "html5" => Some("html"),
        "java" => Some("java"),
        "javascript" | "js" | "node" | "nodejs" => Some("javascript"),
        "json" => Some("json"),
        "julia" | "jl" => Some("julia"),
        "kotlin" | "kt" => Some("kotlin"),
        "lua" => Some("lua"),
        "nim" => Some("nim"),
        "ocaml" | "ml" => Some("ocaml"),
        "perl" => Some("perl"),
        "php" => Some("php"),
        "python" | "python3" | "py" => Some("python"),
        "r" => Some("r"),
        "ruby" | "rb" => Some("ruby"),
        "rust" | "rs" => Some("rust"),
        "scala" => Some("scala"),
        "sql" => Some("sql"),
        "swift" => Some("swift"),
        "typescript" | "ts" => Some("typescript"),
        "xml" => Some("xml"),
        "yaml" | "yml" => Some("yaml"),
        "zig" => Some("zig"),
        "zsh" => Some("zsh"),
        _ => None,
    }
}

#[derive(Debug)]
struct RawResponse {
    schema: u8,
    language: String,
    code: String,
    explanation: String,
    citations: Vec<usize>,
}

impl<'de> Deserialize<'de> for RawResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RawResponseVisitor)
    }
}

struct RawResponseVisitor;

impl<'de> Visitor<'de> for RawResponseVisitor {
    type Value = RawResponse;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .write_str("an object with exactly schema, language, code, explanation, and citations")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut seen = HashSet::with_capacity(5);
        let mut schema = None;
        let mut language = None;
        let mut code = None;
        let mut explanation = None;
        let mut citations = None;

        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!("duplicate key {key:?}")));
            }
            match key.as_str() {
                "schema" => schema = Some(map.next_value::<u8>()?),
                "language" => language = Some(map.next_value::<String>()?),
                "code" => code = Some(map.next_value::<String>()?),
                "explanation" => explanation = Some(map.next_value::<String>()?),
                "citations" => citations = Some(map.next_value::<Vec<usize>>()?),
                _ => return Err(de::Error::custom(format!("unknown key {key:?}"))),
            }
        }

        Ok(RawResponse {
            schema: schema.ok_or_else(|| de::Error::missing_field("schema"))?,
            language: language.ok_or_else(|| de::Error::missing_field("language"))?,
            code: code.ok_or_else(|| de::Error::missing_field("code"))?,
            explanation: explanation.ok_or_else(|| de::Error::missing_field("explanation"))?,
            citations: citations.ok_or_else(|| de::Error::missing_field("citations"))?,
        })
    }
}

fn check_json_envelope(input: &str) -> Result<(), ResponseError> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for ch in input.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            } else if forbidden_char(ch, false) {
                return Err(ResponseError::new("forbidden control or bidi character"));
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => {
                depth += 1;
                if depth > MAX_JSON_DEPTH {
                    return Err(ResponseError::new("JSON nesting is too deep"));
                }
            }
            '}' | ']' => depth = depth.saturating_sub(1),
            _ if forbidden_char(ch, true) => {
                return Err(ResponseError::new("forbidden control or bidi character"));
            }
            _ => {}
        }
    }
    Ok(())
}

fn check_text(name: &str, text: &str, allow_layout: bool) -> Result<(), ResponseError> {
    if text.contains("```") {
        return Err(ResponseError::new(format!(
            "{name} contains a Markdown fence"
        )));
    }
    if text.chars().any(|ch| forbidden_char(ch, allow_layout)) {
        return Err(ResponseError::new(format!(
            "{name} contains a forbidden control or bidi character"
        )));
    }
    Ok(())
}

fn forbidden_char(ch: char, allow_layout: bool) -> bool {
    if matches!(
        ch,
        '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'
    ) {
        return true;
    }
    if allow_layout && matches!(ch, '\n' | '\t') {
        return false;
    }
    ch.is_control()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> &'static str {
        r#"{"schema":1,"language":"py","code":"print(1)\n","explanation":"Example.","citations":[1]}"#
    }

    #[test]
    fn parses_and_normalizes_closed_shape() {
        let response = parse_response(valid(), 1).unwrap();
        assert_eq!(response.language, "python");
        assert_eq!(response.citations, vec![1]);
    }

    #[test]
    fn rejects_duplicate_unknown_and_trailing_json() {
        assert!(parse_response(
            r#"{"schema":1,"schema":1,"language":"py","code":"","explanation":"","citations":[]}"#,
            0
        )
        .is_err());
        assert!(parse_response(
            r#"{"schema":1,"language":"py","code":"","explanation":"","citations":[],"extra":0}"#,
            0
        )
        .is_err());
        assert!(parse_response(&format!("{} {{}}", valid()), 1).is_err());
    }

    #[test]
    fn rejects_controls_and_fences() {
        let nul =
            r#"{"schema":1,"language":"py","code":"x\u0000y","explanation":"","citations":[]}"#;
        let bidi =
            r#"{"schema":1,"language":"py","code":"x\u202ey","explanation":"","citations":[]}"#;
        assert!(parse_response(nul, 0).is_err());
        assert!(parse_response(bidi, 0).is_err());
        assert!(parse_response("```json\n{}\n```", 0).is_err());
        let escaped_fence = r#"{"schema":1,"language":"py","code":"\u0060\u0060\u0060","explanation":"","citations":[]}"#;
        assert!(parse_response(escaped_fence, 0).is_err());
        assert!(parse_response(
            r#"{"schema":1,"language":"python","code":" ","explanation":"x","citations":[]}"#,
            0
        )
        .is_err());
    }

    #[test]
    fn enforces_citation_rules() {
        for citations in ["[0]", "[2]", "[1,1]", "[1,2,3,4]"] {
            let input = format!(
                r#"{{"schema":1,"language":"py","code":"","explanation":"","citations":{citations}}}"#
            );
            assert!(parse_response(&input, 1).is_err(), "accepted {citations}");
        }
    }

    #[test]
    fn enforces_size_line_depth_and_language_bounds() {
        let code = "x\n".repeat(MAX_CODE_LINES + 1);
        let value = serde_json::json!({
            "schema": 1, "language": "python", "code": code,
            "explanation": "", "citations": []
        });
        assert!(parse_response(&value.to_string(), 0).is_err());
        let value = serde_json::json!({
            "schema": 1, "language": "python", "code": "x".repeat(MAX_CODE_BYTES + 1),
            "explanation": "", "citations": []
        });
        assert!(parse_response(&value.to_string(), 0).is_err());
        let value = serde_json::json!({
            "schema": 1, "language": "python", "code": "",
            "explanation": "x".repeat(MAX_EXPLANATION_BYTES + 1), "citations": []
        });
        assert!(parse_response(&value.to_string(), 0).is_err());
        assert!(parse_response(&"[".repeat(MAX_JSON_DEPTH + 1), 0).is_err());
        assert_eq!(normalize_language(" C++ "), Some("cpp"));
        assert_eq!(normalize_language("python3"), Some("python"));
        assert_eq!(normalize_language("brainfuck"), None);
    }
}
