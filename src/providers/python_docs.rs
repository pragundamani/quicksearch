// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Search the official Python documentation without executing the JavaScript
//! wrapper used by Sphinx's search index.

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::semantic::{plan_query, QueryPlan};
use crate::text::{collapse_ws, html_unescape, strip_tags, truncate};

const ROOT: &str = "https://docs.python.org/3/";
const INDEX_URL: &str = "https://docs.python.org/3/searchindex.js";
const FETCH_LIVE_PAGE_SNIPPETS: bool = false;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SearchIndex {
    #[serde(default)]
    docnames: Vec<String>,
    #[serde(default)]
    titles: Vec<String>,
    #[serde(default)]
    terms: HashMap<String, Value>,
    #[serde(default)]
    titleterms: HashMap<String, Value>,
    #[serde(default)]
    objects: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
struct Candidate {
    doc: usize,
    score: i32,
    object_score: i32,
    anchor: Option<String>,
    object_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PageContent {
    chunks: Vec<Chunk>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Chunk {
    heading: String,
    anchor: Option<String>,
    text: String,
}

/// Search docs.python.org with a plan made by the common semantic analyser.
pub(crate) fn search_with_plan(
    client: &Client,
    query: &str,
    plan: &QueryPlan,
    limit: usize,
) -> Result<Outcome, String> {
    let index = load_index(client)?;
    let candidates = rank_candidates(&index, query, plan, limit.saturating_mul(3).max(8));
    let mut outcome = Outcome::new("python", query);
    outcome.source_url = Some(format!(
        "{ROOT}search.html?q={}",
        crate::text::encode_query(query)
    ));

    for candidate in candidates.into_iter().take(limit) {
        let Some(docname) = index.docnames.get(candidate.doc) else {
            continue;
        };
        let title = candidate
            .object_name
            .clone()
            .or_else(|| index.titles.get(candidate.doc).cloned())
            .unwrap_or_else(|| docname.to_string());
        let page = FETCH_LIVE_PAGE_SNIPPETS
            .then(|| load_page(client, docname).ok())
            .flatten();
        let best = page
            .as_ref()
            .and_then(|page| best_chunk(&page.chunks, plan));
        let anchor = best
            .and_then(|chunk| chunk.anchor.as_deref())
            .or(candidate.anchor.as_deref());
        let url = document_url(docname, anchor)?;
        let snippet = best
            .map(|chunk| {
                if chunk.heading.is_empty() {
                    truncate(&chunk.text, 360)
                } else {
                    truncate(&format!("{} — {}", chunk.heading, chunk.text), 360)
                }
            })
            .unwrap_or_else(|| index.titles.get(candidate.doc).cloned().unwrap_or_default());
        outcome.hits.push(Hit::new(title, url).snippet(snippet));
    }
    if outcome.hits.is_empty() {
        return Err(format!("no official Python documents matched {query}"));
    }
    Ok(outcome)
}

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let plan = plan_query(query);
    search_with_plan(client, query, &plan, limit)
}

fn load_index(client: &Client) -> Result<SearchIndex, String> {
    let source = client.get_text(INDEX_URL, APP_UA, &[])?;
    parse_search_index(&source).map_err(|error| format!("Python documentation index: {error}"))
}

/// Accept exactly Sphinx's `Search.setIndex(JSON)` wrapper. No JavaScript is
/// interpreted, and leading or trailing statements are rejected.
fn parse_search_index(source: &str) -> Result<SearchIndex, String> {
    const PREFIX: &str = "Search.setIndex(";
    let source = source.trim();
    let body = source
        .strip_prefix(PREFIX)
        .ok_or_else(|| "Python search index has an unexpected wrapper".to_string())?;
    let body = body
        .strip_suffix(";")
        .unwrap_or(body)
        .strip_suffix(')')
        .ok_or_else(|| "Python search index wrapper is not closed".to_string())?;
    let index: SearchIndex = serde_json::from_str(body)
        .map_err(|error| format!("invalid JSON in Python search index: {error}"))?;
    if index.docnames.is_empty() || index.docnames.len() != index.titles.len() {
        return Err("Python search index has inconsistent document arrays".to_string());
    }
    Ok(index)
}

fn rank_candidates(
    index: &SearchIndex,
    query: &str,
    plan: &QueryPlan,
    limit: usize,
) -> Vec<Candidate> {
    let mut scores: HashMap<usize, Candidate> = HashMap::new();
    let mut terms = plan.positive_terms.clone();
    for identifier in &plan.identifiers {
        push_unique(&mut terms, identifier.to_ascii_lowercase());
    }
    if terms.is_empty() {
        terms.extend(words(query));
    }

    for term in &terms {
        add_postings(index, &index.terms, term, 10, &mut scores);
        add_postings(index, &index.titleterms, term, 28, &mut scores);
    }
    collect_object_matches(&index.objects, "", &terms, &mut scores);

    for candidate in scores.values_mut() {
        let docname = index
            .docnames
            .get(candidate.doc)
            .map(String::as_str)
            .unwrap_or("");
        let title = index
            .titles
            .get(candidate.doc)
            .map(String::as_str)
            .unwrap_or("");
        let haystack = format!("{docname} {title}").to_ascii_lowercase();
        for term in &terms {
            if haystack == *term {
                candidate.score += 80;
            } else if title.eq_ignore_ascii_case(term) {
                candidate.score += 60;
            } else if haystack.contains(term) {
                candidate.score += 14;
            }
        }
        for term in &plan.negative_terms {
            if haystack.contains(term) {
                candidate.score -= 180;
            }
        }
        candidate.score += category_adjustment(docname, query);
    }

    let mut result: Vec<_> = scores.into_values().collect();
    result.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.doc.cmp(&right.doc))
    });
    result.truncate(limit);
    result
}

