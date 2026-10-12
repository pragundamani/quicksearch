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

use serde::Deserialize;
use serde_json::Value;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::providers::fallback;
use crate::text::{collapse_ws, encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    if let Some(number) = rfc_number(query) {
        return one_rfc(client, number);
    }
    let trimmed = query.trim();
    let numeric = trimmed
        .strip_prefix("rfc")
        .or_else(|| trimmed.strip_prefix("RFC"))
        .unwrap_or(trimmed)
        .trim();
    if !numeric.is_empty() && numeric.chars().all(|ch| ch.is_ascii_digit()) {
        return Ok(search_fallback(query, "invalid RFC number"));
    }
    if let Some(number) = canonical_rfc_hint(query) {
        let mut outcome = one_rfc(client, number)?;
        outcome.query = query.to_string();
        outcome.source_url = Some(format!(
            "https://datatracker.ietf.org/doc/search/?name={}",
            encode_query(query)
        ));
        return Ok(outcome);
    }
    let mut objects = match fetch_title_matches(client, query, limit) {
        Ok(objects) => objects,
        Err(reason) => return Ok(search_fallback(query, &reason)),
    };
    if let Some(broader) = broader_title_query(query) {
        if objects.is_empty() || should_include_broader_results(query) {
            for object in fetch_title_matches(client, &broader, limit).unwrap_or_default() {
                let name = object.get("name").and_then(Value::as_str);
                if !objects
                    .iter()
                    .any(|existing| existing.get("name").and_then(Value::as_str) == name)
                {
                    objects.push(object);
                }
            }
        }
    }
    rank_title_matches(&mut objects, query);
    let mut outcome = Outcome::new("rfc", query);
    outcome.source_url = Some(format!(
        "https://datatracker.ietf.org/doc/search/?name={}",
        encode_query(query)
    ));
    for object in objects.into_iter().take(limit) {
        let title = object
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let name = object
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if title.is_empty() && name.is_empty() {
            continue;
        }
        let number = object
            .get("rfc")
            .and_then(|value| {
                value
                    .as_str()
                    .map(str::to_string)
                    .or_else(|| value.as_i64().map(|n| n.to_string()))
            })
            .or_else(|| name.strip_prefix("rfc").map(str::to_string));
        let url = number
            .as_ref()
            .map(|number| format!("https://www.rfc-editor.org/rfc/rfc{number}"))
            .unwrap_or_else(|| format!("https://datatracker.ietf.org/doc/{name}"));
        let label = number.map(|number| format!("RFC {number}")).unwrap_or(name);
        outcome.hits.push(
            Hit::new(
                if title.is_empty() {
                    label.clone()
                } else {
                    title
                },
                url,
            )
            .meta_line(label),
        );
    }
    if outcome.hits.is_empty() {
        return Ok(search_fallback(query, &format!("no RFCs matched {query}")));
    }
    Ok(outcome)
}

