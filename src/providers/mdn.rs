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

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let plan = crate::semantic::plan_query(query);
    let mut positive_query = if plan.negative_terms.is_empty() {
        query.to_string()
    } else {
        plan.positive_terms.join(" ")
    };
    if plan
        .positive_terms
        .iter()
        .any(|term| matches!(term.as_str(), "first" | "one"))
    {
        if let Some(rejected) = query
            .split_whitespace()
            .skip_while(|word| !word.eq_ignore_ascii_case("not"))
            .skip(1)
            .find(|word| word.ends_with("All") && word.len() > 3)
        {
            let alternative = rejected.trim_end_matches("All");
            if !alternative.is_empty() {
                positive_query.push(' ');
                positive_query.push_str(alternative);
            }
        }
    }
    let url = format!(
        "https://developer.mozilla.org/api/v1/search?locale=en-US&q={}",
        encode_query(&positive_query)
    );
    let response: MdnResponse = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("mdn", query);
    outcome.source_url = Some(format!(
        "https://developer.mozilla.org/en-US/search?q={}",
        encode_query(&positive_query)
    ));
    let mut identifiers = plan.identifiers.clone();
    identifiers.extend(
        positive_query
            .split_whitespace()
            .map(|part| part.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '.'))
            .filter(|part| part.contains('.') && !part.ends_with('.'))
            .map(str::to_string),
    );
    identifiers.sort();
    identifiers.dedup();
    let mut documents: Vec<_> = response
        .documents
        .into_iter()
        .filter(|doc| !matches_all_negative_terms(doc, &plan.negative_terms))
        .map(|doc| (mdn_rank(&doc, &plan.positive_terms, &identifiers), doc))
        .collect();
    documents.sort_by_key(|document| std::cmp::Reverse(document.0));
    outcome.hits = documents
        .into_iter()
        .take(limit)
        .map(|(_, doc)| {
            let url = if doc.mdn_url.starts_with("http") {
                doc.mdn_url
            } else {
                format!("https://developer.mozilla.org{}", doc.mdn_url)
            };
            Hit::new(doc.title, url).snippet(truncate(&doc.summary, 320))
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no MDN documents matched {query}"));
    }
    Ok(outcome)
}

fn searchable_text(doc: &MdnDoc) -> String {
    format!("{} {} {}", doc.title, doc.mdn_url, doc.summary).to_ascii_lowercase()
}

fn matches_all_negative_terms(doc: &MdnDoc, negative_terms: &[String]) -> bool {
    if negative_terms.is_empty() {
        return false;
    }
    let text = searchable_text(doc);
    negative_terms.iter().all(|term| text.contains(term))
}

fn mdn_rank(doc: &MdnDoc, positive_terms: &[String], identifiers: &[String]) -> i32 {
    let text = searchable_text(doc);
    let mut score = positive_terms
        .iter()
        .filter(|term| text.contains(term.as_str()))
        .count() as i32
        * 100;
    let canonical_url = doc
        .mdn_url
        .to_ascii_lowercase()
        .replace(['_', '-', '/'], "");
    let canonical_url = canonical_url
        .strip_suffix("static")
        .unwrap_or(&canonical_url);
    for identifier in identifiers {
        let identifier = identifier
            .to_ascii_lowercase()
            .replace(['.', ':', '_', '-'], "");
        if !identifier.is_empty() && canonical_url.ends_with(&identifier) {
            score += 10_000;
        }
    }
    score
}

#[derive(Debug, Deserialize)]
struct MdnResponse {
    #[serde(default)]
    documents: Vec<MdnDoc>,
}

#[derive(Debug, Deserialize)]
struct MdnDoc {
    mdn_url: String,
    title: String,
    #[serde(default)]
    summary: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(title: &str, url: &str) -> MdnDoc {
        MdnDoc {
            title: title.into(),
            mdn_url: url.into(),
            summary: String::new(),
        }
    }

    #[test]
    fn exact_identifiers_rank_and_negative_alternatives_are_removed() {
        let exact = doc(
            "AbortSignal: any() static method",
            "/en-US/docs/Web/API/AbortSignal/any_static",
        );
        assert!(mdn_rank(&exact, &["abort".into()], &["AbortSignal.any".into()]) >= 10_000);

        let all = doc(
            "Element: querySelectorAll()",
            "/Web/API/Element/querySelectorAll",
        );
        assert!(matches_all_negative_terms(
            &all,
            &["query".into(), "selector".into(), "all".into()]
        ));
        let one = doc("Element: querySelector()", "/Web/API/Element/querySelector");
        assert!(!matches_all_negative_terms(
            &one,
            &["query".into(), "selector".into(), "all".into()]
        ));
    }
}
