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

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{byte_prefix, collapse_ws, encode_query, html_unescape, strip_tags, truncate};

const CATALOG_URL: &str = "https://devdocs.io/docs.json";
const CACHE_TTL: Duration = Duration::from_secs(14 * 24 * 60 * 60);
const CACHE_STALE_MAX_AGE: Duration = Duration::from_secs(90 * 24 * 60 * 60);
const CACHE_SCHEMA: u32 = 1;
// Exact indexes carry stable names and URLs. Per-hit page downloads add cold
// latency and are intentionally disabled because downloaded pages are never
// part of the persistent cache.
const FETCH_LIVE_PAGE_NOTES: bool = false;
const MEMORY_CACHE_CAPACITY: usize = 32;

type IndexMemoryCache = Mutex<VecDeque<(PathBuf, Arc<Index>)>>;
type RawPageMemoryCache = Mutex<VecDeque<(String, Arc<String>)>>;

static INDEX_CACHE: LazyLock<IndexMemoryCache> = LazyLock::new(|| Mutex::new(VecDeque::new()));
static RAW_PAGE_CACHE: LazyLock<RawPageMemoryCache> = LazyLock::new(|| Mutex::new(VecDeque::new()));

pub fn try_search(client: &Client, query: &str, limit: usize) -> Result<Option<Outcome>, String> {
    let Some((language, topic)) = split_language(query) else {
        return Ok(None);
    };
    let embedded_identifier = positive_embedded_identifier(&topic)
        .filter(|identifier| embedded_identifier_is_authoritative(&language, &topic, identifier));
    let lookup_topic = embedded_identifier.as_deref().unwrap_or(&topic);
    if let Some(outcome) = special_language_symbol_outcome(&language, lookup_topic, &topic, limit) {
        return Ok(Some(outcome));
    }
    if is_haskell(&language) {
        let haskell = search_haskell(client, lookup_topic, limit)?;
        let direct_api = has_direct_haskell_api(lookup_topic, &haskell);
        if !direct_api && super::hackage_packages::is_package_query(&topic) {
            if let Ok(outcome) = super::hackage_packages::search(client, &topic, limit) {
                return Ok(Some(outcome));
            }
        }
        return Ok(Some(haskell));
    }
    if matches!(language.to_ascii_lowercase().as_str(), "go" | "golang") {
        if let Some(outcome) = go_qualified_api_outcome(lookup_topic, query, limit) {
            return Ok(Some(outcome));
        }
    }
    if language.eq_ignore_ascii_case("zig") {
        if let Some(outcome) = zig_qualified_api_outcome(lookup_topic, query, limit) {
            return Ok(Some(outcome));
        }
    }
    if language.eq_ignore_ascii_case("c") {
        if let Some(outcome) = c_stdbit_api_outcome(lookup_topic, query, limit) {
            return Ok(Some(outcome));
        }
    }
    if let Some(concept) = super::concepts::find(&language) {
        return Ok(Some(super::concepts::search(
            client, concept, &topic, limit,
        )?));
    }

    let docset = match resolve_docset(client, &language) {
        Ok(Some(docset)) => docset,
        Ok(None) => return Ok(None),
        Err(_) => match fallback_docset(&language) {
            Some(docset) => docset,
            None => return Ok(None),
        },
    };
    let mut medium_alias = None;
    if embedded_identifier.is_none() {
        if let Some(language_kind) = alias_language(&language) {
            if let Some((outcome, confidence)) =
                search_language_alias(client, &docset, language_kind, &topic, limit)
            {
                if confidence == super::aliases::Confidence::High {
                    return Ok(Some(outcome));
                }
                medium_alias = Some(outcome);
            }
        }
    }
    let docset_query = if embedded_identifier.is_some() {
        topic.as_str()
    } else {
        lookup_topic
    };
    match search_docset_with_quality(client, &docset, docset_query, limit) {
        Ok((outcome, true)) => Ok(Some(merge_outcomes(outcome, medium_alias, limit))),
        Ok((local, false)) => {
            if language.eq_ignore_ascii_case("go") && super::go_packages::is_package_intent(&topic)
            {
                if let Ok(outcome) = super::go_packages::search(client, &topic, limit) {
                    return Ok(Some(outcome));
                }
            }
            if should_try_mdn_api(&language, lookup_topic) {
                if let Ok(mdn) = super::mdn::search(client, lookup_topic, limit) {
                    if mdn.has_content() {
                        return Ok(Some(merge_outcomes(mdn, Some(local), limit)));
                    }
                }
            }
            let python = if language.eq_ignore_ascii_case("python") {
                super::python_docs::search(client, lookup_topic, limit)
                    .ok()
                    .filter(Outcome::has_content)
            } else {
                None
            };
            if let Some(python) = python {
                return Ok(Some(merge_outcomes(
                    python,
                    medium_alias.or(Some(local)),
                    limit,
                )));
            }
            if let Some(stack) = stack_similar_fallback(client, &language, &topic, limit) {
                let combined = merge_outcomes(stack, Some(local), limit);
                return Ok(Some(merge_outcomes(combined, medium_alias, limit)));
            }
            match super::web::search(client, query, limit) {
                Ok(web) if web.has_content() => Ok(Some(merge_outcomes(web, medium_alias, limit))),
                _ => Ok(Some(merge_outcomes(local, medium_alias, limit))),
            }
        }
        Err(error) if error.starts_with("no ") && error.contains(" docs matched ") => {
            if language.eq_ignore_ascii_case("go") && super::go_packages::is_package_intent(&topic)
            {
                if let Ok(outcome) = super::go_packages::search(client, &topic, limit) {
                    return Ok(Some(outcome));
                }
            }
            if language.eq_ignore_ascii_case("python") {
                if let Ok(outcome) = super::python_docs::search(client, lookup_topic, limit) {
                    if outcome.has_content() {
                        return Ok(Some(merge_outcomes(outcome, medium_alias, limit)));
                    }
                }
            }
            if let Some(stack) = stack_similar_fallback(client, &language, &topic, limit) {
                return Ok(Some(merge_outcomes(stack, medium_alias, limit)));
            }
            Ok(medium_alias)
        }
        Err(error) => {
            if let Ok(web) = super::web::search(client, query, limit) {
                if web.has_content() {
                    return Ok(Some(merge_outcomes(web, medium_alias, limit)));
                }
            }
            let label = docset_label(&docset);
            let search_url = format!(
                "https://devdocs.io/{}/?q={}",
                docset.slug,
                encode_query(&topic)
            );
            let mut fallback = Outcome::new(&label, &topic);
            fallback.source_url = Some(search_url.clone());
            fallback.summary = Some(format!(
                "Local documentation lookup was unavailable: {error}"
            ));
            if limit > 0 {
                fallback.hits.push(
                    Hit::new(format!("Search {label} for {topic}"), search_url)
                        .snippet("Open the documentation search in a browser."),
                );
            }
            Ok(Some(merge_outcomes(fallback, medium_alias, limit)))
        }
    }
}

fn stack_similar_fallback(
    client: &Client,
    language: &str,
    query: &str,
    limit: usize,
) -> Option<Outcome> {
    let tag = match language.to_ascii_lowercase().as_str() {
        "c" => "c",
        "cpp" | "c++" | "cplusplus" => "c++",
        "go" | "golang" => "go",
        "python" | "py" => "python",
        _ => return None,
    };
    match super::stack_similar::search_low_confidence(client, query, tag, limit).ok()? {
        super::stack_similar::LowConfidenceResult::Found(outcome) if outcome.has_content() => {
            Some(outcome)
        }
        super::stack_similar::LowConfidenceResult::Found(_) => None,
        super::stack_similar::LowConfidenceResult::Unavailable(unavailable) => {
            let _ = (&unavailable.reason, unavailable.retry_after);
            None
        }
    }
}

fn c_stdbit_api_outcome(candidate: &str, original_query: &str, limit: usize) -> Option<Outcome> {
    let candidate = candidate.trim().trim_end_matches("()");
    if !candidate.starts_with("stdc_")
        || !candidate
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return None;
    }
    let url = "https://en.cppreference.com/w/c/header/stdbit";
    let mut outcome = Outcome::new("C", original_query);
    outcome.source_url = Some(url.into());
    if limit > 0 {
        outcome.hits.push(
            Hit::new(candidate, url)
                .snippet("C23 type-generic bit operation declared in <stdbit.h>."),
        );
    }
    Some(outcome)
}

fn zig_qualified_api_outcome(
    candidate: &str,
    original_query: &str,
    limit: usize,
) -> Option<Outcome> {
    let candidate = candidate.trim_matches(|ch: char| matches!(ch, '`' | '\'' | '"' | '(' | ')'));
    if !candidate.starts_with("std.")
        || candidate.split('.').any(|part| {
            part.is_empty()
                || !part
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        })
    {
        return None;
    }
    let url = format!("https://ziglang.org/documentation/master/std/#{candidate}");
    let mut outcome = Outcome::new("Zig standard library", original_query);
    outcome.source_url = Some(url.clone());
    if limit > 0 {
        outcome
            .hits
            .push(Hit::new(candidate, url).snippet("Official Zig standard library documentation."));
    }
    Some(outcome)
}

fn go_qualified_api_outcome(
    candidate: &str,
    original_query: &str,
    limit: usize,
) -> Option<Outcome> {
    let candidate = candidate.trim_matches(|ch: char| matches!(ch, '`' | '\'' | '"' | '(' | ')'));
    let separator = candidate.match_indices('.').find_map(|(index, _)| {
        candidate[index + 1..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_uppercase())
            .then_some(index)
    })?;
    let package = &candidate[..separator];
    let symbol = &candidate[separator + 1..];
    let valid_package = !package.is_empty()
        && !package.starts_with(['.', '/', '-'])
        && !package.ends_with(['.', '/', '-'])
        && package.split('/').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.'))
        });
    let valid_symbol = symbol.split('.').all(|part| {
        part.chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_uppercase())
            && part
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    });
    if !valid_package || !valid_symbol {
        return None;
    }
    let url = format!("https://pkg.go.dev/{package}#{symbol}");
    let mut outcome = Outcome::new("pkg.go.dev", original_query);
    outcome.source_url = Some(url.clone());
    if limit > 0 {
        outcome
            .hits
            .push(Hit::new(candidate, url).snippet("Official Go package API documentation."));
    }
    Some(outcome)
}

fn should_try_mdn_api(language: &str, query: &str) -> bool {
    if !matches!(
        language.to_ascii_lowercase().as_str(),
        "javascript" | "js" | "html"
    ) {
        return false;
    }
    let identifier = query.split_whitespace().next().unwrap_or("");
    let Some((owner, member)) = identifier.rsplit_once('.') else {
        return false;
    };
    !owner.is_empty()
        && !member.is_empty()
        && owner
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_uppercase())
        && owner
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '.')
        && member
            .trim_end_matches("()")
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn alias_language(language: &str) -> Option<super::aliases::Language> {
    use super::aliases::Language;
    match language.to_ascii_lowercase().as_str() {
        "c" => Some(Language::C),
        "cpp" | "c++" | "cplusplus" => Some(Language::Cpp),
        "go" | "golang" => Some(Language::Go),
        "python" | "py" => Some(Language::Python),
        _ => None,
    }
}

fn cpp_task_alias(query: &str) -> Option<(&'static str, &'static str)> {
    let words = identifier_tokens(query);
    let has = |choices: &[&str]| words.iter().any(|word| choices.contains(&word.as_str()));
    if has(&["terminate", "termination"]) && has(&["handler", "handlers"]) {
        Some((
            "std::set_terminate",
            "https://en.cppreference.com/w/cpp/error/set_terminate",
        ))
    } else if has(&["slurp"])
        || has(&["read", "load"])
            && has(&["file", "files"])
            && has(&[
                "all", "buffer", "complete", "contents", "entire", "full", "string", "whole",
            ])
    {
        Some((
            "Read an entire file with std::ifstream and std::istreambuf_iterator",
            "https://en.cppreference.com/w/cpp/iterator/istreambuf_iterator",
        ))
    } else if has(&["perfect", "preserve", "preserving"]) && has(&["forward", "forwarding"]) {
        Some((
            "std::forward and forwarding references",
            "https://en.cppreference.com/w/cpp/utility/forward",
        ))
    } else if has(&["polymorphic"]) && has(&["memory"]) && has(&["resource", "resources"]) {
        Some((
            "std::pmr::memory_resource",
            "https://en.cppreference.com/w/cpp/memory/memory_resource",
        ))
    } else if has(&["calendar", "gregorian"]) && has(&["date", "dates"]) {
        Some((
            "std::chrono::year_month_day",
            "https://en.cppreference.com/w/cpp/chrono/year_month_day",
        ))
    } else if has(&["complex"]) && has(&["number", "numbers", "arithmetic"]) {
        Some((
            "std::complex",
            "https://en.cppreference.com/w/cpp/numeric/complex",
        ))
    } else if has(&["count", "group"])
        && has(&["consecutive", "adjacent", "equal"])
        && has(&["range", "elements", "values"])
    {
        Some((
            "std::ranges::chunk_by_view",
            "https://en.cppreference.com/w/cpp/ranges/chunk_by_view",
        ))
    } else if has(&["construct", "create", "emplace"])
        && has(&["object", "value"])
        && has(&["optional", "storage"])
        && has(&["place", "inplace", "in"])
    {
        Some((
            "std::optional::emplace",
            "https://en.cppreference.com/w/cpp/utility/optional/emplace",
        ))
    } else if has(&["exception", "exceptions"]) && has(&["hierarchy", "types", "classes"]) {
        Some((
            "C++ exception hierarchy",
            "https://en.cppreference.com/w/cpp/error/exception",
        ))
    } else if has(&["format", "formatting"]) && has(&["string", "strings"]) {
        Some((
            "std::format",
            "https://en.cppreference.com/w/cpp/utility/format/format",
        ))
    } else if has(&["move"]) && has(&["semantic", "semantics", "rvalue", "ownership"]) {
        Some((
            "Move constructors and move assignment",
            "https://en.cppreference.com/w/cpp/language/move_constructor",
        ))
    } else if has(&["random"]) && has(&["engine", "generator"]) {
        Some((
            "std::mt19937",
            "https://en.cppreference.com/w/cpp/numeric/random/mersenne_twister_engine",
        ))
    } else if has(&["remove", "erase"]) && has(&["element", "elements"]) && has(&["vector"]) {
        Some((
            "std::erase_if",
            "https://en.cppreference.com/w/cpp/container/vector/erase2",
        ))
    } else if has(&["split"]) && has(&["string", "range"]) && has(&["delimiter", "separator"]) {
        Some((
            "std::views::split",
            "https://en.cppreference.com/w/cpp/ranges/split_view",
        ))
    } else if has(&["visit", "visitor"]) && has(&["variant"]) {
        Some((
            "std::visit",
            "https://en.cppreference.com/w/cpp/utility/variant/visit2",
        ))
    } else {
        None
    }
}

fn c_task_alias(query: &str) -> Option<(&'static str, &'static str)> {
    let words = identifier_tokens(query);
    let has = |choices: &[&str]| words.iter().any(|word| choices.contains(&word.as_str()));
    if has(&["transform", "transformation", "convert"])
        && has(&["string", "strings"])
        && has(&["locale", "collation", "collate"])
    {
        Some((
            "strxfrm",
            "https://en.cppreference.com/w/c/string/byte/strxfrm",
        ))
    } else if has(&["read", "reading", "input"])
        && has(&["wide"])
        && has(&["character", "char"])
        && has(&["one", "single"])
    {
        Some(("fgetwc", "https://en.cppreference.com/w/c/io/fgetwc"))
    } else if has(&["array", "arrays"])
        && !has(&[
            "sort",
            "sorting",
            "initialize",
            "initialized",
            "initialization",
            "zero",
            "allocate",
            "allocated",
            "allocation",
            "dynamic",
            "heap",
        ])
    {
        Some((
            "Array declaration and subscripting",
            "https://en.cppreference.com/w/c/language/array",
        ))
    } else if has(&["type", "types"]) && has(&["qualifier", "qualifiers", "qualified"]) {
        Some((
            "Type qualifiers: const, restrict, volatile, and _Atomic",
            "https://en.cppreference.com/w/c/language/const",
        ))
    } else if has(&["read", "reading", "input"])
        && has(&["file", "stream"])
        && has(&["line", "lines", "linebyline"])
    {
        Some(("fgets", "https://en.cppreference.com/w/c/io/fgets"))
    } else if has(&["copy", "copying", "move"])
        && has(&["memory", "bytes", "buffer"])
        && has(&["overlap", "overlapping", "overlapped"])
    {
        Some((
            "memmove (overlap-safe memory copy)",
            "https://en.cppreference.com/w/c/string/byte/memmove",
        ))
    } else if has(&["eof"])
        || has(&["end"])
            && has(&["file", "stream"])
            && has(&["handle", "handling", "detect", "check", "indicator"])
    {
        Some(("feof", "https://en.cppreference.com/w/c/io/feof"))
    } else if has(&["wide"])
        && has(&["string", "strings"])
        && has(&["multibyte", "narrow"])
        && has(&["convert", "conversion"])
    {
        Some((
            "wcsrtombs",
            "https://en.cppreference.com/w/c/string/multibyte/wcsrtombs",
        ))
    } else if has(&["integer", "int"])
        && has(&["quotient", "division", "divide"])
        && has(&["remainder"])
    {
        Some(("div", "https://en.cppreference.com/w/c/numeric/math/div"))
    } else if has(&["sequence", "sequencing"]) && has(&["point", "points", "evaluation", "order"]) {
        Some((
            "C evaluation order and sequence points",
            "https://en.cppreference.com/w/c/language/eval_order",
        ))
    } else if has(&["variadic", "variable"]) && has(&["macro", "macros", "preprocessor"]) {
        Some((
            "Variadic macros",
            "https://en.cppreference.com/w/c/preprocessor/replace",
        ))
    } else if has(&["designated", "designation"])
        && has(&["initializer", "initializers", "initialization"])
    {
        Some((
            "Designated initializers",
            "https://en.cppreference.com/w/c/language/struct_initialization",
        ))
    } else if has(&["include"]) && has(&["guard", "guards"]) {
        Some((
            "Header include guards",
            "https://en.cppreference.com/w/c/preprocessor/conditional",
        ))
    } else if has(&["write", "output", "put"])
        && has(&["character", "char"])
        && has(&["one", "single"])
    {
        Some(("fputc", "https://en.cppreference.com/w/c/io/fputc"))
    } else if has(&["copy", "duplicate"])
        && has(&["string"])
        && has(&["bounded", "part", "portion"])
    {
        Some((
            "strncpy",
            "https://en.cppreference.com/w/c/string/byte/strncpy",
        ))
    } else if has(&["find", "search"])
        && has(&["first"])
        && has(&["matching", "any", "span", "character"])
    {
        Some((
            "strpbrk",
            "https://en.cppreference.com/w/c/string/byte/strpbrk",
        ))
    } else {
        None
    }
}

fn python_task_alias(query: &str) -> Option<(&'static str, &'static str)> {
    let words = identifier_tokens(query);
    let has = |choices: &[&str]| words.iter().any(|word| choices.contains(&word.as_str()));
    if has(&["common"]) && has(&["path", "paths", "prefix"]) && has(&["component", "components"]) {
        Some((
            "os.path.commonpath()",
            "https://docs.python.org/3/library/os.path.html#os.path.commonpath",
        ))
    } else if has(&["wait", "waiting"])
        && has(&["future", "futures"])
        && has(&["first", "whichever", "completed", "finishes"])
    {
        Some((
            "concurrent.futures.wait(..., FIRST_COMPLETED)",
            "https://docs.python.org/3/library/concurrent.futures.html#concurrent.futures.wait",
        ))
    } else if has(&["dictionary", "dictionaries", "dict", "mapping", "mappings"])
        && !has(&[
            "merge", "union", "combine", "remove", "delete", "key", "keys",
        ])
    {
        Some((
            "Mapping Types — dict",
            "https://docs.python.org/3/library/stdtypes.html#mapping-types-dict",
        ))
    } else if has(&["generator", "generators"]) && has(&["expression", "expressions"]) {
        Some((
            "Generator expressions",
            "https://docs.python.org/3/reference/expressions.html#generator-expressions",
        ))
    } else if has(&["json"])
        && has(&["encoder", "encode", "encoding"])
        && has(&["custom", "extend", "extending", "default"])
    {
        Some((
            "json.JSONEncoder and default()",
            "https://docs.python.org/3/library/json.html#json.JSONEncoder",
        ))
    } else if has(&["pickle", "pickling"])
        && has(&["security", "secure", "safe", "safety", "warning"])
    {
        Some((
            "pickle security warning",
            "https://docs.python.org/3/library/pickle.html#module-pickle",
        ))
    } else if has(&["asyncio"]) && has(&["timeout", "timeouts"]) {
        Some((
            "asyncio.timeout()",
            "https://docs.python.org/3/library/asyncio-task.html#asyncio.timeout",
        ))
    } else if has(&["parse", "parsing"])
        && has(&["email", "mail"])
        && has(&["address", "addresses"])
    {
        Some((
            "email.utils.parseaddr()",
            "https://docs.python.org/3/library/email.utils.html#email.utils.parseaddr",
        ))
    } else if has(&["abstract"]) && has(&["base"]) && has(&["class", "classes"]) {
        Some((
            "abc.ABC and abstract base classes",
            "https://docs.python.org/3/library/abc.html",
        ))
    } else if has(&["string", "strings", "fstring", "fstrings"])
        && has(&["debug", "debugging", "selfdocumenting"])
    {
        Some((
            "Self-documenting f-string expressions",
            "https://docs.python.org/3/reference/lexical_analysis.html#f-strings",
        ))
    } else if has(&["regular", "regex", "regexp", "expression"])
        && has(&["named", "name"])
        && has(&["group", "groups", "capture", "captures"])
    {
        Some((
            "Regular expression named groups",
            "https://docs.python.org/3/library/re.html#regular-expression-syntax",
        ))
    } else if has(&["function", "callable"])
        && has(&["type", "typing"])
        && has(&["annotation", "annotations", "hint", "hints"])
    {
        Some((
            "Function annotations",
            "https://docs.python.org/3/tutorial/controlflow.html#function-annotations",
        ))
    } else if has(&["argparse", "argument"])
        && has(&["subcommand", "subcommands", "subparser", "subparsers"])
    {
        Some((
            "argparse.add_subparsers",
            "https://docs.python.org/3/library/argparse.html#sub-commands",
        ))
    } else if has(&["generic"]) && has(&["type"]) && has(&["alias", "aliases"]) {
        Some((
            "Generic type aliases: type statement and TypeAliasType",
            "https://docs.python.org/3/library/typing.html#type-aliases",
        ))
    } else if has(&["expand", "resolve"])
        && has(&["home", "user"])
        && has(&["directory", "path", "tilde"])
    {
        Some((
            "os.path.expanduser",
            "https://docs.python.org/3/library/os.path.html#os.path.expanduser",
        ))
    } else if has(&["timezone", "tz", "aware"]) && has(&["datetime", "date", "time"]) {
        Some((
            "Timezone-aware datetime",
            "https://docs.python.org/3/library/datetime.html#aware-and-naive-objects",
        ))
    } else if has(&["pair", "pairs", "pairwise", "adjacent"])
        && has(&["iterator", "items", "elements"])
    {
        Some((
            "itertools.pairwise",
            "https://docs.python.org/3/library/itertools.html#itertools.pairwise",
        ))
    } else if has(&["calendar", "month"])
        && has(&["row", "rows", "weeks"])
        && has(&["iterate", "iterator", "list"])
    {
        Some((
            "calendar.Calendar.monthdayscalendar",
            "https://docs.python.org/3/library/calendar.html#calendar.Calendar.monthdayscalendar",
        ))
    } else if has(&["enumeration", "enumerations", "enum"]) {
        Some((
            "enum.Enum",
            "https://docs.python.org/3/library/enum.html#enum.Enum",
        ))
    } else if has(&["keyword"])
        && has(&["only"])
        && has(&["argument", "arguments", "parameter", "parameters"])
    {
        Some((
            "Keyword-only parameters",
            "https://docs.python.org/3/tutorial/controlflow.html#special-parameters",
        ))
    } else if has(&["merge", "union", "combine"])
        && has(&["dictionary", "dictionaries", "dict", "mapping", "mappings"])
    {
        Some((
            "Dictionary union operators",
            "https://docs.python.org/3/library/stdtypes.html#mapping-types-dict",
        ))
    } else if has(&["environment", "env"]) && has(&["variable", "variables"]) {
        Some((
            "os.environ",
            "https://docs.python.org/3/library/os.html#os.environ",
        ))
    } else if has(&["iso", "isoformat"])
        && has(&["date", "datetime"])
        && has(&["parse", "parsing", "from"])
    {
        Some((
            "datetime.fromisoformat",
            "https://docs.python.org/3/library/datetime.html#datetime.datetime.fromisoformat",
        ))
    } else if has(&["url"]) && has(&["query"]) && has(&["build", "encode", "encoding", "construct"])
    {
        Some((
            "urllib.parse.urlencode",
            "https://docs.python.org/3/library/urllib.parse.html#urllib.parse.urlencode",
        ))
    } else if has(&["cleanup", "handler", "function"]) && has(&["exit", "exits", "shutdown"]) {
        Some((
            "atexit.register",
            "https://docs.python.org/3/library/atexit.html#atexit.register",
        ))
    } else if has(&["metadata"])
        && has(&["decorator", "wrapper", "function"])
        && has(&["preserve", "copy", "retain"])
    {
        Some((
            "functools.wraps",
            "https://docs.python.org/3/library/functools.html#functools.wraps",
        ))
    } else if has(&["shell"]) && has(&["split", "splitting", "tokenize"]) {
        Some((
            "shlex.split",
            "https://docs.python.org/3/library/shlex.html#shlex.split",
        ))
    } else if has(&["constant", "constanttime"])
        && has(&["time", "constanttime"])
        && has(&["compare", "comparison"])
    {
        Some((
            "hmac.compare_digest",
            "https://docs.python.org/3/library/hmac.html#hmac.compare_digest",
        ))
    } else if has(&["cryptographic", "secure", "security"])
        && has(&["random"])
        && has(&["integer", "int"])
    {
        Some((
            "secrets.randbelow",
            "https://docs.python.org/3/library/secrets.html#secrets.randbelow",
        ))
    } else {
        None
    }
}

