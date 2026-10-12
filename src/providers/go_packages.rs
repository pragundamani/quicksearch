// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Package discovery through pkg.go.dev's documented v1 search API.
//!
//! This provider is intentionally limited to package and library discovery.
//! Language syntax and known standard-library identifiers belong to the Go
//! documentation provider instead.

use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize};

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, truncate};

const API_ROOT: &str = "https://pkg.go.dev/v1/search";
const CACHE_TTL: Duration = Duration::from_secs(60 * 60);
const CACHE_SCHEMA: u32 = 1;
/// Keep interactive searches cheap and avoid using the API as a crawler.
const MAX_RESULTS: usize = 10;

/// Return whether `query` has package/library-discovery intent.
///
/// This function is pure so routing can use it without doing I/O. Exact known
/// standard-library paths and Go syntax questions are deliberately rejected.
pub fn is_package_intent(query: &str) -> bool {
    let query = normalize_query(query);
    if query.is_empty() {
        return false;
    }
    let lower = query.to_ascii_lowercase();
    if lower.split_whitespace().count() == 1
        && lower
            .rsplit_once('/')
            .is_some_and(|(_, tail)| tail.contains('.'))
    {
        return false;
    }
    if is_stdlib_path(&lower) || is_syntax_query(&lower) {
        return false;
    }

    let words: Vec<_> = lower.split_whitespace().collect();
    if words.len() == 1 {
        return is_package_token(words[0]);
    }

    // A path can contain spaces only in a malformed query. For prose, require
    // an explicit discovery noun to avoid stealing general Go documentation.
    words.iter().any(|word| {
        matches!(
            word.trim_matches(|c: char| !c.is_ascii_alphanumeric()),
            "package" | "packages" | "library" | "libraries" | "module" | "modules" | "sdk"
        )
    })
}

/// Search the official pkg.go.dev package index.
pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    if !is_package_intent(query) {
        return Err(format!(
            "not a Go package discovery query: {}",
            collapse_ws(query)
        ));
    }
    search_any(client, query, limit)
}

pub(crate) fn search_any(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let display_query = collapse_ws(query);
    let page_size = limit.min(MAX_RESULTS);
    let mut outcome = empty_outcome(query, &display_query);
    if page_size == 0 {
        return Ok(outcome);
    }

    let key = cache_key(&display_query, page_size);
    let path = cache_path(&key);
    let url = api_url(&display_query, page_size);
    let response =
        crate::cache::get_or_fetch(&path, CACHE_TTL, CACHE_TTL, CACHE_SCHEMA, &key, || {
            client.get_json::<SearchResponse>(&url, APP_UA, &[("Accept", "application/json")])
        })
        .map_err(|error| error.to_string())?
        .value;

    outcome.hits = hits_from_response(&display_query, page_size, response);
    if outcome.hits.is_empty() {
        return Err(format!("no Go packages matched {display_query}"));
    }
    Ok(outcome)
}

fn empty_outcome(original_query: &str, display_query: &str) -> Outcome {
    let mut outcome = Outcome::new("pkg.go.dev", original_query);
    outcome.source_url = Some(format!(
        "https://pkg.go.dev/search?q={}",
        encode_query(display_query)
    ));
    outcome
}

fn hits_from_response(query: &str, limit: usize, response: SearchResponse) -> Vec<Hit> {
    let target = package_target(query);
    let mut indexed: Vec<_> = response
        .items
        .into_iter()
        .enumerate()
        .filter(|(_, item)| !item.package_path.trim().is_empty())
        .collect();
    indexed.sort_by_key(|(position, item)| (rank(item, &target), *position));
    indexed
        .into_iter()
        .take(limit.min(MAX_RESULTS))
        .map(|(_, item)| package_hit(item))
        .collect()
}

fn package_hit(item: PackageItem) -> Hit {
    let package_path = item.package_path.trim().to_owned();
    let mut details = Vec::new();
    if !item.version.trim().is_empty() {
        details.push(item.version.trim().to_owned());
    }
    if !item.module_path.trim().is_empty() && item.module_path.trim() != package_path {
        details.push(format!("module {}", item.module_path.trim()));
    }
    Hit::new(package_path.clone(), package_url(&package_path))
        .snippet(truncate(&item.synopsis, 280))
        .meta_line(details.join("  ·  "))
}

fn rank(item: &PackageItem, target: &str) -> u8 {
    let package = item.package_path.trim().to_ascii_lowercase();
    let module = item.module_path.trim().to_ascii_lowercase();
    if package == target {
        return 0;
    }
    if !module.is_empty() && module == target {
        return 1;
    }

    let basename_match = package.rsplit('/').next() == Some(target);
    let relevant = basename_match
        || (!target.is_empty() && package.contains(target))
        || target
            .split(|c: char| c.is_whitespace() || c == '/')
            .filter(|part| !part.is_empty())
            .any(|part| package.split('/').any(|segment| segment == part));
    if relevant {
        if is_stdlib_path(&package) {
            2
        } else if package.starts_with("golang.org/x/") {
            3
        } else {
            4
        }
    } else {
        5
    }
}

fn package_target(query: &str) -> String {
    let lower = normalize_query(query).to_ascii_lowercase();
    let mut words: Vec<&str> = lower.split_whitespace().collect();
    words.retain(|word| {
        !matches!(
            *word,
            "go" | "golang"
                | "package"
                | "packages"
                | "library"
                | "libraries"
                | "module"
                | "modules"
                | "sdk"
        )
    });
    words.join(" ")
}

fn api_url(query: &str, limit: usize) -> String {
    format!(
        "{API_ROOT}?q={}&limit={}",
        encode_query(query),
        limit.min(MAX_RESULTS)
    )
}

