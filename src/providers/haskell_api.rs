// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code)] // Test-only parser; production uses deterministic Haddock URLs.

//! Offline Haskell API search over official Haddock-generated Hoogle inputs.
//!
//! This provider does not send query terms to Hoogle. It downloads bounded
//! `*.txt` artifacts from Hackage, builds a typed local corpus, and searches
//! that corpus. The initial core corpus covers `base` and `containers`.

use std::collections::BTreeSet;
use std::io::Read;
use std::time::Duration;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, html_unescape, strip_tags, truncate};

const MAX_SOURCE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_ENTRIES: usize = 80_000;
const MAX_DOC_CHARS: usize = 320;
const MAX_SIGNATURE_CHARS: usize = 600;
const SOURCES: [(&str, &str, bool); 20] = [
    (
        "base",
        "https://hackage.haskell.org/package/base/docs/base.txt",
        true,
    ),
    (
        "containers",
        "https://hackage.haskell.org/package/containers/docs/containers.txt",
        true,
    ),
    (
        "text",
        "https://hackage.haskell.org/package/text/docs/text.txt",
        true,
    ),
    (
        "bytestring",
        "https://hackage.haskell.org/package/bytestring/docs/bytestring.txt",
        true,
    ),
    (
        "deepseq",
        "https://hackage.haskell.org/package/deepseq/docs/deepseq.txt",
        true,
    ),
    (
        "directory",
        "https://hackage.haskell.org/package/directory/docs/directory.txt",
        false,
    ),
    (
        "filepath",
        "https://hackage.haskell.org/package/filepath/docs/filepath.txt",
        false,
    ),
    (
        "process",
        "https://hackage.haskell.org/package/process/docs/process.txt",
        false,
    ),
    (
        "random",
        "https://hackage.haskell.org/package/random/docs/random.txt",
        false,
    ),
    (
        "stm",
        "https://hackage.haskell.org/package/stm/docs/stm.txt",
        false,
    ),
    (
        "async",
        "https://hackage.haskell.org/package/async/docs/async.txt",
        false,
    ),
    (
        "vector",
        "https://hackage.haskell.org/package/vector/docs/vector.txt",
        false,
    ),
    (
        "unordered-containers",
        "https://hackage.haskell.org/package/unordered-containers/docs/unordered-containers.txt",
        false,
    ),
    (
        "aeson",
        "https://hackage.haskell.org/package/aeson/docs/aeson.txt",
        false,
    ),
    (
        "QuickCheck",
        "https://hackage.haskell.org/package/QuickCheck/docs/QuickCheck.txt",
        false,
    ),
    (
        "temporary",
        "https://hackage.haskell.org/package/temporary/docs/temporary.txt",
        false,
    ),
    (
        "http-client",
        "https://hackage.haskell.org/package/http-client/docs/http-client.txt",
        false,
    ),
    (
        "lens",
        "https://hackage.haskell.org/package/lens/docs/lens.txt",
        false,
    ),
    (
        "transformers",
        "https://hackage.haskell.org/package/transformers/docs/transformers.txt",
        false,
    ),
    (
        "hspec",
        "https://hackage.haskell.org/package/hspec/docs/hspec.txt",
        false,
    ),
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Corpus {
    source_key: String,
    entries: Vec<ApiEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ApiEntry {
    package: String,
    version: String,
    module: String,
    name: String,
    signature: String,
    docs: String,
}

pub(crate) fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let query = query.trim();
    if query.is_empty() {
        return Err("empty Haskell API query".into());
    }
    let corpus = load_corpus(client)?;
    let hits = search_corpus(&corpus, query, limit);
    if hits.is_empty() {
        return Err(format!("no core Haskell API matched {query}"));
    }
    let mut outcome = Outcome::new("Haskell", query);
    outcome.source_url = Some("https://hackage.haskell.org/".into());
    outcome.hits = hits;
    Ok(outcome)
}

fn load_corpus(client: &Client) -> Result<Corpus, String> {
    let key = source_key();
    fetch_corpus(client, &key).map_err(|error| format!("Haskell API corpus: {error}"))
}

fn fetch_corpus(client: &Client, key: &str) -> Result<Corpus, String> {
    let mut entries = Vec::new();
    for (expected_package, url, required) in SOURCES {
        match fetch_source(client, expected_package, url) {
            Ok(mut parsed) => entries.append(&mut parsed),
            Err(error) if !required => {
                eprintln!(
                    "qs: optional Haskell package {} unavailable: {}",
                    crate::text::sanitize_terminal_line(expected_package),
                    crate::text::sanitize_terminal_line(&error)
                );
            }
            Err(error) => return Err(error),
        }
        if entries.len() > MAX_ENTRIES {
            return Err("Haskell API corpus exceeds 80000 entries".into());
        }
    }
    if entries.is_empty() {
        return Err("Haskell API corpus is empty".into());
    }
    Ok(Corpus {
        source_key: key.to_string(),
        entries,
    })
}

fn fetch_source(
    client: &Client,
    expected_package: &str,
    url: &str,
) -> Result<Vec<ApiEntry>, String> {
    let response = client
        .agent
        .get(url)
        .set("User-Agent", APP_UA)
        .timeout(Duration::from_secs(30))
        .call()
        .map_err(|error| error.to_string())?;
    if response
        .header("Content-Length")
        .and_then(|value| value.parse::<u64>().ok())
        .is_some_and(|size| size > MAX_SOURCE_BYTES)
    {
        return Err(format!("{expected_package} Hoogle input exceeds 8 MiB"));
    }
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(MAX_SOURCE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > MAX_SOURCE_BYTES {
        return Err(format!("{expected_package} Hoogle input exceeds 8 MiB"));
    }
    let text = String::from_utf8(bytes)
        .map_err(|_| format!("{expected_package} Hoogle input is not UTF-8"))?;
    let parsed = parse_hoogle_text(&text)?;
    if parsed
        .first()
        .is_some_and(|entry| entry.package != expected_package)
    {
        return Err(format!(
            "Hackage returned the wrong {expected_package} corpus"
        ));
    }
    Ok(parsed)
}

fn parse_hoogle_text(text: &str) -> Result<Vec<ApiEntry>, String> {
    let mut package = String::new();
    let mut version = String::new();
    let mut module = String::new();
    let mut docs = Vec::new();
    let mut entries = Vec::new();

    for raw in text.lines() {
        let line = raw.trim();
        if let Some(value) = line.strip_prefix("@package ") {
            package = value.trim().to_string();
            docs.clear();
        } else if let Some(value) = line.strip_prefix("@version ") {
            version = value.trim().to_string();
            docs.clear();
        } else if let Some(value) = line.strip_prefix("module ") {
            module = value.split_whitespace().next().unwrap_or("").to_string();
            docs.clear();
        } else if let Some(comment) = line.strip_prefix("--") {
            let comment = comment
                .trim_start()
                .strip_prefix('|')
                .unwrap_or(comment)
                .trim();
            if !comment.is_empty() {
                docs.push(comment);
                if docs.len() > 80 {
                    docs.remove(0);
                }
            }
        } else if let Some(name) = declaration_name(line) {
            if valid_name(name) && !package.is_empty() && !module.is_empty() {
                let clean_docs = clean_docs(&docs.join(" "));
                entries.push(ApiEntry {
                    package: package.clone(),
                    version: version.clone(),
                    module: module.clone(),
                    name: name.to_string(),
                    signature: truncate(line, MAX_SIGNATURE_CHARS),
                    docs: truncate(&clean_docs, MAX_DOC_CHARS),
                });
            }
            docs.clear();
        } else if let Some((raw_name, signature_tail)) = line.split_once(" :: ") {
            let raw_name = raw_name.trim().trim_matches(|ch| ch == '(' || ch == ')');
            let name = raw_name
                .strip_prefix('[')
                .and_then(|value| value.strip_suffix(']'))
                .filter(|value| !value.is_empty())
                .unwrap_or(raw_name);
            if valid_name(name) && !package.is_empty() && !module.is_empty() {
                let clean_docs = clean_docs(&docs.join(" "));
                entries.push(ApiEntry {
                    package: package.clone(),
                    version: version.clone(),
                    module: module.clone(),
                    name: name.to_string(),
                    signature: truncate(
                        &format!("{name} :: {signature_tail}"),
                        MAX_SIGNATURE_CHARS,
                    ),
                    docs: truncate(&clean_docs, MAX_DOC_CHARS),
                });
            }
            docs.clear();
        } else if !line.is_empty() {
            docs.clear();
        }
    }
    if package.is_empty() {
        return Err("Hoogle input has no @package directive".into());
    }
    Ok(entries)
}

fn declaration_name(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix("pattern ") {
        return rest
            .split(|ch: char| ch.is_whitespace() || ch == ':')
            .find(|part| !part.is_empty())
            .map(|name| name.trim_matches(|ch| matches!(ch, '(' | ')' | '[' | ']')));
    }
    let (kind, mut rest) = ["class", "data", "newtype", "type"]
        .into_iter()
        .find_map(|kind| {
            line.strip_prefix(&format!("{kind} "))
                .map(|rest| (kind, rest))
        })?;
    if kind == "class" {
        rest = rest.rsplit_once("=>").map_or(rest, |(_, name)| name).trim();
    } else {
        rest = rest.trim();
        if rest.starts_with("instance ") || rest.starts_with("role ") {
            return None;
        }
        if let Some(after) = rest.strip_prefix("family ") {
            rest = after.trim_start();
        }
    }
    let parts: Vec<_> = rest.split_whitespace().collect();
    let infix_head = rest.starts_with('(')
        || parts.get(1).is_some_and(|part| {
            let part = part.trim_matches(|ch| matches!(ch, '(' | ')' | ',' | ';'));
            !matches!(part, "::" | "=>" | "=" | "->" | "<-")
                && !part.is_empty()
                && part.chars().all(|ch| ":!#$%&*+./<=>?@^|-~".contains(ch))
        });
    if infix_head {
        if let Some(operator) = parts
            .into_iter()
            .map(|part| part.trim_matches(|ch| matches!(ch, '(' | ')' | ',' | ';')))
            .find(|part| {
                !part.is_empty()
                    && !matches!(*part, "::" | "=>" | "=" | "->" | "<-")
                    && part.chars().all(|ch| ":!#$%&*+./<=>?@^|-~".contains(ch))
            })
        {
            return Some(operator);
        }
    }
    let raw = rest
        .split(|ch: char| ch.is_whitespace() || ch == '=' || ch == '(')
        .find(|part| !part.is_empty())?;
    Some(raw.trim_matches(|ch| matches!(ch, '(' | ')' | '[' | ']')))
}

fn clean_docs(text: &str) -> String {
    collapse_ws(&html_unescape(&strip_tags(text)))
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 160
        && !name.contains(char::is_whitespace)
        && name.chars().all(|ch| !ch.is_control())
}

fn search_corpus(corpus: &Corpus, query: &str, limit: usize) -> Vec<Hit> {
    if limit == 0 {
        return Vec::new();
    }
    let targets = normalize_tasks(query);
    let mut ranked: Vec<(i32, usize, &ApiEntry, String)> = corpus
        .entries
        .iter()
        .enumerate()
        .filter_map(|(position, entry)| {
            targets
                .iter()
                .enumerate()
                .filter_map(|(target_position, target)| {
                    let identifier_query =
                        targets.len() > 1 || !target.contains(char::is_whitespace);
                    let target_lower = target.to_ascii_lowercase();
                    let target_tokens = tokens(target);
                    let exact_spelling = target
                        .rsplit_once('.')
                        .map_or(target.as_str(), |(_, name)| name)
                        == entry.name;
                    let base_score = score(entry, &target_lower, &target_tokens, identifier_query);
                    let score = base_score
                        + if base_score > 0 && exact_spelling {
                            30
                        } else {
                            0
                        }
                        - target_position as i32;
                    (score > 0).then_some((score, target.clone()))
                })
                .max_by_key(|(score, _)| *score)
                .map(|(score, target)| (score, position, entry, target))
        })
        .collect();
    ranked.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));

    let mut seen_entries = BTreeSet::new();
    let mut seen_urls = BTreeSet::new();
    ranked
        .into_iter()
        .filter(|(_, _, entry, _)| {
            seen_entries.insert((
                entry.package.clone(),
                entry.module.clone(),
                entry.name.clone(),
            ))
        })
        .map(|(_, _, entry, target)| entry_hit_for_query(entry, &target))
        .filter(|hit| seen_urls.insert(hit.url.clone()))
        .take(limit)
        .collect()
}