fn add_postings(
    index: &SearchIndex,
    table: &HashMap<String, Value>,
    term: &str,
    weight: i32,
    scores: &mut HashMap<usize, Candidate>,
) {
    let term = term.to_ascii_lowercase();
    for (indexed, posting) in table {
        let indexed = indexed.to_ascii_lowercase();
        let exact = indexed == term;
        let related = term.len() >= 4
            && indexed.len() >= 4
            && (indexed.starts_with(&term) || term.starts_with(&indexed));
        if !exact && !related {
            continue;
        }
        for doc in posting_docs(posting) {
            if doc < index.docnames.len() {
                scores
                    .entry(doc)
                    .or_insert(Candidate {
                        doc,
                        score: 0,
                        object_score: 0,
                        anchor: None,
                        object_name: None,
                    })
                    .score += if exact { weight } else { weight / 3 };
            }
        }
    }
}

fn posting_docs(value: &Value) -> Vec<usize> {
    match value {
        Value::Number(number) => number
            .as_u64()
            .map(|n| vec![n as usize])
            .unwrap_or_default(),
        Value::Array(items) => items
            .iter()
            .filter_map(|item| item.as_u64().map(|n| n as usize))
            .collect(),
        _ => Vec::new(),
    }
}

fn collect_object_matches(
    value: &HashMap<String, Value>,
    namespace: &str,
    terms: &[String],
    scores: &mut HashMap<usize, Candidate>,
) {
    for (name, value) in value {
        let qualified = if namespace.is_empty() {
            name.clone()
        } else if name.is_empty() {
            namespace.to_string()
        } else {
            format!("{namespace}.{name}")
        };
        walk_object_value(value, &qualified, terms, scores);
    }
}

