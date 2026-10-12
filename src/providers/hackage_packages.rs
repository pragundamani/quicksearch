// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Package discovery through Hackage's JSON API.
//!
//! This provider intentionally does not use Hoogle or Haddock. Hoogle is for
//! API/symbol search; this module only handles package-discovery queries.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{encode_query, truncate};

const CACHE_SCHEMA: u32 = 1;
const SEARCH_TTL: Duration = Duration::from_secs(6 * 60 * 60);
const METADATA_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const STALE_MAX_AGE: Duration = Duration::from_secs(14 * 24 * 60 * 60);
const MAX_SEARCH_RESULTS: usize = 24;
const MAX_ENRICHED_RESULTS: usize = 3;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct SearchItem {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct PackageMetadata {
    #[serde(default)]
    synopsis: Option<String>,
    #[serde(default)]
    license: Option<String>,
}

#[derive(Clone, Debug)]
struct Candidate {
    name: String,
    metadata: Option<(String, PackageMetadata)>,
}

/// True only when `query` looks like package discovery rather than Haskell
/// syntax, a module name, a type signature, or a prose/API question.
pub fn is_package_query(query: &str) -> bool {
    let query = query.trim();
    if query.is_empty()
        || query.contains("::")
        || query.contains("->")
        || query.contains(['(', ')', '[', ']', '{', '}', '.'])
        || query.split_whitespace().any(|word| {
            matches!(
                word.to_ascii_lowercase().as_str(),
                "function"
                    | "type"
                    | "class"
                    | "module"
                    | "syntax"
                    | "signature"
                    | "example"
                    | "error"
                    | "how"
                    | "why"
                    | "what"
            )
        })
    {
        return false;
    }

    let lower = query.to_ascii_lowercase();
    if ["hackage ", "haskell package ", "cabal package ", "package "]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        return package_terms(query).is_some();
    }

    // Bare discovery is deliberately limited to one lowercase package token.
    !query.contains(char::is_whitespace)
        && query
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        && valid_package_name(query)
}

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    if !is_package_query(query) {
        return Err(format!("not a Hackage package query: {query}"));
    }
    let terms = package_terms(query).ok_or_else(|| "empty Hackage package query".to_string())?;
    let normalized = normalize_query(&terms);
    let url = search_url(&normalized);
    let mut items = cached_search(client, &normalized, &url)?;
    rank_items(&mut items, &normalized);
    items.truncate(limit.min(MAX_SEARCH_RESULTS));

    if items.is_empty() {
        return Err(format!("no Hackage packages matched {terms}"));
    }

    let mut candidates: Vec<Candidate> = items
        .into_iter()
        .map(|item| Candidate {
            name: item.name,
            metadata: None,
        })
        .collect();
    for candidate in candidates.iter_mut().take(MAX_ENRICHED_RESULTS) {
        // Metadata is useful but optional. A package page remains a valid
        // result when either metadata endpoint is unavailable.
        candidate.metadata = load_latest_metadata(client, &candidate.name).ok().flatten();
    }

    let mut outcome = Outcome::new("hackage", query);
    outcome.source_url = Some(format!(
        "https://hackage.haskell.org/packages/search?terms={}",
        encode_query(&normalized)
    ));
    outcome.hits = candidates.into_iter().map(candidate_hit).collect();
    Ok(outcome)
}

fn cached_search(client: &Client, normalized: &str, url: &str) -> Result<Vec<SearchItem>, String> {
    let path = cache_path("search", normalized);
    crate::cache::get_or_fetch(
        &path,
        SEARCH_TTL,
        STALE_MAX_AGE,
        CACHE_SCHEMA,
        normalized,
        || {
            let text = client.get_text(url, APP_UA, &[("Accept", "application/json")])?;
            parse_search(&text)
        },
    )
    .map(|cached| cached.value)
    .map_err(|error| format!("Hackage search: {error}"))
}

fn load_latest_metadata(
    client: &Client,
    name: &str,
) -> Result<Option<(String, PackageMetadata)>, String> {
    if !valid_package_name(name) {
        return Err("invalid Hackage package name".into());
    }
    let versions_url = package_versions_url(name);
    let versions: HashMap<String, String> =
        cached_json(client, "versions", name, &versions_url, METADATA_TTL)?;
    let version = latest_version(&versions).ok_or_else(|| format!("no releases for {name}"))?;
    let metadata_url = version_metadata_url(name, &version);
    let metadata: PackageMetadata = cached_json(
        client,
        "metadata",
        &format!("{name}-{version}"),
        &metadata_url,
        METADATA_TTL,
    )?;
    Ok(Some((version, metadata)))
}