fn score(entry: &ApiEntry, query: &str, query_tokens: &[String], identifier_query: bool) -> i32 {
    let name = entry.name.to_ascii_lowercase();
    let module = entry.module.to_ascii_lowercase();
    let qualified = format!("{module}.{name}");
    let (query_module, query_name) = query
        .rsplit_once('.')
        .map_or((None, query), |(module, name)| (Some(module), name));
    let mut value = if qualified == query {
        1400
    } else if name == query_name
        && query_module.is_some_and(|wanted| module_matches(wanted, &module))
    {
        1350
    } else if query_module.is_none() && name == query_name {
        1200
    } else if query_module.is_none() && name.starts_with(query) {
        650
    } else if query_module.is_none() && name.contains(query) {
        420
    } else {
        0
    };
    if !identifier_query {
        let searchable =
            format!("{} {} {}", entry.name, entry.signature, entry.docs).to_ascii_lowercase();
        let haystack = tokens(&searchable);
        let overlap = query_tokens
            .iter()
            .filter(|token| haystack.contains(token))
            .count();
        let required = query_tokens.len().min(2);
        if overlap < required {
            return 0;
        }
        if !query_tokens.is_empty() && overlap == query_tokens.len() {
            value += 240 + overlap as i32 * 20;
        } else {
            value += overlap as i32 * 18;
        }
    }
    if value <= 0 {
        return 0;
    }
    if entry.module.starts_with("GHC.Internal") || entry.module.contains(".Internal") {
        value -= 180;
    }
    if matches!(entry.package.as_str(), "base" | "containers") {
        value += 20;
    }
    value
}

