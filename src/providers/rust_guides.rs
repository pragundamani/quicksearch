// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Full-text search over the official mdBook indexes for the Rust Book and
//! Reference.  The JavaScript wrapper is decoded as data; it is never run.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use serde::Deserialize;

use crate::http::{Client, APP_UA};
use crate::model::Hit;
use crate::semantic::{plan_query, QueryPlan};
use crate::text::{collapse_ws, truncate};

const BOOK_ROOT: &str = "https://doc.rust-lang.org/book/";
const REFERENCE_ROOT: &str = "https://doc.rust-lang.org/reference/";

static CACHE: LazyLock<Mutex<HashMap<Guide, Arc<SearchData>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Guide {
    Book,
    Reference,
}

impl Guide {
    fn root(self) -> &'static str {
        match self {
            Self::Book => BOOK_ROOT,
            Self::Reference => REFERENCE_ROOT,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Book => "Rust Book",
            Self::Reference => "Rust Reference",
        }
    }
}

#[derive(Debug)]
struct SearchData {
    sections: Vec<Section>,
    average_length: f64,
}

#[derive(Debug)]
struct Section {
    title: String,
    breadcrumbs: String,
    body: String,
    url: String,
    title_terms: Vec<String>,
    breadcrumb_terms: Vec<String>,
    body_terms: Vec<String>,
}

impl Section {
    fn length(&self) -> usize {
        self.title_terms.len() + self.breadcrumb_terms.len() + self.body_terms.len()
    }

    fn contains(&self, term: &str) -> bool {
        self.title_terms
            .iter()
            .chain(&self.breadcrumb_terms)
            .chain(&self.body_terms)
            .any(|word| word == term)
    }
}

#[derive(Deserialize)]
struct MdBookPayload {
    doc_urls: Vec<String>,
    index: MdBookIndex,
}

#[derive(Deserialize)]
struct MdBookIndex {
    #[serde(rename = "documentStore")]
    document_store: DocumentStore,
}

#[derive(Deserialize)]
struct DocumentStore {
    docs: HashMap<String, StoredDocument>,
}

#[derive(Deserialize)]
struct StoredDocument {
    #[serde(default)]
    title: String,
    #[serde(default)]
    breadcrumbs: String,
    #[serde(default)]
    body: String,
}

struct RankedHit {
    score: f64,
    hit: Hit,
}

/// Search both official guides and merge their results by relevance.
pub fn search(client: &Client, query: &str, limit: usize) -> Result<Vec<Hit>, String> {
    if limit == 0 {
        return Ok(Vec::new());
    }
    let plan = plan_query(query);
    if plan.positive_terms.is_empty() && plan.identifiers.is_empty() && plan.phrases.is_empty() {
        return Ok(Vec::new());
    }

    let book = load(client, Guide::Book)?;
    let reference = load(client, Guide::Reference)?;
    let mut ranked = rank(&book, Guide::Book, query, &plan);
    ranked.extend(rank(&reference, Guide::Reference, query, &plan));
    ranked.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.hit.url.cmp(&right.hit.url))
    });

    let mut seen = HashSet::new();
    Ok(ranked
        .into_iter()
        .filter(|item| seen.insert(item.hit.url.clone()))
        .take(limit)
        .map(|item| item.hit)
        .collect())
}

fn load(client: &Client, guide: Guide) -> Result<Arc<SearchData>, String> {
    if let Some(data) = CACHE
        .lock()
        .map_err(|_| "Rust guide cache lock poisoned")?
        .get(&guide)
        .cloned()
    {
        return Ok(data);
    }
    let html = client.get_text(guide.root(), APP_UA, &[])?;
    let relative = discover_search_index(&html)?;
    let index_url = resolve_index_url(guide.root(), &relative)?;
    let javascript = client.get_text(&index_url, APP_UA, &[])?;
    let data = Arc::new(parse_search_index(&javascript, guide.root())?);
    CACHE
        .lock()
        .map_err(|_| "Rust guide cache lock poisoned")?
        .insert(guide, Arc::clone(&data));
    Ok(data)
}