fn walk_object_value(
    value: &Value,
    qualified: &str,
    terms: &[String],
    scores: &mut HashMap<usize, Candidate>,
) {
    match value {
        Value::Object(map) => {
            for (name, child) in map {
                let next = if qualified.is_empty() {
                    name.clone()
                } else {
                    format!("{qualified}.{name}")
                };
                walk_object_value(child, &next, terms, scores);
            }
        }
        Value::Array(items) => {
            if items.len() >= 4 && items.first().and_then(Value::as_u64).is_some() {
                let doc = items[0].as_u64().unwrap_or(0) as usize;
                let anchor = items.get(3).and_then(Value::as_str).unwrap_or("");
                let display = items.get(4).and_then(Value::as_str).unwrap_or(qualified);
                let full = format!("{qualified} {display}").to_ascii_lowercase();
                let mut score = 0;
                for term in terms {
                    let term = term.to_ascii_lowercase();
                    if full == term || display.eq_ignore_ascii_case(&term) {
                        score += 130;
                    } else if full.contains(&term) {
                        score += 52;
                    }
                }
                if score > 0 {
                    let entry = scores.entry(doc).or_insert(Candidate {
                        doc,
                        score: 0,
                        object_score: 0,
                        anchor: None,
                        object_name: None,
                    });
                    if score > entry.object_score {
                        entry.score += score - entry.object_score;
                        entry.object_score = score;
                        entry.object_name = Some(display.to_string());
                        entry.anchor = object_anchor(anchor, display);
                    }
                }
            } else {
                for child in items {
                    walk_object_value(child, qualified, terms, scores);
                }
            }
        }
        _ => {}
    }
}

fn object_anchor(anchor: &str, display: &str) -> Option<String> {
    match anchor {
        "" => None,
        "$" => Some(display.to_string()),
        value => Some(value.to_string()),
    }
}

fn category_adjustment(docname: &str, query: &str) -> i32 {
    let doc = docname.to_ascii_lowercase();
    let query = query.to_ascii_lowercase();
    let requested_whatsnew = query.contains("what's new")
        || query.contains("whatsnew")
        || query.contains("release note");
    let requested_changelog = query.contains("changelog") || query.contains("change log");
    let requested_c_api = query.contains("c api")
        || query.contains("c-api")
        || query.contains("python/c")
        || query.contains("embedding python")
        || query.contains("extending python");
    let unwanted_whatsnew = doc.starts_with("whatsnew/") && !requested_whatsnew;
    let unwanted_changelog =
        (doc.starts_with("changelog/") || doc.contains("changelog")) && !requested_changelog;
    let unwanted_c_api =
        (doc.starts_with("c-api/") || doc.starts_with("extending/")) && !requested_c_api;
    if unwanted_whatsnew || unwanted_changelog || unwanted_c_api {
        -500
    } else {
        0
    }
}

fn load_page(client: &Client, docname: &str) -> Result<PageContent, String> {
    let url = document_url(docname, None)?;
    let html = client.get_text(&url, APP_UA, &[])?;
    Ok(extract_chunks(&html))
}

fn document_url(docname: &str, anchor: Option<&str>) -> Result<String, String> {
    if docname.is_empty()
        || docname.starts_with('/')
        || docname.contains("//")
        || docname
            .split('/')
            .any(|part| matches!(part, "" | "." | ".."))
        || !docname
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '_' | '-' | '.'))
    {
        return Err("unsafe document name in Python search index".to_string());
    }
    let mut url = format!("{ROOT}{docname}.html");
    if let Some(anchor) = anchor.filter(|anchor| safe_fragment(anchor)) {
        url.push('#');
        url.push_str(anchor);
    }
    Ok(url)
}

fn safe_fragment(fragment: &str) -> bool {
    !fragment.is_empty()
        && fragment.chars().all(|ch| {
            ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':' | '(' | ')' | ',')
        })
}

static BLOCK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?is)<(h[1-4]|p)\b([^>]*)>(.*?)</(?:h[1-4]|p)>"#).expect("valid block regex")
});
static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\bid\s*=\s*["']([^"']+)["']"#).expect("valid id regex"));