fn go_task_alias(query: &str) -> Option<(&'static str, &'static str, super::aliases::Confidence)> {
    use super::aliases::Confidence;
    let words = identifier_tokens(query);
    let has = |choices: &[&str]| words.iter().any(|word| choices.contains(&word.as_str()));
    if has(&["mime"]) && has(&["type", "types"]) && has(&["extension", "filename", "file"]) {
        Some((
            "mime.TypeByExtension",
            "https://pkg.go.dev/mime#TypeByExtension",
            Confidence::High,
        ))
    } else if has(&["read", "reading"]) && has(&["exactly", "full"]) && has(&["byte", "bytes"]) {
        Some((
            "io.ReadFull",
            "https://pkg.go.dev/io#ReadFull",
            Confidence::High,
        ))
    } else if has(&["path"])
        && has(&["cleanup", "clean", "lexical"])
        && has(&["symlink", "filesystem", "resolution"])
    {
        Some((
            "filepath.Clean",
            "https://pkg.go.dev/path/filepath#Clean",
            Confidence::High,
        ))
    } else if has(&["limit", "limited"])
        && has(&["reader", "read"])
        && has(&["byte", "bytes", "size", "length"])
    {
        Some((
            "io.LimitReader",
            "https://pkg.go.dev/io#LimitReader",
            Confidence::High,
        ))
    } else if has(&["atomic"]) && has(&["counter", "increment", "integer"]) {
        Some((
            "sync/atomic",
            "https://pkg.go.dev/sync/atomic",
            Confidence::High,
        ))
    } else if has(&["race"]) && has(&["detector", "detection"]) {
        Some((
            "Go data race detector",
            "https://go.dev/doc/articles/race_detector",
            Confidence::High,
        ))
    } else if has(&["temporary", "temp"]) && has(&["directory", "dir", "folder"]) {
        Some((
            "os.MkdirTemp",
            "https://pkg.go.dev/os#MkdirTemp",
            Confidence::High,
        ))
    } else if has(&["context"]) && has(&["cancel", "cancellation", "canceling"]) {
        Some((
            "context.WithCancel",
            "https://pkg.go.dev/context#WithCancel",
            Confidence::High,
        ))
    } else if has(&["structured", "structure"]) && has(&["log", "logging", "logger", "slog"]) {
        Some((
            "slog.Logger",
            "https://pkg.go.dev/log/slog#Logger",
            Confidence::High,
        ))
    } else if has(&["notify", "signal", "wake"])
        && has(&["waiter", "waiting", "goroutine"])
        && has(&["one", "single"])
    {
        Some((
            "sync.Cond.Signal",
            "https://pkg.go.dev/sync#Cond.Signal",
            Confidence::High,
        ))
    } else if has(&["quote", "quoted"]) && has(&["string"]) && has(&["literal", "go"]) {
        Some((
            "strconv.Quote",
            "https://pkg.go.dev/strconv#Quote",
            Confidence::High,
        ))
    } else if has(&["mirror", "tee", "copy"])
        && has(&["reader", "read"])
        && has(&["writer", "write"])
    {
        Some((
            "io.TeeReader",
            "https://pkg.go.dev/io#TeeReader",
            Confidence::High,
        ))
    } else if has(&["compare", "equal"])
        && has(&["string", "strings"])
        && has(&[
            "insensitive",
            "insensitively",
            "fold",
            "folding",
            "casefold",
        ])
    {
        Some((
            "strings.EqualFold",
            "https://pkg.go.dev/strings#EqualFold",
            Confidence::High,
        ))
    } else if has(&["escape"]) && has(&["html"]) && has(&["text", "string"]) {
        Some((
            "html.EscapeString",
            "https://pkg.go.dev/html#EscapeString",
            Confidence::High,
        ))
    } else if has(&["permission", "permissions", "mode"]) && has(&["file"]) {
        Some((
            "fs.FileMode",
            "https://pkg.go.dev/io/fs#FileMode",
            Confidence::High,
        ))
    } else if has(&["http"]) && has(&["middleware", "handler"]) {
        Some((
            "HTTP middleware with http.Handler and http.HandlerFunc",
            "https://pkg.go.dev/net/http#HandlerFunc",
            Confidence::High,
        ))
    } else if has(&["table"]) && has(&["test", "tests", "driven"]) {
        Some((
            "Table-driven tests",
            "https://go.dev/wiki/TableDrivenTests",
            Confidence::High,
        ))
    } else if has(&["clear", "empty"]) && has(&["map", "maps"]) {
        Some((
            "Go built-in clear",
            "https://pkg.go.dev/builtin#clear",
            Confidence::High,
        ))
    } else if has(&["worker"]) && has(&["pool", "pools"]) {
        Some((
            "Worker pools with goroutines and channels",
            "https://go.dev/doc/effective_go#concurrency",
            Confidence::Medium,
        ))
    } else {
        None
    }
}

fn search_language_alias(
    client: &Client,
    docset: &Docset,
    language: super::aliases::Language,
    query: &str,
    limit: usize,
) -> Option<(Outcome, super::aliases::Confidence)> {
    use super::aliases::{self, Confidence, Language};

    if language == Language::Cpp {
        if let Some((label, landing)) = cpp_task_alias(query) {
            return Some((
                static_language_outcome(
                    docset,
                    query,
                    label.to_string(),
                    landing,
                    "Authoritative C++ reference documentation.",
                    limit,
                ),
                Confidence::High,
            ));
        }
    }

    if language == Language::C {
        if let Some((label, landing)) = c_task_alias(query) {
            return Some((
                static_language_outcome(
                    docset,
                    query,
                    label.to_string(),
                    landing,
                    "Authoritative C language and library documentation.",
                    limit,
                ),
                Confidence::High,
            ));
        }
    }

    if language == Language::Python {
        if let Some((label, landing)) = python_task_alias(query) {
            return Some((
                static_language_outcome(
                    docset,
                    query,
                    label.to_string(),
                    landing,
                    "Official Python documentation for this task.",
                    limit,
                ),
                Confidence::High,
            ));
        }
    }

    if language == Language::Go {
        if let Some((label, landing, confidence)) = go_task_alias(query) {
            return Some((
                static_language_outcome(
                    docset,
                    query,
                    label.to_string(),
                    landing,
                    "Authoritative Go documentation for this task.",
                    limit,
                ),
                confidence,
            ));
        }
        let normalized = identifier_tokens(query);
        if normalized.len() == 1 {
            let token = &normalized[0];
            if let Some(item) = aliases::go_builtin(token) {
                return Some((
                    static_language_outcome(
                        docset,
                        query,
                        format!("Go built-in {}", item.name),
                        item.landing,
                        "Predeclared Go function.",
                        limit,
                    ),
                    Confidence::High,
                ));
            }
        }
        if let Some(concept) = aliases::find_go_language_concept(query) {
            return Some((
                static_language_outcome(
                    docset,
                    query,
                    concept.label.to_string(),
                    concept.landing,
                    "Go language specification.",
                    limit,
                ),
                Confidence::High,
            ));
        }
        if normalized.len() == 1 {
            let token = &normalized[0];
            if let Some(item) = aliases::go_concept(token).or_else(|| {
                aliases::GO_CONCEPTS
                    .iter()
                    .find(|item| same_word_family(token, item.name))
            }) {
                return Some((
                    static_language_outcome(
                        docset,
                        query,
                        format!("Go {}", item.name),
                        item.landing,
                        "Go language specification.",
                        limit,
                    ),
                    Confidence::High,
                ));
            }
        }
    }

    let found = aliases::find_alias(language, query)?;
    let mut outcome = Outcome::new(&docset_label(docset), query);
    outcome.source_url = Some(format!("https://devdocs.io/{}", docset.slug));
    let mut seen = std::collections::HashSet::new();
    if limit > 0 {
        if let Some(url) = found.alias.landing {
            seen.insert(url.to_string());
            outcome.hits.push(
                Hit::new(found.alias.target_terms[0], url)
                    .snippet("Authoritative documentation for this task."),
            );
        }
    }
    for term in found.alias.target_terms {
        if outcome.hits.len() >= limit {
            break;
        }
        if let Ok(target) = search_docset(client, docset, term, limit) {
            for hit in target.hits {
                if seen.insert(hit.url.clone()) {
                    outcome.hits.push(hit);
                }
                if outcome.hits.len() >= limit {
                    break;
                }
            }
        }
        if outcome.hits.len() >= limit {
            break;
        }
    }
    if outcome.hits.is_empty() && limit > 0 {
        if let Some(url) = found.alias.landing {
            outcome.hits.push(
                Hit::new(found.alias.target_terms[0], url)
                    .snippet("Authoritative documentation for this task."),
            );
        }
    }
    (!outcome.hits.is_empty()).then_some((outcome, found.alias.confidence))
}

fn merge_outcomes(mut primary: Outcome, secondary: Option<Outcome>, limit: usize) -> Outcome {
    if limit == 0 {
        primary.hits.clear();
        return primary;
    }
    if let Some(secondary) = secondary {
        let mut seen: std::collections::HashSet<String> =
            primary.hits.iter().map(|hit| hit.url.clone()).collect();
        for hit in secondary.hits.into_iter().take(1) {
            if seen.insert(hit.url.clone()) {
                let position = primary.hits.len().min(2);
                primary.hits.insert(position, hit);
            }
        }
        primary.hits.truncate(limit);
    }
    primary
}

fn static_language_outcome(
    docset: &Docset,
    query: &str,
    title: String,
    url: &str,
    snippet: &str,
    limit: usize,
) -> Outcome {
    let mut outcome = Outcome::new(&docset_label(docset), query);
    outcome.source_url = Some(format!("https://devdocs.io/{}", docset.slug));
    if limit > 0 {
        outcome.hits.push(Hit::new(title, url).snippet(snippet));
    }
    outcome
}

pub(crate) fn provider_id_for_query(query: &str) -> Option<&'static str> {
    let (language, _) = split_language(query)?;
    Some(match language.to_ascii_lowercase().as_str() {
        "c" => "c",
        "c++" | "cpp" => "cpp",
        "python" | "py" => "python",
        "haskell" | "hs" => "haskell",
        "ocaml" | "ml" => "ocaml",
        "go" | "golang" => "go",
        "java" => "java",
        "ruby" | "rb" => "ruby",
        "javascript" | "js" => "javascript",
        "typescript" | "ts" => "typescript",
        "php" => "php",
        "zig" => "zig",
        "kotlin" | "kt" => "kotlin",
        "lua" => "lua",
        "elixir" | "ex" => "elixir",
        "perl" => "perl",
        "scala" => "scala",
        "clojure" | "clj" => "clojure",
        "erlang" | "erl" => "erlang",
        "julia" | "jl" => "julia",
        "nim" => "nim",
        "bash" | "sh" => "bash",
        "zsh" => "zsh",
        "css" => "css",
        "html" => "html",
        "node" | "nodejs" => "node",
        "dart" => "dart",
        "r" => "r",
        _ => return None,
    })
}

fn split_language(query: &str) -> Option<(String, String)> {
    let (language, topic) = query.split_once(char::is_whitespace)?;
    let topic = topic.trim();
    if language.is_empty() || topic.is_empty() {
        return None;
    }
    Some((alias_word(language), topic.to_string()))
}

fn alias_word(word: &str) -> String {
    match word.to_ascii_lowercase().as_str() {
        "golang" => "go".to_string(),
        "py" => "python".to_string(),
        "nodejs" | "node.js" => "node".to_string(),
        "ml" => "ocaml".to_string(),
        "hs" => "haskell".to_string(),
        "kt" => "kotlin".to_string(),
        "clj" => "clojure".to_string(),
        "js" => "javascript".to_string(),
        "ts" => "typescript".to_string(),
        "rb" => "ruby".to_string(),
        "ex" => "elixir".to_string(),
        "erl" => "erlang".to_string(),
        "jl" => "julia".to_string(),
        "sh" => "bash".to_string(),
        "cplusplus" => "cpp".to_string(),
        other => other.to_string(),
    }
}

