// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Full-text search for the official GHC User's Guide Sphinx index.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::semantic::{plan_query, Facet, QueryPlan};
use crate::text::{html_unescape, strip_tags, truncate};

const INDEX_URL: &str = "https://downloads.haskell.org/ghc/latest/docs/users_guide/searchindex.js";
const GUIDE_ROOT: &str = "https://downloads.haskell.org/ghc/latest/docs/users_guide";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SearchIndex {
    docnames: Vec<String>,
    #[serde(default)]
    titles: Vec<String>,
    #[serde(default)]
    terms: HashMap<String, Posting>,
    #[serde(default)]
    titleterms: HashMap<String, Posting>,
    #[serde(default)]
    objects: HashMap<String, Vec<ObjectEntry>>,
    #[serde(default)]
    objnames: HashMap<String, (String, String, String)>,
    #[serde(default)]
    alltitles: HashMap<String, Vec<TitleEntry>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Posting {
    One(usize),
    Many(Vec<usize>),
}

impl Posting {
    fn docs(&self) -> Box<dyn Iterator<Item = usize> + '_> {
        match self {
            Self::One(doc) => Box::new(std::iter::once(*doc)),
            Self::Many(docs) => Box::new(docs.iter().copied()),
        }
    }
}

type ObjectEntry = (usize, usize, usize, String, String);
type TitleEntry = (usize, Option<String>);

#[derive(Clone, Debug)]
struct Candidate {
    tier: u8,
    score: f64,
    doc: usize,
    anchor: String,
    title: String,
    snippet: String,
}

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let index = load_index(client)?;
    Ok(search_index(&index, query, limit))
}

fn load_index(client: &Client) -> Result<SearchIndex, String> {
    eprintln!("qs: fetching GHC User's Guide search index...");
    let source = client.get_text(INDEX_URL, APP_UA, &[])?;
    parse_wrapper(&source).map_err(|error| format!("GHC User's Guide index: {error}"))
}

fn parse_wrapper(source: &str) -> Result<SearchIndex, String> {
    const PREFIX: &str = "Search.setIndex(";
    let source = source.trim();
    let json = source
        .strip_prefix(PREFIX)
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| "expected exactly one Search.setIndex(...) wrapper".to_string())?;
    if json.trim().is_empty() {
        return Err("empty GHC search index".into());
    }
    let index: SearchIndex = serde_json::from_str(json)
        .map_err(|error| format!("invalid GHC search index JSON: {error}"))?;
    validate(&index)?;
    Ok(index)
}

fn validate(index: &SearchIndex) -> Result<(), String> {
    if index.docnames.is_empty() || index.titles.len() != index.docnames.len() {
        return Err("GHC search index has inconsistent documents and titles".into());
    }
    let count = index.docnames.len();
    let valid_posting = |posting: &Posting| posting.docs().all(|doc| doc < count);
    if index.terms.values().any(|p| !valid_posting(p))
        || index.titleterms.values().any(|p| !valid_posting(p))
        || index
            .alltitles
            .values()
            .flatten()
            .any(|(doc, _)| *doc >= count)
        || index
            .objects
            .values()
            .flatten()
            .any(|(doc, _, _, _, _)| *doc >= count)
    {
        return Err("GHC search index contains an invalid document number".into());
    }
    Ok(())
}