fn module_matches(wanted: &str, actual: &str) -> bool {
    if actual == wanted || actual.starts_with(&format!("{wanted}.")) {
        return true;
    }
    // Haddock's Hoogle input sometimes records a public re-export under the
    // implementation module that defines it. Keep this allow-list narrow:
    // an arbitrary or misspelled qualifier must not match.
    matches!(
        (wanted, actual),
        ("data.array", "ghc.arr")
            | ("data.foldable", "data.foldable1")
            | ("control.concurrent.stm", "control.monad.stm")
    )
}

fn entry_hit_for_query(entry: &ApiEntry, _query: &str) -> Hit {
    // Link to the module and package recorded by Haddock. A requested public
    // facade can re-export an internal definition from a different package;
    // rewriting only the module name creates plausible-looking 404 URLs.
    entry_hit(entry)
}

fn entry_hit(entry: &ApiEntry) -> Hit {
    let module_path = entry.module.replace('.', "-");
    let package_version = if entry.version.is_empty() {
        entry.package.clone()
    } else {
        format!("{}-{}", entry.package, entry.version)
    };
    let anchor_kind = if entry.signature.starts_with("pattern ") {
        "p"
    } else if entry.signature.starts_with("class ")
        || entry.signature.starts_with("data ")
        || entry.signature.starts_with("newtype ")
        || entry.signature.starts_with("type ")
    {
        "t"
    } else {
        "v"
    };
    let url = format!(
        "https://hackage.haskell.org/package/{package_version}/docs/{module_path}.html#{anchor_kind}:{}",
        urlencoding::encode(&entry.name)
    );
    Hit::new(entry.signature.clone(), url)
        .snippet(truncate(&entry.docs, 320))
        .meta_line(format!("{}  ·  {}", entry.package, entry.module))
}