/// Extract mdBook's hashed index filename from its root page.
pub(crate) fn discover_search_index(html: &str) -> Result<String, String> {
    let marker = "window.path_to_searchindex_js";
    let tail = html
        .find(marker)
        .map(|at| &html[at + marker.len()..])
        .ok_or_else(|| "Rust guide page has no search index declaration".to_string())?;
    let equals = tail
        .find('=')
        .ok_or_else(|| "invalid search index declaration".to_string())?;
    let value = tail[equals + 1..].trim_start();
    let quote = value
        .chars()
        .next()
        .filter(|c| *c == '\'' || *c == '"')
        .ok_or_else(|| "invalid search index path".to_string())?;
    let rest = &value[quote.len_utf8()..];
    let end = rest
        .find(quote)
        .ok_or_else(|| "unterminated search index path".to_string())?;
    let path = &rest[..end];
    if !path.starts_with("searchindex-")
        || !path.ends_with(".js")
        || !path
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err("unsafe Rust guide search index path".to_string());
    }
    Ok(path.to_string())
}

fn resolve_index_url(root: &str, relative: &str) -> Result<String, String> {
    if !root.starts_with("https://doc.rust-lang.org/")
        || !root.ends_with('/')
        || relative.contains('/')
        || relative.contains(':')
        || relative.contains("..")
    {
        return Err("unsafe Rust guide search index URL".to_string());
    }
    Ok(format!("{root}{relative}"))
}

fn canonical_section_url(root: &str, relative: &str) -> Option<String> {
    if !root.starts_with("https://doc.rust-lang.org/")
        || !root.ends_with('/')
        || relative.starts_with('/')
        || relative.contains(':')
        || relative.split(['/', '#', '?']).any(|part| part == "..")
    {
        return None;
    }
    Some(format!("{root}{relative}"))
}

/// Parse the `JSON.parse('...')` wrapper without executing JavaScript.
fn parse_search_index(javascript: &str, root: &str) -> Result<SearchData, String> {
    let json = decode_json_parse_argument(javascript)?;
    let raw: MdBookPayload =
        serde_json::from_str(&json).map_err(|err| format!("invalid mdBook search index: {err}"))?;
    let mut sections = Vec::new();
    for (id, document) in raw.index.document_store.docs {
        let Ok(index) = id.parse::<usize>() else {
            continue;
        };
        let Some(relative) = raw.doc_urls.get(index) else {
            continue;
        };
        let Some(url) = canonical_section_url(root, relative) else {
            continue;
        };
        sections.push(Section {
            title_terms: tokenize(&document.title),
            breadcrumb_terms: tokenize(&document.breadcrumbs),
            body_terms: tokenize(&document.body),
            title: document.title,
            breadcrumbs: document.breadcrumbs,
            body: document.body,
            url,
        });
    }
    sections.sort_by(|a, b| a.url.cmp(&b.url));
    let average_length = if sections.is_empty() {
        1.0
    } else {
        sections.iter().map(Section::length).sum::<usize>() as f64 / sections.len() as f64
    };
    Ok(SearchData {
        sections,
        average_length,
    })
}

fn decode_json_parse_argument(source: &str) -> Result<String, String> {
    let marker = "JSON.parse(";
    let start = source
        .find(marker)
        .ok_or_else(|| "search index has no JSON.parse wrapper".to_string())?
        + marker.len();
    let input = source[start..].trim_start();
    let quote = input
        .chars()
        .next()
        .filter(|c| *c == '\'' || *c == '"')
        .ok_or_else(|| "JSON.parse argument is not a string".to_string())?;
    let mut chars = input[quote.len_utf8()..].chars().peekable();
    let mut output = String::new();
    while let Some(c) = chars.next() {
        if c == quote {
            return Ok(output);
        }
        if c != '\\' {
            output.push(c);
            continue;
        }
        let escaped = chars
            .next()
            .ok_or_else(|| "trailing escape in search index".to_string())?;
        match escaped {
            '\n' => {} // JavaScript line continuation
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
            }
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            'b' => output.push('\u{8}'),
            'f' => output.push('\u{c}'),
            'v' => output.push('\u{b}'),
            '0' => output.push('\0'),
            'x' => output.push(read_hex_char(&mut chars, 2)?),
            'u' => output.push(read_hex_char(&mut chars, 4)?),
            other => output.push(other), // includes escaped quote, slash, and backslash
        }
    }
    Err("unterminated JSON.parse string".to_string())
}