fn search_index(index: &SearchIndex, query: &str, limit: usize) -> Outcome {
    let plan = plan_query(query);
    let positive = expanded_terms(&plan, false);
    let negative = expanded_terms(&plan, true);
    let mut candidates = typed_candidates(index, query, &plan, &positive, &negative);
    candidates.extend(heading_candidates(index, &plan, &positive, &negative));
    candidates.extend(body_candidates(index, &plan, &positive, &negative));
    let excluded_docs: HashSet<usize> = negative
        .iter()
        .filter_map(|term| lookup_posting(&index.terms, term))
        .flat_map(Posting::docs)
        .collect();
    candidates.retain(|candidate| !excluded_docs.contains(&candidate.doc));

    candidates.sort_by(|a, b| {
        a.tier
            .cmp(&b.tier)
            .then_with(|| b.score.total_cmp(&a.score))
            .then_with(|| a.title.cmp(&b.title))
            .then_with(|| a.doc.cmp(&b.doc))
    });
    let mut seen = HashSet::new();
    candidates.retain(|candidate| seen.insert((candidate.doc, candidate.anchor.clone())));

    let mut outcome = Outcome::new("ghc-guide", query);
    outcome.source_url = Some(INDEX_URL.to_string());
    outcome.hits = candidates
        .into_iter()
        .take(limit)
        .map(|candidate| {
            Hit::new(
                strip_tags(&html_unescape(&candidate.title)),
                guide_url(&index.docnames[candidate.doc], &candidate.anchor),
            )
            .snippet(truncate(
                &strip_tags(&html_unescape(&candidate.snippet)),
                320,
            ))
        })
        .collect();
    outcome
}

fn typed_candidates(
    index: &SearchIndex,
    raw_query: &str,
    plan: &QueryPlan,
    positive: &[String],
    negative: &[String],
) -> Vec<Candidate> {
    let wanted = normalize(raw_query);
    let mut result = Vec::new();
    for (prefix, entries) in &index.objects {
        for (doc, type_id, _, anchor, display) in entries {
            let display = html_unescape(display);
            let prefix = prefix.trim();
            let title = if prefix.is_empty() {
                display.clone()
            } else {
                format!("{prefix}.{display}")
            };
            let kind = index
                .objnames
                .get(&type_id.to_string())
                .map(|entry| entry.2.as_str())
                .unwrap_or("GHC object");
            let generated_anchor;
            let anchor = if anchor == "-" && kind == "extension" {
                generated_anchor = format!(
                    "extension-{}",
                    display.strip_prefix("-X").unwrap_or(&display)
                );
                generated_anchor.as_str()
            } else if anchor == "-" {
                &display
            } else {
                anchor
            };
            let haystack = normalize(&format!("{title} {anchor} {kind}"));
            if forbidden(&haystack, negative) {
                continue;
            }
            let exact = exact_object_match(&wanted, &normalize(&title), kind);
            let coverage = term_coverage(&haystack, positive);
            if !exact && (positive.is_empty() || coverage < positive.len()) {
                continue;
            }
            result.push(Candidate {
                tier: 0,
                score: if exact {
                    10_000.0
                } else {
                    1_000.0 + coverage as f64
                },
                doc: *doc,
                anchor: anchor.to_string(),
                title,
                snippet: kind.to_string(),
            });
        }
    }
    // Facet polarity is already reflected in expanded terms. Keep this argument
    // explicit so provider ranking cannot accidentally bypass QueryPlan.
    let _ = plan;
    result
}

fn exact_object_match(query: &str, title: &str, kind: &str) -> bool {
    query == title
        || query == normalize(title.trim_start_matches("-x"))
        || query == normalize(&format!("{title} {kind}"))
        || (kind == "extension"
            && query == normalize(&format!("{} extension", title.trim_start_matches("-X"))))
}

fn heading_candidates(
    index: &SearchIndex,
    plan: &QueryPlan,
    positive: &[String],
    negative: &[String],
) -> Vec<Candidate> {
    let mut result = Vec::new();
    for (title, entries) in &index.alltitles {
        let normalized = normalize(title);
        if forbidden(&normalized, negative) {
            continue;
        }
        let coverage = term_coverage(&normalized, positive);
        let phrase = plan.phrases.iter().any(|item| normalized.contains(item));
        let required = positive.len().min(2);
        if coverage < required && !phrase {
            continue;
        }
        for (doc, anchor) in entries {
            result.push(Candidate {
                tier: 1,
                score: coverage as f64 * 20.0 + if phrase { 25.0 } else { 0.0 },
                doc: *doc,
                anchor: anchor.clone().unwrap_or_default(),
                title: title.clone(),
                snippet: index.titles[*doc].clone(),
            });
        }
    }
    result
}