fn package_url(package_path: &str) -> String {
    let path = package_path
        .split('/')
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect::<Vec<_>>()
        .join("/");
    format!("https://pkg.go.dev/{path}")
}

fn normalize_query(query: &str) -> String {
    collapse_ws(query)
}

fn cache_key(query: &str, limit: usize) -> String {
    format!(
        "pkg-go-dev-search-v{CACHE_SCHEMA}|{}|{}",
        normalize_query(query).to_ascii_lowercase(),
        limit.min(MAX_RESULTS)
    )
}

fn cache_path(key: &str) -> PathBuf {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    super::langs::cache_dir()
        .join("pkg-go-dev-search-v1")
        .join(format!("{:016x}.json", hasher.finish()))
}

fn is_package_token(token: &str) -> bool {
    !token.is_empty()
        && !token.contains(['(', ')', '{', '}', ';', '=', ':'])
        && token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '/' | '_' | '-'))
}

fn is_syntax_query(query: &str) -> bool {
    if query.contains(":=") || query.contains("=>") || query.contains(['(', ')', '{', '}', ';']) {
        return true;
    }
    let first = query.split_whitespace().next().unwrap_or_default();
    matches!(
        first,
        "break"
            | "case"
            | "chan"
            | "const"
            | "continue"
            | "defer"
            | "else"
            | "fallthrough"
            | "for"
            | "func"
            | "goto"
            | "if"
            | "interface"
            | "map"
            | "range"
            | "return"
            | "select"
            | "struct"
            | "switch"
            | "type"
            | "var"
    )
}

fn is_stdlib_path(path: &str) -> bool {
    const STDLIB: &[&str] = &[
        "archive/tar",
        "archive/zip",
        "bufio",
        "bytes",
        "cmp",
        "compress/bzip2",
        "compress/flate",
        "compress/gzip",
        "compress/lzw",
        "compress/zlib",
        "container/heap",
        "container/list",
        "container/ring",
        "context",
        "crypto",
        "database/sql",
        "embed",
        "encoding",
        "encoding/base64",
        "encoding/binary",
        "encoding/csv",
        "encoding/gob",
        "encoding/hex",
        "encoding/json",
        "encoding/xml",
        "errors",
        "expvar",
        "flag",
        "fmt",
        "hash",
        "html",
        "html/template",
        "image",
        "io",
        "io/fs",
        "log",
        "log/slog",
        "maps",
        "math",
        "math/big",
        "mime",
        "net",
        "net/http",
        "net/url",
        "os",
        "os/exec",
        "path",
        "path/filepath",
        "reflect",
        "regexp",
        "runtime",
        "slices",
        "sort",
        "strconv",
        "strings",
        "sync",
        "sync/atomic",
        "syscall",
        "testing",
        "text/template",
        "time",
        "unicode",
        "unicode/utf8",
        "unsafe",
    ];
    STDLIB.contains(&path)
}

fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    #[serde(default, deserialize_with = "null_as_default")]
    items: Vec<PackageItem>,
    #[serde(default)]
    total: u64,
    #[serde(default)]
    next_page_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PackageItem {
    package_path: String,
    #[serde(default)]
    module_path: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    synopsis: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> SearchResponse {
        serde_json::from_str(include_str!("fixtures/go_packages_search.json")).unwrap()
    }

    #[test]
    fn parses_documented_search_fields() {
        let response = fixture();
        assert_eq!(response.total, 4);
        assert_eq!(response.next_page_token, "page-two");
        assert_eq!(response.items[0].package_path, "github.com/acme/errgroup");
        assert_eq!(response.items[1].module_path, "golang.org/x/sync");
        assert_eq!(response.items[1].version, "v0.12.0");
    }

    #[test]
    fn null_items_are_a_valid_empty_search_response() {
        let response: SearchResponse = serde_json::from_str(r#"{"items":null,"total":0}"#).unwrap();
        assert!(response.items.is_empty());
        assert_eq!(response.total, 0);
    }

    #[test]
    fn exact_then_first_party_ranking_is_stable() {
        let hits = hits_from_response("errgroup package", 10, fixture());
        let paths: Vec<_> = hits.iter().map(|hit| hit.title.as_str()).collect();
        assert_eq!(
            paths,
            [
                "errgroup",
                "golang.org/x/sync/errgroup",
                "github.com/acme/errgroup",
                "example.com/unrelated"
            ]
        );
    }

    #[test]
    fn intent_gate_rejects_stdlib_and_syntax() {
        assert!(!is_package_intent("fmt"));
        assert!(!is_package_intent("net/http"));
        assert!(!is_package_intent("for range slice"));
        assert!(!is_package_intent("func Parse(s string)"));
        assert!(!is_package_intent("go channels"));
        assert!(!is_package_intent("crypto/rand.Read"));
        assert!(!is_package_intent("crypto/ecdh.X25519"));
        assert!(is_package_intent("cobra"));
        assert!(is_package_intent("github.com/gorilla/mux"));
        assert!(is_package_intent("http client package"));
        assert!(is_package_intent("go package github.com/gorilla/mux"));
    }

    #[test]
    fn cache_key_normalizes_query_and_caps_limit() {
        assert_eq!(
            cache_key("  Gorilla/MUX  ", 99),
            cache_key("gorilla/mux", MAX_RESULTS)
        );
        assert_ne!(cache_key("gorilla/mux", 3), cache_key("gorilla/mux", 4));
    }

    #[test]
    fn builds_api_and_package_urls() {
        assert_eq!(
            api_url("gorilla mux", 99),
            "https://pkg.go.dev/v1/search?q=gorilla%20mux&limit=10"
        );
        assert_eq!(
            package_url("github.com/gorilla/mux"),
            "https://pkg.go.dev/github.com/gorilla/mux"
        );
    }
}