fn fetch_title_matches(client: &Client, query: &str, limit: usize) -> Result<Vec<Value>, String> {
    let url = format!(
        "https://datatracker.ietf.org/api/v1/doc/document/?format=json&limit={limit}&type__slug=rfc&title__icontains={}",
        encode_query(query)
    );
    let response: Value = client.get_json(&url, APP_UA, &[])?;
    Ok(response
        .get("objects")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

fn broader_title_query(query: &str) -> Option<String> {
    let mut words: Vec<_> = query.split_whitespace().collect();
    (words.len() > 1).then(|| {
        words.pop();
        words.join(" ")
    })
}

fn should_include_broader_results(query: &str) -> bool {
    query.split_whitespace().count() == 2
}

fn rank_title_matches(objects: &mut [Value], query: &str) {
    let terms: Vec<String> = query
        .split_whitespace()
        .map(str::to_ascii_lowercase)
        .collect();
    let first = terms.first().map(String::as_str).unwrap_or("");
    objects.sort_by_key(|object| {
        let title = object
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_ascii_lowercase();
        let missing = terms
            .iter()
            .filter(|term| !title.contains(term.as_str()))
            .count();
        let does_not_start_with_topic = !first.is_empty() && !title.starts_with(first);
        let is_draft = object
            .get("rfc")
            .is_none_or(|value| value.is_null() || value.as_str().is_some_and(str::is_empty));
        (missing, does_not_start_with_topic, is_draft, title.len())
    });
}

fn one_rfc(client: &Client, number: &str) -> Result<Outcome, String> {
    let url = format!("https://www.rfc-editor.org/rfc/rfc{number}.json");
    let doc: RfcDoc = match client.get_json(&url, APP_UA, &[]) {
        Ok(doc) => doc,
        Err(reason) => return Ok(number_fallback(number, &reason)),
    };
    let title = doc.title.unwrap_or_else(|| format!("RFC {number}"));
    let page = format!("https://www.rfc-editor.org/rfc/rfc{number}");
    let mut outcome = Outcome::new("rfc", &format!("RFC {number}"));
    outcome.source_url = Some(page.clone());
    if let Some(abstract_text) = doc.abstract_text.filter(|text| !text.trim().is_empty()) {
        outcome.summary = Some(truncate(&collapse_ws(&abstract_text), 700));
        outcome.summary_title = Some(title.clone());
        outcome.summary_url = Some(page.clone());
    }
    outcome
        .hits
        .push(Hit::new(title, page).meta_line(format!("RFC {number}")));
    Ok(outcome)
}

fn search_fallback(query: &str, reason: &str) -> Outcome {
    let url = format!(
        "https://datatracker.ietf.org/doc/search/?name={}",
        encode_query(query)
    );
    fallback::search(
        "rfc",
        query,
        format!("Search IETF documents for {query}"),
        url,
        reason,
    )
}

fn number_fallback(number: &str, reason: &str) -> Outcome {
    if reason.contains("404") {
        return search_fallback(&format!("RFC {number}"), reason);
    }
    let page = format!("https://www.rfc-editor.org/rfc/rfc{number}");
    fallback::search(
        "rfc",
        &format!("RFC {number}"),
        format!("RFC {number}"),
        page,
        reason,
    )
}

fn canonical_rfc_hint(query: &str) -> Option<&'static str> {
    let words: Vec<String> = query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    let topic_words: Vec<_> = words.iter().filter(|word| word.as_str() != "rfc").collect();
    if topic_words.len() == 1 && topic_words[0] == "quic" {
        return Some("9000");
    }
    [(&["quic", "transport"][..], "9000")]
        .into_iter()
        .find(|(required, _)| {
            required
                .iter()
                .all(|term| words.iter().any(|word| word == term))
        })
        .map(|(_, number)| number)
}

fn rfc_number(query: &str) -> Option<&str> {
    let query = query.trim();
    let digits = query
        .strip_prefix("rfc")
        .or_else(|| query.strip_prefix("RFC"))
        .unwrap_or(query)
        .trim();
    if digits
        .parse::<u32>()
        .is_ok_and(|number| (1..=99_999).contains(&number))
    {
        Some(digits)
    } else {
        None
    }
}

#[derive(Deserialize)]
struct RfcDoc {
    #[serde(default)]
    title: Option<String>,
    #[serde(default, rename = "abstract")]
    abstract_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broadens_over_specific_title_phrases_once() {
        assert_eq!(
            broader_title_query("structured field values HTTP").as_deref(),
            Some("structured field values")
        );
        assert_eq!(
            broader_title_query("QUIC transport").as_deref(),
            Some("QUIC")
        );
    }

    #[test]
    fn protocol_terms_resolve_to_the_core_standard() {
        assert_eq!(canonical_rfc_hint("QUIC"), Some("9000"));
        assert_eq!(canonical_rfc_hint("RFC QUIC"), Some("9000"));
        assert_eq!(canonical_rfc_hint("QUIC transport"), Some("9000"));
        assert_eq!(canonical_rfc_hint("QUIC applicability"), None);
    }

    #[test]
    fn core_protocol_title_ranks_before_qualified_documents() {
        let mut objects = vec![
            serde_json::json!({
                "title": "Applicability of the QUIC Transport Protocol",
                "name": "rfc9308",
                "rfc": "9308"
            }),
            serde_json::json!({
                "title": "QUIC: A UDP-Based Multiplexed and Secure Transport",
                "name": "rfc9000",
                "rfc": "9000"
            }),
        ];
        rank_title_matches(&mut objects, "QUIC transport");
        assert_eq!(objects[0]["rfc"], "9000");
    }

    #[test]
    fn title_fallback_uses_datatracker_search() {
        let got = search_fallback("structured field values HTTP", "no matches");
        assert!(got.hits[0]
            .url
            .contains("name=structured%20field%20values%20HTTP"));
    }

    #[test]
    fn zero_and_implausibly_large_values_are_not_rfc_numbers() {
        assert_eq!(rfc_number("1"), Some("1"));
        assert_eq!(rfc_number("RFC 99999"), Some("99999"));
        assert_eq!(rfc_number("0"), None);
        assert_eq!(rfc_number("100000"), None);
    }

    #[test]
    fn numeric_fallback_uses_canonical_rfc_page() {
        let got = number_fallback("99999", "network unavailable");
        assert_eq!(got.hits[0].url, "https://www.rfc-editor.org/rfc/rfc99999");
        let missing = number_fallback("99999", "HTTP 404");
        assert!(missing.hits[0]
            .url
            .contains("datatracker.ietf.org/doc/search"));
    }
}