fn extract_chunks(html: &str) -> PageContent {
    let mut heading = String::new();
    let mut anchor = None;
    let mut chunks = Vec::new();
    for captures in BLOCK_RE.captures_iter(html) {
        let kind = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let attrs = captures.get(2).map(|m| m.as_str()).unwrap_or("");
        let body = captures.get(3).map(|m| m.as_str()).unwrap_or("");
        let text = clean_html(body);
        if kind.to_ascii_lowercase().starts_with('h') {
            heading = text;
            anchor = ID_RE
                .captures(attrs)
                .and_then(|found| found.get(1))
                .map(|found| found.as_str().to_string());
        } else if !text.is_empty() {
            chunks.push(Chunk {
                heading: heading.clone(),
                anchor: anchor.clone(),
                text,
            });
        }
    }
    PageContent { chunks }
}

fn clean_html(html: &str) -> String {
    collapse_ws(&html_unescape(&strip_tags(html)))
}

fn best_chunk<'a>(chunks: &'a [Chunk], plan: &QueryPlan) -> Option<&'a Chunk> {
    chunks.iter().max_by_key(|chunk| chunk_score(chunk, plan))
}

fn chunk_score(chunk: &Chunk, plan: &QueryPlan) -> i32 {
    let heading = chunk.heading.to_ascii_lowercase();
    let text = chunk.text.to_ascii_lowercase();
    let mut score = 0;
    for term in &plan.positive_terms {
        if heading.contains(term) {
            score += 15;
        }
        if text.contains(term) {
            score += 4;
        }
    }
    for identifier in &plan.identifiers {
        let identifier = identifier.to_ascii_lowercase();
        if heading.contains(&identifier) {
            score += 24;
        }
        if text.contains(&identifier) {
            score += 8;
        }
    }
    for term in &plan.negative_terms {
        if heading.contains(term) || text.contains(term) {
            score -= 80;
        }
    }
    score
}

fn words(text: &str) -> Vec<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|word| word.len() > 1)
        .map(str::to_ascii_lowercase)
        .collect()
}

fn push_unique(items: &mut Vec<String>, item: String) {
    if !items.contains(&item) {
        items.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INDEX_FIXTURE: &str = include_str!("../../tests/fixtures/python_docs_searchindex.js");
    const PAGE_FIXTURE: &str = include_str!("../../tests/fixtures/python_docs_page.html");

    #[test]
    fn parses_only_the_expected_wrapper() {
        let index = parse_search_index(INDEX_FIXTURE).unwrap();
        assert_eq!(index.docnames.len(), 4);
        assert!(parse_search_index(&format!("alert(1);{INDEX_FIXTURE}")).is_err());
        assert!(parse_search_index("Search.setIndex({not_json:true})").is_err());
    }

    #[test]
    fn normal_library_page_beats_whatsnew_and_c_api() {
        let index = parse_search_index(INDEX_FIXTURE).unwrap();
        let plan = plan_query("json loads");
        let ranked = rank_candidates(&index, "json loads", &plan, 4);
        assert_eq!(index.docnames[ranked[0].doc], "library/json");
        assert!(ranked.iter().position(|c| c.doc == 0) < ranked.iter().position(|c| c.doc == 1));
        assert!(ranked.iter().position(|c| c.doc == 0) < ranked.iter().position(|c| c.doc == 2));
    }

    #[test]
    fn negative_terms_avoid_matching_chunk() {
        let page = extract_chunks(PAGE_FIXTURE);
        let plan = plan_query("decode JSON but not file");
        let best = best_chunk(&page.chunks, &plan).unwrap();
        assert_eq!(best.anchor.as_deref(), Some("json.loads"));
    }

    #[test]
    fn constructs_only_official_safe_urls() {
        assert_eq!(
            document_url("library/json", Some("json.loads")).unwrap(),
            "https://docs.python.org/3/library/json.html#json.loads"
        );
        assert!(document_url("../evil", None).is_err());
        assert_eq!(
            document_url("library/json", Some("bad#fragment")).unwrap(),
            "https://docs.python.org/3/library/json.html"
        );
    }
}