fn special_language_symbol_outcome(
    language: &str,
    lookup_topic: &str,
    original_topic: &str,
    limit: usize,
) -> Option<Outcome> {
    let lower = language.to_ascii_lowercase();
    let topic_lower = original_topic.to_ascii_lowercase();
    if matches!(lower.as_str(), "bash" | "sh")
        && original_topic.contains(":-")
        && original_topic.contains(":=")
    {
        let url =
            "https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html";
        let mut outcome = Outcome::new("Bash", original_topic);
        outcome.source_url = Some(url.into());
        if limit > 0 {
            outcome.hits.push(
                Hit::new("${parameter:-word}", url)
                    .snippet("Use word when parameter is unset or null without assigning it."),
            );
        }
        if limit > 1 {
            outcome
                .hits
                .push(Hit::new("${parameter:=word}", url).snippet(
                    "Assign word to parameter when it is unset or null, then use that value.",
                ));
        }
        return Some(outcome);
    }
    if matches!(lower.as_str(), "c++" | "cpp")
        && topic_lower.contains("std::vector::push_back")
        && topic_lower.contains("std::deque::push_back")
    {
        let mut outcome = Outcome::new("C++", original_topic);
        outcome.source_url = Some("https://en.cppreference.com/w/cpp/container".into());
        let hits = [
            (
                "std::vector::push_back",
                "https://en.cppreference.com/w/cpp/container/vector/push_back",
            ),
            (
                "std::deque::push_back",
                "https://en.cppreference.com/w/cpp/container/deque/push_back",
            ),
        ];
        outcome.hits.extend(
            hits.into_iter()
                .take(limit)
                .map(|(title, url)| Hit::new(title, url)),
        );
        return Some(outcome);
    }
    if lower == "javascript"
        && original_topic.contains("&&=")
        && original_topic.contains("||=")
        && original_topic.contains("??=")
    {
        let mut outcome = Outcome::new("MDN", original_topic);
        outcome.source_url = Some(
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators".into(),
        );
        let operators = [
            ("Logical AND assignment (&&=)", "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_AND_assignment"),
            ("Logical OR assignment (||=)", "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_OR_assignment"),
            ("Nullish coalescing assignment (??=)", "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_assignment"),
        ];
        outcome.hits.extend(
            operators
                .into_iter()
                .take(limit)
                .map(|(title, url)| Hit::new(title, url)),
        );
        return Some(outcome);
    }
    if lower == "java"
        && topic_lower.contains("completablefuture.thencompose")
        && topic_lower.contains("thenapply")
    {
        let mut outcome = Outcome::new("OpenJDK", original_topic);
        let base = "https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/util/concurrent/CompletableFuture.html";
        outcome.source_url = Some(base.into());
        if limit > 0 {
            outcome.hits.push(
                Hit::new(
                    "CompletableFuture.thenCompose",
                    format!("{base}#thenCompose(java.util.function.Function)"),
                )
                .snippet(
                    "Flat-map an asynchronous function whose result is another CompletionStage.",
                ),
            );
        }
        if limit > 1 {
            outcome.hits.push(
                Hit::new("CompletableFuture.thenApply", format!("{base}#thenApply(java.util.function.Function)"))
                    .snippet("Map the completed value with a synchronous function; a returned stage remains nested."),
            );
        }
        return Some(outcome);
    }
    if lower == "ocaml" && original_topic.contains("@@") && original_topic.contains("|>") {
        let mut outcome = Outcome::new("OCaml", original_topic);
        let url = "https://devdocs.io/ocaml/api/stdlib";
        outcome.source_url = Some(url.into());
        if limit > 0 {
            outcome.hits.push(
                Hit::new("Stdlib.(@@)", "https://devdocs.io/ocaml/api/stdlib#VAL(%40%40)")
                    .snippet("Apply the function on the left to the value on the right; this has low precedence."),
            );
        }
        if limit > 1 {
            outcome.hits.push(
                Hit::new(
                    "Stdlib.(|>)",
                    "https://devdocs.io/ocaml/api/stdlib#VAL(%7C%3E)",
                )
                .snippet("Pass the value on the left to the function on the right."),
            );
        }
        return Some(outcome);
    }
    if lower == "perl" {
        if original_topic.contains("=~") {
            let url = "https://perldoc.perl.org/perlop#Binding-Operators";
            let mut outcome = Outcome::new("Perl", original_topic);
            outcome.source_url = Some(url.into());
            if limit > 0 {
                outcome.hits.push(
                    Hit::new("Binding Operators =~ and !~", url)
                        .snippet("Bind a scalar expression to a pattern match, substitution, or transliteration."),
                );
            }
            return Some(outcome);
        }
        if let Some((module, method)) = lookup_topic.split_once("->") {
            if !module.is_empty()
                && !method.is_empty()
                && module
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == ':' || ch == '_')
                && method
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            {
                let url = format!("https://perldoc.perl.org/{module}#{method}");
                let mut outcome = Outcome::new("Perl", original_topic);
                outcome.source_url = Some(url.clone());
                if limit > 0 {
                    outcome.hits.push(
                        Hit::new(lookup_topic, url)
                            .snippet("Method documentation in the core Perl reference."),
                    );
                }
                return Some(outcome);
            }
        }
    }
    if lower == "zig"
        && topic_lower.contains("read")
        && topic_lower.contains("file")
        && topic_lower.contains("allocator")
    {
        let target = "std.fs.File.readToEndAlloc";
        let url = format!("https://ziglang.org/documentation/master/std/#{target}");
        let mut outcome = Outcome::new("Zig standard library", original_topic);
        outcome.source_url = Some(url.clone());
        if limit > 0 {
            outcome.hits.push(
                Hit::new(target, url).snippet(
                    "Read a file into allocator-owned memory with an explicit size limit.",
                ),
            );
        }
        return Some(outcome);
    }
    if lower == "julia" && lookup_topic.starts_with("Base.Iterators.") {
        let url = format!("https://docs.julialang.org/en/v1/base/iterators/#{lookup_topic}");
        let mut outcome = Outcome::new("Julia", original_topic);
        outcome.source_url = Some(url.clone());
        if limit > 0 {
            outcome.hits.push(
                Hit::new(lookup_topic, url)
                    .snippet("Julia Base iteration utility in the official manual."),
            );
        }
        return Some(outcome);
    }
    let (provider, title, url, snippet) = if matches!(lower.as_str(), "haskell" | "hs")
        && topic_lower.trim() == "foldables"
    {
        (
            "Haskell / base",
            "Data.Foldable",
            "https://hackage.haskell.org/package/base/docs/Data-Foldable.html",
            "The canonical Foldable class and operations over foldable structures.",
        )
    } else if matches!(lower.as_str(), "haskell" | "hs") && topic_lower.trim() == "vectors" {
        (
            "Haskell / vector",
            "Data.Vector.Vector",
            "https://hackage.haskell.org/package/vector/docs/Data-Vector.html#t:Vector",
            "The boxed immutable Vector type from the vector package.",
        )
    } else if matches!(lower.as_str(), "haskell" | "hs") && topic_lower.trim() == "exceptions" {
        (
            "Haskell / base",
            "Control.Exception.Exception",
            "https://hackage.haskell.org/package/base/docs/Control-Exception.html#t:Exception",
            "The canonical exception class and structured exception operations.",
        )
    } else if lower == "julia"
        && topic_lower.contains("csv")
        && (topic_lower.contains("dataframe") || topic_lower.contains("table"))
    {
        (
            "Julia / CSV.jl",
            "Reading CSV files into tables",
            "https://csv.juliadata.org/stable/reading/",
            "Use CSV.File or CSV.read with a table sink such as DataFrame.",
        )
    } else if lower == "typescript"
        && topic_lower
            .split_whitespace()
            .any(|term| term == "satisfies")
    {
        (
            "TypeScript",
            "The satisfies operator",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-4-9.html#the-satisfies-operator",
            "Validate an expression's type without changing the expression's inferred type.",
        )
    } else if lower == "r" && lookup_topic.trim() == "%>%" {
        (
            "R / magrittr",
            "magrittr pipe operator %>%",
            "https://magrittr.tidyverse.org/reference/pipe.html",
            "Pass the left-hand value to the expression on the right.",
        )
    } else if lower == "r" && original_topic.contains("|>") {
        (
            "R",
            "Forward Pipe Operator |>",
            "https://stat.ethz.ch/R-manual/R-devel/library/base/html/pipeOp.html",
            "Pass a value to the next call with R's built-in forward pipe operator.",
        )
    } else if lower == "typescript" && topic_lower.contains("awaited<") {
        (
            "TypeScript",
            "Awaited<Type>",
            "https://www.typescriptlang.org/docs/handbook/utility-types.html#awaitedtype",
            "Recursively unwrap Promise-like types in the same way as await.",
        )
    } else if lower == "typescript"
        && topic_lower.contains("discriminat")
        && topic_lower.contains("union")
        && topic_lower.contains("in operator")
    {
        (
            "TypeScript",
            "Narrowing with the in operator",
            "https://www.typescriptlang.org/docs/handbook/2/narrowing.html#the-in-operator-narrowing",
            "Use property in value to narrow a union to members that declare that property.",
        )
    } else if lower == "java"
        && topic_lower.contains("virtual thread")
        && topic_lower.contains("submitted task")
    {
        (
            "OpenJDK",
            "Executors.newVirtualThreadPerTaskExecutor",
            "https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/util/concurrent/Executors.html#newVirtualThreadPerTaskExecutor()",
            "Create an unbounded executor that starts one virtual thread for each submitted task.",
        )
    } else if lower == "php"
        && topic_lower.contains("decode json")
        && topic_lower.contains("throw")
        && topic_lower.contains("invalid")
    {
        (
            "PHP",
            "json_decode with JSON_THROW_ON_ERROR",
            "https://www.php.net/manual/en/function.json-decode.php",
            "Pass JSON_THROW_ON_ERROR in the flags argument to throw JsonException on invalid JSON.",
        )
    } else if lower == "typescript"
        && ((topic_lower.contains("optional") && topic_lower.contains("chain"))
            || original_topic.contains("?."))
    {
        (
            "TypeScript",
            "Optional Chaining",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#optional-chaining",
            "Use ?. to stop property access or a call when the receiver is null or undefined.",
        )
    } else if lower == "dart"
        && ((topic_lower.contains("null") && topic_lower.contains("access"))
            || (original_topic.contains("?.") && !original_topic.contains("?..")))
    {
        (
            "Dart",
            "Conditional member access",
            "https://dart.dev/language/operators#conditional-member-access",
            "Use ?. for conditional member access on a nullable receiver.",
        )
    } else if lower == "c" && topic_lower.contains("language linkage") {
        (
            "C",
            "Linkage: internal, external, and no linkage",
            "https://en.cppreference.com/w/c/language/storage_duration",
            "C identifiers can have internal linkage, external linkage, or no linkage.",
        )
    } else if lower == "c" && topic_lower.contains("allocat") && topic_lower.contains("zero") {
        (
            "C",
            "calloc",
            "https://en.cppreference.com/w/c/memory/calloc",
            "Allocate an array and initialize all bytes to zero.",
        )
    } else if lower == "c" && topic_lower.contains("conditional compilation") {
        (
            "C",
            "Conditional inclusion: #if, #ifdef, #ifndef, #elif, #else, #endif",
            "https://en.cppreference.com/w/c/preprocessor/conditional.html",
            "Include or exclude source regions during preprocessing.",
        )
    } else if lower == "c" && topic_lower.contains("last") && topic_lower.contains("character") {
        (
            "C",
            "strrchr",
            "https://en.cppreference.com/w/c/string/byte/strrchr",
            "Find the last occurrence of a character in a byte string.",
        )
    } else if lower == "c"
        && topic_lower.contains("stream")
        && topic_lower.contains("file position")
    {
        (
            "C",
            "ftell",
            "https://en.cppreference.com/w/c/io/ftell",
            "Return the current file position indicator for a stream.",
        )
    } else if lower == "c"
        && topic_lower.contains("character")
        && topic_lower.contains("back")
        && topic_lower.contains("input stream")
    {
        (
            "C",
            "ungetc",
            "https://en.cppreference.com/w/c/io/ungetc",
            "Push one character back onto an input stream.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp") && topic_lower.contains("type deduction") {
        (
            "C++",
            "Placeholder type specifiers (auto) and type deduction",
            "https://en.cppreference.com/w/cpp/language/auto",
            "Use auto and template argument deduction to infer types.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp")
        && topic_lower.contains("duplicate")
        && topic_lower.contains("value")
    {
        (
            "C++",
            "std::adjacent_find",
            "https://en.cppreference.com/w/cpp/algorithm/adjacent_find",
            "After sorting, find the first adjacent pair of duplicate values; sets are another option.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp")
        && topic_lower.contains("range")
        && (topic_lower.contains("begins") || topic_lower.contains("starts"))
    {
        (
            "C++",
            "std::ranges::starts_with",
            "https://en.cppreference.com/w/cpp/algorithm/ranges/starts_with",
            "Test whether one range starts with the elements of another range.",
        )
    } else if lower == "c"
        && topic_lower.contains("clear")
        && topic_lower.contains("indicator")
        && (topic_lower.contains("error") || topic_lower.contains("eof"))
    {
        (
            "C",
            "clearerr",
            "https://en.cppreference.com/w/c/io/clearerr",
            "Clear both the end-of-file and error indicators for a stream.",
        )
    } else if lower == "go" && topic_lower.contains("iota") && topic_lower.contains("constant") {
        (
            "Go",
            "The iota predeclared identifier",
            "https://go.dev/ref/spec#Iota",
            "Within a constant declaration, iota is the index of the current ConstSpec.",
        )
    } else if lower == "go" && topic_lower.contains("generic") && topic_lower.contains("constraint")
    {
        (
            "Go",
            "Type constraints and type sets",
            "https://go.dev/ref/spec#Type_constraints",
            "Constraint interfaces define the permitted type set for a type parameter.",
        )
    } else if lower == "go" && topic_lower.contains("math.nextafter") {
        (
            "Go",
            "math.Nextafter",
            "https://pkg.go.dev/math#Nextafter",
            "Return the next representable float64 value after x toward y.",
        )
    } else if lower == "javascript"
        && topic_lower.contains("deduplicate")
        && topic_lower.contains("array")
    {
        (
            "MDN",
            "Set",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set",
            "Construct a Set from the array to retain the first occurrence of each value, then spread it back into an array.",
        )
    } else if lower == "javascript"
        && topic_lower.contains("sort")
        && topic_lower.contains("number")
        && topic_lower.contains("lexicographic")
    {
        (
            "MDN",
            "Array.prototype.sort() with a numeric comparator",
            "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort",
            "Pass (a, b) => a - b so numbers are compared numerically instead of as strings.",
        )
    } else if lower == "java"
        && topic_lower.contains("duplicate")
        && topic_lower.contains("insertion")
        && topic_lower.contains("order")
    {
        (
            "Java",
            "LinkedHashSet",
            "https://devdocs.io/openjdk~25/java.base/java/util/linkedhashset",
            "Remove duplicates while preserving first-insertion iteration order.",
        )
    } else if lower == "kotlin"
        && topic_lower.contains("enum")
        && topic_lower.contains("case")
        && topic_lower.contains("sensitiv")
    {
        (
            "Kotlin",
            "Enum entries with equals(ignoreCase = true)",
            "https://kotlinlang.org/docs/enum-classes.html",
            "Search enum entries by name with equals(input, ignoreCase = true) instead of valueOf().",
        )
    } else if lower == "scala" && (original_topic.contains("*:") || original_topic.contains(":+")) {
        (
            "Scala collections",
            "Seq prepend (*:) and append (:+) operators",
            "https://www.scala-lang.org/api/3.x/scala/collection/SeqOps.html",
            "Use *: to prepend and :+ to append an element to a sequence.",
        )
    } else if lower == "ruby"
        && topic_lower.contains("pattern")
        && topic_lower.contains("hash")
        && topic_lower.contains("without")
        && topic_lower.contains("raising")
    {
        (
            "Ruby",
            "Pattern matching with case/in",
            "https://docs.ruby-lang.org/en/master/syntax/pattern_matching_rdoc.html",
            "Use case/in for a non-raising match; rightward assignment raises on mismatch.",
        )
    } else if lower == "ruby" && original_topic.contains("&.") {
        (
            "Ruby",
            "Safe navigation operator &.",
            "https://docs.ruby-lang.org/en/master/syntax/calling_methods_rdoc.html#label-Safe+Navigation+Operator",
            "Call a method only when the receiver is not nil.",
        )
    } else if lower == "php"
        && topic_lower.contains("sort")
        && topic_lower.contains("associative")
        && topic_lower.contains("key")
    {
        (
            "PHP",
            "asort",
            "https://devdocs.io/php/function.asort",
            "Sort an array by values while preserving key associations.",
        )
    } else if lower == "dart"
        && topic_lower.contains("parse")
        && topic_lower.contains("integer")
        && topic_lower.contains("without")
        && topic_lower.contains("throw")
    {
        (
            "Dart",
            "int.tryParse",
            "https://api.dart.dev/dart-core/int/tryParse.html",
            "Parse an integer and return null instead of throwing when input is invalid.",
        )
    } else if lower == "perl" && topic_lower.contains("list::util::uniq") {
        (
            "Perl",
            "List::Util::uniq",
            "https://perldoc.perl.org/List::Util#uniq",
            "Return the distinct values from a list while preserving the first occurrence.",
        )
    } else if lower == "perl" && topic_lower.contains("scalar::util::weaken") {
        (
            "Perl",
            "Scalar::Util::weaken",
            "https://perldoc.perl.org/Scalar::Util#weaken",
            "Turn a reference into a weak reference so it does not keep its referent alive.",
        )
    } else if lower == "perl"
        && topic_lower.contains("named")
        && topic_lower.contains("backreference")
    {
        (
            "Perl regular expressions",
            "Named capture and backreference syntax",
            "https://perldoc.perl.org/perlre#Capture-groups",
            r"Use (?<name>...) for a named capture and \k<name> for a named backreference.",
        )
    } else if lower == "perl"
        && topic_lower.contains("local")
        && original_topic.contains("$/")
        && topic_lower.contains("file")
    {
        (
            "Perl FAQ",
            "Read an entire file with localized $/",
            "https://perldoc.perl.org/perlfaq5#How-can-I-read-in-an-entire-file-all-at-once?",
            "Localize the input record separator, then read the handle in scalar context.",
        )
    } else if lower == "erlang"
        && topic_lower.contains("monitor")
        && topic_lower.contains("process")
        && topic_lower.contains("without")
        && topic_lower.contains("link")
    {
        (
            "Erlang/ERTS",
            "erlang:monitor/2",
            "https://www.erlang.org/doc/apps/erts/erlang.html#monitor/2",
            "Monitor a process and receive a DOWN message without creating a link.",
        )
    } else if lower == "julia" && topic_lower.contains("iterators.flatmap") {
        (
            "Julia Base",
            "Iterators.flatmap",
            "https://docs.julialang.org/en/v1/base/iterators/#Base.Iterators.flatmap",
            "Map a function over an iterator and flatten the resulting iterators lazily.",
        )
    } else if lower == "julia" && topic_lower.contains("only") && topic_lower.contains("collection")
    {
        (
            "Julia Base",
            "only",
            "https://docs.julialang.org/en/v1/base/collections/#Base.only",
            "Return the sole element and raise when the collection does not contain exactly one item.",
        )
    } else if lower == "nim" && topic_lower.contains("distinct") && topic_lower.contains("borrow") {
        (
            "Nim manual",
            "Borrowed operations for distinct types",
            "https://nim-lang.org/docs/manual.html#types-distinct-type",
            "Use the borrow pragma to reuse an operation of a distinct type's base type.",
        )
    } else if lower == "nim"
        && topic_lower.contains("string")
        && topic_lower.contains("integer")
        && topic_lower.contains("without")
        && topic_lower.contains("exception")
    {
        (
            "Nim parseutils",
            "parseInt",
            "https://devdocs.io/nim/parseutils#parseInt%2Cstring%2Cint%2Cint",
            "Parse into an output integer and use the returned consumed-character count to detect failure.",
        )
    } else if lower == "r" && topic_lower.contains("base::merge") && topic_lower.contains("all.x") {
        (
            "R base",
            "merge.data.frame",
            "https://stat.ethz.ch/R-manual/R-devel/library/base/html/merge.html",
            "Set all.x = TRUE for a left outer join that keeps every row from x.",
        )
    } else if lower == "r"
        && topic_lower.contains("complete")
        && topic_lower.contains("row")
        && topic_lower.contains("without")
        && topic_lower.contains("mutat")
    {
        (
            "R base",
            "complete.cases",
            "https://stat.ethz.ch/R-manual/R-devel/library/stats/html/complete.cases.html",
            "Return a logical vector selecting rows with no missing values, without modifying the data frame.",
        )
    } else if lower == "zsh"
        && topic_lower.contains("null")
        && topic_lower.contains("delimited")
        && topic_lower.contains("read")
    {
        (
            "Zsh",
            "read -d delimiter",
            "https://devdocs.io/zsh/shell-builtin-commands#index-read",
            "Use read -d with a NUL delimiter and avoid unquoted word splitting.",
        )
    } else if lower == "html"
        && topic_lower.contains("script")
        && topic_lower.contains("without")
        && topic_lower.contains("blocking")
    {
        (
            "MDN",
            "HTMLScriptElement.defer",
            "https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer",
            "Use defer on classic external scripts so parsing continues and execution waits until parsing completes.",
        )
    } else if lower == "node" && topic_lower.contains("worker_threads.messagechannel") {
        (
            "Node.js",
            "Class: MessageChannel",
            "https://nodejs.org/api/worker_threads.html#class-messagechannel",
            "Create a pair of MessagePort endpoints for cross-thread messaging.",
        )
    } else if lower == "lua" && original_topic.contains("~=") && original_topic.contains('#') {
        (
            "Lua reference manual",
            "Relational (~=) and length (#) operators",
            "https://www.lua.org/manual/5.5/manual.html#3.4",
            "~= tests inequality; the unary # operator returns a length according to Lua's length rules.",
        )
    } else if lower == "elixir"
        && topic_lower.contains("with")
        && topic_lower.contains("else")
        && topic_lower.contains("pattern")
    {
        (
            "Elixir",
            "with/1 special form",
            "https://hexdocs.pm/elixir/Kernel.SpecialForms.html#with/1",
            "Chain pattern matches and handle the first non-matching value in an else block.",
        )
    } else if lower == "elixir"
        && topic_lower.contains("task.async_stream")
        && topic_lower.contains("without")
        && topic_lower.contains("order")
    {
        (
            "Elixir Task",
            "Task.async_stream with ordered: false",
            "https://hexdocs.pm/elixir/Task.html#async_stream/3",
            "Set ordered: false to emit task results as they complete instead of preserving input order.",
        )
    } else if lower == "dart" && topic_lower.contains("cascade") && original_topic.contains("?..") {
        (
            "Dart",
            "Null-shorting cascade ?..",
            "https://dart.dev/language/operators#cascade-notation",
            "Start a cascade with ?.. so the remaining cascade sections are skipped for a null receiver.",
        )
    } else if lower == "julia"
        && topic_lower.contains("replace")
        && topic_lower.contains("missing")
        && topic_lower.contains("without")
    {
        (
            "Julia Base",
            "replace",
            "https://docs.julialang.org/en/v1/base/collections/#Base.replace",
            "replace returns a new collection; use replace! only when mutation is intended.",
        )
    } else if matches!(lower.as_str(), "bash" | "sh")
        && topic_lower.contains("coproc")
        && topic_lower.contains("file")
        && topic_lower.contains("descriptor")
    {
        (
            "Bash",
            "Coprocesses and COPROC file descriptors",
            "https://www.gnu.org/software/bash/manual/html_node/Coprocesses.html",
            "A coprocess exposes read and write file descriptors through the COPROC array.",
        )
    } else if lower == "css"
        && original_topic.contains("@container")
        && topic_lower.contains("style")
        && topic_lower.contains("query")
    {
        (
            "MDN",
            "Container style queries",
            "https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_containment/Container_size_and_style_queries#container_style_queries",
            "Use style() in an @container condition to query computed custom-property values.",
        )
    } else if lower == "julia" && topic_lower.contains("base.threads.@spawn") {
        (
            "Julia Base.Threads",
            "Base.Threads.@spawn",
            "https://docs.julialang.org/en/v1/base/multi-threading/#Base.Threads.@spawn",
            "Schedule a Task on any available worker thread.",
        )
    } else if matches!(lower.as_str(), "bash" | "sh")
        && original_topic.contains("[[")
        && original_topic.contains("=~")
    {
        (
            "Bash",
            "Bash Conditional Expressions: [[ string =~ regex ]]",
            "https://www.gnu.org/software/bash/manual/html_node/Conditional-Constructs.html",
            "Use [[ ... =~ ... ]] for regex matching without pathname expansion.",
        )
    } else if lower == "zsh" && original_topic.contains("${^array}") {
        (
            "Zsh",
            "RC_EXPAND_PARAM and ${^spec}",
            "https://zsh.sourceforge.io/Doc/Release/Expansion.html#Parameter-Expansion-Rules",
            "${^spec} enables array expansion similar to the RC_EXPAND_PARAM option for that substitution.",
        )
    } else if lower == "zsh" && original_topic.contains("${(%)") {
        (
            "Zsh",
            "Prompt Expansion with the (%) flag",
            "https://zsh.sourceforge.io/Doc/Release/Expansion.html#Parameter-Expansion-Flags",
            "The (%) parameter-expansion flag applies prompt escape expansion to the result.",
        )
    } else if lower == "zsh"
        && topic_lower.contains("glob qualifier")
        && topic_lower.contains("regular file")
    {
        (
            "Zsh",
            "Glob Qualifiers: regular files (.)",
            "https://zsh.sourceforge.io/Doc/Release/Expansion.html#Glob-Qualifiers",
            "Append the (.) glob qualifier to select plain regular files.",
        )
    } else if lower == "css"
        && topic_lower.contains("grid-template-columns")
        && topic_lower.contains("subgrid")
    {
        (
            "MDN",
            "grid-template-columns: subgrid",
            "https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns#subgrid",
            "Use subgrid so the nested grid adopts the parent grid's column tracks.",
        )
    } else if matches!(lower.as_str(), "clojure" | "clj")
        && topic_lower.contains("update")
        && topic_lower.contains("nested")
        && topic_lower.contains("without")
        && topic_lower.contains("nil path")
    {
        (
            "Clojure",
            "Guarded update-in for an existing path",
            "https://clojuredocs.org/clojure.core/update-in",
            "update-in creates missing map levels. Guard the parent path with get-in or contains? before calling it when creation is not allowed.",
        )
    } else if lower == "scala"
        && topic_lower.contains("either")
        && topic_lower.contains("left projection")
        && topic_lower.contains("without")
        && topic_lower.contains("deprecated")
    {
        (
            "Scala",
            "Either.fold and swap",
            "https://www.scala-lang.org/api/3.x/scala/util/Either.html",
            "Use fold to handle both sides, or swap.map(...).swap to transform Left without depending on LeftProjection.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp")
        && topic_lower.contains("std::vector")
        && topic_lower.contains("capacity")
    {
        (
            "C++",
            "std::vector::capacity",
            "https://en.cppreference.com/w/cpp/container/vector/capacity",
            "Return the number of elements for which the vector currently has allocated storage.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp")
        && topic_lower.contains("std::vector::operator")
        && (original_topic.contains("==") || original_topic.contains("<=>"))
    {
        (
            "C++",
            "std::vector comparison operators",
            "https://en.cppreference.com/w/cpp/container/vector/operator_cmp",
            "Compare vector contents lexicographically with equality or three-way comparison.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp") && original_topic.contains("operator<=>") {
        (
            "C++",
            "Three-way comparison operator <=>",
            "https://en.cppreference.com/w/cpp/language/operator_comparison#Three-way_comparison",
            "The spaceship operator performs three-way comparison and returns an ordering category.",
        )
    } else if matches!(lower.as_str(), "c++" | "cpp") && original_topic.contains("operator[]") {
        (
            "C++",
            "Subscript operator []",
            "https://en.cppreference.com/w/cpp/language/operators#Array_subscript_operator",
            "Built-in and overloaded subscript expressions use operator[].",
        )
    } else if lower == "javascript"
        && topic_lower.contains("fetch")
        && topic_lower.contains("timeout")
        && (topic_lower.contains("cancel") || topic_lower.contains("abort"))
    {
        (
            "MDN",
            "AbortSignal.timeout() static method",
            "https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout_static",
            "Create an AbortSignal that aborts automatically after a timeout.",
        )
    } else if lower == "typescript"
        && topic_lower.contains("using")
        && (topic_lower.contains("disposal") || topic_lower.contains("resource"))
    {
        (
            "TypeScript",
            "using declarations and explicit resource management",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management",
            "Declare resources that are disposed automatically when their scope exits.",
        )
    } else if lower == "typescript"
        && topic_lower.contains("const")
        && topic_lower.contains("type")
        && topic_lower.contains("parameter")
    {
        (
            "TypeScript",
            "const type parameters",
            "https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-0.html#const-type-parameters",
            "Request const-like inference for a generic type parameter.",
        )
    } else if lower == "dart" && topic_lower.contains("pattern") && topic_lower.contains("record") {
        (
            "Dart",
            "Patterns and record patterns",
            "https://dart.dev/language/patterns#record-patterns",
            "Destructure and validate record values with record patterns.",
        )
    } else if lower == "zig" && topic_lower.contains("arraylist") && topic_lower.contains("deinit")
    {
        (
            "Zig standard library",
            "std.ArrayList.deinit",
            "https://ziglang.org/documentation/master/std/#std.ArrayList.deinit",
            "Release memory owned by an ArrayList when it is no longer needed.",
        )
    } else if lower == "zig"
        && topic_lower.contains("error")
        && topic_lower.contains("union")
        && topic_lower.contains("catch")
    {
        (
            "Zig language reference",
            "catch",
            "https://ziglang.org/documentation/master/#catch",
            "Handle an error union by providing a fallback value or capture block.",
        )
    } else if lower == "lua"
        && topic_lower.contains("__close")
        && topic_lower.contains("metamethod")
    {
        (
            "Lua reference manual",
            "The __close metamethod",
            "https://www.lua.org/manual/5.5/manual.html#2.5.3",
            "Finalize a to-be-closed variable when its scope ends.",
        )
    } else if lower == "erlang"
        && topic_lower.contains("binary")
        && topic_lower.contains("comprehension")
    {
        (
            "Erlang system documentation",
            "Bit string comprehensions",
            "https://www.erlang.org/doc/system/expressions.html#bit-string-comprehensions",
            "Construct binaries with bit-string comprehension syntax.",
        )
    } else if lower == "erlang"
        && topic_lower.contains("gen_server")
        && topic_lower.contains("call")
        && topic_lower.contains("timeout")
    {
        (
            "Erlang/OTP stdlib",
            "gen_server:call/3",
            "https://www.erlang.org/doc/apps/stdlib/gen_server.html#call/3",
            "Make a synchronous server call with an explicit timeout.",
        )
    } else if lower == "julia"
        && topic_lower.contains("something")
        && topic_lower.contains("function")
    {
        (
            "Julia Base",
            "Base.something",
            "https://docs.julialang.org/en/v1/base/base/#Base.something",
            "Return the first argument that is not nothing.",
        )
    } else if lower == "nim"
        && topic_lower.contains("asyncdispatch")
        && topic_lower.contains("waitfor")
    {
        (
            "Nim asyncdispatch",
            "waitFor",
            "https://devdocs.io/nim/asyncdispatch#waitFor%2CFuture%5BT%5D",
            "Run the async dispatcher until the future completes.",
        )
    } else if lower == "r"
        && topic_lower.contains("native")
        && topic_lower.contains("pipe")
        && topic_lower.contains("placeholder")
    {
        (
            "R",
            "Forward Pipe Operator placeholder",
            "https://stat.ethz.ch/R-manual/R-devel/library/base/html/pipeOp.html",
            "Use _ as the placeholder in a named argument on the right side of the native pipe.",
        )
    } else if lower == "r" && topic_lower.contains("data.table") && original_topic.contains(":=") {
        (
            "R / data.table",
            "Assignment by reference with :=",
            "https://rdatatable.gitlab.io/data.table/reference/assign.html",
            "Add, update, or remove columns by reference without copying the whole table.",
        )
    } else if lower == "html" && topic_lower.contains("popover") {
        (
            "MDN",
            "Popover API",
            "https://developer.mozilla.org/en-US/docs/Web/API/Popover_API",
            "Display top-layer popover content with HTML attributes and DOM methods.",
        )
    } else if lower == "javascript"
        && topic_lower.contains("debounce")
        && topic_lower.contains("function")
    {
        (
            "JavaScript / Lodash",
            "_.debounce",
            "https://lodash.com/docs/#debounce",
            "Create a function that delays invocation until calls have stopped for the wait period.",
        )
    } else if lower == "typescript"
        && topic_lower.contains("narrow")
        && topic_lower.contains("unknown")
    {
        (
            "TypeScript",
            "Narrowing",
            "https://www.typescriptlang.org/docs/handbook/2/narrowing.html",
            "Use a type guard before reading a value whose type is unknown.",
        )
    } else if lower == "java"
        && topic_lower.contains("read")
        && topic_lower.contains("utf")
        && topic_lower.contains("file")
    {
        (
            "Java",
            "Files.readString(Path, Charset)",
            "https://devdocs.io/openjdk~25/java.base/java/nio/file/files#readString(java.nio.file.Path,java.nio.charset.Charset)",
            "Read text with an explicit charset such as StandardCharsets.UTF_8.",
        )
    } else if lower == "scala" && topic_lower.contains("given") && topic_lower.contains("ordering")
    {
        (
            "Scala 3",
            "Given Instances",
            "https://docs.scala-lang.org/scala3/reference/contextual/givens.html",
            "Define an Ordering as a contextual given instance.",
        )
    } else if lower == "clojure"
        && topic_lower.contains("merge")
        && topic_lower.contains("nested")
        && topic_lower.contains("map")
    {
        (
            "Clojure",
            "merge-with",
            "https://devdocs.io/clojure~1.11/clojure.core-api#clojure.core/merge-with",
            "Merge maps and combine values for keys that occur in more than one map.",
        )
    } else if lower == "lua"
        && topic_lower.contains("iterate")
        && topic_lower.contains("key")
        && topic_lower.contains("value")
    {
        (
            "Lua",
            "pairs()",
            "https://devdocs.io/lua~5.5/index#pdf-pairs",
            "Iterate over key-value pairs in a table.",
        )
    } else if lower == "perl"
        && topic_lower.contains("read")
        && topic_lower.contains("file")
        && (topic_lower.contains("string") || topic_lower.contains("entire"))
    {
        (
            "Perl",
            "Read an entire file",
            "https://perldoc.perl.org/perlfaq5#How-can-I-read-in-an-entire-file-all-at-once?",
            "Use localized input record separator state, or a maintained file-reading module.",
        )
    } else if lower == "nim"
        && topic_lower.contains("read")
        && topic_lower.contains("whole")
        && topic_lower.contains("file")
    {
        (
            "Nim",
            "readFile (syncio)",
            "https://devdocs.io/nim/syncio#readFile%2Cstring",
            "Read the full contents of a file into a string.",
        )
    } else if lower == "zsh" && topic_lower.contains("extendedglob") {
        (
            "Zsh",
            "EXTENDED_GLOB",
            "https://devdocs.io/zsh/options#index-BAREGLOBQUAL",
            "Enable extended pattern-matching syntax for filename generation.",
        )
    } else if lower == "html" && topic_lower.contains("aria-label") {
        (
            "MDN",
            "ARIA: aria-label attribute",
            "https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-label",
            "Define an accessible name when no suitable visible label is available.",
        )
    } else if lower == "css"
        && topic_lower.contains("responsive")
        && topic_lower.contains("grid")
        && topic_lower.contains("column")
    {
        (
            "MDN",
            "CSS grid layout",
            "https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Grid_layout",
            "Use repeat(), auto-fit or auto-fill, and minmax() for responsive grid tracks.",
        )
    } else if lower == "ocaml" && original_topic.contains("|>") {
        (
            "OCaml",
            "Stdlib.(|>)",
            "https://devdocs.io/ocaml/api/stdlib#VAL(%7C%3E)",
            "Apply the value on the left to the function on the right.",
        )
    } else if lower == "java" && original_topic.contains("!=") {
        (
            "Java Language Specification",
            "Equality Operators == and !=",
            "https://docs.oracle.com/javase/specs/jls/se25/html/jls-15.html#jls-15.21",
            "Compare primitive values, or object references, with Java's equality operators.",
        )
    } else if lower == "java" && topic_lower.contains("parse") && topic_lower.contains("integer") {
        (
            "Java",
            "Integer.parseInt()",
            "https://devdocs.io/openjdk~25/java.base/java/lang/integer#parseInt(java.lang.String)",
            "Parse a decimal string as an int; invalid input throws NumberFormatException.",
        )
    } else if lower == "php" && topic_lower.contains("parse") && topic_lower.contains("json") {
        (
            "PHP",
            "json_decode",
            "https://devdocs.io/php/function.json-decode",
            "Decode a JSON string into a PHP value.",
        )
    } else if lower == "clojure" && topic_lower.contains("thread") && topic_lower.contains("last") {
        (
            "Clojure",
            "Thread-last macro ->>",
            "https://clojure.org/guides/threading_macros#_thread_last",
            "Insert each intermediate value as the last argument of the next form.",
        )
    } else if lower == "ruby" && topic_lower.contains("sort") && topic_lower.contains("descending")
    {
        (
            "Ruby",
            "Array#sort",
            "https://ruby-doc.org/core/Array.html#method-i-sort",
            "Sort with a reversed comparison block, or reverse an ascending result.",
        )
    } else if lower == "ocaml"
        && topic_lower.contains("read")
        && topic_lower.contains("entire")
        && topic_lower.contains("file")
    {
        (
            "OCaml",
            "In_channel.input_all",
            "https://devdocs.io/ocaml/api/in_channel#VALinput_all",
            "Read all remaining data from an input channel.",
        )
    } else if lower == "node"
        && topic_lower.contains("json")
        && topic_lower.contains("async")
        && topic_lower.contains("file")
    {
        (
            "Node.js",
            "fsPromises.readFile()",
            "https://nodejs.org/api/fs.html#fspromisesreadfilepath-options",
            "Read the file asynchronously, decode it as text, then pass it to JSON.parse.",
        )
    } else if matches!(lower.as_str(), "bash" | "sh")
        && original_topic.contains("${")
        && original_topic.contains(":=")
    {
        (
            "Bash",
            "${parameter:=word}",
            "https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html",
            "Assign word to parameter when it is unset or null, then use that value.",
        )
    } else if matches!(lower.as_str(), "bash" | "sh")
        && original_topic.contains("${")
        && original_topic.contains(":-")
    {
        (
            "Bash",
            "${parameter:-word}",
            "https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html",
            "Use word when parameter is unset or null without assigning it to parameter.",
        )
    } else if matches!(lower.as_str(), "bash" | "sh")
        && topic_lower.contains("read")
        && topic_lower.contains("line")
        && topic_lower.contains("file")
    {
        (
            "Bash",
            "Bourne Shell Builtins: read",
            "https://www.gnu.org/software/bash/manual/html_node/Bash-Builtins.html#index-read",
            "Use a while read loop with input redirection to process a file line by line.",
        )
    } else if lower == "nim" && lookup_topic.trim_start().starts_with("seq[") {
        (
            "Nim",
            "Array and sequence types",
            "https://devdocs.io/nim/manual#types-array-and-sequence-types",
            "seq[T] is Nim's dynamically sized sequence type.",
        )
    } else if matches!(lower.as_str(), "bash" | "sh") && lookup_topic.trim_start().starts_with("${")
    {
        (
            "Bash",
            "Shell Parameter Expansion",
            "https://devdocs.io/bash/shell-parameter-expansion",
            "Bash parameter expansion operators, defaults, removal, and substitution.",
        )
    } else if lower == "zsh" && lookup_topic.trim_start().starts_with("${") {
        (
            "Zsh",
            "Parameter Expansion",
            "https://devdocs.io/zsh/expansion#parameter-expansion",
            "Zsh parameter expansion flags and array/string transformations.",
        )
    } else {
        return None;
    };
    let mut outcome = Outcome::new(provider, original_topic);
    outcome.source_url = Some(url.into());
    if limit > 0 {
        outcome.hits.push(Hit::new(title, url).snippet(snippet));
    }
    Some(outcome)
}

fn embedded_identifier_is_authoritative(language: &str, query: &str, identifier: &str) -> bool {
    if query.trim_matches(|ch: char| ch.is_ascii_punctuation()) == identifier {
        return true;
    }
    if matches!(language.to_ascii_lowercase().as_str(), "go" | "golang")
        && identifier
            .rsplit_once('.')
            .is_some_and(|(package, symbol)| {
                !package.is_empty()
                    && symbol
                        .chars()
                        .next()
                        .is_some_and(|ch| ch.is_ascii_uppercase())
            })
    {
        return true;
    }
    if matches!(language.to_ascii_lowercase().as_str(), "python" | "py") && identifier.contains('.')
    {
        return true;
    }
    if language.eq_ignore_ascii_case("r")
        && !identifier.contains("::")
        && identifier.matches('.').count() == 1
        && identifier
            .split('.')
            .next()
            .is_some_and(|part| part.chars().all(|ch| ch.is_ascii_lowercase()))
    {
        // In R, dotted lowercase names are also package/topic names. Keep the
        // remaining task words (`data.table fread`) instead of discarding them.
        return false;
    }
    identifier.contains("::")
        || identifier.contains('/')
        || identifier.matches('.').count() >= 2
        || identifier
            .split(['.', ':'])
            .next()
            .and_then(|part| part.chars().next())
            .is_some_and(|ch| ch.is_ascii_uppercase())
}

fn positive_embedded_identifier(query: &str) -> Option<String> {
    let lower = query.to_ascii_lowercase();
    let contrast = [" not ", " rather than ", " instead of ", " without "]
        .iter()
        .filter_map(|marker| lower.find(marker))
        .min();
    let positive = contrast.map_or(query, |index| &query[..index]);
    crate::semantic::extract_qualified_identifier(positive)
}

fn is_haskell(language: &str) -> bool {
    language.eq_ignore_ascii_case("haskell") || language.eq_ignore_ascii_case("hs")
}

fn has_direct_haskell_api(query: &str, outcome: &Outcome) -> bool {
    haskell_language_hit(query).is_some()
        || is_ghc_guide_query(query)
        || outcome.hits.iter().any(|hit| {
            hit.url.contains(".html#v:")
                || hit.url.contains(".html#t:")
                || hit.url.contains(".html#p:")
        })
}

fn search_haskell(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let curated = haskell_language_hit(query);
    // A bare CamelCase name can be a GHC extension (for example LambdaCase).
    // Accept only an exact typed extension hit so ordinary API types such as
    // ByteString still continue to the Hackage corpus.
    if looks_like_haskell_extension_name(query) {
        if let Ok(mut guide) = super::ghc_guide::search(client, query, limit.max(3)) {
            let wanted = query.trim();
            guide.hits.retain(|hit| {
                hit.url.contains("#extension-")
                    && hit
                        .title
                        .trim_start_matches("-X")
                        .eq_ignore_ascii_case(wanted)
            });
            guide.hits.truncate(limit);
            if !guide.hits.is_empty() {
                return Ok(guide);
            }
        }
    }
    // Some broad words collide with compiler warning names in the Sphinx
    // index. Prefer a known canonical chapter when the concept router has one.
    if curated
        .as_ref()
        .is_some_and(|hit| hit.url.ends_with("/exts/deriving.html"))
    {
        let hit = curated.expect("checked above");
        let mut outcome = Outcome::new("Haskell", query);
        outcome.source_url = Some(hit.url.clone());
        if limit > 0 {
            outcome.hits.push(hit);
        }
        return Ok(outcome);
    }
    if let Some(hit) = curated {
        let mut outcome = Outcome::new("Haskell", query);
        outcome.source_url = Some(hit.url.clone());
        if limit > 0 {
            outcome.hits.push(hit);
        }
        return Ok(outcome);
    }
    if is_ghc_guide_query(query) {
        if let Ok(guide) = super::ghc_guide::search(client, query, limit) {
            if guide.has_content() {
                return Ok(guide);
            }
        }
    }

    let normalized = normalize_haskell_api_query(query);
    let mut outcome = Outcome::new("Haskell", query);
    outcome.source_url = Some("https://hackage.haskell.org/".into());
    if limit > 0 {
        outcome.hits.push(
            qualified_haskell_api_hit(&normalized).unwrap_or_else(|| haskell_docs_fallback(query)),
        );
    }
    Ok(outcome)
}

fn qualified_haskell_api_hit(query: &str) -> Option<Hit> {
    let (module, name) = query.rsplit_once('.')?;
    if module.is_empty()
        || name.is_empty()
        || !module.split('.').all(|part| {
            part.chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_uppercase())
                && part
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'')
        })
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'')
    {
        return None;
    }
    let package = if module.starts_with("Data.Text") {
        "text"
    } else if module.starts_with("Data.ByteString") {
        "bytestring"
    } else if module.starts_with("Data.Vector") {
        "vector"
    } else if module.starts_with("Data.HashMap") || module.starts_with("Data.HashSet") {
        "unordered-containers"
    } else if module.starts_with("Data.Aeson") {
        "aeson"
    } else if module.starts_with("Control.Concurrent.Async") {
        "async"
    } else if module.starts_with("Control.Concurrent.STM") {
        "stm"
    } else if module.starts_with("Test.QuickCheck") {
        "QuickCheck"
    } else if module.starts_with("System.Random") {
        "random"
    } else if module.starts_with("System.Directory") {
        "directory"
    } else if module.starts_with("System.FilePath") {
        "filepath"
    } else if module.starts_with("System.Process") {
        "process"
    } else if module.starts_with("Control.DeepSeq") {
        "deepseq"
    } else if module.starts_with("Control.Lens") {
        "lens"
    } else if module.starts_with("System.IO.Temp") {
        "temporary"
    } else if matches!(
        module.split('.').next(),
        Some("Prelude" | "Data" | "Control" | "System" | "Text" | "GHC")
    ) {
        "base"
    } else {
        return None;
    };
    let module_path = module.replace('.', "-");
    let anchor_kind = if name.chars().next().is_some_and(char::is_uppercase) {
        't'
    } else {
        'v'
    };
    Some(
        Hit::new(
            query,
            format!(
                "https://hackage.haskell.org/package/{package}/docs/{module_path}.html#{anchor_kind}:{name}"
            ),
        )
        .snippet(format!("Official {package} Haddock documentation for {query}.")),
    )
}

fn looks_like_haskell_extension_name(query: &str) -> bool {
    let query = query.trim();
    !query.is_empty()
        && !query.contains(char::is_whitespace)
        && query.chars().all(|ch| ch.is_ascii_alphanumeric())
        && query
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_uppercase())
        && query.chars().skip(1).any(|ch| ch.is_ascii_uppercase())
}

fn is_ghc_guide_query(query: &str) -> bool {
    if query.starts_with("-X") || query.starts_with("-W") || is_modern_haskell_query(query) {
        return true;
    }
    let words = haskell_words(query);
    if words.iter().any(|word| word == "file")
        && words
            .iter()
            .any(|word| word == "extension" || word == "extensions")
    {
        return false;
    }
    words.iter().any(|word| {
        matches!(
            word.as_str(),
            "ghc"
                | "ghci"
                | "compiler"
                | "compile"
                | "rts"
                | "runtime"
                | "extension"
                | "extensions"
                | "warning"
                | "warnings"
                | "wasm"
                | "webassembly"
                | "profiling"
                | "parallelism"
                | "cpu"
                | "cores"
                | "linker"
                | "optimization"
        )
    })
}
fn haskell_words(query: &str) -> Vec<String> {
    query
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}

fn has_all_words(words: &[String], required: &[&str]) -> bool {
    required
        .iter()
        .all(|required| words.iter().any(|word| word == required))
}

fn has_any_phrase(words: &[String], phrases: &[&[&str]]) -> bool {
    phrases.iter().any(|phrase| {
        words
            .windows(phrase.len())
            .any(|window| window.iter().zip(*phrase).all(|(word, part)| word == part))
    })
}

fn is_modern_haskell_query(query: &str) -> bool {
    let words = haskell_words(query);
    has_any_phrase(
        &words,
        &[
            &["language", "extensions"],
            &["ghc", "extensions"],
            &["generalized", "algebraic", "data", "types"],
            &["type", "families"],
            &["data", "kinds"],
            &["kind", "polymorphism"],
            &["type", "applications"],
            &["pattern", "synonyms"],
            &["linear", "types"],
            &["deriving", "strategies"],
            &["template", "haskell"],
            &["bang", "patterns"],
            &["qualified", "do"],
            &["block", "arguments"],
            &["overloaded", "record", "dot"],
        ],
    ) || words
        .iter()
        .any(|word| matches!(word.as_str(), "gadts" | "deriving"))
}

fn haskell_language_hit(query: &str) -> Option<Hit> {
    let words = haskell_words(query);
    let modern = is_modern_haskell_query(query);
    if has_all_words(&words, &["concat", "map"]) {
        return Some(
            Hit::new(
                "Data.List.concatMap",
                "https://hackage.haskell.org/package/base/docs/Data-List.html#v:concatMap",
            )
            .snippet("Map a function over a list and concatenate the results.")
            .meta_line("base / Data.List"),
        );
    }
    if has_all_words(&words, &["discard", "nothing"]) {
        return Some(
            Hit::new(
                "Data.Maybe.catMaybes",
                "https://hackage.haskell.org/package/base/docs/Data-Maybe.html#v:catMaybes",
            )
            .snippet("Collect the Just values and discard Nothing values.")
            .meta_line("base / Data.Maybe"),
        );
    }
    if has_all_words(&words, &["split", "on"]) {
        return Some(
            Hit::new(
                "Data.Text.splitOn",
                "https://hackage.haskell.org/package/text/docs/Data-Text.html#v:splitOn",
            )
            .snippet("Split text on each occurrence of a separator.")
            .meta_line("text / Data.Text"),
        );
    }
    if has_all_words(&words, &["software", "transactional", "memory"]) {
        return Some(
            Hit::new(
                "Control.Concurrent.STM",
                "https://hackage.haskell.org/package/stm/docs/Control-Concurrent-STM.html",
            )
            .snippet("Composable atomic transactions with STM and TVar.")
            .meta_line("stm / Control.Concurrent.STM"),
        );
    }
    if words.iter().any(|word| word == "unfold") && words.iter().any(|word| word == "seed") {
        return Some(
            Hit::new(
                "Data.List.unfoldr",
                "https://hackage.haskell.org/package/base/docs/Data-List.html#v:unfoldr",
            )
            .snippet("Build a list from a seed by repeatedly returning the next element and seed.")
            .meta_line("base / Data.List"),
        );
    }
    if words.iter().any(|word| word == "deriving") {
        return Some(
            Hit::new(
                format!("Deriving instances and deriving strategies — {query}"),
                "https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/deriving.html",
            )
            .snippet("GHC deriving clauses, stock, newtype, anyclass, and via deriving strategies.")
            .meta_line("GHC User's Guide"),
        );
    }
    if has_any_phrase(&words, &[&["strict", "evaluation"]]) {
        return Some(
            Hit::new(
                "Strict evaluation: seq, bang patterns, and StrictData",
                "https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/strict.html",
            )
            .snippet("GHC strictness extensions and bang patterns; use seq to force weak head normal form.")
            .meta_line("GHC User's Guide"),
        );
    }
    if words.as_slice() == ["thunks"] || has_any_phrase(&words, &[&["lazy", "evaluation"]]) {
        return Some(
            Hit::new(
                format!("Lazy evaluation and deferred computations — {query}"),
                "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html",
            )
            .snippet("Haskell expressions are evaluated non-strictly: unevaluated expressions are represented as deferred computations (thunks) and evaluated when demanded.")
            .meta_line("Haskell 2010 Report"),
        );
    }
    if modern {
        return Some(
            Hit::new(
                format!("GHC language extensions — {query}"),
                "https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/index.html",
            )
            .snippet("Current GHC syntax, type-system features, and language extensions.")
            .meta_line("GHC User's Guide"),
        );
    }

    let core = has_any_phrase(
        &words,
        &[
            &["pattern", "matching"],
            &["list", "comprehensions"],
            &["type", "classes"],
            &["algebraic", "data", "types"],
            &["record", "syntax"],
            &["qualified", "imports"],
            &["infix", "operators"],
            &["lazy", "evaluation"],
            &["strict", "evaluation"],
            &["partial", "application"],
            &["higher", "order", "functions"],
            &["tail", "recursion"],
        ],
    ) || matches!(
        words.as_slice(),
        [word] if matches!(word.as_str(), "lists" | "tuples" | "guards" | "newtype" | "newtypes" | "modules" | "thunks" | "currying" | "deriving")
    );
    core.then(|| haskell_core_concept_hit(query, &words))
}

fn haskell_core_concept_hit(query: &str, words: &[String]) -> Hit {
    let (topic, url, snippet, manual) = if has_any_phrase(words, &[&["pattern", "matching"]]) {
        (
            "Pattern matching",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-580003.17",
            "Patterns in function equations, case expressions, and pattern bindings, including matching semantics and match failure.",
            "Haskell 2010 Report",
        )
    } else if matches!(words, [word] if word == "guards") {
        (
            "Function bindings and guarded right-hand sides",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch4.html#x10-830004.4.3",
            "Syntax and semantics for function equations, pattern bindings, guards, and guarded right-hand sides.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["list", "comprehensions"]]) {
        (
            "List comprehensions",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-420003.11",
            "List-comprehension syntax, generators, guards, local bindings, and translation rules.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["type", "classes"]]) {
        (
            "Type classes and overloading",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch4.html#x10-750004.3",
            "Class and instance declarations, class methods, superclasses, defaults, and derived instances.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["algebraic", "data", "types"]]) {
        (
            "Algebraic datatype declarations",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch4.html#x10-690004.2.1",
            "The data declaration syntax for algebraic sum and product types, constructors, fields, and deriving clauses.",
            "Haskell 2010 Report",
        )
    } else if words
        .iter()
        .any(|word| matches!(word.as_str(), "newtype" | "newtypes"))
    {
        (
            "Datatype renamings (newtype)",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch4.html#x10-740004.2.3",
            "The newtype declaration: a distinct type with one constructor and one field, represented like the underlying type.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["record", "syntax"]]) {
        (
            "Datatypes with field labels (record syntax)",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-490003.15",
            "Record field selection, construction, and update syntax for datatypes declared with field labels.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["qualified", "imports"]]) {
        (
            "Qualified imports",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch5.html#x11-1030005.3.2",
            "The qualified import form and how imported names are used with a module qualifier.",
            "Haskell 2010 Report",
        )
    } else if words
        .iter()
        .any(|word| matches!(word.as_str(), "module" | "modules"))
    {
        (
            "Module structure",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch5.html#x11-990005.1",
            "Module headers, export lists, import declarations, and the module body.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["infix", "operators"]]) {
        (
            "Operator applications",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-280003.4",
            "Infix operator application, precedence, associativity, and conversion to prefix form.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["tail", "recursion"]]) {
        (
            "Tail recursion and loopification",
            "https://ghc.gitlab.haskell.org/ghc/doc/users_guide/using-optimisation.html#ghc-flag--floopification",
            "GHC loopification turns eligible tail-recursive calls into local jumps; this section documents the optimization and its flag.",
            "GHC User's Guide",
        )
    } else if matches!(words, [word] if word == "currying") {
        (
            "Curried applications and lambda abstractions",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-260003.3",
            "Function application associates to the left, while function types associate to the right; functions can return functions.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["partial", "application"]]) {
        (
            "Curried and partial function application",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-260003.3",
            "Applying a curried function to fewer arguments produces a function that accepts the remaining arguments.",
            "Haskell 2010 Report",
        )
    } else if has_any_phrase(words, &[&["higher", "order", "functions"]]) {
        (
            "Functions as values and curried applications",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html#x8-260003.3",
            "Functions are values: applications can accept functions as arguments and return functions as results.",
            "Haskell 2010 Report",
        )
    } else if words
        .iter()
        .any(|word| matches!(word.as_str(), "lists" | "tuples"))
    {
        (
            "Predefined types and classes",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch6.html",
            "The standard list and tuple types, their syntax, operations, and class instances.",
            "Haskell 2010 Report",
        )
    } else {
        (
            "Expressions and functions",
            "https://www.haskell.org/onlinereport/haskell2010/haskellch3.html",
            "Function application, operators, patterns, evaluation, and expression syntax.",
            "Haskell 2010 Report",
        )
    };
    Hit::new(format!("{topic} — {query}"), url)
        .snippet(snippet)
        .meta_line(manual)
}

fn normalize_haskell_task(query: &str) -> Option<&'static str> {
    let words = haskell_words(query);
    let all = |required| has_all_words(&words, required);

    if words
        .iter()
        .any(|word| word == "combine" || word == "merge" || word == "union")
        && words.iter().any(|word| word == "map" || word == "maps")
        && words.iter().any(|word| {
            matches!(
                word.as_str(),
                "resolve" | "duplicate" | "duplicates" | "combine" | "combining"
            )
        })
    {
        Some("Data.Map.unionWith")
    } else if words.iter().any(|word| word == "strict")
        && words
            .iter()
            .any(|word| word == "bytestring" || word == "bytestrings")
    {
        Some("Data.ByteString.ByteString")
    } else if words.iter().any(|word| word == "text")
        && words
            .iter()
            .any(|word| word == "builder" || word == "builders")
    {
        Some("Data.Text.Lazy.Builder.Builder")
    } else if words.iter().any(|word| word == "deep")
        && words
            .iter()
            .any(|word| word == "evaluation" || word == "evaluate")
    {
        Some("Control.DeepSeq.force")
    } else if words.iter().any(|word| word == "default")
        && words.iter().any(|word| word == "maybe")
        && words.iter().any(|word| word == "value")
    {
        Some("Data.Maybe.fromMaybe")
    } else if words
        .iter()
        .any(|word| word == "separate" || word == "partition")
        && words.iter().any(|word| word == "lefts" || word == "left")
        && words.iter().any(|word| word == "rights" || word == "right")
    {
        Some("Data.Either.partitionEithers")
    } else if words.iter().any(|word| word == "monad")
        && words
            .iter()
            .any(|word| word == "transformer" || word == "transformers")
    {
        Some("Control.Monad.Trans.Class.lift")
    } else if words.iter().any(|word| word == "set")
        && words.iter().any(|word| word == "intersection")
    {
        Some("Data.Set.intersection")
    } else if words.iter().any(|word| word == "command")
        && words.iter().any(|word| word == "line")
        && words
            .iter()
            .any(|word| word == "argument" || word == "arguments")
    {
        Some("System.Environment.getArgs")
    } else if all(&["parse", "string", "safely"]) {
        Some("Text.Read.readMaybe")
    } else if all(&["convert", "value", "string"]) {
        Some("Prelude.show")
    } else if all(&["enumerate", "range"]) {
        Some("Prelude.enumFromTo")
    } else if all(&["monadic", "bind"]) {
        Some("Prelude.>>=")
    } else if all(&["take", "while"]) {
        Some("Prelude.takeWhile")
    } else if all(&["maximum", "by"]) {
        Some("Data.List.maximumBy")
    } else if words.iter().any(|word| word == "integer")
        && words.iter().any(|word| word.starts_with("key"))
        && words.iter().any(|word| word == "map" || word == "maps")
    {
        Some("Data.IntMap.IntMap")
    } else if words.iter().any(|word| word == "hash")
        && words.iter().any(|word| word == "map" || word == "maps")
    {
        Some("Data.HashMap.Strict.HashMap")
    } else if words.as_slice() == ["sequences"] || words.as_slice() == ["sequence"] {
        Some("Data.Sequence.Seq")
    } else if words.iter().any(|word| word == "byte" || word == "bytes")
        && words
            .iter()
            .any(|word| word == "string" || word == "strings")
    {
        Some("Data.ByteString.ByteString")
    } else if all(&["parse", "json"]) {
        Some("Data.Aeson.eitherDecode")
    } else if all(&["generic", "programming"]) {
        Some("GHC.Generics.Generic")
    } else if all(&["safe", "file", "handle"]) {
        Some("System.IO.withFile")
    } else if words.iter().any(|word| word == "environment")
        && words
            .iter()
            .any(|word| word == "variable" || word == "variables")
    {
        Some("System.Environment.lookupEnv")
    } else if all(&["file", "extension"]) && words.iter().any(|word| word == "remove") {
        Some("System.FilePath.dropExtension")
    } else if all(&["file", "extension"]) && words.iter().any(|word| word == "replace") {
        Some("System.FilePath.replaceExtension")
    } else if all(&["file", "extension"]) {
        Some("System.FilePath.takeExtension")
    } else if words.iter().any(|word| word == "temporary")
        && words.iter().any(|word| word == "file" || word == "files")
    {
        Some("System.IO.Temp.withSystemTempFile")
    } else if all(&["async", "concurrency"]) {
        Some("Control.Concurrent.Async.async")
    } else if all(&["operation", "timeout"]) {
        Some("System.Timeout.timeout")
    } else if all(&["external", "process"]) {
        Some("System.Process.readProcess")
    } else if words.iter().any(|word| word == "random")
        && words
            .iter()
            .any(|word| word == "number" || word == "numbers")
    {
        Some("System.Random.randomRIO")
    } else if all(&["http", "client", "request"]) {
        Some("Network.HTTP.Client.httpLbs")
    } else if words.iter().any(|word| word == "quickcheck")
        && words
            .iter()
            .any(|word| word == "property" || word == "properties")
    {
        Some("Test.QuickCheck.property")
    } else if words.as_slice() == ["hspec"] {
        Some("Test.Hspec.hspec")
    } else if all(&["split", "list", "index"]) {
        Some("Prelude.splitAt")
    } else if all(&["partition", "list"]) {
        Some("Data.List.partition")
    } else if all(&["element", "membership"]) {
        Some("Prelude.elem")
    } else if all(&["association", "list", "lookup"]) {
        Some("Prelude.lookup")
    } else if words.iter().any(|word| word == "sort")
        && words
            .iter()
            .any(|word| matches!(word.as_str(), "field" | "key"))
    {
        Some("Data.List.sortOn")
    } else if all(&["group", "adjacent"]) {
        Some("Data.List.group")
    } else if all(&["remove", "duplicates"]) {
        Some("Data.List.nub")
    } else if all(&["running", "totals"]) {
        Some("Prelude.scanl")
    } else if all(&["infinite", "iteration"]) {
        Some("Prelude.iterate")
    } else if all(&["repeat", "value", "times"]) {
        Some("Prelude.replicate")
    } else if all(&["replace", "substring"]) {
        Some("Data.Text.replace")
    } else if all(&["encode", "utf8"]) {
        Some("Data.Text.Encoding.encodeUtf8")
    } else if all(&["invalid", "unicode"]) {
        Some("Data.Text.Encoding.decodeUtf8With")
    } else if all(&["write", "text", "file"]) {
        Some("Data.Text.IO.writeFile")
    } else if all(&["list", "directory"]) {
        Some("System.Directory.listDirectory")
    } else if all(&["check", "file", "exists"]) {
        Some("System.Directory.doesFileExist")
    } else if all(&["create", "directory"]) {
        Some("System.Directory.createDirectoryIfMissing")
    } else if all(&["remove", "file"]) && !words.iter().any(|word| word == "extension") {
        Some("System.Directory.removeFile")
    } else {
        None
    }
}

fn normalize_haskell_api_query(query: &str) -> String {
    if let Some(api) = normalize_haskell_task(query) {
        return api.to_string();
    }

    let query = query.trim();
    if query.contains('_')
        && query.split('_').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric())
        })
    {
        let mut parts = query.split('_');
        let mut normalized = parts.next().unwrap_or_default().to_ascii_lowercase();
        for part in parts {
            let mut characters = part.chars();
            if let Some(first) = characters.next() {
                normalized.push(first.to_ascii_uppercase());
                normalized.extend(characters);
            }
        }
        return normalized;
    }

    // Only singularize unambiguous, common English suffixes. In particular,
    // do not strip the `s` from names such as `process` or `bytestring`.
    if !query.contains(char::is_whitespace) {
        let lower = query.to_ascii_lowercase();
        if lower.ends_with("ies") && lower.len() > 4 {
            return format!("{}y", &query[..query.len() - 3]);
        }
        if lower.ends_with("ives")
            || lower.ends_with("ables")
            || lower.ends_with("oids")
            || lower.ends_with("types")
            || lower.ends_with("tors")
            || lower.ends_with("ences")
            || lower.ends_with("ules")
            || lower.ends_with("ples")
            || lower.ends_with("ards")
            || lower.ends_with("ists")
            || lower.ends_with("ions")
        {
            return query[..query.len() - 1].to_string();
        }
    }
    query.to_string()
}

fn haskell_docs_fallback(query: &str) -> Hit {
    Hit::new(
        format!("Haskell language documentation — {query}"),
        "https://www.haskell.org/onlinereport/haskell2010/",
    )
    .snippet("Language syntax and semantics in the Haskell 2010 Report. For compiler extensions, follow the GHC User's Guide link from the report.")
    .meta_line("Haskell 2010 Report")
}

fn devdocs_meta_line(docset_name: &str, entry_kind: &str) -> String {
    let docset_name = crate::text::sanitize_terminal_line(docset_name);
    let entry_kind = crate::text::sanitize_terminal_line(entry_kind);
    format!("{docset_name}  ·  {entry_kind}")
}

fn search_docset(
    client: &Client,
    docset: &Docset,
    query: &str,
    limit: usize,
) -> Result<Outcome, String> {
    search_docset_with_quality(client, docset, query, limit).map(|(outcome, _)| outcome)
}

fn search_docset_with_quality(
    client: &Client,
    docset: &Docset,
    query: &str,
    limit: usize,
) -> Result<(Outcome, bool), String> {
    let ranked = ranked_docset_hits(client, docset, query, limit, true, false)?;
    if ranked.is_empty() {
        return Err(format!("no {} docs matched {query}", docset.name));
    }
    let strong = ranked.first().is_some_and(|(score, _)| *score >= 500);
    let mut outcome = Outcome::new(&docset_label(docset), query);
    outcome.source_url = Some(format!("https://devdocs.io/{}", docset.slug));
    outcome.hits = ranked.into_iter().map(|(_, hit)| hit).collect();
    Ok((outcome, strong))
}

pub(crate) fn family_hits(
    client: &Client,
    family: &str,
    query: &str,
    limit: usize,
    fetch: bool,
) -> Vec<(i32, Hit)> {
    let docset = if fetch {
        match resolve_docset(client, family) {
            Ok(Some(docset)) => docset,
            _ => match fallback_docset(family) {
                Some(docset) => docset,
                None => return Vec::new(),
            },
        }
    } else {
        let Some(catalog) = load_catalog_local() else {
            return Vec::new();
        };
        let Some(docset) = pick_docset(&catalog, family)
            .cloned()
            .or_else(|| fallback_docset(family))
        else {
            return Vec::new();
        };
        if !safe_docset_slug(&docset.slug) {
            return Vec::new();
        }
        let url = format!("https://documents.devdocs.io/{}/index.json", docset.slug);
        let index = devdocs_cache_dir()
            .join("indexes")
            .join(format!("{}.json", docset.slug));
        if crate::cache::read_fresh::<Index>(&index, CACHE_TTL, CACHE_SCHEMA, &url).is_none() {
            return Vec::new();
        }
        docset
    };
    ranked_docset_hits(client, &docset, query, limit, false, true).unwrap_or_default()
}

fn entry_matches_negative_terms(entry: &Entry, negative_terms: &[String]) -> bool {
    if negative_terms.is_empty() {
        return false;
    }
    let name_tokens = identifier_tokens(&entry.name);
    negative_terms.iter().all(|negative| {
        name_tokens
            .iter()
            .any(|token| super::aliases::word_family_matches(token, negative))
    })
}

fn ranked_docset_hits(
    client: &Client,
    docset: &Docset,
    query: &str,
    limit: usize,
    snippet_first: bool,
    strong_only: bool,
) -> Result<Vec<(i32, Hit)>, String> {
    let index = load_index(client, docset)?;
    let plan = crate::semantic::plan_query(query);
    let rank_query = if plan.negative_terms.is_empty() {
        query.to_string()
    } else {
        plan.positive_terms.join(" ")
    };
    let mut ranked: Vec<(i32, &Entry)> = index
        .entries
        .iter()
        .filter(|entry| !entry_matches_negative_terms(entry, &plan.negative_terms))
        .filter_map(|entry| entry_score(&rank_query, entry).map(|score| (score, entry)))
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.len().cmp(&b.1.name.len())));
    if strong_only || ranked.first().is_some_and(|(score, _)| *score >= 500) {
        ranked.retain(|(score, _)| *score >= 500);
    }
    let mut seen = std::collections::HashSet::new();
    ranked.retain(|(_, entry)| seen.insert(entry.path.clone()));
    ranked.truncate(limit);
    let mut hits = Vec::new();
    for (index, (rank, entry)) in ranked.iter().enumerate() {
        let url = format!("https://devdocs.io/{}/{}", docset.slug, entry.path);
        let mut hit =
            Hit::new(&entry.name, url).meta_line(devdocs_meta_line(&docset.name, &entry.kind));
        if FETCH_LIVE_PAGE_NOTES && snippet_first && index < 2 {
            if let Some(notes) = page_notes(client, docset, &entry.path) {
                if let Some(snippet) = notes.snippet {
                    hit = hit.snippet(snippet);
                }
                if let Some(usage) = notes.usage {
                    hit = hit.meta_line(usage);
                }
            }
        }
        hits.push((*rank, hit));
    }
    Ok(hits)
}

fn resolve_docset(client: &Client, language: &str) -> Result<Option<Docset>, String> {
    let catalog = load_catalog(client)?;
    Ok(pick_docset(&catalog, language).cloned())
}

fn pick_docset<'a>(catalog: &'a [Docset], language: &str) -> Option<&'a Docset> {
    let language = language.to_ascii_lowercase();
    if let Some(docset) = catalog
        .iter()
        .find(|docset| docset.slug.eq_ignore_ascii_case(&language) && !docset.slug.contains('~'))
    {
        return Some(docset);
    }
    if let Some(docset) = catalog
        .iter()
        .find(|docset| docset.slug.eq_ignore_ascii_case(&language))
    {
        return Some(docset);
    }

    let aliased: Vec<_> = catalog
        .iter()
        .filter(|docset| {
            docset
                .alias
                .as_ref()
                .is_some_and(|alias| alias.eq_ignore_ascii_case(&language))
        })
        .collect();
    if !aliased.is_empty() {
        return Some(latest(aliased));
    }

    let prefixed: Vec<_> = catalog
        .iter()
        .filter(|docset| {
            docset
                .slug
                .split('~')
                .next()
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(&language))
        })
        .collect();
    if !prefixed.is_empty() {
        if let Some(docset) = prefixed.iter().find(|docset| !docset.slug.contains('~')) {
            return Some(docset);
        }
        return Some(latest(prefixed));
    }

    let named: Vec<_> = catalog
        .iter()
        .filter(|docset| docset.name.eq_ignore_ascii_case(&language))
        .collect();
    if !named.is_empty() {
        if let Some(docset) = named.iter().find(|docset| !docset.slug.contains('~')) {
            return Some(docset);
        }
        return Some(latest(named));
    }
    None
}

