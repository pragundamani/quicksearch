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

use crate::http::Client;
use crate::model::{Hit, Outcome};
use crate::providers::crates::{docs_hit, load_crate, search as search_crates, CrateInfo};
use crate::providers::rustdoc::{self, normalize_std_path};
use crate::text::encode_query;

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let (first_name, first_rest) = split_crate_query(query);
    if is_std_crate(&first_name) {
        let path = match &first_rest {
            Some(rest) => format!("{first_name}::{rest}"),
            None => first_name.clone(),
        };
        return rustdoc::search_std(client, &normalize_std_path(&path), limit);
    }

    for name in crate_candidates(query).into_iter().take(2) {
        if let Some(info) = load_crate(client, &name)? {
            let rest = query_without_crate(query, &name);
            return Ok(exact_docs(client, query, info, rest.as_deref(), limit));
        }
    }

    let mut outcome = search_crates(client, query, limit)?;
    outcome.provider = "docs".to_string();
    outcome.summary_title = Some(format!("No crate named {first_name}"));
    outcome.summary = Some("Closest crates on crates.io.".to_string());
    Ok(outcome)
}

fn exact_docs(
    client: &Client,
    query: &str,
    info: CrateInfo,
    rest: Option<&str>,
    limit: usize,
) -> Outcome {
    let mut outcome = Outcome::new("docs", query);
    let lib = info.lib_name();
    let root = format!("https://docs.rs/{}/latest/{}", info.name, lib);
    outcome.source_url = Some(root.clone());

    if let Some(rest) = rest.filter(|rest| !rest.is_empty()) {
        let direct_crate_path = query
            .trim()
            .strip_prefix(&format!("{}::", info.name))
            .is_some();
        let exact_path = rest
            .split_whitespace()
            .next()
            .filter(|path| path.contains("::") || direct_crate_path);
        let exact_page = exact_path.and_then(|path| rustdoc::resolve(client, &root, path));
        if let Some(page) = exact_page {
            outcome.hits.push(
                Hit::new(page.title, page.url).snippet(if page.snippet.is_empty() {
                    info.description.clone().unwrap_or_default()
                } else {
                    page.snippet
                }),
            );
        } else if let Some(symbol) = derive_symbol(rest) {
            let url = format!("{root}/derive.{symbol}.html");
            if client.get_text(&url, crate::http::APP_UA, &[]).is_ok() {
                outcome.hits.push(
                    Hit::new(format!("#[derive({symbol})]"), url)
                        .snippet(format!("Derive macro reexported by {}.", info.name)),
                );
            }
            if outcome.hits.is_empty() {
                let search = format!("{root}/?search={}", encode_query(rest));
                outcome
                    .hits
                    .push(Hit::new(format!("{} search: {rest}", info.name), search));
            }
        } else {
            let words: Vec<&str> = rest
                .split_whitespace()
                .map(|word| word.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_'))
                .filter(|word| !word.is_empty())
                .collect();
            if let Some(owner) = words.first().filter(|word| {
                word.chars()
                    .next()
                    .is_some_and(|ch| ch.is_ascii_uppercase())
            }) {
                for member in words.iter().skip(1).take(4).filter(|word| {
                    word.chars()
                        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
                        && !matches!(**word, "and" | "or" | "method" | "methods")
                }) {
                    if let Some(page) =
                        rustdoc::resolve(client, &root, &format!("{owner}::{member}"))
                    {
                        if !outcome.hits.iter().any(|hit| hit.url == page.url) {
                            outcome
                                .hits
                                .push(Hit::new(page.title, page.url).snippet(page.snippet));
                        }
                    }
                    if outcome.hits.len() >= limit.saturating_sub(1).max(1) {
                        break;
                    }
                }
            }
            if outcome.hits.is_empty() {
                let search = format!("{root}/?search={}", encode_query(rest));
                outcome.hits.push(
                    Hit::new(format!("{} search: {rest}", info.name), search)
                        .snippet("No exact page matched. This opens the crate search."),
                );
            }
        }
    }

    outcome.hits.push(docs_hit(info));
    outcome.hits.truncate(limit);
    outcome
}

fn derive_symbol(rest: &str) -> Option<&str> {
    let words = rest
        .split_whitespace()
        .map(|word| word.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_'))
        .collect::<Vec<_>>();
    words
        .iter()
        .any(|word| word.eq_ignore_ascii_case("derive"))
        .then(|| {
            words
                .iter()
                .copied()
                .find(|word| word.chars().next().is_some_and(char::is_uppercase))
        })
        .flatten()
}

fn crate_candidates(query: &str) -> Vec<String> {
    let direct = query
        .trim()
        .split_once("::")
        .map(|(name, _)| name.trim().to_string());
    if let Some(name) = direct {
        return vec![name];
    }
    let words: Vec<&str> = query.split_whitespace().collect();
    if words.len() == 1 {
        return words.into_iter().map(str::to_string).collect();
    }
    let is_natural_word = |word: &str| {
        matches!(
            word.to_ascii_lowercase().as_str(),
            "a" | "an"
                | "and"
                | "argument"
                | "arguments"
                | "async"
                | "asynchronous"
                | "client"
                | "command"
                | "create"
                | "file"
                | "for"
                | "from"
                | "http"
                | "in"
                | "into"
                | "json"
                | "line"
                | "not"
                | "of"
                | "parse"
                | "parser"
                | "read"
                | "response"
                | "rust"
                | "struct"
                | "the"
                | "to"
                | "use"
                | "with"
                | "write"
        )
    };
    let valid = |word: &str| {
        !word.is_empty()
            && !word.chars().any(char::is_uppercase)
            && word
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
            && !is_natural_word(word)
    };
    let mut candidates = Vec::new();
    for word in [words[0], words[words.len() - 1]]
        .into_iter()
        .chain(words.iter().rev().copied())
    {
        let word =
            word.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_');
        if valid(word) && !candidates.iter().any(|candidate| candidate == word) {
            candidates.push(word.to_string());
        }
    }
    candidates
}

fn query_without_crate(query: &str, name: &str) -> Option<String> {
    if let Some(rest) = query.trim().strip_prefix(&format!("{name}::")) {
        return (!rest.is_empty()).then(|| rest.to_string());
    }
    let mut removed = false;
    let rest = query
        .split_whitespace()
        .filter(|word| {
            if !removed
                && word
                    .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_')
                    == name
            {
                removed = true;
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    (!rest.is_empty()).then_some(rest)
}

pub fn split_crate_query(query: &str) -> (String, Option<String>) {
    let query = query.trim();
    if let Some((name, rest)) = query.split_once("::") {
        let rest = rest.trim();
        return (
            name.trim().to_string(),
            if rest.is_empty() {
                None
            } else {
                Some(rest.to_string())
            },
        );
    }
    if let Some((name, rest)) = query.split_once(char::is_whitespace) {
        return (name.to_string(), Some(rest.trim().to_string()));
    }
    (query.to_string(), None)
}

fn is_std_crate(name: &str) -> bool {
    matches!(name, "std" | "core" | "alloc" | "proc_macro" | "test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_paths_and_words() {
        assert_eq!(
            split_crate_query("tokio::task::spawn"),
            ("tokio".into(), Some("task::spawn".into()))
        );
        assert_eq!(
            split_crate_query("serde Serialize"),
            ("serde".into(), Some("Serialize".into()))
        );
        assert_eq!(split_crate_query("serde"), ("serde".into(), None));
    }
}