fn read_hex_char<I>(chars: &mut std::iter::Peekable<I>, count: usize) -> Result<char, String>
where
    I: Iterator<Item = char>,
{
    let mut value = 0_u32;
    for _ in 0..count {
        let c = chars
            .next()
            .ok_or_else(|| "short hexadecimal escape".to_string())?;
        value = value * 16
            + c.to_digit(16)
                .ok_or_else(|| "invalid hexadecimal escape".to_string())?;
    }
    char::from_u32(value).ok_or_else(|| "invalid Unicode escape".to_string())
}

fn rank(data: &SearchData, guide: Guide, query: &str, plan: &QueryPlan) -> Vec<RankedHit> {
    let mut terms = plan.positive_terms.clone();
    for identifier in &plan.identifiers {
        for term in tokenize(identifier) {
            if !terms.contains(&term) {
                terms.push(term);
            }
        }
    }
    let mut negatives = plan.negative_terms.clone();
    // Facets are provider-independent concepts. Their debug names give useful
    // guide vocabulary such as `async`, `borrowed`, `checked`, and `split`.
    for facet in &plan.positive_facets {
        for term in tokenize(&format!("{facet:?}")) {
            if !terms.contains(&term) {
                terms.push(term);
            }
        }
    }
    for facet in &plan.negative_facets {
        for term in tokenize(&format!("{facet:?}")) {
            if !negatives.contains(&term) {
                negatives.push(term);
            }
        }
    }
    if terms.is_empty() {
        terms = tokenize(query);
    }

    let count = data.sections.len() as f64;
    let idfs: Vec<f64> = terms
        .iter()
        .map(|term| {
            let df = data
                .sections
                .iter()
                .filter(|section| section.contains(term))
                .count() as f64;
            ((count - df + 0.5) / (df + 0.5) + 1.0).ln()
        })
        .collect();
    let prior = guide_prior(guide, query);
    let mut output = Vec::new();
    for section in &data.sections {
        if negatives.iter().any(|term| section.contains(term)) {
            continue;
        }
        let mut score = 0.0;
        let mut matched = false;
        for (term, idf) in terms.iter().zip(&idfs) {
            let title_tf = frequency(&section.title_terms, term);
            let crumb_tf = frequency(&section.breadcrumb_terms, term);
            let body_tf = frequency(&section.body_terms, term);
            if title_tf + crumb_tf + body_tf > 0.0 {
                matched = true;
            }
            score += idf
                * (3.0 * bm25_tf(title_tf, section.title_terms.len(), data.average_length)
                    + 1.6
                        * bm25_tf(
                            crumb_tf,
                            section.breadcrumb_terms.len(),
                            data.average_length,
                        )
                    + bm25_tf(body_tf, section.body_terms.len(), data.average_length));
        }
        if !matched {
            continue;
        }
        let haystack = format!("{} {} {}", section.title, section.breadcrumbs, section.body)
            .to_ascii_lowercase();
        for phrase in &plan.phrases {
            if haystack.contains(phrase) {
                score += 3.0;
            }
        }
        score *= prior;
        let title = if section.title.trim().is_empty() {
            section.breadcrumbs.clone()
        } else {
            section.title.clone()
        };
        let snippet = make_snippet(section, &terms);
        output.push(RankedHit {
            score,
            hit: Hit::new(title, &section.url)
                .snippet(snippet)
                .meta_line(guide.label()),
        });
    }
    output
}

fn bm25_tf(tf: f64, length: usize, average: f64) -> f64 {
    if tf == 0.0 {
        return 0.0;
    }
    let k1 = 1.2;
    let norm = 0.25 + 0.75 * length as f64 / average.max(1.0);
    tf * (k1 + 1.0) / (tf + k1 * norm)
}

fn frequency(words: &[String], term: &str) -> f64 {
    words.iter().filter(|word| word.as_str() == term).count() as f64
}

fn guide_prior(guide: Guide, query: &str) -> f64 {
    let words: HashSet<String> = tokenize(query).into_iter().collect();
    let task = [
        "how", "example", "learn", "build", "create", "use", "tutorial",
    ]
    .iter()
    .any(|word| words.contains(*word));
    let law = [
        "syntax",
        "grammar",
        "reference",
        "rule",
        "valid",
        "keyword",
        "expression",
        "behavior",
    ]
    .iter()
    .any(|word| words.contains(*word));
    match (guide, task, law) {
        (Guide::Book, true, false) | (Guide::Reference, false, true) => 1.18,
        (Guide::Reference, true, false) | (Guide::Book, false, true) => 0.92,
        _ => 1.0,
    }
}