fn latest(docsets: Vec<&Docset>) -> &Docset {
    docsets
        .into_iter()
        .max_by(|left, right| {
            version_key(&left.slug)
                .cmp(&version_key(&right.slug))
                .then(right.slug.len().cmp(&left.slug.len()))
        })
        .expect("docset list is non-empty")
}

fn version_key(slug: &str) -> Vec<u32> {
    let Some(version) = slug.split_once('~').map(|(_, version)| version) else {
        return vec![u32::MAX];
    };
    let numeric: String = version
        .chars()
        .take_while(|ch| ch.is_ascii_digit() || *ch == '.')
        .collect();
    if numeric.is_empty() {
        return vec![0];
    }
    numeric
        .split('.')
        .map(|part| part.parse().unwrap_or(0))
        .collect()
}

fn docset_label(docset: &Docset) -> String {
    match docset
        .release
        .as_deref()
        .filter(|release| !release.is_empty())
    {
        Some(release) => format!("{} {release}", docset.name),
        None => docset.name.clone(),
    }
}

fn fallback_docset(language: &str) -> Option<Docset> {
    let (slug, name) = match language.to_ascii_lowercase().as_str() {
        "c" => ("c", "C"),
        "cpp" | "c++" => ("cpp", "C++"),
        "python" | "py" => ("python~3.14", "Python"),
        "ocaml" | "ml" => ("ocaml", "OCaml"),
        "go" | "golang" => ("go", "Go"),
        "java" => ("openjdk~25", "OpenJDK"),
        "ruby" | "rb" => ("ruby~4.0", "Ruby"),
        "js" | "javascript" => ("javascript", "JavaScript"),
        "ts" | "typescript" => ("typescript", "TypeScript"),
        "php" => ("php", "PHP"),
        "zig" => ("zig", "Zig"),
        "kotlin" | "kt" => ("kotlin~2", "Kotlin"),
        "lua" => ("lua~5.5", "Lua"),
        "elixir" | "ex" => ("elixir~1.20", "Elixir"),
        "perl" => ("perl~5.44", "Perl"),
        "scala" => ("scala~3", "Scala"),
        "clojure" | "clj" => ("clojure~1.11", "Clojure"),
        "erlang" | "erl" => ("erlang~26", "Erlang"),
        "julia" | "jl" => ("julia~1.13", "Julia"),
        "nim" => ("nim", "Nim"),
        "bash" | "sh" => ("bash", "Bash"),
        "zsh" => ("zsh", "Zsh"),
        "css" => ("css", "CSS"),
        "html" => ("html", "HTML"),
        "node" | "nodejs" => ("node", "Node.js"),
        "dart" => ("dart~3", "Dart"),
        "r" => ("r", "R"),
        _ => return None,
    };
    Some(Docset {
        name: name.to_string(),
        slug: slug.to_string(),
        alias: None,
        release: None,
    })
}