fn cached_json<T>(
    client: &Client,
    namespace: &str,
    key: &str,
    url: &str,
    ttl: Duration,
) -> Result<T, String>
where
    T: Serialize + serde::de::DeserializeOwned,
{
    crate::cache::get_or_fetch(
        &cache_path(namespace, key),
        ttl,
        STALE_MAX_AGE,
        CACHE_SCHEMA,
        url,
        || client.get_json(url, APP_UA, &[("Accept", "application/json")]),
    )
    .map(|cached| cached.value)
    .map_err(|error| format!("Hackage metadata: {error}"))
}

fn parse_search(text: &str) -> Result<Vec<SearchItem>, String> {
    let items: Vec<SearchItem> = serde_json::from_str(text)
        .map_err(|error| format!("invalid Hackage search JSON: {error}"))?;
    let mut result = Vec::new();
    for item in items {
        if valid_package_name(&item.name)
            && !result
                .iter()
                .any(|old: &SearchItem| old.name.eq_ignore_ascii_case(&item.name))
        {
            result.push(item);
            if result.len() == MAX_SEARCH_RESULTS {
                break;
            }
        }
    }
    Ok(result)
}

fn rank_items(items: &mut [SearchItem], query: &str) {
    items.sort_by(|a, b| {
        rank(&a.name, query)
            .cmp(&rank(&b.name, query))
            .then_with(|| a.name.len().cmp(&b.name.len()))
            .then_with(|| {
                a.name
                    .to_ascii_lowercase()
                    .cmp(&b.name.to_ascii_lowercase())
            })
    });
}

fn rank(name: &str, query: &str) -> (u8, u8) {
    let name = name.to_ascii_lowercase();
    let common = is_common_package(&name);
    let tier = if name == query {
        0
    } else if name.starts_with(&format!("{query}-")) {
        1
    } else if name.contains(query) {
        2
    } else {
        3
    };
    (tier, if common { 0 } else { 1 })
}

fn is_common_package(name: &str) -> bool {
    matches!(
        name,
        "aeson"
            | "async"
            | "base"
            | "bytestring"
            | "containers"
            | "directory"
            | "filepath"
            | "lens"
            | "mtl"
            | "network"
            | "parsec"
            | "servant"
            | "stm"
            | "text"
            | "time"
            | "transformers"
            | "unordered-containers"
            | "vector"
    )
}

fn latest_version(versions: &HashMap<String, String>) -> Option<String> {
    versions
        .iter()
        .filter(|(_, status)| !status.eq_ignore_ascii_case("deprecated"))
        .map(|(version, _)| version)
        .max_by(|a, b| compare_versions(a, b))
        .cloned()
        .or_else(|| {
            versions
                .keys()
                .max_by(|a, b| compare_versions(a, b))
                .cloned()
        })
}

fn compare_versions(a: &str, b: &str) -> Ordering {
    let parts = |value: &str| {
        value
            .split('.')
            .map(|part| part.parse::<u64>().ok())
            .collect::<Vec<_>>()
    };
    let (a_parts, b_parts) = (parts(a), parts(b));
    for index in 0..a_parts.len().max(b_parts.len()) {
        match (
            a_parts.get(index).copied().flatten(),
            b_parts.get(index).copied().flatten(),
        ) {
            (Some(left), Some(right)) if left != right => return left.cmp(&right),
            (Some(left), None) if left != 0 => return Ordering::Greater,
            (None, Some(right)) if right != 0 => return Ordering::Less,
            (None, None) => break,
            _ => {}
        }
    }
    a.cmp(b)
}

fn candidate_hit(candidate: Candidate) -> Hit {
    let mut hit = Hit::new(candidate.name.clone(), package_page_url(&candidate.name));
    if let Some((version, metadata)) = candidate.metadata {
        hit = hit
            .snippet(truncate(metadata.synopsis.as_deref().unwrap_or(""), 280))
            .meta_line(version);
        if let Some(license) = metadata.license.filter(|value| !value.trim().is_empty()) {
            hit = hit.meta_line(license);
        }
    }
    hit
}