fn body_candidates(
    index: &SearchIndex,
    _plan: &QueryPlan,
    positive: &[String],
    negative: &[String],
) -> Vec<Candidate> {
    let count = index.docnames.len() as f64;
    let mut scores: HashMap<usize, (f64, usize)> = HashMap::new();
    for term in positive {
        if let Some(posting) = lookup_posting(&index.terms, term) {
            let df = posting.docs().count() as f64;
            // BM25's IDF component. Sphinx only stores presence, not term
            // frequency or document length, so tf and length normalization are 1.
            let idf = ((count - df + 0.5) / (df + 0.5) + 1.0).ln();
            for doc in posting.docs() {
                let entry = scores.entry(doc).or_default();
                entry.0 += idf;
                entry.1 += 1;
            }
        }
    }
    let excluded: HashSet<usize> = negative
        .iter()
        .filter_map(|term| lookup_posting(&index.terms, term))
        .flat_map(Posting::docs)
        .collect();
    scores
        .into_iter()
        .filter(|(doc, (_, matched))| !excluded.contains(doc) && *matched > 0)
        .map(|(doc, (score, matched))| Candidate {
            tier: if matched == positive.len() { 1 } else { 2 },
            score: score + 2.0 * matched as f64 / positive.len().max(1) as f64,
            doc,
            anchor: String::new(),
            title: index.titles[doc].clone(),
            snippet: format!("Matched guide text: {}", positive.join(", ")),
        })
        .collect()
}

fn lookup_posting<'a>(terms: &'a HashMap<String, Posting>, term: &str) -> Option<&'a Posting> {
    let lower = term.to_ascii_lowercase();
    terms.get(term).or_else(|| terms.get(&lower)).or_else(|| {
        lower
            .strip_suffix('s')
            .filter(|singular| singular.len() > 2)
            .and_then(|singular| terms.get(singular))
    })
}

fn expanded_terms(plan: &QueryPlan, negative: bool) -> Vec<String> {
    let (terms, facets) = if negative {
        (&plan.negative_terms, &plan.negative_facets)
    } else {
        (&plan.positive_terms, &plan.positive_facets)
    };
    let mut result: Vec<String> = terms
        .iter()
        .filter(|term| {
            !matches!(
                term.as_str(),
                "a" | "an" | "how" | "the" | "to" | "use" | "using"
            )
        })
        .cloned()
        .collect();
    if result.iter().any(|term| {
        matches!(
            term.as_str(),
            "cpu" | "cpus" | "core" | "cores" | "multicore"
        )
    }) {
        for synonym in ["parallel", "smp"] {
            if !result.iter().any(|item| item == synonym) {
                result.push(synonym.to_string());
            }
        }
    }
    for facet in facets {
        for term in facet_words(*facet) {
            if !result.iter().any(|item| item == term) {
                result.push((*term).to_string());
            }
        }
    }
    result
}