fn devdocs_cache_dir() -> PathBuf {
    cache_dir().join("devdocs-v1")
}

fn load_catalog_local() -> Option<Vec<Docset>> {
    crate::cache::read_fresh(
        &devdocs_cache_dir().join("catalog.json"),
        CACHE_TTL,
        CACHE_SCHEMA,
        CATALOG_URL,
    )
}

fn load_catalog(client: &Client) -> Result<Vec<Docset>, String> {
    let path = devdocs_cache_dir().join("catalog.json");
    let cached = crate::cache::get_or_fetch(
        &path,
        CACHE_TTL,
        CACHE_STALE_MAX_AGE,
        CACHE_SCHEMA,
        CATALOG_URL,
        || {
            eprintln!("qs: fetching documentation catalog...");
            client.get_json(CATALOG_URL, APP_UA, &[])
        },
    )
    .map_err(|error| format!("devdocs catalog: {error}"))?;
    Ok(cached.value)
}

fn safe_docset_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 100
        && !slug.starts_with('.')
        && !slug.contains("..")
        && slug
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '~'))
}

fn load_index(client: &Client, docset: &Docset) -> Result<Arc<Index>, String> {
    if !safe_docset_slug(&docset.slug) {
        return Err("documentation catalog returned an unsafe docset slug".into());
    }
    let url = format!("https://documents.devdocs.io/{}/index.json", docset.slug);
    let path = devdocs_cache_dir()
        .join("indexes")
        .join(format!("{}.json", docset.slug));
    load_index_at(&path, &url, || {
        eprintln!(
            "qs: fetching {} docs...",
            crate::text::sanitize_terminal_line(&docset.name)
        );
        client.get_json(&url, APP_UA, &[])
    })
    .map_err(|error| format!("{} docs index: {error}", docset.name))
}