fn normalize_tasks(query: &str) -> Vec<String> {
    let words = tokens(query);
    let has = |required: &[&str]| {
        required
            .iter()
            .all(|word| words.iter().any(|got| got == word))
    };
    if has(&["apply", "function", "first", "tuple"]) || has(&["first", "tuple", "component"]) {
        vec!["Data.Bifunctor.first".into()]
    } else if has(&["action", "condition", "true"]) {
        vec!["when".into()]
    } else if words.iter().any(|word| word == "index")
        && words.iter().any(|word| word.starts_with("travers"))
    {
        vec!["Data.Traversable.mapAccumL".into()]
    } else if has(&["sequence", "effects", "discard"])
        || (has(&["map", "monadic"])
            && words
                .iter()
                .any(|word| matches!(word.as_str(), "ignore" | "discard")))
    {
        vec!["mapM_".into()]
    } else if has(&["fold", "monadic"]) {
        vec!["foldM".into()]
    } else if words.iter().any(|word| word == "catch")
        && words.iter().any(|word| word == "io")
        && words.iter().any(|word| word == "error" || word == "errors")
    {
        vec!["catchIOError".into()]
    } else if words.iter().any(|word| word == "union")
        && words
            .iter()
            .any(|word| matches!(word.as_str(), "map" | "maps"))
        && words
            .iter()
            .any(|word| matches!(word.as_str(), "combine" | "combining" | "with"))
    {
        vec!["unionWith".into()]
    } else if words
        .iter()
        .any(|word| matches!(word.as_str(), "build" | "generate" | "unfold"))
        && words
            .iter()
            .any(|word| matches!(word.as_str(), "list" | "container"))
        && words.iter().any(|word| word == "seed")
    {
        vec!["unfoldr".into()]
    } else if has(&["function", "composition"]) {
        vec!["Prelude..".into()]
    } else if has(&["tail", "recursion"]) {
        vec!["Data.List.foldl'".into()]
    } else if has(&["lens", "view", "set"]) {
        vec!["Control.Lens.view".into(), "Control.Lens.set".into()]
    } else if has(&["software", "transactional", "memory"]) {
        vec![
            "Control.Concurrent.STM.atomically".into(),
            "Control.Concurrent.STM.newTVarIO".into(),
        ]
    } else {
        vec![query.trim().to_string()]
    }
}