fn package_terms(query: &str) -> Option<String> {
    let trimmed = query.trim();
    let lower = trimmed.to_ascii_lowercase();
    for prefix in ["haskell package ", "cabal package ", "hackage ", "package "] {
        if lower.starts_with(prefix) {
            let terms = trimmed[prefix.len()..].trim();
            return (!terms.is_empty()).then(|| terms.to_string());
        }
    }
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn normalize_query(query: &str) -> String {
    query
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphanumeric())
        && name
            .chars()
            .last()
            .is_some_and(|ch| ch.is_ascii_alphanumeric())
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
}

fn search_url(terms: &str) -> String {
    format!(
        "https://hackage.haskell.org/packages/search.json?terms={}",
        encode_query(terms)
    )
}

fn package_versions_url(name: &str) -> String {
    format!(
        "https://hackage.haskell.org/package/{}.json",
        encode_query(name)
    )
}

fn version_metadata_url(name: &str, version: &str) -> String {
    format!(
        "https://hackage.haskell.org/package/{}-{}.json",
        encode_query(name),
        encode_query(version)
    )
}

fn package_page_url(name: &str) -> String {
    format!("https://hackage.haskell.org/package/{}", encode_query(name))
}

fn cache_path(namespace: &str, key: &str) -> PathBuf {
    crate::providers::langs::cache_dir()
        .join("hackage-v1")
        .join(namespace)
        .join(format!("{:016x}.json", stable_hash(key.as_bytes())))
}

fn stable_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_search_fixture_and_rejects_bad_entries() {
        let items = parse_search(include_str!("fixtures/hackage-search.json")).unwrap();
        assert_eq!(
            items
                .iter()
                .map(|item| item.name.as_str())
                .collect::<Vec<_>>(),
            ["aeson-pretty", "aeson", "text", "lens-aeson"]
        );
        assert!(parse_search("{not json").is_err());
    }

    #[test]
    fn exact_then_common_names_are_ranked_first() {
        let mut items = parse_search(include_str!("fixtures/hackage-search.json")).unwrap();
        rank_items(&mut items, "aeson");
        assert_eq!(items[0].name, "aeson");
        assert_eq!(items[1].name, "aeson-pretty");

        rank_items(&mut items, "json");
        let unrelated = items
            .iter()
            .filter(|item| !item.name.contains("json"))
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        assert!(matches!(unrelated.first().copied(), Some("aeson" | "text")));
    }

    #[test]
    fn intent_gate_is_conservative() {
        for accepted in [
            "aeson",
            "unordered-containers",
            "hackage aeson",
            "haskell package text",
        ] {
            assert!(is_package_query(accepted), "{accepted}");
        }
        for rejected in [
            "Data.Text.pack",
            "mapMaybe signature",
            "a -> b",
            "how to parse json",
            "map (f x)",
        ] {
            assert!(!is_package_query(rejected), "{rejected}");
        }
    }

    #[test]
    fn resolves_latest_normal_metadata_url() {
        let versions: HashMap<String, String> =
            serde_json::from_str(include_str!("fixtures/hackage-versions.json")).unwrap();
        let latest = latest_version(&versions).unwrap();
        assert_eq!(latest, "2.2.3.0");
        assert_eq!(
            version_metadata_url("aeson", &latest),
            "https://hackage.haskell.org/package/aeson-2.2.3.0.json"
        );
        assert_eq!(
            package_page_url("aeson"),
            "https://hackage.haskell.org/package/aeson"
        );
    }

    #[test]
    fn metadata_fixture_builds_hit() {
        let metadata: PackageMetadata =
            serde_json::from_str(include_str!("fixtures/hackage-metadata.json")).unwrap();
        let hit = candidate_hit(Candidate {
            name: "aeson".into(),
            metadata: Some(("2.2.3.0".into(), metadata)),
        });
        assert_eq!(hit.snippet, "Fast JSON parsing and encoding");
        assert_eq!(hit.meta, ["2.2.3.0", "BSD-3-Clause"]);
    }

    #[test]
    fn malformed_metadata_and_empty_versions_fail_cleanly() {
        assert!(serde_json::from_str::<PackageMetadata>(r#""bad metadata""#).is_err());
        assert_eq!(latest_version(&HashMap::new()), None);
        assert!(!valid_package_name("../aeson"));
    }
}