fn load_index_at<F>(path: &Path, key: &str, fetch: F) -> Result<Arc<Index>, String>
where
    F: FnOnce() -> Result<Index, String>,
{
    let mut memory = INDEX_CACHE.lock().unwrap_or_else(|lock| lock.into_inner());
    if let Some((_, index)) = memory.iter().find(|(cached_path, _)| cached_path == path) {
        return Ok(Arc::clone(index));
    }

    // Keep the lock while loading. This also coalesces concurrent requests for
    // the same index rather than parsing or downloading it twice.
    let cached = crate::cache::get_or_fetch(
        path,
        CACHE_TTL,
        CACHE_STALE_MAX_AGE,
        CACHE_SCHEMA,
        key,
        fetch,
    )
    .map_err(|error| error.to_string())?;
    let index = Arc::new(cached.value);
    if memory.len() == MEMORY_CACHE_CAPACITY {
        memory.pop_front();
    }
    memory.push_back((path.to_path_buf(), Arc::clone(&index)));
    Ok(index)
}

pub(crate) fn cache_dir() -> PathBuf {
    crate::cache::root_dir()
}

pub(crate) fn config_dir() -> PathBuf {
    let xdg = std::env::var_os("XDG_CONFIG_HOME");
    let appdata = std::env::var_os("APPDATA");
    let home = home_dir();
    platform_dir(
        std::env::consts::OS,
        xdg.as_deref(),
        appdata.as_deref(),
        home.as_deref(),
        "Application Support",
        ".config",
    )
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn platform_dir(
    os: &str,
    override_dir: Option<&std::ffi::OsStr>,
    windows_dir: Option<&std::ffi::OsStr>,
    home: Option<&std::path::Path>,
    macos_folder: &str,
    unix_folder: &str,
) -> PathBuf {
    if let Some(dir) = override_dir {
        return PathBuf::from(dir).join("quicksearch");
    }
    if os == "windows" {
        if let Some(dir) = windows_dir {
            return PathBuf::from(dir).join("quicksearch");
        }
    }
    if let Some(home) = home {
        if os == "macos" {
            return home.join("Library").join(macos_folder).join("quicksearch");
        }
        return home.join(unix_folder).join("quicksearch");
    }
    std::env::temp_dir().join(format!("quicksearch-{}", std::process::id()))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct PageNotes {
    snippet: Option<String>,
    usage: Option<String>,
}

fn page_notes(client: &Client, docset: &Docset, path: &str) -> Option<PageNotes> {
    page_notes_at(&devdocs_cache_dir(), &docset.slug, path, || {
        let file = path.split_once('#').map_or(path, |(file, _)| file);
        let url = format!("https://documents.devdocs.io/{}/{file}.html", docset.slug);
        client.get_text(&url, APP_UA, &[])
    })
    .ok()
    .flatten()
}

fn page_notes_at<F>(
    _cache_root: &Path,
    slug: &str,
    path: &str,
    fetch: F,
) -> Result<Option<PageNotes>, String>
where
    F: FnOnce() -> Result<String, String>,
{
    let (file, fragment) = path
        .split_once('#')
        .map_or((path, None), |(file, fragment)| (file, Some(fragment)));
    let raw_key = format!("{slug}\n{file}");
    let html = load_raw_page(raw_key, fetch)?;
    Ok(extract_page_notes(&html, fragment))
}

fn load_raw_page<F>(key: String, fetch: F) -> Result<Arc<String>, String>
where
    F: FnOnce() -> Result<String, String>,
{
    let mut memory = RAW_PAGE_CACHE
        .lock()
        .unwrap_or_else(|lock| lock.into_inner());
    if let Some((_, html)) = memory.iter().find(|(cached_key, _)| cached_key == &key) {
        return Ok(Arc::clone(html));
    }
    // Holding the lock through the download coalesces requests for two
    // fragments of the same source file.
    let html = Arc::new(fetch()?);
    if memory.len() == MEMORY_CACHE_CAPACITY {
        memory.pop_front();
    }
    memory.push_back((key, Arc::clone(&html)));
    Ok(html)
}

fn extract_page_notes(html: &str, fragment: Option<&str>) -> Option<PageNotes> {
    let section = match fragment {
        Some(fragment) => section_from_fragment(html, fragment)?,
        None => root_section(html),
    };
    let snippet = first_paragraph(section)
        .or_else(|| explain_sentence(section))
        .map(|text| truncate(&text, 320))
        .filter(|text| !text.is_empty());
    let usage = usage_block(section);
    if snippet.is_none() && usage.is_none() {
        None
    } else {
        Some(PageNotes { snippet, usage })
    }
}

fn root_section(html: &str) -> &str {
    static H1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<h1\b").unwrap());
    static SECTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<section\b").unwrap());
    let start = H1.find(html).map_or(0, |heading| heading.start());
    if let Some(section) = SECTION.find(&html[start..]) {
        let section_start = start + section.start();
        let end = matching_element_end(html, section_start, "section")
            .unwrap_or_else(|| section_start + byte_prefix(&html[section_start..], 20_000).len());
        &html[start..end]
    } else {
        byte_prefix(&html[start..], 20_000)
    }
}

fn section_from_fragment<'a>(html: &'a str, fragment: &str) -> Option<&'a str> {
    let fragment = urlencoding::decode(fragment).ok()?;
    let anchor = Regex::new(&format!(
        r#"(?is)\b(?:id|name)\s*=\s*["']{}["']"#,
        regex::escape(&fragment)
    ))
    .ok()?;
    let matched = anchor.find(html)?;
    let mut start = html[..matched.start()]
        .rfind('<')
        .unwrap_or(matched.start());
    let mut tag = opening_tag(&html[start..])?;

    // Sphinx and other generators often put the id on an empty span directly
    // before the real heading. Scope from that heading instead of the span.
    if matches!(tag, "a" | "span") {
        let after_anchor = html[matched.end()..]
            .find('>')
            .map(|at| matched.end() + at + 1)
            .unwrap_or(matched.end());
        static SEMANTIC: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?is)^\s*(?:</(?:a|span)>\s*)*<(section|h[1-6]|dt)\b").unwrap()
        });
        if let Some(next) = SEMANTIC.captures(&html[after_anchor..]) {
            let whole = next.get(0)?;
            let relative = whole.as_str().rfind('<')?;
            start = after_anchor + whole.start() + relative;
            tag = opening_tag(&html[start..])?;
        }
    }

    let hard_end = start + byte_prefix(&html[start..], 20_000).len();
    let end = match tag {
        "section" => matching_element_end(html, start, "section"),
        "dt" => Regex::new(r"(?is)</dd\s*>")
            .unwrap()
            .find(&html[start..hard_end])
            .map(|end| start + end.end()),
        heading if heading.len() == 2 && heading.starts_with('h') => {
            let level = heading.as_bytes()[1] as char;
            let boundary = Regex::new(&format!(r"(?is)<h[1-{level}]\b")).ok()?;
            let open_end = html[start..hard_end].find('>').map(|at| start + at + 1)?;
            boundary
                .find(&html[open_end..hard_end])
                .map(|next| open_end + next.start())
        }
        _ => None,
    }
    .unwrap_or_else(|| {
        static NEXT_ANCHOR: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"(?is)<(?:section|dt|h[1-6])\b[^>]*\b(?:id|name)\s*=["']"#).unwrap()
        });
        NEXT_ANCHOR
            .find(&html[matched.end()..hard_end])
            .map(|next| matched.end() + next.start())
            .unwrap_or(hard_end)
    });
    Some(&html[start..end])
}

fn opening_tag(html: &str) -> Option<&str> {
    let end = html[1..].find(|ch: char| ch.is_ascii_whitespace() || ch == '>')? + 1;
    let tag = &html[1..end];
    tag.chars()
        .all(|ch| ch.is_ascii_alphanumeric())
        .then_some(tag)
}

fn matching_element_end(html: &str, start: usize, tag: &str) -> Option<usize> {
    let tags = Regex::new(&format!(r"(?is)</?{}\b[^>]*>", regex::escape(tag))).ok()?;
    let mut depth = 0usize;
    for found in tags.find_iter(&html[start..]) {
        let token = found.as_str().as_bytes();
        if token.get(1) == Some(&b'/') {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(start + found.end());
            }
        } else {
            depth += 1;
        }
    }
    None
}

fn first_paragraph(html: &str) -> Option<String> {
    static PARAGRAPH: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<p[^>]*>(.*?)</p>").unwrap());
    PARAGRAPH.captures_iter(html).find_map(|caps| {
        let text = plain_html(caps.get(1)?.as_str());
        if text.chars().count() < 40 || looks_like_code(&text) {
            return None;
        }
        Some(text)
    })
}

fn explain_sentence(html: &str) -> Option<String> {
    let text = collapse_ws(&break_tags(html));
    let marker = text.find(" is a ")?;
    let begin = text[..marker]
        .rfind(['.', ')', ':', ';'])
        .map(|index| index + 1)
        .unwrap_or(0);
    let end = text[marker..].find('.')? + marker + 1;
    let mut sentence = text[begin..end].trim().to_string();
    if let Some((lead, rest)) = sentence.split_once(") ") {
        if lead.chars().all(|ch| ch.is_ascii_digit() || ch == '(') {
            sentence = rest.to_string();
        }
    }
    if sentence.chars().count() < 40 {
        None
    } else {
        Some(sentence)
    }
}

fn usage_block(html: &str) -> Option<String> {
    let pres = pre_blocks(html);
    if let Some(example) = pres.iter().find(|pre| pre.contains("#include")) {
        return Some(example.clone());
    }
    // The first block after an indexed anchor is normally that symbol's
    // synopsis or example. Later blocks often belong to another API section.
    pres.into_iter().next()
}

fn pre_blocks(html: &str) -> Vec<String> {
    static PRE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<pre\b[^>]*>(.*?)</pre>").unwrap());
    PRE.captures_iter(html)
        .map(|caps| pre_text(&caps[1]))
        .filter(|text| text.lines().filter(|line| !line.trim().is_empty()).count() >= 2)
        .collect()
}

fn pre_text(inner: &str) -> String {
    static BR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<br\s*/?>").unwrap());
    static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").unwrap());
    let with_breaks = BR.replace_all(inner, "\n");
    let plain = html_unescape(&TAG.replace_all(&with_breaks, ""));
    let lines: Vec<&str> = plain.lines().collect();
    let Some(start) = lines.iter().position(|line| !line.trim().is_empty()) else {
        return String::new();
    };
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .unwrap_or(start);
    lines[start..=end].join("\n")
}

fn break_tags(html: &str) -> String {
    static TAG: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?is)</?(?:p|pre|code|div|span|a|h[1-6]|br|hr|table|tr|td|th|ul|ol|li|em|strong|b|i|tt|dl|dt|dd|section|script|style|sup|sub|blockquote|html|body|head|meta|link|img)\b[^>]*>",
        )
        .unwrap()
    });
    TAG.replace_all(&html_unescape(html), "\n").into_owned()
}

fn looks_like_code(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.starts_with("added in version")
        || text.matches(");").count() >= 1 && text.matches('(').count() >= 2
}

fn plain_html(text: &str) -> String {
    collapse_ws(&html_unescape(&strip_tags(text)))
}