fn make_snippet(section: &Section, terms: &[String]) -> String {
    let body = collapse_ws(&section.body);
    let lower = body.to_ascii_lowercase();
    let at = terms
        .iter()
        .filter_map(|term| lower.find(term))
        .min()
        .unwrap_or(0);
    let char_at = lower[..at].chars().count();
    let start = char_at.saturating_sub(55);
    let excerpt: String = body.chars().skip(start).take(240).collect();
    let prefix = if start > 0 { "…" } else { "" };
    truncate(&format!("{prefix}{excerpt}"), 240)
}

fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(docs: &[(&str, &str, &str, &str)]) -> SearchData {
        let sections: Vec<_> = docs
            .iter()
            .map(|(url, title, breadcrumbs, body)| Section {
                title: (*title).into(),
                breadcrumbs: (*breadcrumbs).into(),
                body: (*body).into(),
                url: format!("{BOOK_ROOT}{url}"),
                title_terms: tokenize(title),
                breadcrumb_terms: tokenize(breadcrumbs),
                body_terms: tokenize(body),
            })
            .collect();
        let average_length =
            sections.iter().map(Section::length).sum::<usize>() as f64 / sections.len() as f64;
        SearchData {
            sections,
            average_length,
        }
    }

    #[test]
    fn discovers_hashed_index_and_rejects_escape() {
        let html = r#"<script>window.path_to_searchindex_js = "searchindex-a19_F0.js";</script>"#;
        assert_eq!(
            discover_search_index(html).unwrap(),
            "searchindex-a19_F0.js"
        );
        assert!(discover_search_index("window.path_to_searchindex_js='../bad.js'").is_err());
    }

    #[test]
    fn parses_escaped_json_wrapper() {
        let js = r#"window.search=JSON.parse('{"doc_urls":["chapter.html#x"],"index":{"documentStore":{"docs":{"0":{"title":"Borrower\'s guide","breadcrumbs":"Book","body":"line\\nnext"}}}}}');"#;
        let parsed = parse_search_index(js, BOOK_ROOT).unwrap();
        assert_eq!(parsed.sections[0].title, "Borrower's guide");
        assert_eq!(parsed.sections[0].body, "line\nnext");
    }

    #[test]
    fn title_weight_and_prior_rank_results() {
        let data = fixture(&[
            (
                "body.html",
                "A chapter",
                "Book",
                "ownership ownership ownership",
            ),
            ("title.html", "Ownership", "Book", "values"),
        ]);
        let plan = plan_query("ownership");
        let ranked = rank(&data, Guide::Book, "ownership", &plan);
        assert_eq!(
            ranked
                .iter()
                .max_by(|a, b| a.score.total_cmp(&b.score))
                .unwrap()
                .hit
                .url,
            format!("{BOOK_ROOT}title.html")
        );
        assert!(
            guide_prior(Guide::Book, "how to build a crate")
                > guide_prior(Guide::Reference, "how to build a crate")
        );
        assert!(
            guide_prior(Guide::Reference, "expression syntax rule")
                > guide_prior(Guide::Book, "expression syntax rule")
        );
    }

    #[test]
    fn negation_excludes_sections() {
        let data = fixture(&[
            ("shared.html", "Borrowing", "Book", "shared reference"),
            ("mutable.html", "Borrowing", "Book", "mutable reference"),
        ]);
        let ranked = rank(
            &data,
            Guide::Book,
            "reference not mutable",
            &plan_query("reference not mutable"),
        );
        assert_eq!(ranked.len(), 1);
        assert!(ranked[0].hit.url.ends_with("shared.html"));
    }

    #[test]
    fn merged_order_is_deterministic() {
        let book = fixture(&[("b.html", "Ownership", "Book", "ownership")]);
        let reference = fixture(&[(
            "r.html",
            "Ownership syntax",
            "Reference",
            "ownership grammar",
        )]);
        let plan = plan_query("ownership syntax");
        let mut all = rank(&book, Guide::Book, "ownership syntax", &plan);
        all.extend(rank(
            &reference,
            Guide::Reference,
            "ownership syntax",
            &plan,
        ));
        all.sort_by(|a, b| {
            b.score
                .total_cmp(&a.score)
                .then_with(|| a.hit.url.cmp(&b.hit.url))
        });
        assert!(all[0].hit.url.ends_with("r.html"));
    }
}
