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

use crate::http::{Client, CURL_UA};
use crate::model::{Hit, Outcome};
use crate::text::strip_ansi;

pub fn search(client: &Client, query: &str, color: bool) -> Result<Outcome, String> {
    match fetch_sheet(client, query, query, color) {
        Ok(outcome) if outcome.body.is_some() => Ok(outcome),
        first => {
            // cheat.sh sheets are normally named after a command.  If a prose
            // query has no sheet, retry its leading topic and let that sheet's
            // examples answer the more specific question.
            let topic = query.split_whitespace().next().unwrap_or(query);
            if topic == query {
                return first;
            }
            match fetch_sheet(client, topic, query, color) {
                Ok(mut outcome) if outcome.body.is_some() => {
                    if let Some(reference) = command_reference(topic, query) {
                        let mut hit = Hit::new(reference.title, reference.url);
                        if let Some(summary) = reference.summary {
                            hit = hit.snippet(summary);
                        }
                        outcome.hits.push(hit);
                    }
                    Ok(outcome)
                }
                _ => {
                    if let Some(reference) = command_reference(topic, query) {
                        return Ok(reference_fallback(query, reference));
                    }
                    first
                }
            }
        }
    }
}

fn reference_fallback(query: &str, reference: CommandReference) -> Outcome {
    let mut outcome = Outcome::new("cheat", query);
    outcome.source_url = Some(format!(
        "https://cheat.sh/~{}",
        crate::text::encode_query(query)
    ));
    outcome.summary_title = Some("Authoritative command reference".into());
    outcome.summary = Some("The live cheat.sh lookup was unavailable.".into());
    let mut hit = Hit::new(reference.title, reference.url);
    if let Some(summary) = reference.summary {
        hit = hit.snippet(summary);
    }
    outcome.hits.push(hit);
    outcome
}

fn fetch_sheet(
    client: &Client,
    path_query: &str,
    display_query: &str,
    color: bool,
) -> Result<Outcome, String> {
    let path = cheat_path(path_query);
    let mut url = format!("https://cheat.sh/{path}");
    if !color {
        url.push_str("?T");
    }
    let body = client.get_text(&url, CURL_UA, &[])?;
    outcome_from_body(display_query, &url, &body, color)
}

fn outcome_from_body(query: &str, url: &str, body: &str, color: bool) -> Result<Outcome, String> {
    let plain = strip_ansi(body);
    if unavailable_body(&plain) {
        return Ok(unavailable_outcome(query));
    }

    let text = if color { body } else { &plain };
    if text.trim().is_empty() {
        return Ok(unavailable_outcome(query));
    }

    let mut outcome = Outcome::new("cheat.sh", query);
    outcome.source_url = Some(url.to_string());
    outcome.body = Some(text.trim_end().to_string());
    Ok(outcome)
}

struct CommandReference {
    title: String,
    url: String,
    summary: Option<&'static str>,
}

fn command_reference(command: &str, query: &str) -> Option<CommandReference> {
    let normalized = query.to_ascii_lowercase().replace('_', "-");
    let (url, summary) = match command.to_ascii_lowercase().as_str() {
        "awk" if normalized.contains("field") && normalized.contains("separator") => (
            "https://www.gnu.org/software/gawk/manual/html_node/Field-Separators.html",
            Some("Use -F fs or set FS to select the input field separator."),
        ),
        "awk" => (
            "https://www.gnu.org/software/gawk/manual/gawk.html",
            None,
        ),
        "tar" if normalized.contains("strip") && normalized.contains("component") => (
            "https://www.gnu.org/software/tar/manual/html_node/transform.html",
            Some("--strip-components=NUMBER removes that many leading path components when extracting."),
        ),
        "tar" => (
            "https://www.gnu.org/software/tar/manual/html_node/Option-Summary.html",
            None,
        ),
        "sed" => ("https://www.gnu.org/software/sed/manual/sed.html", None),
        "grep" => ("https://www.gnu.org/software/grep/manual/grep.html", None),
        "find" => (
            "https://www.gnu.org/software/findutils/manual/html_mono/find.html",
            None,
        ),
        _ => return None,
    };
    Some(CommandReference {
        title: format!("{command} manual: {query}"),
        url: url.to_string(),
        summary,
    })
}