fn facet_words(facet: Facet) -> &'static [&'static str] {
    use crate::semantic::{
        Archive::*, Arithmetic::*, Cardinality::*, Comparison::*, Direction::*, Effect::*,
        Operation::*, Ownership::*, Representation::*,
    };
    match facet {
        Facet::Direction(First) => &["first"],
        Facet::Direction(Last) => &["last"],
        Facet::Direction(Prefix) => &["prefix"],
        Facet::Direction(Suffix) => &["suffix"],
        Facet::Direction(Forward) => &["forward"],
        Facet::Direction(Backward) => &["backward"],
        Facet::Ownership(Owning) => &["owning", "ownership"],
        Facet::Ownership(Unique) => &["unique", "exclusive"],
        Facet::Ownership(Shared) => &["shared"],
        Facet::Ownership(Weak) => &["weak"],
        Facet::Ownership(Borrowed) => &["borrowed"],
        Facet::Arithmetic(Checked) => &["checked"],
        Facet::Arithmetic(Wrapping) => &["wrapping"],
        Facet::Arithmetic(Saturating) => &["saturating"],
        Facet::Representation(Text) => &["text"],
        Facet::Representation(Bytes) => &["bytes"],
        Facet::Representation(Regex) => &["regex"],
        Facet::Representation(Shell) => &["shell"],
        Facet::Representation(Url) => &["url"],
        Facet::Effect(Pure) => &["pure"],
        Facet::Effect(Monadic) => &["monadic", "monad"],
        Facet::Effect(Async) => &["async"],
        Facet::Effect(Blocking) => &["blocking"],
        Facet::Cardinality(One) => &["one"],
        Facet::Cardinality(All) => &["all"],
        Facet::Comparison(CaseSensitive) => &["case", "sensitive"],
        Facet::Comparison(CaseInsensitive) => &["case", "insensitive"],
        Facet::Comparison(ConstantTime) => &["constant", "time"],
        Facet::Archive(Gzip) => &["gzip"],
        Facet::Archive(Zip) => &["zip"],
        Facet::Operation(Find) => &["find", "search"],
        Facet::Operation(Remove) => &["remove"],
        Facet::Operation(Read) => &["read"],
        Facet::Operation(Write) => &["write"],
        Facet::Operation(Split) => &["split"],
    }
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

fn term_coverage(haystack: &str, terms: &[String]) -> usize {
    terms
        .iter()
        .filter(|term| haystack.contains(&normalize(term)))
        .count()
}

fn forbidden(haystack: &str, terms: &[String]) -> bool {
    terms.iter().any(|term| haystack.contains(&normalize(term)))
}

fn guide_url(docname: &str, anchor: &str) -> String {
    let mut url = format!("{GUIDE_ROOT}/{docname}.html");
    if !anchor.is_empty() {
        url.push('#');
        url.push_str(anchor);
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> SearchIndex {
        parse_wrapper(include_str!("../../tests/fixtures/ghc_searchindex.js")).unwrap()
    }

    #[test]
    fn wrapper_parser_is_strict_and_typed() {
        let index = fixture();
        assert_eq!(index.docnames.len(), 4);
        assert!(parse_wrapper("alert(1); Search.setIndex({})").is_err());
        assert!(parse_wrapper("Search.setIndex({}); alert(1)").is_err());
        assert!(parse_wrapper("Search.setIndex({\"docnames\":[]})").is_err());
    }

    #[test]
    fn exact_extension_and_flag_rank_before_headings_and_body() {
        let index = fixture();
        let extension = search_index(&index, "LambdaCase extension", 5);
        assert_eq!(extension.hits[0].title, "-XLambdaCase");
        let flag = search_index(&index, "-Wall", 5);
        assert_eq!(flag.hits[0].title, "-Wall");
    }

    #[test]
    fn extension_sentinel_generates_the_sphinx_extension_anchor() {
        let mut index = fixture();
        index.objects.get_mut(" ").unwrap()[0].3 = "-".to_string();
        let result = search_index(&index, "LambdaCase extension", 1);
        assert!(result.hits[0].url.ends_with("#extension-LambdaCase"));
    }

    #[test]
    fn concept_search_uses_heading_then_body_ranking() {
        let result = search_index(&fixture(), "type inference", 5);
        assert_eq!(result.hits[0].title, "Type inference");
        assert!(result
            .hits
            .iter()
            .any(|hit| hit.title == "Using type inference"));
    }

    #[test]
    fn negative_spans_exclude_matching_documents() {
        let result = search_index(&fixture(), "type inference, not runtime", 5);
        assert!(result.hits.iter().all(|hit| !hit.title.contains("Runtime")));
        assert_eq!(result.hits[0].title, "Type inference");
    }

    #[test]
    fn official_urls_use_docnames_and_anchors() {
        let result = search_index(&fixture(), "-XLambdaCase", 1);
        assert_eq!(
            result.hits[0].url,
            format!("{GUIDE_ROOT}/exts/lambda_case.html#extension-LambdaCase")
        );
        assert_eq!(guide_url("using", ""), format!("{GUIDE_ROOT}/using.html"));
    }
}