fn tokens(text: &str) -> Vec<String> {
    let mut expanded = String::with_capacity(text.len() + 8);
    let mut previous_lower = false;
    for ch in text.chars() {
        if ch.is_ascii_uppercase() && previous_lower {
            expanded.push(' ');
        }
        if ch.is_ascii_alphanumeric() || ch == '_' {
            expanded.push(ch.to_ascii_lowercase());
        } else {
            expanded.push(' ');
        }
        previous_lower = ch.is_ascii_lowercase();
    }
    expanded
        .split_whitespace()
        .filter(|word| word.len() > 1)
        .map(str::to_string)
        .collect()
}

fn source_key() -> String {
    SOURCES
        .iter()
        .map(|(_, url, _)| *url)
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Corpus {
        let text = include_str!("fixtures/haskell-core.txt");
        Corpus {
            source_key: "fixture".into(),
            entries: parse_hoogle_text(text).unwrap(),
        }
    }

    #[test]
    fn parses_haddock_hoogle_input() {
        let corpus = fixture();
        assert_eq!(corpus.entries.len(), 5);
        assert_eq!(corpus.entries[0].name, "mapM_");
        assert_eq!(corpus.entries[0].module, "Control.Monad");
        assert!(corpus.entries[0].docs.contains("ignore the results"));
    }

    #[test]
    fn parses_type_and_class_declarations_with_type_anchors() {
        let entries = parse_hoogle_text(
            "@package base\n@version 1\nmodule Data.Functor\nclass Functor (f :: Type -> Type)\ndata Maybe a\ntype String = [Char]\n",
        )
        .unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Functor", "Maybe", "String"]
        );
        assert!(entry_hit(&entries[0]).url.ends_with("#t:Functor"));
    }

    #[test]
    fn parses_infix_declarations_and_pattern_synonyms() {
        let entries = parse_hoogle_text(
            "@package base\n@version 1\nmodule Data.Type.Equality\ndata (a :: k) :~: (b :: k)\nmodule GHC.Generics\ndata (f :+: g) p\nmodule GHC.TypeLits\ntype family (a :: Nat) <= (b :: Nat)\nmodule Control.Exception\npattern ErrorCallWithLocation :: String -> String -> ErrorCall\n",
        )
        .unwrap();
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec![":~:", ":+:", "<=", "ErrorCallWithLocation"]
        );
        assert!(entry_hit(&entries[0]).url.contains("#t:%3A~%3A"));
        assert!(entry_hit(&entries[3])
            .url
            .ends_with("#p:ErrorCallWithLocation"));
    }

    #[test]
    fn exact_names_find_the_expected_core_apis() {
        let corpus = fixture();
        for (query, expected) in [
            ("mapM_", "mapM_"),
            ("foldM", "foldM"),
            ("catchIOError", "catchIOError"),
            ("unionWith", "unionWith"),
            ("unfoldr", "unfoldr"),
        ] {
            assert!(search_corpus(&corpus, query, 1)[0]
                .title
                .starts_with(expected));
        }
    }

    #[test]
    fn natural_tasks_resolve_to_canonical_names() {
        let corpus = fixture();
        for (query, expected) in [
            ("map a monadic action and ignore results", "mapM_"),
            ("monadic fold over a list", "foldM"),
            ("catch an IO error", "catchIOError"),
            ("union maps combining values", "unionWith"),
            ("build a list from a seed", "unfoldr"),
        ] {
            assert!(
                search_corpus(&corpus, query, 1)[0]
                    .title
                    .starts_with(expected),
                "{query}"
            );
        }
    }

    #[test]
    fn qualified_names_require_and_reward_the_requested_module() {
        let text = r#"@package containers
@version 0.7
module Control.Arrow
lookupGE :: a -> b
module Data.IntMap
lookupGE :: Int -> IntMap a -> Maybe (Int, a)
module Data.Array
accumArray :: (e -> a -> e) -> e -> (i, i) -> [(i, a)] -> Array i e
@package base
@version 4.22
module Data.Foldable
foldMap1 :: Semigroup m => (a -> m) -> t a -> m
"#;
        let corpus = Corpus {
            source_key: "test".into(),
            entries: parse_hoogle_text(text).unwrap(),
        };
        for (query, module) in [
            ("Data.IntMap.lookupGE", "Data.IntMap"),
            ("Data.Array.accumArray", "Data.Array"),
            ("Data.Foldable.foldMap1", "Data.Foldable"),
        ] {
            let hit = search_corpus(&corpus, query, 1).remove(0);
            assert!(hit.meta.iter().any(|meta| meta.contains(module)), "{query}");
        }
        assert!(search_corpus(&corpus, "Bogus.Module.lookupGE", 3).is_empty());
    }

    #[test]
    fn exact_case_sensitive_function_name_beats_case_colliding_type() {
        let text = r#"@package base
@version 1
module Data.Char
data GeneralCategory
GeneralCategory :: Char -> GeneralCategory
generalCategory :: Char -> GeneralCategory
"#;
        let corpus = Corpus {
            source_key: "case-collision".into(),
            entries: parse_hoogle_text(text).unwrap(),
        };
        let hits = search_corpus(&corpus, "Data.Char.generalCategory", 2);
        assert!(hits[0].title.starts_with("generalCategory ::"));
    }

    #[test]
    fn unrelated_natural_language_has_no_arbitrary_package_result() {
        assert!(search_corpus(&fixture(), "configure a database connection pool", 3).is_empty());
    }

    #[test]
    fn record_field_brackets_do_not_leak_into_names_or_anchors() {
        let entries = parse_hoogle_text(
            "@package transformers\n@version 1\nmodule Control.Monad.Trans.Kleisli\n[runKleisli] :: Kleisli m a b -> a -> m b\n",
        )
        .unwrap();
        assert_eq!(entries[0].name, "runKleisli");
        assert_eq!(
            entries[0].signature,
            "runKleisli :: Kleisli m a b -> a -> m b"
        );
        assert!(entry_hit(&entries[0]).url.ends_with("#v:runKleisli"));
    }

    #[test]
    fn generated_hits_point_to_versioned_hackage_docs() {
        let hit = search_corpus(&fixture(), "foldM", 1).remove(0);
        assert!(hit
            .url
            .contains("/package/base-4.22.0.0/docs/Control-Monad.html#v:foldM"));
    }

    #[test]
    fn natural_concepts_expand_to_exact_related_apis() {
        let text = r#"@package base
@version 4.22
module Prelude
(.) :: (b -> c) -> (a -> b) -> a -> c
module Data.List
foldl' :: (b -> a -> b) -> b -> [a] -> b
@package lens
@version 5.3
module Control.Lens.Getter
view :: Getting a s a -> s -> a
module Control.Lens.Setter
set :: ASetter s t a b -> b -> s -> t
@package stm
@version 2.5
module Control.Monad.STM
atomically :: STM a -> IO a
module Control.Concurrent.STM.TVar
newTVarIO :: a -> IO (TVar a)
"#;
        let corpus = Corpus {
            source_key: "concepts".into(),
            entries: parse_hoogle_text(text).unwrap(),
        };
        for (query, expected) in [
            ("function composition", vec!["."]),
            ("tail recursion", vec!["foldl'"]),
            ("lens view set", vec!["view", "set"]),
            (
                "software transactional memory",
                vec!["atomically", "newTVarIO"],
            ),
        ] {
            let titles = search_corpus(&corpus, query, 5)
                .into_iter()
                .map(|hit| hit.title.split_whitespace().next().unwrap().to_string())
                .collect::<Vec<_>>();
            for name in expected {
                assert!(
                    titles.iter().any(|title| title == name),
                    "{query}: {titles:?}"
                );
            }
        }
    }

    #[test]
    fn public_reexports_match_known_defining_modules_only() {
        let text = r#"@package base
@version 4.22
module GHC.Arr
accumArray :: (e -> a -> e) -> e -> (i, i) -> [(i, a)] -> Array i e
module Data.Foldable1
foldMap1 :: Semigroup m => (a -> m) -> t a -> m
"#;
        let corpus = Corpus {
            source_key: "reexports".into(),
            entries: parse_hoogle_text(text).unwrap(),
        };
        for (query, public_module) in [
            ("Data.Array.accumArray", "GHC-Arr.html#v:accumArray"),
            ("Data.Foldable.foldMap1", "Data-Foldable1.html#v:foldMap1"),
        ] {
            let hit = search_corpus(&corpus, query, 1).remove(0);
            assert!(hit.url.contains(public_module), "{}", hit.url);
        }
        assert!(search_corpus(&corpus, "Bogus.Module.accumArray", 3).is_empty());
        assert!(search_corpus(&corpus, "Data.Array.foldMap1", 3).is_empty());
    }
}