fn unavailable_body(body: &str) -> bool {
    let trimmed = body.trim_start();
    let prefix = trimmed
        .char_indices()
        .nth(2048)
        .map_or(trimmed, |(end, _)| &trimmed[..end])
        .to_ascii_lowercase();

    let has_404_marker = trimmed.lines().take(12).any(|line| {
        line.trim()
            .trim_matches(|ch: char| matches!(ch, '/' | '*' | '#' | '-' | ':'))
            .trim()
            .eq_ignore_ascii_case("404 not found")
    });

    prefix.starts_with("<!doctype")
        || prefix.starts_with("<html")
        || prefix.contains("unknown cheat sheet")
        || prefix.contains("unknown topic")
        || has_404_marker
}

fn unavailable_outcome(query: &str) -> Outcome {
    let search_url = format!("https://cheat.sh/~{}", cheat_path(query));
    let mut outcome = Outcome::new("cheat.sh", query);
    outcome.source_url = Some(search_url.clone());
    outcome.summary_title = Some("No exact cheat.sh sheet".to_string());
    outcome.summary = Some("Search cheat.sh for a related sheet.".to_string());
    outcome.summary_url = Some(search_url.clone());
    outcome.hits.push(
        Hit::new(format!("Search cheat.sh for {query}"), search_url)
            .snippet("The exact topic was unavailable."),
    );
    outcome
}

pub fn cheat_path(query: &str) -> String {
    query
        .trim()
        .trim_matches('/')
        .split('/')
        .map(|segment| {
            segment
                .split_whitespace()
                .map(encode_segment)
                .collect::<Vec<_>>()
                .join("+")
        })
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/")
}

fn encode_segment(segment: &str) -> String {
    let mut out = String::new();
    for ch in segment.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '~') {
            out.push(ch);
        } else {
            for byte in ch.encode_utf8(&mut [0; 4]).as_bytes() {
                out.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prose_queries_can_link_to_official_command_references() {
        let generic = command_reference("tar", "extract selected paths").unwrap();
        assert!(generic.title.contains("extract selected paths"));
        assert!(generic.url.contains("gnu.org/software/tar/manual"));
        assert!(generic.summary.is_none());

        let strip = command_reference("tar", "extract --strip-components").unwrap();
        assert!(strip.url.ends_with("/transform.html"));
        assert!(strip.summary.unwrap().contains("--strip-components=NUMBER"));
        assert!(command_reference("unknown-tool", "option").is_none());
    }

    #[test]
    fn spaces_become_plus_and_slashes_stay() {
        assert_eq!(cheat_path("tar"), "tar");
        assert_eq!(cheat_path("rust read file"), "rust+read+file");
        assert_eq!(cheat_path("rust/vec push"), "rust/vec+push");
        assert_eq!(cheat_path("c++"), "c%2B%2B");
    }

    #[test]
    fn http_success_404_fixture_becomes_search_result() {
        let body = include_str!("fixtures/cheat_404.txt");
        let outcome = outcome_from_body(
            "language topic that is absent",
            "https://cheat.sh/language+topic+that+is+absent?T",
            body,
            false,
        )
        .unwrap();

        assert!(outcome.body.is_none());
        assert_eq!(outcome.hits.len(), 1);
        assert_eq!(
            outcome.hits[0].url,
            "https://cheat.sh/~language+topic+that+is+absent"
        );
        assert!(outcome
            .summary
            .as_deref()
            .unwrap()
            .contains("Search cheat.sh"));
    }

    #[test]
    fn html_error_page_becomes_search_result() {
        let outcome = outcome_from_body(
            "missing topic",
            "https://cheat.sh/missing+topic?T",
            "<!DOCTYPE html><title>404 Not Found</title>",
            false,
        )
        .unwrap();

        assert!(outcome.body.is_none());
        assert_eq!(outcome.hits[0].url, "https://cheat.sh/~missing+topic");
    }

    #[test]
    fn valid_sheet_is_preserved() {
        let body = "\u{1b}[32m$ tar -xf archive.tar\u{1b}[0m\n";
        let plain = outcome_from_body("tar", "https://cheat.sh/tar?T", body, false).unwrap();
        let colored = outcome_from_body("tar", "https://cheat.sh/tar", body, true).unwrap();

        assert_eq!(plain.body.as_deref(), Some("$ tar -xf archive.tar"));
        assert_eq!(colored.body.as_deref(), Some(body.trim_end()));
        assert!(plain.hits.is_empty());
        assert_eq!(plain.source_url.as_deref(), Some("https://cheat.sh/tar?T"));
    }

    #[test]
    fn a_sheet_that_mentions_404_is_not_rejected() {
        let body = "# Return 404 NOT FOUND from nginx\nreturn 404;\n";
        let outcome =
            outcome_from_body("nginx 404", "https://cheat.sh/nginx+404?T", body, false).unwrap();

        assert_eq!(outcome.body.as_deref(), Some(body.trim_end()));
    }
}
