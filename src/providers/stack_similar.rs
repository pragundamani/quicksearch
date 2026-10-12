// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Stack Overflow title matches for the low-confidence language-search path.
//!
//! This is deliberately not a general Stack Overflow provider. Callers should
//! use it only after local language documentation has produced a weak result.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape};

const API_ROOT: &str = "https://api.stackexchange.com/2.3/similar";
const CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const CACHE_SCHEMA: u32 = 1;
const MAX_PAGE_SIZE: usize = 100;
const QUOTA_BACKOFF: u64 = 24 * 60 * 60;
static BACKOFF_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// A normal result or a temporary, nonfatal API refusal.
#[derive(Debug)]
pub(crate) enum LowConfidenceResult {
    Found(Outcome),
    Unavailable(Unavailable),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Unavailable {
    pub reason: String,
    pub retry_after: Option<u64>,
}

/// Search by similar question title, constrained to one Stack Overflow tag.
///
/// The caller is expected to invoke this only as a low-confidence fallback.
pub(crate) fn search_low_confidence(
    client: &Client,
    query: &str,
    language_tag: &str,
    limit: usize,
) -> Result<LowConfidenceResult, String> {
    let title = normalize_display_query(query);
    let tag = normalize_tag(language_tag);
    if title.is_empty() || tag.is_empty() || limit == 0 {
        return Ok(LowConfidenceResult::Found(empty_outcome(
            query, &title, &tag,
        )));
    }

    if let Some(unavailable) = active_backoff() {
        return Ok(LowConfidenceResult::Unavailable(unavailable));
    }

    let page_size = limit.min(MAX_PAGE_SIZE);
    let key = cache_key(&title, &tag, page_size);
    let path = cache_path(&key);
    let url = build_url(&title, &tag, page_size);
    let cached =
        crate::cache::get_or_fetch(&path, CACHE_TTL, CACHE_TTL, CACHE_SCHEMA, &key, || {
            let response: SoResponse = client
                .get_json_gzip(
                    &url,
                    APP_UA,
                    &[("Accept-Encoding", "gzip"), ("Accept", "application/json")],
                )
                .map_err(FetchFailure::Http)?;
            classify_response(&response).map_err(FetchFailure::Unavailable)?;
            Ok(response)
        });

    let response = match cached {
        Ok(cached) => cached.value,
        Err(crate::cache::CacheError::Fetch(FetchFailure::Unavailable(reason))) => {
            record_backoff(&reason);
            return Ok(LowConfidenceResult::Unavailable(reason));
        }
        Err(error) => return Err(error.to_string()),
    };
    // Cached records are validated before being written. Keep this check so a
    // future schema change cannot accidentally turn backoff into a success.
    if let Err(reason) = classify_response(&response) {
        record_backoff(&reason);
        return Ok(LowConfidenceResult::Unavailable(reason));
    }
    Ok(LowConfidenceResult::Found(outcome_from_response(
        query, &title, &tag, limit, response,
    )))
}

fn empty_outcome(original_query: &str, title: &str, tag: &str) -> Outcome {
    let mut outcome = Outcome::new("stackoverflow-similar", original_query);
    outcome.source_url = Some(stack_search_url(title, tag));
    outcome
}

fn outcome_from_response(
    original_query: &str,
    title: &str,
    tag: &str,
    limit: usize,
    response: SoResponse,
) -> Outcome {
    let mut items: Vec<_> = response
        .items
        .into_iter()
        .filter(|item| item.closed_date.is_none() && item.score >= 0)
        .collect();
    // `/similar` already returns relevance order. A stable sort only moves
    // unanswered questions behind answered questions.
    items.sort_by_key(|item| !is_answered(item));

    let mut outcome = empty_outcome(original_query, title, tag);
    outcome.hits = items
        .into_iter()
        .take(limit)
        .map(|item| {
            let answer_state = if item.accepted_answer_id.is_some() {
                "accepted answer"
            } else if is_answered(&item) {
                "answered"
            } else {
                "unanswered"
            };
            let tags = item
                .tags
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            let byline = item
                .owner
                .and_then(|owner| owner.display_name)
                .map(|name| format!("  ·  by {}", html_unescape(&name)))
                .unwrap_or_default();
            let tag_meta = if tags.is_empty() {
                String::new()
            } else {
                format!("  ·  {tags}")
            };
            Hit::new(html_unescape(&item.title), item.link)
                .meta_line(format!(
                    "score {}  ·  {} answers  ·  {answer_state}{tag_meta}",
                    item.score, item.answer_count
                ))
                .meta_line(format!("Source: Stack Overflow{byline}  ·  CC BY-SA 4.0"))
        })
        .collect();
    outcome
}

fn is_answered(item: &SoItem) -> bool {
    item.accepted_answer_id.is_some() || item.is_answered || item.answer_count > 0
}

fn classify_response(response: &SoResponse) -> Result<(), Unavailable> {
    if let Some(seconds) = response.backoff.filter(|seconds| *seconds > 0) {
        return Err(Unavailable {
            reason: format!("Stack Exchange requested a {seconds}s backoff"),
            retry_after: Some(seconds),
        });
    }
    if response.quota_remaining == Some(0) {
        return Err(Unavailable {
            reason: "Stack Exchange quota exhausted".into(),
            retry_after: None,
        });
    }
    if let Some(message) = response.error_message.as_deref().filter(|s| !s.is_empty()) {
        return Err(Unavailable {
            reason: format!("Stack Exchange API unavailable: {message}"),
            retry_after: None,
        });
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct BackoffState {
    until: u64,
    reason: String,
}

fn active_backoff() -> Option<Unavailable> {
    let _guard = BACKOFF_LOCK.lock().unwrap_or_else(|lock| lock.into_inner());
    let path = backoff_path();
    crate::cache::with_exclusive_lock(&path, || {
        let Some(state) = read_backoff(&path) else {
            return Ok(None);
        };
        let Some(now) = unix_now() else {
            return Ok(None);
        };
        if state.until <= now {
            let _ = crate::cache::remove_cache_file(&path);
            return Ok(None);
        }
        Ok(Some(Unavailable {
            reason: state.reason,
            retry_after: Some(state.until - now),
        }))
    })
    .ok()
    .flatten()
}

fn record_backoff(unavailable: &Unavailable) {
    let Some(now) = unix_now() else {
        return;
    };
    let delay = unavailable.retry_after.unwrap_or(QUOTA_BACKOFF).max(1);
    let proposed = BackoffState {
        until: now.saturating_add(delay),
        reason: unavailable.reason.clone(),
    };
    let _guard = BACKOFF_LOCK.lock().unwrap_or_else(|lock| lock.into_inner());
    let path = backoff_path();
    let _ = crate::cache::with_exclusive_lock(&path, || {
        let state = merge_backoff(read_backoff(&path), proposed);
        crate::cache::write_json_atomic(&path, &state)
    });
}

fn read_backoff(path: &std::path::Path) -> Option<BackoffState> {
    let text = crate::cache::read_cache_text(path, 16 * 1024).ok()??;
    serde_json::from_str(&text).ok()
}

fn merge_backoff(current: Option<BackoffState>, proposed: BackoffState) -> BackoffState {
    match current {
        Some(current) if current.until >= proposed.until => current,
        _ => proposed,
    }
}

fn unix_now() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|time| time.as_secs())
}

fn backoff_path() -> PathBuf {
    super::langs::cache_dir()
        .join("stackoverflow-similar-v1")
        .join("backoff.json")
}

fn build_url(title: &str, tag: &str, page_size: usize) -> String {
    format!(
        "{API_ROOT}?site=stackoverflow&order=desc&sort=relevance&pagesize={}&tagged={}&title={}",
        page_size.min(MAX_PAGE_SIZE),
        encode_query(tag),
        encode_query(title)
    )
}

fn stack_search_url(title: &str, tag: &str) -> String {
    let query = if tag.is_empty() {
        title.to_owned()
    } else {
        format!("[{tag}] {title}")
    };
    format!(
        "https://stackoverflow.com/search?q={}",
        encode_query(&query)
    )
}

fn normalize_display_query(query: &str) -> String {
    collapse_ws(query)
}

fn normalize_tag(tag: &str) -> String {
    collapse_ws(tag).to_lowercase()
}

fn cache_key(query: &str, tag: &str, page_size: usize) -> String {
    format!(
        "stackoverflow-similar-v{CACHE_SCHEMA}|{}|{}|{}",
        normalize_display_query(query).to_lowercase(),
        normalize_tag(tag),
        page_size.min(MAX_PAGE_SIZE)
    )
}

fn cache_path(key: &str) -> PathBuf {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    super::langs::cache_dir()
        .join("stackoverflow-similar-v1")
        .join(format!("{:016x}.json", hasher.finish()))
}

#[derive(Debug)]
enum FetchFailure {
    Http(String),
    Unavailable(Unavailable),
}

impl fmt::Display for FetchFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => formatter.write_str(error),
            Self::Unavailable(reason) => formatter.write_str(&reason.reason),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SoResponse {
    #[serde(default)]
    items: Vec<SoItem>,
    #[serde(default)]
    backoff: Option<u64>,
    #[serde(default)]
    quota_remaining: Option<u64>,
    #[serde(default)]
    error_message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SoItem {
    title: String,
    link: String,
    #[serde(default)]
    score: i64,
    #[serde(default)]
    answer_count: i64,
    #[serde(default)]
    is_answered: bool,
    #[serde(default)]
    accepted_answer_id: Option<u64>,
    #[serde(default)]
    closed_date: Option<u64>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    owner: Option<SoOwner>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SoOwner {
    #[serde(default)]
    display_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> SoResponse {
        let json = match name {
            "ranking" => include_str!("fixtures/stack_similar_ranking.json"),
            "backoff" => include_str!("fixtures/stack_similar_backoff.json"),
            "quota" => include_str!("fixtures/stack_similar_quota.json"),
            _ => panic!("unknown fixture"),
        };
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn shorter_backoff_cannot_shorten_existing_deadline() {
        let current = BackoffState {
            until: 10_000,
            reason: "quota exhausted".into(),
        };
        let proposed = BackoffState {
            until: 10,
            reason: "one second".into(),
        };
        let merged = merge_backoff(Some(current), proposed);
        assert_eq!(merged.until, 10_000);
        assert_eq!(merged.reason, "quota exhausted");
    }

    #[test]
    fn fixture_filters_bad_items_and_demotes_unanswered() {
        let outcome = outcome_from_response(
            "borrow value",
            "borrow value",
            "rust",
            10,
            fixture("ranking"),
        );
        let titles: Vec<_> = outcome.hits.iter().map(|hit| hit.title.as_str()).collect();
        assert_eq!(titles, ["Answered later", "Accepted", "Unanswered first"]);
        assert!(outcome.hits.iter().all(|hit| {
            hit.meta.iter().any(|line| line.contains("CC BY-SA 4.0"))
                && hit
                    .meta
                    .iter()
                    .any(|line| line.contains("Source: Stack Overflow"))
        }));
    }

    #[test]
    fn fixture_backoff_and_quota_are_nonfatal_unavailability() {
        let backoff = classify_response(&fixture("backoff")).unwrap_err();
        assert_eq!(backoff.retry_after, Some(17));
        assert!(backoff.reason.contains("backoff"));

        let quota = classify_response(&fixture("quota")).unwrap_err();
        assert_eq!(quota.retry_after, None);
        assert!(quota.reason.contains("quota exhausted"));
    }

    #[test]
    fn cache_key_normalizes_case_and_whitespace() {
        assert_eq!(
            cache_key("  Borrow   VALUE ", " Rust ", 8),
            cache_key("borrow value", "rust", 8)
        );
        assert_ne!(
            cache_key("borrow value", "rust", 8),
            cache_key("borrow value", "rust", 9)
        );
    }

    #[test]
    fn similar_url_has_language_relevance_and_bounded_page_size() {
        assert_eq!(
            build_url("vector of refs", "c++", 500),
            "https://api.stackexchange.com/2.3/similar?site=stackoverflow&order=desc&sort=relevance&pagesize=100&tagged=c%2B%2B&title=vector%20of%20refs"
        );
    }
}