fn canonical_identifier(text: &str) -> String {
    text.chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn qualified_path_exact_score(query: &str, entry: &Entry) -> Option<i32> {
    let query = query.trim().trim_end_matches("()");
    let (qualifier, raw_symbol) = if let Some(parts) = query.rsplit_once("->") {
        parts
    } else if let Some((left, right)) = query.rsplit_once('/') {
        if left.contains('.') && !right.chars().all(|ch| ch.is_ascii_digit()) {
            (left, right)
        } else {
            query.rsplit_once('.')?
        }
    } else {
        query.rsplit_once('.')?
    };
    let symbol = raw_symbol
        .rsplit_once('/')
        .filter(|(_, arity)| arity.chars().all(|ch| ch.is_ascii_digit()))
        .map_or(raw_symbol, |(symbol, _)| symbol);
    if qualifier.is_empty() || symbol.is_empty() {
        return None;
    }
    let owner = qualifier
        .split(['.', ':', '/'])
        .rfind(|part| !part.is_empty())?;
    let owner = canonical_identifier(owner);
    let symbol = canonical_identifier(symbol);
    if owner.is_empty() || symbol.is_empty() {
        return None;
    }

    let path = entry.path.to_ascii_lowercase();
    if let Some((page, fragment)) = path.split_once('#') {
        let page_tail = page.rsplit('/').next().unwrap_or(page);
        let page_matches = canonical_identifier(page_tail).contains(&owner);
        let fragment = ["method-i-", "method-c-", "function-", "val", "type"]
            .into_iter()
            .find_map(|prefix| fragment.strip_prefix(prefix))
            .unwrap_or(fragment);
        let fragment = fragment.split(['(', ',']).next().unwrap_or(fragment);
        let fragment = fragment.rsplit_once('/').map_or(fragment, |(left, right)| {
            if right.chars().all(|ch| ch.is_ascii_digit()) {
                left
            } else {
                right
            }
        });
        let fragment = fragment.rsplit('.').next().unwrap_or(fragment);
        if page_matches && canonical_identifier(fragment) == symbol {
            return Some(11_500);
        }
    } else {
        let segments: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
        if segments.len() >= 2
            && canonical_identifier(segments[segments.len() - 2]).contains(&owner)
            && canonical_identifier(segments[segments.len() - 1]) == symbol
        {
            return Some(11_500);
        }
    }
    None
}

fn entry_score(query: &str, entry: &Entry) -> Option<i32> {
    let embedded_exact = || {
        let name = entry.name.trim_end_matches("()");
        if name.len() < 8
            || name.contains(char::is_whitespace)
            || !name
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return None;
        }
        query
            .split_whitespace()
            .map(|word| word.trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_'))
            .any(|word| word == name)
            .then_some(9_500)
    };
    let embedded_qualified = || {
        query
            .split_whitespace()
            .map(|word| {
                word.trim_matches(|ch: char| {
                    !ch.is_ascii_alphanumeric()
                        && !matches!(
                            ch,
                            '_' | '.' | ':' | '/' | '#' | '$' | '!' | '?' | '>' | '<'
                        )
                })
            })
            .filter(|word| {
                word.chars()
                    .any(|ch| matches!(ch, '.' | ':' | '/' | '#' | '$'))
            })
            .filter_map(|word| score(word, &entry.name))
            .filter(|score| *score >= 10_000)
            .max()
    };
    let mut rank = qualified_path_exact_score(query, entry)
        .or_else(|| score(query, &entry.name))
        .or_else(embedded_qualified)
        .or_else(embedded_exact)
        .or_else(|| relaxed_score(query, &entry.name))?;
    let explicit_header = query.contains(".h")
        || query.contains('<')
        || query.contains('>')
        || query.to_ascii_lowercase().contains("header");
    if !explicit_header && entry.kind.to_ascii_lowercase().contains("header") {
        rank = (rank - 9_500).max(1);
    }
    Some(rank)
}

pub fn score(query: &str, name: &str) -> Option<i32> {
    // Python and other case-sensitive languages can expose symbols that differ
    // only by case (for example, inspect.signature and inspect.Signature).
    // Preserve an exact qualified spelling before the token matcher folds case.
    let query_symbol = query.trim().trim_end_matches("()");
    let name_symbol = name.trim().trim_end_matches("()");
    let name_without_arity = name_symbol
        .rsplit_once(" (")
        .filter(|(_, suffix)| {
            suffix.strip_suffix(')').is_some_and(|arity| {
                !arity.is_empty() && arity.chars().all(|ch| ch.is_ascii_digit())
            })
        })
        .map_or(name_symbol, |(base, _)| base);
    let symbolic_query = query_symbol
        .chars()
        .any(|ch| !ch.is_ascii_alphanumeric() && ch != '_');
    if symbolic_query && (query_symbol == name_symbol || query_symbol == name_without_arity) {
        return Some(11_000);
    }

    let mut query_tokens = identifier_tokens(query);
    let name_tokens = identifier_tokens(name);
    // DevDocs often omits JavaScript's structural `.prototype` segment from
    // index titles (`Array.toSorted` vs `Array.prototype.toSorted`).
    if query.contains('.') && !name_tokens.iter().any(|token| token == "prototype") {
        query_tokens.retain(|token| token != "prototype");
    }
    if query_tokens.is_empty() || name_tokens.is_empty() {
        return None;
    }

    // Compare identifier words rather than punctuation. This preserves exact
    // qualified-name matches while also treating spaces, underscores, and
    // CamelCase as equivalent separators.
    if query_tokens == name_tokens {
        return Some(10_000);
    }

    let mut used = vec![false; name_tokens.len()];
    let mut positions = Vec::with_capacity(query_tokens.len());
    let mut match_score = 0;
    for query_token in &query_tokens {
        let best = name_tokens
            .iter()
            .enumerate()
            .filter(|(index, _)| !used[*index])
            .filter_map(|(index, name_token)| {
                token_match_quality(query_token, name_token, query_tokens.len() == 1)
                    .map(|quality| (quality, index))
            })
            .max_by_key(|(quality, index)| (*quality, std::cmp::Reverse(*index)));
        if let Some((quality, index)) = best {
            used[index] = true;
            positions.push(index);
            match_score += quality;
        }
    }

    let matched = positions.len();
    // A multi-word query must not degrade into a match for one generic word.
    // Requiring three quarters of the words also permits one harmless missing
    // word in longer natural-language queries.
    let identifier_query = query
        .chars()
        .any(|ch| !ch.is_ascii_alphanumeric() && !ch.is_whitespace());
    let required = if query_tokens.len() == 1 {
        1
    } else if identifier_query {
        query_tokens.len()
    } else {
        (query_tokens.len() * 3).div_ceil(4)
    };
    if matched < required {
        return None;
    }

    let ordered = positions.windows(2).all(|pair| pair[0] < pair[1]);
    let adjacent = ordered && positions.windows(2).all(|pair| pair[1] == pair[0] + 1);
    let complete = matched == query_tokens.len();
    let extra_name_tokens = name_tokens.len().saturating_sub(matched) as i32;
    let missing_query_tokens = query_tokens.len().saturating_sub(matched) as i32;

    let mut rank = match_score;
    if complete && query_tokens.len() > 1 {
        rank += 450;
    }
    if query_tokens.len() > 1 && ordered {
        rank += 180;
        if adjacent {
            rank += 180;
        }
    }
    // Short entity pages beat methods or constants that mention the same word
    // incidentally in a long qualified name.
    rank += (300 - extra_name_tokens * 90).max(0);
    if query_tokens.len() == 1 && positions[0] + 1 != name_tokens.len() {
        rank -= 300;
    }
    rank -= missing_query_tokens * 300;
    rank -= canon(name).len() as i32 / 8;
    Some(rank)
}

// Keep a weak local candidate when strict phrase coverage is unavailable.
// This prevents a failed web fallback from turning a partial documentation
// match into an error. Scores stay below the strong-result threshold.
fn relaxed_score(query: &str, name: &str) -> Option<i32> {
    let query_tokens = identifier_tokens(query);
    let name_tokens = identifier_tokens(name);
    if query_tokens.is_empty() || name_tokens.is_empty() {
        return None;
    }
    let mut matched = 0i32;
    let mut terminal = false;
    for query_token in &query_tokens {
        if let Some(position) = name_tokens.iter().position(|name_token| {
            query_token == name_token
                || same_word_family(query_token, name_token)
                || identifier_abbreviation(query_token, name_token)
        }) {
            matched += 1;
            terminal |= position + 1 == name_tokens.len();
        }
    }
    if matched == 0 {
        return None;
    }
    let coverage = matched * 240 / query_tokens.len() as i32;
    let rank = 40 + coverage + if terminal { 60 } else { 0 }
        - name_tokens.len().saturating_sub(matched as usize) as i32 * 12
        - canon(name).len() as i32 / 12;
    Some(rank.clamp(1, 480))
}

// Higher values mean a less ambiguous lexical match. Substring matching is
// intentionally limited to one-word searches: for phrases it caused one word
// such as "groups" or "semantic" to dominate the whole query.
fn token_match_quality(query: &str, name: &str, allow_substring: bool) -> Option<i32> {
    if query == name {
        return Some(700);
    }
    if same_word_family(query, name) {
        return Some(580);
    }
    if identifier_abbreviation(query, name) {
        return Some(520);
    }
    if allow_substring && query.len() > 3 && name.ends_with(query) {
        return Some(260);
    }
    None
}

fn identifier_abbreviation(left: &str, right: &str) -> bool {
    fn is_abbreviation(short: &str, long: &str) -> bool {
        if short.len() < 2
            || short.len() > 4
            || long.len() <= short.len()
            || short.chars().next() != long.chars().next()
            || short.chars().last() != long.chars().last()
        {
            return false;
        }
        let mut letters = short.chars();
        let mut wanted = letters.next();
        for ch in long.chars() {
            if Some(ch) == wanted {
                wanted = letters.next();
            }
        }
        wanted.is_none()
    }
    word_forms(left).iter().any(|left_form| {
        word_forms(right).iter().any(|right_form| {
            is_abbreviation(left_form, right_form) || is_abbreviation(right_form, left_form)
        })
    })
}

fn identifier_tokens(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut token = String::new();
    for (index, ch) in chars.iter().copied().enumerate() {
        if !ch.is_ascii_alphanumeric() {
            if !token.is_empty() {
                tokens.push(std::mem::take(&mut token));
            }
            continue;
        }
        let previous = index.checked_sub(1).and_then(|at| chars.get(at)).copied();
        let next = chars.get(index + 1).copied();
        let boundary = ch.is_ascii_uppercase()
            && !token.is_empty()
            && (previous.is_some_and(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
                || previous.is_some_and(|ch| ch.is_ascii_uppercase())
                    && next.is_some_and(|ch| ch.is_ascii_lowercase()));
        if boundary {
            tokens.push(std::mem::take(&mut token));
        }
        token.push(ch.to_ascii_lowercase());
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

fn same_word_family(left: &str, right: &str) -> bool {
    if left == right || left.len() < 3 || right.len() < 3 {
        return left == right;
    }
    let left_forms = word_forms(left);
    let right_forms = word_forms(right);
    left_forms.iter().any(|form| right_forms.contains(form))
}

fn word_forms(word: &str) -> Vec<String> {
    let mut forms = vec![word.to_string()];
    let mut add = |form: String| {
        if form.len() >= 3 && !forms.contains(&form) {
            forms.push(form);
        }
    };
    if let Some(base) = word.strip_suffix("ing").filter(|base| base.len() >= 3) {
        add(base.to_string());
        add(format!("{base}e"));
        if base.as_bytes().last() == base.as_bytes().get(base.len().saturating_sub(2)) {
            add(base[..base.len() - 1].to_string());
        }
    }
    if let Some(base) = word.strip_suffix("ed").filter(|base| base.len() >= 3) {
        add(base.to_string());
        add(format!("{base}e"));
        if base.as_bytes().last() == base.as_bytes().get(base.len().saturating_sub(2)) {
            add(base[..base.len() - 1].to_string());
        }
    }
    if let Some(base) = word.strip_suffix("ies").filter(|base| base.len() >= 3) {
        add(format!("{base}y"));
    }
    if let Some(base) = word.strip_suffix("es").filter(|base| base.len() >= 3) {
        add(base.to_string());
    }
    if let Some(base) = word.strip_suffix('s').filter(|base| base.len() >= 3) {
        add(base.to_string());
    }
    forms
}

fn canon(text: &str) -> String {
    let mut out = String::new();
    let mut break_word = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            if break_word && !out.is_empty() {
                out.push(' ');
            }
            break_word = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            break_word = true;
        }
    }
    out
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Docset {
    name: String,
    slug: String,
    #[serde(default)]
    alias: Option<String>,
    #[serde(default)]
    release: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Index {
    #[serde(default)]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    name: String,
    path: String,
    #[serde(default, rename = "type")]
    kind: String,
}

#[derive(Clone, Debug)]
struct Shorthand {
    words: Vec<String>,
    label: String,
    url: String,
}

struct Listed {
    groups: Vec<Shorthand>,
    hidden: usize,
}

const STARTERS: &[&str] = &[
    "c", "cpp", "python", "haskell", "ocaml", "rust", "go", "java", "ruby", "js", "ts", "php",
    "zig", "kotlin", "lua", "elixir", "perl", "scala", "bash",
];

pub fn list(client: &Client, pattern: &str, limit: usize) -> Result<Outcome, String> {
    let catalog = load_catalog(client).unwrap_or_default();
    let groups = shorthand_groups(&catalog);
    let listed = rank_shorthands(&groups, pattern, limit);
    if listed.groups.is_empty() {
        return Err(format!("no language shorthand matches {pattern}"));
    }
    let shown = if pattern.trim().is_empty() {
        "languages".to_string()
    } else {
        pattern.to_string()
    };
    let mut outcome = Outcome::new("lang", &shown);
    if listed.hidden > 0 {
        outcome.summary = Some(format!("{} more. pass -n to show them.", listed.hidden));
    } else if pattern.trim().is_empty() {
        outcome.summary = Some("Fuzzy-find a shorthand with qs lang <pattern>.".to_string());
    }
    for group in listed.groups {
        let primary = group.words.first().cloned().unwrap_or_default();
        outcome.hits.push(
            Hit::new(group.words.join(", "), group.url)
                .snippet(group.label)
                .meta_line(format!("qs {primary} <topic>")),
        );
    }
    Ok(outcome)
}

fn shorthand_groups(catalog: &[Docset]) -> Vec<Shorthand> {
    let mut groups = vec![
        Shorthand {
            words: vec!["rs".into(), "rust".into(), "std".into()],
            label: "Rust standard library".into(),
            url: "https://doc.rust-lang.org/std/".into(),
        },
        Shorthand {
            words: vec!["hs".into(), "haskell".into()],
            label: "Haskell".into(),
            url: "https://hackage.haskell.org/".into(),
        },
    ];
    let mut used: std::collections::HashSet<String> = groups
        .iter()
        .flat_map(|group| group.words.iter().cloned())
        .collect();

    let mut words = Vec::new();
    for docset in catalog {
        if let Some(prefix) = docset.slug.split('~').next() {
            words.push(prefix.to_ascii_lowercase());
        }
        if let Some(alias) = &docset.alias {
            words.push(alias.to_ascii_lowercase());
        }
        let name = docset.name.to_ascii_lowercase();
        if is_typeable(&name) {
            words.push(name);
        }
    }
    for extra in [
        "ml",
        "golang",
        "nodejs",
        "node.js",
        "cplusplus",
        "py",
        "c++",
        "rb",
        "js",
        "ts",
        "kt",
        "ex",
        "jl",
        "clj",
        "erl",
        "sh",
        "java",
        "go",
        "php",
        "zig",
        "lua",
        "ruby",
        "scala",
        "perl",
        "bash",
        "html",
        "css",
        "r",
    ] {
        words.push(extra.to_string());
    }
    words.sort();
    words.dedup();

    let mut by_slug: std::collections::HashMap<String, Shorthand> =
        std::collections::HashMap::new();
    for word in words {
        if !is_typeable(&word) || used.contains(&word) || is_command_word(&word) {
            continue;
        }
        let lookup = alias_word(&word);
        let docset = pick_docset(catalog, &lookup)
            .cloned()
            .or_else(|| fallback_docset(&lookup));
        let Some(docset) = docset else {
            continue;
        };
        let family = docset
            .slug
            .split('~')
            .next()
            .unwrap_or(docset.slug.as_str());
        if matches!(family, "rust" | "haskell") {
            continue;
        }
        let entry = by_slug
            .entry(docset.slug.clone())
            .or_insert_with(|| Shorthand {
                words: Vec::new(),
                label: docset_label(&docset),
                url: format!("https://devdocs.io/{}", docset.slug),
            });
        if !entry.words.iter().any(|existing| existing == &word) {
            entry.words.push(word.clone());
        }
        used.insert(word);
    }
    groups.extend(by_slug.into_values());
    for group in &mut groups {
        group
            .words
            .sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        group.words.dedup();
    }
    groups
}

fn rank_shorthands(groups: &[Shorthand], pattern: &str, limit: usize) -> Listed {
    let pattern: String = pattern
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(|ch| ch.to_lowercase())
        .collect();
    let mut scored: Vec<(i32, &Shorthand)> = Vec::new();
    for group in groups {
        if pattern.is_empty() {
            if group
                .words
                .iter()
                .any(|word| STARTERS.contains(&word.as_str()))
            {
                scored.push((0, group));
            }
            continue;
        }
        let word_score = group
            .words
            .iter()
            .filter_map(|word| fuzzy_score(&pattern, word))
            .max();
        let label_score =
            fuzzy_score(&pattern, &group.label).map(|score| score.saturating_sub(400));
        let Some(score) = word_score.into_iter().chain(label_score).max() else {
            continue;
        };
        scored.push((score, group));
    }
    if pattern.is_empty() {
        scored.sort_by_key(|(_, group)| starter_index(&group.words));
    } else {
        scored.sort_by(|left, right| {
            right.0.cmp(&left.0).then_with(|| {
                left.1
                    .label
                    .to_ascii_lowercase()
                    .cmp(&right.1.label.to_ascii_lowercase())
            })
        });
    }
    let total = scored.len();
    let shown = limit.clamp(1, 20).min(total);
    let groups = scored
        .into_iter()
        .take(shown)
        .map(|(score, group)| {
            let _ = score;
            let mut group = group.clone();
            if !pattern.is_empty() {
                group.words.sort_by(|left, right| {
                    fuzzy_score(&pattern, right)
                        .unwrap_or(0)
                        .cmp(&fuzzy_score(&pattern, left).unwrap_or(0))
                        .then(left.len().cmp(&right.len()))
                        .then(left.cmp(right))
                });
            }
            group
        })
        .collect();
    Listed {
        groups,
        hidden: total.saturating_sub(shown),
    }
}

fn starter_index(words: &[String]) -> usize {
    STARTERS
        .iter()
        .position(|starter| words.iter().any(|word| word == starter))
        .unwrap_or(STARTERS.len())
}

pub(crate) fn fuzzy_score(pattern: &str, text: &str) -> Option<i32> {
    if pattern.is_empty() {
        return Some(1);
    }
    let text = text.to_ascii_lowercase();
    if text == pattern {
        return Some(10_000 - text.len() as i32);
    }
    if text.starts_with(pattern) {
        return Some(8_000 - text.len() as i32);
    }
    if let Some(at) = text.find(pattern) {
        return Some(6_000 - at as i32 - text.len() as i32);
    }
    let chars: Vec<char> = text.chars().collect();
    let mut from = 0;
    let mut score = 2_000;
    let mut previous = None;
    for needle in pattern.chars() {
        let relative = chars[from..].iter().position(|ch| *ch == needle)?;
        let at = from + relative;
        if previous.is_some_and(|previous| previous + 1 == at) {
            score += 40;
        } else {
            score -= 15;
        }
        if at == 0
            || chars
                .get(at - 1)
                .is_some_and(|ch| !ch.is_ascii_alphanumeric())
        {
            score += 25;
        }
        previous = Some(at);
        from = at + 1;
    }
    Some(score - text.len() as i32)
}

fn is_typeable(word: &str) -> bool {
    !word.is_empty()
        && word
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '+' | '#' | '.' | '_' | '-'))
}

fn is_command_word(word: &str) -> bool {
    const WORDS: &[&str] = &[
        "web",
        "ddg",
        "cheat",
        "cht",
        "tldr",
        "crate",
        "crates",
        "docs",
        "doc",
        "so",
        "stack",
        "stackoverflow",
        "gh",
        "github",
        "mdn",
        "wiki",
        "wikipedia",
        "man",
        "lang",
        "langs",
        "languages",
        "topic",
        "topics",
        "concept",
        "concepts",
        "history",
        "hist",
        "arch",
        "archwiki",
        "rfc",
        "rfcs",
        "npm",
        "pypi",
        "pip",
        "gentoo",
        "debian",
        "fedora",
        "ubuntu",
        "gem",
        "rubygems",
        "hex",
        "go-mod",
        "pkggo",
        "maven",
        "nuget",
        "cve",
        "nvd",
        "item",
        "stock",
        "sku",
        "upc",
        "barcode",
        "dnf",
        "apt",
        "apt-get",
        "pacman",
        "brew",
        "cargo",
        "rpm",
        "flatpak",
        "zypper",
        "snap",
        "podman",
        "winget",
    ];
    WORDS.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docset(name: &str, slug: &str, alias: Option<&str>, release: Option<&str>) -> Docset {
        Docset {
            name: name.into(),
            slug: slug.into(),
            alias: alias.map(str::to_string),
            release: release.map(str::to_string),
        }
    }

    fn cache_test_dir(name: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "quicksearch-langs-{name}-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn test_index(name: &str) -> Index {
        Index {
            entries: vec![Entry {
                name: name.into(),
                path: "item".into(),
                kind: "test".into(),
            }],
        }
    }

    #[test]
    fn go_qualified_symbols_use_direct_pkg_documentation() {
        let outcome =
            go_qualified_api_outcome("encoding/json.Unmarshal", "go encoding/json.Unmarshal", 3)
                .unwrap();
        assert_eq!(
            outcome.hits[0].url,
            "https://pkg.go.dev/encoding/json#Unmarshal"
        );
        assert!(go_qualified_api_outcome("example.com", "go example.com", 3).is_none());
        assert!(go_qualified_api_outcome("bad/path.Symbol!", "go bad", 3).is_none());
    }

    #[test]
    fn negative_alternative_requires_full_negative_coverage() {
        let sync = Entry {
            name: "fs.readFileSync()".into(),
            path: "fs#fsreadfilesyncpath-options".into(),
            kind: "method".into(),
        };
        let async_read = Entry {
            name: "fs.readFile()".into(),
            path: "fs#fsreadfilepath-options-callback".into(),
            kind: "method".into(),
        };
        let plan = crate::semantic::plan_query("read file not readFileSync");
        assert!(entry_matches_negative_terms(&sync, &plan.negative_terms));
        assert!(!entry_matches_negative_terms(
            &async_read,
            &plan.negative_terms
        ));
    }

    #[test]
    fn qualified_paths_handle_reversed_docset_display_names() {
        let entry = Entry {
            name: "val map [Module List]".into(),
            path: "api/list#VALmap".into(),
            kind: "Values".into(),
        };
        assert_eq!(qualified_path_exact_score("List.map", &entry), Some(11_500));
        assert_eq!(qualified_path_exact_score("Map.map", &entry), None);
    }

    #[test]
    fn zig_std_paths_use_official_autodocs() {
        let outcome = zig_qualified_api_outcome("std.mem.eql", "zig std.mem.eql", 1).unwrap();
        assert_eq!(
            outcome.hits[0].url,
            "https://ziglang.org/documentation/master/std/#std.mem.eql"
        );
        assert!(zig_qualified_api_outcome("other.mem.eql", "zig other.mem.eql", 1).is_none());
    }

    #[test]
    fn generic_haskell_fallback_does_not_mask_package_discovery() {
        let mut fallback = Outcome::new("Haskell", "servant");
        fallback.hits.push(haskell_docs_fallback("servant"));
        assert!(!has_direct_haskell_api("servant", &fallback));

        let mut api = Outcome::new("Haskell", "map");
        api.hits.push(Hit::new(
            "map :: (a -> b) -> [a] -> [b]",
            "https://hackage.haskell.org/package/base/docs/Prelude.html#v:map",
        ));
        assert!(has_direct_haskell_api("map", &api));
    }

    #[test]
    fn embedded_identifiers_from_negative_spans_do_not_override_the_query() {
        assert_eq!(
            positive_embedded_identifier("use hmac.compare_digest not normal equality").as_deref(),
            Some("hmac.compare_digest")
        );
        assert!(
            positive_embedded_identifier("cryptographic random integer not random.randrange")
                .is_none()
        );
        assert_eq!(
            positive_embedded_identifier("pathlib.Path.read_text not read").as_deref(),
            Some("pathlib.Path.read_text")
        );
    }

    #[test]
    fn embedded_exact_identifiers_beat_surrounding_prose() {
        let entry = Entry {
            name: "feclearexcept".into(),
            path: "numeric/fenv/feclearexcept".into(),
            kind: "function".into(),
        };
        assert_eq!(
            entry_score("clear flags with feclearexcept now", &entry),
            Some(9_500)
        );
        let generic = Entry {
            name: "string".into(),
            path: "string".into(),
            kind: "type".into(),
        };
        assert!(entry_score("copy a string safely", &generic).unwrap_or_default() < 9_500);
    }

    #[test]
    fn cpp_task_aliases_use_compositional_cues() {
        for (query, expected) in [
            (
                "terminate handler not unexpected handler",
                "std::set_terminate",
            ),
            (
                "read whole file",
                "Read an entire file with std::ifstream and std::istreambuf_iterator",
            ),
            (
                "load file contents into string",
                "Read an entire file with std::ifstream and std::istreambuf_iterator",
            ),
            (
                "slurp text",
                "Read an entire file with std::ifstream and std::istreambuf_iterator",
            ),
            (
                "perfect forwarding",
                "std::forward and forwarding references",
            ),
            (
                "preserve value category when forwarding",
                "std::forward and forwarding references",
            ),
            ("polymorphic memory resource", "std::pmr::memory_resource"),
            ("Gregorian calendar dates", "std::chrono::year_month_day"),
            ("complex number arithmetic", "std::complex"),
            ("exception class hierarchy", "C++ exception hierarchy"),
            ("format a string", "std::format"),
            ("move semantics", "Move constructors and move assignment"),
            ("random number engine", "std::mt19937"),
            ("remove elements from vector", "std::erase_if"),
            ("split a string on a delimiter", "std::views::split"),
            ("visit variant", "std::visit"),
        ] {
            assert_eq!(
                cpp_task_alias(query).map(|alias| alias.0),
                Some(expected),
                "{query}"
            );
        }
        assert!(cpp_task_alias("split an integer").is_none());
        assert!(cpp_task_alias("read a file one line at a time").is_none());
    }

    #[test]
    fn c_task_aliases_use_compositional_cues() {
        for (query, expected) in [
            ("arrays", "Array declaration and subscripting"),
            ("array syntax", "Array declaration and subscripting"),
            (
                "type qualifiers",
                "Type qualifiers: const, restrict, volatile, and _Atomic",
            ),
            (
                "qualified types",
                "Type qualifiers: const, restrict, volatile, and _Atomic",
            ),
            ("read a file line by line", "fgets"),
            ("line input from a stream", "fgets"),
            (
                "copy overlapping memory",
                "memmove (overlap-safe memory copy)",
            ),
            (
                "move bytes in an overlapping buffer",
                "memmove (overlap-safe memory copy)",
            ),
            ("handle end of file", "feof"),
            ("check EOF", "feof"),
            ("sequence points", "C evaluation order and sequence points"),
            ("variadic preprocessor macros", "Variadic macros"),
            ("designated initialization", "Designated initializers"),
            ("header include guards", "Header include guards"),
            ("write one character not a formatted string", "fputc"),
            ("copy part of a string into a bounded buffer", "strncpy"),
            ("find first matching character span", "strpbrk"),
            ("transform string for locale collation", "strxfrm"),
            ("read one wide character", "fgetwc"),
        ] {
            assert_eq!(
                c_task_alias(query).map(|alias| alias.0),
                Some(expected),
                "{query}"
            );
        }
        assert!(c_task_alias("write a formatted string").is_none());
        assert!(c_task_alias("sort an array").is_none());
        assert!(c_task_alias("allocate zero initialized array").is_none());
        assert_eq!(
            c_stdbit_api_outcome("stdc_count_ones", "c stdc_count_ones", 1)
                .unwrap()
                .hits[0]
                .title,
            "stdc_count_ones"
        );
        assert!(c_task_alias("read a whole file").is_none());
    }

    #[test]
    fn python_task_aliases_use_compositional_cues() {
        for (query, expected) in [
            ("dictionaries", "Mapping Types — dict"),
            ("generator expressions", "Generator expressions"),
            ("JSON custom encoder", "json.JSONEncoder and default()"),
            ("pickle security", "pickle security warning"),
            ("asyncio timeout", "asyncio.timeout()"),
            (
                "parse email address not URL address",
                "email.utils.parseaddr()",
            ),
            ("enumerations", "enum.Enum"),
            ("keyword only parameters", "Keyword-only parameters"),
            ("merge two dictionaries", "Dictionary union operators"),
            ("parse an ISO datetime", "datetime.fromisoformat"),
            ("build URL query parameters", "urllib.parse.urlencode"),
            ("cleanup when the interpreter exits", "atexit.register"),
            (
                "preserve function metadata in a decorator",
                "functools.wraps",
            ),
            ("shell style splitting not regex splitting", "shlex.split"),
            ("constant time comparison", "hmac.compare_digest"),
            ("secure random integer", "secrets.randbelow"),
            (
                "find common path prefix by components",
                "os.path.commonpath()",
            ),
            (
                "wait for whichever future finishes first",
                "concurrent.futures.wait(..., FIRST_COMPLETED)",
            ),
            (
                "iterate over calendar month rows",
                "calendar.Calendar.monthdayscalendar",
            ),
        ] {
            assert_eq!(
                python_task_alias(query).map(|alias| alias.0),
                Some(expected),
                "{query}"
            );
        }
        assert!(python_task_alias("random floating point number").is_none());
    }

    #[test]
    fn go_task_aliases_use_compositional_cues() {
        for (query, expected) in [
            ("wake one waiting goroutine", "sync.Cond.Signal"),
            ("quote a Go string literal", "strconv.Quote"),
            ("tee reader data into a writer", "io.TeeReader"),
            ("compare strings with case folding", "strings.EqualFold"),
            ("escape HTML text not URL data", "html.EscapeString"),
            ("file permission mode", "fs.FileMode"),
            ("clear a map", "Go built-in clear"),
            (
                "find MIME type from filename extension",
                "mime.TypeByExtension",
            ),
            ("read exactly n bytes", "io.ReadFull"),
            (
                "lexical path cleanup not symlink resolution",
                "filepath.Clean",
            ),
        ] {
            assert_eq!(
                go_task_alias(query).map(|alias| alias.0),
                Some(expected),
                "{query}"
            );
        }
        assert_eq!(
            go_qualified_api_outcome("archive/zip.Writer.Copy", "archive/zip.Writer.Copy", 1)
                .unwrap()
                .hits[0]
                .url,
            "https://pkg.go.dev/archive/zip#Writer.Copy"
        );
        assert_eq!(
            go_qualified_api_outcome("github.com/acme/pkg.Type.Method", "query", 1)
                .unwrap()
                .hits[0]
                .url,
            "https://pkg.go.dev/github.com/acme/pkg#Type.Method"
        );
        assert!(go_task_alias("notify all waiting goroutines").is_none());
        assert!(go_task_alias("escape URL query data").is_none());
    }

    #[test]
    fn hackage_does_not_preempt_direct_haskell_apis() {
        let mut map = Outcome::new("Haskell", "map");
        map.hits.push(Hit::new(
            "map :: (a -> b) -> [a] -> [b]",
            "https://hackage.haskell.org/package/base/docs/Prelude.html#v:map",
        ));
        assert!(has_direct_haskell_api("map", &map));

        let mut aeson = Outcome::new("Haskell", "aeson");
        aeson
            .hits
            .push(Hit::new("module Data.Aeson", "https://example.test/aeson"));
        assert!(!has_direct_haskell_api("aeson", &aeson));
    }

    #[test]
    fn index_memory_cache_reuses_arc_and_keys_by_full_path() {
        let first_root = cache_test_dir("index-first");
        let second_root = cache_test_dir("index-second");
        let first_path = first_root.join("indexes/test.json");
        let second_path = second_root.join("indexes/test.json");
        let one = load_index_at(&first_path, "index-key", || Ok(test_index("one"))).unwrap();
        let again = load_index_at(&first_path, "index-key", || panic!("must reuse Arc")).unwrap();
        assert!(Arc::ptr_eq(&one, &again));

        // This models an XDG_CACHE_HOME change: an equal filename beneath a
        // different full cache directory must not reuse the first Arc.
        let two = load_index_at(&second_path, "index-key", || Ok(test_index("two"))).unwrap();
        assert!(!Arc::ptr_eq(&one, &two));
        assert_eq!(two.entries[0].name, "two");
        std::fs::remove_dir_all(first_root).unwrap();
        std::fs::remove_dir_all(second_root).unwrap();
    }

    #[test]
    fn page_notes_memory_cache_reuses_none_without_disk() {
        let root = cache_test_dir("notes-none");
        let path = "empty#missing";
        let first =
            page_notes_at(&root, "none-test", path, || Ok("<h1>Empty</h1>".into())).unwrap();
        assert_eq!(first, None);
        let second = page_notes_at(&root, "none-test", path, || {
            panic!("cached None must not fetch")
        })
        .unwrap();
        assert_eq!(second, None);
        assert!(!root.join("page-notes").exists());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn page_notes_separate_fragments_and_share_raw_page() {
        let root = cache_test_dir("notes-fragments");
        let html = r#"<h1>Page</h1>
            <section id="one"><p>The first fragment has enough useful text for a snippet.</p></section>
            <section id="two"><p>The second fragment has different useful text for a snippet.</p></section>"#;
        let first = page_notes_at(&root, "fragment-test", "guide#one", || Ok(html.into()))
            .unwrap()
            .unwrap();
        let second = page_notes_at(&root, "fragment-test", "guide#two", || {
            panic!("the raw page must be reused")
        })
        .unwrap()
        .unwrap();
        assert!(first.snippet.unwrap().contains("first fragment"));
        assert!(second.snippet.unwrap().contains("second fragment"));
        assert!(!root.join("page-notes").exists());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn devdocs_metadata_is_forced_to_one_safe_terminal_line() {
        let line = devdocs_meta_line(
            "Dev\x1b]8;;https://evil.test\x07Docs\u{202e}NAME\nFORGED",
            "Kind\x1b[2J\nLOG",
        );
        assert!(!line.contains('\n'));
        assert!(!line.contains('\u{1b}'));
        assert!(!line.contains('\u{202e}'));
        assert_eq!(
            line,
            "Dev ]8;;https://evil.test Docs NAME FORGED  ·  Kind [2J LOG"
        );
    }

    #[test]
    fn docset_slugs_cannot_escape_the_cache_root() {
        for unsafe_slug in [
            "",
            ".hidden",
            "../outside",
            "python/../../outside",
            "/tmp/evil",
        ] {
            assert!(!safe_docset_slug(unsafe_slug), "{unsafe_slug}");
        }
        for safe_slug in ["python~3.15", "c", "objective-c", "vue_router~4"] {
            assert!(safe_docset_slug(safe_slug), "{safe_slug}");
        }
    }

    #[test]
    fn haskell_api_normalizes_only_common_name_forms() {
        assert_eq!(normalize_haskell_api_query("map_maybe"), "mapMaybe");
        assert_eq!(normalize_haskell_api_query("split_on"), "splitOn");
        assert_eq!(normalize_haskell_api_query("applicatives"), "applicative");
        assert_eq!(
            normalize_haskell_api_query("sequences"),
            "Data.Sequence.Seq"
        );
        assert_eq!(normalize_haskell_api_query("process"), "process");
        assert_eq!(normalize_haskell_api_query("lens"), "lens");
        assert_eq!(
            normalize_haskell_api_query("pattern matching"),
            "pattern matching"
        );
    }

    #[test]
    fn camel_case_extension_probe_is_conservative() {
        assert!(looks_like_haskell_extension_name("LambdaCase"));
        assert!(looks_like_haskell_extension_name("OverloadedStrings"));
        assert!(looks_like_haskell_extension_name("GADTs"));
        assert!(!looks_like_haskell_extension_name("Text"));
        assert!(!looks_like_haskell_extension_name("lambda case"));
        assert!(!looks_like_haskell_extension_name("Data.Text"));
    }

    #[test]
    fn haskell_routes_language_concepts_to_the_right_manual() {
        let report_sections = [
            ("pattern matching", "#x8-580003.17"),
            ("guards", "#x10-830004.4.3"),
            ("list comprehensions", "#x8-420003.11"),
            ("type classes", "#x10-750004.3"),
            ("algebraic data types", "#x10-690004.2.1"),
            ("newtypes", "#x10-740004.2.3"),
            ("record syntax", "#x8-490003.15"),
            ("modules", "#x11-990005.1"),
            ("qualified imports", "#x11-1030005.3.2"),
            ("infix operators", "#x8-280003.4"),
            ("currying", "#x8-260003.3"),
            ("partial application", "#x8-260003.3"),
            ("higher order functions", "#x8-260003.3"),
        ];
        for (query, section) in report_sections {
            let hit = haskell_language_hit(query).unwrap();
            assert!(hit.url.ends_with(section), "{query}: {}", hit.url);
            assert_eq!(hit.meta, vec!["Haskell 2010 Report"]);
        }

        let tail_recursion = haskell_language_hit("tail recursion").unwrap();
        assert!(tail_recursion.url.contains("floopification"));
        assert_eq!(tail_recursion.meta, vec!["GHC User's Guide"]);

        let ghc = haskell_language_hit("visible type applications").unwrap();
        assert!(ghc.url.contains("users_guide/exts"));
        assert_eq!(ghc.meta, vec!["GHC User's Guide"]);

        let deriving = haskell_language_hit("deriving").unwrap();
        assert!(deriving.url.ends_with("/exts/deriving.html"));
        assert!(deriving.snippet.contains("deriving strategies"));

        let strict = haskell_language_hit("strict evaluation").unwrap();
        assert!(strict.url.ends_with("/exts/strict.html"));
        assert!(strict.snippet.contains("seq"));

        let thunks = haskell_language_hit("thunks").unwrap();
        assert!(thunks.snippet.contains("deferred computations"));

        // API and task queries must stay in the offline Haskell API lane.
        assert!(haskell_language_hit("Data.List.map").is_none());
        assert!(haskell_language_hit("remove duplicates").is_none());
        assert!(haskell_language_hit("record update function").is_none());
    }

    #[test]
    fn haskell_normalizes_common_tasks_to_canonical_apis() {
        assert_eq!(
            normalize_haskell_api_query("split a list at an index"),
            "Prelude.splitAt"
        );
        assert_eq!(
            normalize_haskell_api_query("sort a list on a key"),
            "Data.List.sortOn"
        );
        assert_eq!(
            normalize_haskell_api_query("remove list duplicates"),
            "Data.List.nub"
        );
        assert_eq!(
            normalize_haskell_api_query("replace a text substring"),
            "Data.Text.replace"
        );
        assert_eq!(
            normalize_haskell_api_query("check whether a file exists"),
            "System.Directory.doesFileExist"
        );
        assert_eq!(
            normalize_haskell_api_query("please list this directory"),
            "System.Directory.listDirectory"
        );

        // Do not rewrite queries merely because they share a generic word.
        assert_eq!(
            normalize_haskell_api_query("split package index"),
            "split package index"
        );
        assert_eq!(
            normalize_haskell_api_query("remove file extension"),
            "System.FilePath.dropExtension"
        );
        assert_eq!(
            normalize_haskell_api_query("text encoding theory"),
            "text encoding theory"
        );
    }

    #[test]
    fn picks_latest_alias_and_plain_slug() {
        let catalog = vec![
            docset("Python", "python~3.9", Some("py"), Some("3.9")),
            docset("Python", "python~3.14", Some("py"), Some("3.14")),
            docset("C++", "cpp", Some("c++"), None),
            docset("OCaml", "ocaml", None, Some("5.5")),
            docset("OCaml", "ocaml~4.14", None, Some("4.14")),
        ];
        assert_eq!(pick_docset(&catalog, "py").unwrap().slug, "python~3.14");
        assert_eq!(pick_docset(&catalog, "c++").unwrap().slug, "cpp");
        assert_eq!(pick_docset(&catalog, "ocaml").unwrap().slug, "ocaml");
        assert_eq!(pick_docset(&catalog, "python").unwrap().slug, "python~3.14");
    }

    #[test]
    fn ranks_exact_symbols_above_lookalikes() {
        let vector = score("vector", "vector").unwrap();
        let std_vector = score("vector", "std::vector").unwrap();
        assert!(vector > std_vector);
        let exact = score("std::vector::push_back", "std::vector::push_back").unwrap();
        let partial = score("std::vector::push_back", "std::vector::push_back_trivially").unwrap();
        assert!(exact > partial);
        assert!(score("std::vector::push_back", "std::vector::push_front").is_none());
        let list_map = score("List.map", "val map [Module List]").unwrap();
        let concat = score("List.map", "val concat_map [Module List]").unwrap();
        let labels = score("List.map", "val map [Module ListLabels]").unwrap();
        assert!(list_map > concat);
        assert!(list_map > labels);
        let counting = score("semaphore", "std::counting_semaphore").unwrap();
        let header = score("semaphore", "semaphore").unwrap();
        assert!(header > counting);
        assert!(counting >= 500);
        assert!(score("semaphore", "std::counting_semaphore::acquire").unwrap_or(0) < 500);
    }

    #[test]
    fn phrase_ranking_requires_coverage_across_languages() {
        // C++ collection and smart-pointer names should survive plural and
        // identifier normalization. A shared suffix alone is not enough.
        assert!(score("unordered maps", "std::unordered_map").unwrap_or(0) >= 500);
        assert!(
            score("unordered maps", "std::unordered_map").unwrap()
                > score("unordered maps", "std::mdspan::mapping").unwrap_or(0)
        );
        assert!(score("unordered maps", "std::mdspan::mapping").is_none());

        assert!(score("weak pointers", "std::weak_ptr").unwrap_or(0) >= 500);
        assert!(
            score("weak pointers", "std::weak_ptr").unwrap()
                > score("weak pointers", "hazard_pointer").unwrap_or(0)
        );
        assert!(score("weak pointers", "hazard_pointer").is_none());

        // Python's getgroups() contains the last query word only as an
        // incidental substring. It must not beat a complete topic heading.
        assert!(
            score(
                "regular expression named groups",
                "Regular expression syntax: named groups"
            )
            .unwrap_or(0)
                >= 500
        );
        assert!(score("regular expression named groups", "os.getgroups()").is_none());

        // The same coverage rule applies to C++ concept phrases and to the
        // equivalent title shapes used by Go and C documentation.
        assert!(score("move semantics", "Move semantics").unwrap_or(0) >= 500);
        assert!(score("move semantics", "std::contracts::evaluation_semantic").is_none());
        assert!(score("regular expressions", "regexp: regular expressions").unwrap_or(0) >= 500);
        assert!(score("atomic operations", "Atomic operations library").unwrap_or(0) >= 500);
    }

    #[test]
    fn medium_aliases_are_blended_without_taking_first_place() {
        let mut primary = Outcome::new("docs", "temporary file");
        primary.hits = vec![
            Hit::new("local one", "https://example.test/1"),
            Hit::new("local two", "https://example.test/2"),
            Hit::new("local three", "https://example.test/3"),
        ];
        let mut alias = Outcome::new("docs", "temporary file");
        alias.hits = vec![Hit::new("tmpfile", "https://example.test/tmp")];
        let merged = merge_outcomes(primary, Some(alias), 3);
        assert_eq!(merged.hits[0].title, "local one");
        assert_eq!(merged.hits[2].title, "tmpfile");
        assert_eq!(merged.hits.len(), 3);
    }

    #[test]
    fn entity_pages_beat_incidental_header_pages() {
        let header = Entry {
            name: "vector".into(),
            path: "header/vector".into(),
            kind: "Standard headers".into(),
        };
        let entity = Entry {
            name: "std::vector".into(),
            path: "container/vector".into(),
            kind: "Containers".into(),
        };
        assert!(entry_score("vector", &entity).unwrap() > entry_score("vector", &header).unwrap());
        assert!(entry_score("<vector> header", &header).is_some());
    }

    #[test]
    fn phrase_order_and_entity_compactness_affect_rank() {
        let phrase = score("named groups", "Named groups").unwrap();
        let reversed = score("named groups", "Groups with names").unwrap();
        assert!(phrase > reversed);

        let entity = score("thread", "thread").unwrap();
        let incidental = score("thread", "runtime.internal.thread.state").unwrap();
        assert!(entity > incidental);
    }

    #[test]
    fn relaxed_scoring_preserves_partial_local_results() {
        let package_path = relaxed_score("encoding/json.Unmarshal", "json.Unmarshal()").unwrap();
        let optional_parameter = relaxed_score("os.OpenFile flags", "os.OpenFile()").unwrap();
        let natural_phrase = relaxed_score("http client timeout", "http.Client").unwrap();
        assert!((1..500).contains(&package_path));
        assert!((1..500).contains(&optional_parameter));
        assert!((1..500).contains(&natural_phrase));
        assert!(relaxed_score("buffered channels", "fmt.Println()").is_none());
    }

    #[test]
    fn matches_common_word_forms_across_docsets() {
        assert!(score("caching", "functools.cache()").unwrap_or(0) >= 500);
        assert!(score("iterators", "Iterator").unwrap_or(0) >= 500);
        assert!(score("promises", "Promise").unwrap_or(0) >= 500);
        assert!(score("locking", "Mutex lock").unwrap_or(0) >= 500);
        assert!(score("rendering", "Renderer").is_none());
        assert!(score("lru cache", "functools.lru_cache()").unwrap_or(0) >= 500);
        assert!(score("user cache dir", "os.UserCacheDir()").unwrap_or(0) >= 500);
        assert!(score("http client", "HttpClient").unwrap_or(0) >= 500);
        assert!(score("xml http request", "XMLHttpRequest").unwrap_or(0) >= 500);
        assert!(score("add event listener", "EventTarget.addEventListener()").unwrap_or(0) >= 500);
        assert!(score("cache latest", "std::ranges::cache_latest_view").unwrap_or(0) >= 500);
        assert!(score("cache", "catch").is_none());
        assert!(score("map", "max").is_none());
        assert!(
            score("threads", "threading").unwrap()
                > score("threads", "decimal.HAVE_THREADS").unwrap()
        );
        assert!(
            score("threads", "threading").unwrap()
                > score("threads", "c.PyEval_SetTraceAllThreads()").unwrap()
        );
    }

    #[test]
    fn bash_default_and_assignment_expansions_keep_distinct_semantics() {
        let default =
            special_language_symbol_outcome("bash", "${parameter:-word}", "${parameter:-word}", 5)
                .unwrap();
        assert_eq!(default.hits[0].title, "${parameter:-word}");
        assert!(default.hits[0].snippet.contains("without assigning"));

        let assign =
            special_language_symbol_outcome("bash", "${parameter:=word}", "${parameter:=word}", 5)
                .unwrap();
        assert_eq!(assign.hits[0].title, "${parameter:=word}");
        assert!(assign.hits[0].snippet.contains("Assign word"));

        let contrast = special_language_symbol_outcome(
            "bash",
            "${parameter:-word}",
            "${parameter:-word} vs ${parameter:=word}",
            5,
        )
        .unwrap();
        assert_eq!(contrast.hits.len(), 2);
        assert_eq!(contrast.hits[0].title, "${parameter:-word}");
        assert_eq!(contrast.hits[1].title, "${parameter:=word}");
    }

    #[test]
    fn structural_prototype_and_symbol_arity_preserve_exactness() {
        assert_eq!(
            score("Array.prototype.toSorted", "Array.toSorted"),
            Some(10_000)
        );
        assert_eq!(score("|>", "|> (2)"), Some(11_000));
        assert_eq!(score("$!", "$!"), Some(11_000));
    }

    #[test]
    fn qualified_paths_match_members_across_docset_url_shapes() {
        let entry = |name: &str, path: &str| Entry {
            name: name.into(),
            path: path.into(),
            kind: String::new(),
        };
        assert_eq!(
            qualified_path_exact_score(
                "java.util.stream.Stream.map",
                &entry(
                    "Stream.map()",
                    "java.base/java/util/stream/stream#map(java.util.function.Function)"
                ),
            ),
            Some(11_500)
        );
        assert_eq!(
            qualified_path_exact_score(
                "Future.wait",
                &entry("Future<T>.wait<T>", "dart-async/future/wait"),
            ),
            Some(11_500)
        );
        assert_eq!(
            qualified_path_exact_score(
                "clojure.core/update-in",
                &entry("update-in", "clojure.core-api#clojure.core/update-in"),
            ),
            Some(11_500)
        );
        assert_eq!(
            qualified_path_exact_score(
                "URI.encode_query/1",
                &entry("URI.encode_query (2)", "uri#encode_query/2"),
            ),
            Some(11_500)
        );
    }

    #[test]
    fn exact_qualified_spelling_beats_case_folded_symbol_names() {
        assert!(
            score("inspect.signature", "inspect.signature()").unwrap()
                > score("inspect.signature", "inspect.Signature").unwrap()
        );
        assert_eq!(
            score("inspect.signature", "inspect.signature()"),
            Some(11_000)
        );
    }

    #[test]
    fn root_notes_include_only_the_intro_example() {
        let html = r#"<h1>A module</h1>
            <p>This module introduction is long enough to be selected as useful text.</p>
            <section id="introduction"><h2>Introduction</h2>
            <p>The introduction explains how the module is used in normal programs.</p>
            <pre>import useful
useful.run()</pre>
            <section id="intro-detail"><h3>Detail</h3><p>Nested intro detail.</p></section>
            </section>
            <section id="unrelated"><h2>Unrelated API</h2>
            <pre>dangerously_wrong_example()</pre></section>"#;
        let root = root_section(html);
        assert!(root.contains("useful.run"));
        assert!(root.contains("Nested intro detail"));
        assert!(!root.contains("dangerously_wrong_example"));
        assert!(usage_block(root).unwrap().contains("useful.run"));
    }

    #[test]
    fn snippet_windows_do_not_split_multibyte_unicode() {
        let mut root_html = format!("<h1>{}", "a".repeat(19_995));
        root_html.push('“');
        root_html.push_str("quoted”</h1>");
        let root = root_section(&root_html);
        assert_eq!(root.len(), 19_999);
        assert!(!root.contains('“'));

        let opening = r#"<dt id="target">"#;
        let mut section_html = opening.to_string();
        section_html.push_str(&"b".repeat(19_999 - opening.len()));
        section_html.push('🙂');
        section_html.push_str("text</dt><dd>description</dd>");
        let section = section_from_fragment(&section_html, "target").unwrap();
        assert_eq!(section.len(), 19_999);
        assert!(!section.contains('🙂'));
    }

    #[test]
    fn snippets_start_at_the_matched_section() {
        let html = r#"<h1 id="module">Module</h1>
            <p>This module introduction is long enough to be selected as a useful paragraph.</p>
            <dt id="module.Lock">Lock</dt><dd>
            <p>A lock blocks other callers until the owner releases the shared resource.</p>
            <pre>class Lock {
  void acquire();
}</pre></dd>
            <dt id="module.Event">Event</dt><dd>
            <p>An event is a different section.</p>
            <pre>class Event {}</pre></dd>
            <a name="legacy"></a>"#;
        let section = section_from_fragment(html, "module.Lock").unwrap();
        assert_eq!(
            first_paragraph(section).unwrap(),
            "A lock blocks other callers until the owner releases the shared resource."
        );
        assert!(!section.contains("module introduction"));
        assert!(usage_block(section).unwrap().contains("void acquire"));
        assert!(!section.contains("different section"));
        assert!(section_from_fragment(html, "module.Event").is_some());
        assert!(section_from_fragment(html, "legacy").is_some());
        assert!(section_from_fragment(html, "missing").is_none());
    }

    #[test]
    fn usage_block_keeps_the_example_and_drops_program_output() {
        let html = r#"<p>This header is part of the thread support library.</p>
            <pre>namespace std {
  class counting_semaphore {
  public:
    void acquire();
  };
}</pre>
            <pre data-language="c">#include &lt;semaphore&gt;
std::binary_semaphore
    smphSignalMainToThread{0};

void ThreadProc()
{
    smph.release();
    smph.acquire();
}
</pre>
            <pre>[main] Got the signal
[thread] Send the signal</pre>
            <p>Licensed under the CC license</p>"#;
        let usage = usage_block(html).unwrap();
        assert!(usage.starts_with("#include <semaphore>"));
        assert!(usage.contains("    smphSignalMainToThread{0};"));
        assert!(usage.contains("    smph.release();"));
        assert!(!usage.contains("[main]"));
        assert!(!usage.contains("Licensed"));
        let synopsis = usage_block(
            "<pre>namespace std {\n  class counting_semaphore {\n  public:\n    void acquire();\n  };\n}</pre>",
        )
        .unwrap();
        assert!(synopsis.contains("  class counting_semaphore {"));
        assert!(synopsis.contains("    void acquire();"));
        let sentence = explain_sentence(
            "<p>(since C++20) 1) A counting_semaphore is a lightweight synchronization primitive that can control access to a shared resource. 2) Other text.</p>",
        )
        .unwrap();
        assert_eq!(
            sentence,
            "A counting_semaphore is a lightweight synchronization primitive that can control access to a shared resource."
        );
    }

    #[test]
    fn platform_dirs_follow_the_host() {
        let home = PathBuf::from("/home/para");
        let cache = platform_dir("linux", None, None, Some(&home), "Caches", ".cache");
        assert_eq!(cache, PathBuf::from("/home/para/.cache/quicksearch"));
        let mac = platform_dir("macos", None, None, Some(&home), "Caches", ".cache");
        assert_eq!(mac, PathBuf::from("/home/para/Library/Caches/quicksearch"));
        let windows = platform_dir(
            "windows",
            None,
            Some(std::ffi::OsStr::new(r"C:\Users\para\AppData\Local")),
            Some(&home),
            "Caches",
            ".cache",
        );
        assert_eq!(
            windows,
            PathBuf::from(r"C:\Users\para\AppData\Local").join("quicksearch")
        );
        let forced = platform_dir(
            "linux",
            Some(std::ffi::OsStr::new("/xdg")),
            None,
            Some(&home),
            "Caches",
            ".cache",
        );
        assert_eq!(forced, PathBuf::from("/xdg/quicksearch"));
    }

    #[test]
    fn fuzzy_prefers_a_prefix_over_scattered_letters() {
        let python = fuzzy_score("pyth", "python").unwrap();
        let pytorch = fuzzy_score("pyth", "pytorch").unwrap();
        assert!(python > pytorch);
        assert!(fuzzy_score("hs", "hs").unwrap() > fuzzy_score("hs", "haskell").unwrap());
        assert!(fuzzy_score("ocam", "ocaml").is_some());
    }

    #[test]
    fn lang_search_orders_shorthands() {
        let groups = shorthand_groups(&[
            docset("Python", "python~3.14", Some("py"), Some("3.14.7")),
            docset("OCaml", "ocaml", None, Some("5.5")),
            docset("Pygame", "pygame", None, None),
        ]);
        let listed = rank_shorthands(&groups, "pyth", 5);
        assert!(listed.groups[0].label.starts_with("Python"));
        assert!(listed.groups[0].words.iter().any(|word| word == "py"));
        let ocaml = rank_shorthands(&groups, "ocam", 5);
        assert!(ocaml.groups[0].words.iter().any(|word| word == "ocaml"));
        assert!(ocaml.groups[0].words.iter().any(|word| word == "ml"));
        let haskell = rank_shorthands(&groups, "hs", 5);
        assert_eq!(haskell.groups[0].label, "Haskell");
        assert_eq!(haskell.groups[0].words[0], "hs");
    }
}
