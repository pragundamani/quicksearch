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

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use regex::Regex;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape, strip_tags, truncate};

const ITEM_KINDS: &[&str] = &[
    "struct",
    "enum",
    "trait",
    "derive",
    "union",
    "fn",
    "macro",
    "type",
    "constant",
    "primitive",
    "keyword",
];

const FRAGMENT_KINDS: &[&str] = &[
    "method",
    "tymethod",
    "variant",
    "associatedconstant",
    "associatedtype",
    "structfield",
];

const PROBE_CACHE_CAPACITY: usize = 256;
const POSITIVE_PROBE_TTL: Duration = Duration::from_secs(10 * 60);
const NEGATIVE_PROBE_TTL: Duration = Duration::from_secs(30);

static PROBE_CACHE: LazyLock<Mutex<BoundedCache<String, ProbeCacheEntry>>> =
    LazyLock::new(|| Mutex::new(BoundedCache::new(PROBE_CACHE_CAPACITY)));

#[derive(Clone, Copy)]
struct ProbeCacheEntry {
    live: bool,
    checked_at: Instant,
}

struct BoundedCache<K, V> {
    capacity: usize,
    entries: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Clone + Eq + Hash, V: Clone> BoundedCache<K, V> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        self.entries.get(key).cloned()
    }

    fn insert(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }
        if self.entries.contains_key(&key) {
            self.order.retain(|cached| cached != &key);
        }
        self.entries.insert(key.clone(), value);
        self.order.push_back(key);
        while self.entries.len() > self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            }
        }
    }

    fn remove(&mut self, key: &K) {
        self.entries.remove(key);
        self.order.retain(|cached| cached != key);
    }
}

pub fn search_std(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let mut outcome = Outcome::new("rust", query);
    let extracted = positive_embedded_identifier(query);
    if extracted.is_none() {
        if let Some(hit) = direct_task_hit(query) {
            outcome.source_url = Some(hit.url.clone());
            if limit > 0 {
                outcome.hits.push(hit);
            }
            return Ok(outcome);
        }
    }

    let api_query = select_api_query(query, extracted);
    let api_query = api_query.trim_end_matches('!');
    let path = normalize_std_path(api_query);
    let search = std_search_url(&path.replace("::", " "));
    outcome.source_url = Some(search.clone());

    let exact_api = api_query.contains("::")
        || api_alias(query).is_some()
        || query.trim().ends_with('!')
        || looks_like_item(api_query);

    // A qualified identifier embedded in prose is more specific than general
    // language/tooling words such as `unsafe`, `iterator`, or `FFI`.
    if !exact_api {
        if let Some(hit) = guide_hit(query) {
            outcome.source_url = Some(hit.url.clone());
            outcome.hits.push(hit);
            return Ok(outcome);
        }
    }

    if exact_api {
        if let Some(page) = resolve(client, "https://doc.rust-lang.org", &path) {
            let mut hit = page_hit(&page);
            enrich_api_hit(query, &mut hit);
            outcome.hits.push(hit);
        }
    } else {
        if let Ok(guides) = super::rust_guides::search(client, query, limit) {
            let mut seen: std::collections::HashSet<String> =
                outcome.hits.iter().map(|hit| hit.url.clone()).collect();
            outcome.hits.extend(
                guides
                    .into_iter()
                    .filter(|hit| seen.insert(hit.url.clone())),
            );
        }
        outcome.hits.truncate(limit);
    }
    if outcome.hits.len() < limit {
        outcome.hits.push(
            Hit::new(
                format!("Search Rust standard library for {api_query}"),
                search,
            )
            .snippet("Searches matching items in std, core, and alloc."),
        );
    }
    Ok(outcome)
}

fn select_api_query(query: &str, embedded: Option<String>) -> String {
    embedded
        .or_else(|| api_alias(query).map(str::to_string))
        .unwrap_or_else(|| query.to_string())
}

/// Extract an API path only from the positive side of a contrast.
///
/// An exact path outranks surrounding prose, but a path supplied only as the
/// rejected alternative must not take over the query.
fn positive_embedded_identifier(query: &str) -> Option<String> {
    let lower = query.to_ascii_lowercase();
    let contrast = [" not ", " rather than ", " instead of ", " without "]
        .iter()
        .filter_map(|marker| lower.find(marker))
        .min();
    let positive = contrast.map_or(query, |index| &query[..index]);
    crate::semantic::extract_qualified_identifier(positive)
}

fn direct_task_hit(query: &str) -> Option<Hit> {
    let q = query.trim().to_ascii_lowercase();
    let words: Vec<_> = q
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|word| !word.is_empty())
        .collect();
    let has = |choices: &[&str]| words.iter().any(|word| choices.contains(word));
    let route = if has(&["lifetime", "lifetimes"]) {
        (
            "Validating references with lifetimes",
            "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html",
            "Lifetime annotations relate borrowed references; lifetime elision infers common signatures.",
        )
    } else if has(&["mutable", "mut"]) && has(&["reference", "references", "borrow", "borrows"]) {
        (
            "Mutable references and borrowing",
            "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html",
            "A mutable borrow is written &mut T; Rust permits only the compatible shared or mutable borrows.",
        )
    } else if has(&["struct", "structs"])
        && has(&["destructure", "destructuring", "pattern", "patterns"])
    {
        (
            "Struct patterns",
            "https://doc.rust-lang.org/reference/patterns.html#struct-patterns",
            "Struct patterns destructure named fields with syntax such as Point { x, y }.",
        )
    } else if q.contains("let else") || q.contains("let-else") {
        (
            "let-else statements",
            "https://doc.rust-lang.org/reference/statements.html#let-statements",
            "let-else destructures a refutable pattern and requires a diverging else block.",
        )
    } else if q.contains("dyn trait") || (has(&["trait"]) && has(&["object", "objects"])) {
        (
            "Trait objects and dyn Trait",
            "https://doc.rust-lang.org/reference/types/trait-object.html",
            "Trait objects use dyn Trait for dynamic dispatch.",
        )
    } else if has(&["generic", "generics"]) && has(&["function", "functions"]) {
        (
            "Generic functions",
            "https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions",
            "Declare type parameters on functions with angle brackets.",
        )
    } else if has(&["where"]) && has(&["clause", "clauses", "bound", "bounds"]) {
        (
            "where clauses",
            "https://doc.rust-lang.org/reference/trait-bounds.html#boundwhere",
            "where clauses express generic trait and lifetime bounds.",
        )
    } else if has(&["const"]) && has(&["generic", "generics", "parameter", "parameters"]) {
        (
            "Const generic parameters",
            "https://doc.rust-lang.org/reference/items/generics.html#const-generics",
            "Const parameters let items abstract over compile-time values.",
        )
    } else if q.contains("pub(crate)") || (has(&["crate"]) && has(&["visibility", "visible"])) {
        (
            "pub(crate) visibility",
            "https://doc.rust-lang.org/reference/visibility-and-privacy.html",
            "Restricted visibility can expose an item to its current crate.",
        )
    } else if has(&["derive"]) && has(&["macro", "macros"]) {
        (
            "Derive macros",
            "https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros",
            "Derive macros generate implementations for items with #[derive(...)].",
        )
    } else if has(&["collect"]) && has(&["iterator", "iter"]) && has(&["vector", "vec"]) {
        (
            "Iterator::collect into Vec",
            "https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect",
            "Collect iterator elements into a Vec or another FromIterator collection.",
        )
    } else if has(&["atomic"]) && has(&["reference"]) && has(&["count", "counted", "counting"]) {
        (
            "Arc atomic reference counting",
            "https://doc.rust-lang.org/std/sync/struct.Arc.html",
            "Arc is an atomically reference-counted pointer for shared ownership across threads.",
        )
    } else if has(&["reference", "counted", "referencecounted"]) && has(&["pointer", "pointers"]) {
        (
            "Rc reference-counted pointer",
            "https://doc.rust-lang.org/std/rc/struct.Rc.html",
            "Rc provides shared single-threaded ownership through reference counting.",
        )
    } else if has(&["unix"]) && has(&["timestamp", "epoch", "time"]) {
        (
            "SystemTime::UNIX_EPOCH",
            "https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html",
            "The Unix timestamp epoch for SystemTime calculations.",
        )
    } else if has(&["inclusive"]) && has(&["range", "ranges"]) {
        (
            "RangeInclusive",
            "https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html",
            "An inclusive a..=b range includes both endpoints.",
        )
    } else if has(&["tuple"]) && has(&["struct", "structs"]) {
        ("Tuple structs", "https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types", "Tuple structs have unnamed fields accessed by position.")
    } else if has(&["named"]) && has(&["field", "fields"]) && has(&["struct", "structs"]) {
        (
            "Structs with named fields",
            "https://doc.rust-lang.org/book/ch05-01-defining-structs.html",
            "Struct definitions associate names and types with each field.",
        )
    } else if has(&["enum", "enums"]) && has(&["data", "field", "fields", "payload"]) {
        (
            "Enums with variant data",
            "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html",
            "Rust enum variants can store distinct kinds and shapes of data.",
        )
    } else if has(&["derive"]) && q.contains("partialeq") && has(&["ord"]) {
        ("Derivable comparison traits", "https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialeq-and-eq-for-equality-comparisons", "Derive PartialEq, Eq, PartialOrd, and Ord for structural comparisons.")
    } else if has(&["procedural", "proc"]) && has(&["macro", "macros"]) {
        (
            "Procedural macros",
            "https://doc.rust-lang.org/reference/procedural-macros.html",
            "Procedural macros have three forms: function-like, derive, and attribute macros.",
        )
    } else if has(&["cfg"]) && has(&["target_os", "target"]) {
        (
            "Conditional compilation attributes",
            "https://doc.rust-lang.org/reference/conditional-compilation.html",
            r#"Use #[cfg(target_os = "...")] to compile an item only for a selected operating system."#,
        )
    } else if has(&["attribute", "attributes"]) {
        (
            "Rust attributes",
            "https://doc.rust-lang.org/reference/attributes.html",
            "Outer attributes use #[...] on the following item; inner attributes use #![...] on the enclosing item.",
        )
    } else if has(&["raw"]) && has(&["pointer", "pointers"]) {
        (
            "Raw pointer types",
            "https://doc.rust-lang.org/reference/types/pointer.html",
            "Raw pointer types are *const T for immutable pointers and *mut T for mutable pointers.",
        )
    } else if (has(&["foreign"]) && has(&["function", "interface"])) || has(&["ffi"]) {
        (
            "External blocks and FFI",
            "https://doc.rust-lang.org/reference/items/external-blocks.html",
            r#"Foreign functions are declared in unsafe extern ABI blocks, such as extern "C"."#,
        )
    } else if q == "macro_rules" || q == "macro_rules!" || (has(&["macro"]) && has(&["rules"])) {
        (
            "Macros by example: macro_rules!",
            "https://doc.rust-lang.org/reference/macros-by-example.html",
            "Declarative macro_rules! syntax and matching rules.",
        )
    } else if q == "no_std" || (has(&["no_std", "nostd"])) {
        (
            "The no_std attribute",
            "https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute",
            "Use core instead of the standard-library prelude in no_std crates.",
        )
    } else if q == "binary_search" || (has(&["binary"]) && has(&["search"]) && has(&["slice"])) {
        (
            "slice::binary_search",
            "https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search",
            "Binary search a sorted slice.",
        )
    } else if q == "chunks_exact_mut"
        || (has(&["chunk", "chunks", "chunks_exact_mut"])
            && has(&["exact"])
            && has(&["mutable", "mut"]))
    {
        (
            "slice::chunks_exact_mut",
            "https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact_mut",
            "Iterate over exact-size mutable slice chunks.",
        )
    } else if has(&["filter", "keep"])
        && has(&["successful", "success", "ok"])
        && has(&["iterator", "values", "results"])
    {
        (
            "Iterator::filter_map",
            "https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map",
            "Transform iterator items while keeping successful Some values.",
        )
    } else if has(&["option"]) && has(&["result"]) && has(&["lazy"]) && has(&["error"]) {
        (
            "Option::ok_or_else",
            "https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else",
            "Convert an Option to Result with a lazily created error.",
        )
    } else if (q.contains("remove prefix")
        || q.contains("strip prefix")
        || q.contains("replace matching slice prefix"))
        && has(&["string", "str", "slice"])
    {
        (
            "slice::strip_prefix",
            "https://doc.rust-lang.org/std/primitive.slice.html#method.strip_prefix",
            "Return a slice with a matching prefix removed.",
        )
    } else if has(&["remove", "strip"]) && has(&["suffix"]) && has(&["string", "str"]) {
        (
            "str::strip_suffix",
            "https://doc.rust-lang.org/std/primitive.str.html#method.strip_suffix",
            "Return the string slice with a matching suffix removed.",
        )
    } else if has(&["split"]) && has(&["once", "one"]) && has(&["slice"]) {
        (
            "slice::split_once",
            "https://doc.rust-lang.org/std/primitive.slice.html#method.split_once",
            "Split a slice at the first matching element.",
        )
    } else if has(&["get"])
        && has(&["two", "multiple", "many"])
        && has(&["mutable", "mut"])
        && has(&["slice", "elements"])
    {
        (
            "slice::get_disjoint_mut",
            "https://doc.rust-lang.org/std/primitive.slice.html#method.get_disjoint_mut",
            "Borrow multiple distinct slice elements mutably and safely.",
        )
    } else if has(&["saturating"]) && has(&["multiplication", "multiply", "mul"]) {
        (
            "saturating_mul",
            "https://doc.rust-lang.org/std/primitive.u32.html#method.saturating_mul",
            "Saturating integer multiplication clamps at the numeric bounds.",
        )
    } else if has(&["checked"]) && has(&["addition", "add"]) {
        (
            "checked_add",
            "https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add",
            "Checked integer addition returns None on overflow.",
        )
    } else if has(&["rotate", "rotate_left"]) && has(&["slice"]) {
        (
            "slice::rotate_left",
            "https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_left",
            "Rotate a slice in-place to the left.",
        )
    } else if has(&["fold"]) && has(&["value", "values", "iterator"]) {
        (
            "Iterator::fold",
            "https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold",
            "Combine iterator elements into one accumulated value.",
        )
    } else if has(&["atomicusize"]) && has(&["ordering", "orderings"]) {
        (
            "AtomicUsize and Ordering",
            "https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html",
            "Atomic integer operations take explicit memory orderings.",
        )
    } else if has(&["move"]) && has(&["closure", "closures"]) {
        ("Moving captured values into closures", "https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits", "Move closures take ownership of captured values.")
    } else if has(&["send"]) && has(&["sync"]) {
        (
            "Send and Sync",
            "https://doc.rust-lang.org/nomicon/send-and-sync.html",
            "The Send and Sync marker traits define thread-safety boundaries.",
        )
    } else if has(&["from"]) && has(&["into"]) {
        (
            "From and Into conversions",
            "https://doc.rust-lang.org/std/convert/index.html",
            "Standard traits for value-to-value conversions.",
        )
    } else if has(&["clone"]) && has(&["copy"]) {
        ("Clone and Copy", "https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#clone-and-copy-for-duplicating-values", "Copy duplicates implicitly; Clone duplicates explicitly.")
    } else if has(&["external", "extern"]) && has(&["crate", "crates"]) {
        (
            "External crates and the extern prelude",
            "https://doc.rust-lang.org/reference/names/preludes.html#extern-prelude",
            "Names from dependencies are available through the extern prelude.",
        )
    } else {
        return None;
    };
    Some(Hit::new(route.0, route.1).snippet(route.2))
}

fn guide_hit(query: &str) -> Option<Hit> {
    let q = query.trim().to_ascii_lowercase();
    let route = if contains_any(
        &q,
        &[
            "cargo feature",
            "optional dependencies",
            "cargo manifest",
            "cargo.toml",
        ],
    ) {
        (
            "Cargo features and optional dependencies",
            "https://doc.rust-lang.org/cargo/reference/features.html",
            "Cargo feature flags enable conditional compilation and optional dependencies.",
        )
    } else if contains_any(&q, &["intra-doc", "intra doc", "rustdoc link"]) {
        (
            "Rustdoc intra-doc links",
            "https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html",
            "Link to Rust items by path and let rustdoc check the link.",
        )
    } else if q.contains("edition") {
        (
            "Rust 2024 Edition Guide",
            "https://doc.rust-lang.org/edition-guide/rust-2024/index.html",
            "Migration notes and changes for the Rust 2024 edition.",
        )
    } else if contains_any(
        &q,
        &[
            "ownership",
            "move closure",
            "clone vs copy",
            "reference counted",
            "atomic reference counting",
        ],
    ) {
        (
            "Understanding ownership",
            "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
            "Ownership, moves, borrowing, slices, Rc, and Arc explained in context.",
        )
    } else if contains_any(
        &q,
        &[
            "borrow checker",
            "mutable reference",
            "lifetimes",
            "lifetime elision",
        ],
    ) {
        (
            "References and borrowing",
            "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html",
            "Rust checks shared and mutable borrows so references remain valid.",
        )
    } else if contains_any(&q, &["recoverable error", "question mark operator"]) {
        (
            "Error handling",
            "https://doc.rust-lang.org/book/ch09-00-error-handling.html",
            "Result represents recoverable errors; the ? operator propagates them.",
        )
    } else if contains_any(&q, &["unit test", "integration test"]) {
        (
            "Writing automated tests",
            "https://doc.rust-lang.org/book/ch11-01-writing-tests.html",
            "Use #[test] functions with Rust's test harness.",
        )
    } else if contains_any(&q, &["documentation test", "doctest"]) {
        (
            "Rustdoc documentation tests",
            "https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html",
            "Rustdoc compiles and runs Rust code blocks as documentation tests.",
        )
    } else if contains_any(
        &q,
        &[
            "pattern matching",
            "destructuring",
            "if let",
            "let else",
            "while let",
        ],
    ) {
        (
            "Patterns",
            "https://doc.rust-lang.org/reference/patterns.html",
            "Patterns destructure values in let bindings, if let, while let, and match.",
        )
    } else if contains_any(
        &q,
        &[
            "trait",
            "associated type",
            "where clause",
            "generic function",
            "const generic",
            "send and sync",
            "from and into",
        ],
    ) {
        (
            "Traits and generics",
            "https://doc.rust-lang.org/reference/items/traits.html",
            "Trait items, associated types, bounds, and implementations.",
        )
    } else if contains_any(
        &q,
        &["module", "pub(crate)", "use declaration", "external crate"],
    ) {
        (
            "Modules and visibility",
            "https://doc.rust-lang.org/reference/items/modules.html",
            "Modules organize items; use imports names and visibility controls access.",
        )
    } else if contains_any(&q, &["procedural macro", "derive macro", "macro_rules"]) {
        (
            "Rust macros",
            "https://doc.rust-lang.org/reference/macros.html",
            "Rust supports declarative macro_rules! macros and procedural macros.",
        )
    } else if contains_any(&q, &["attribute", "cfg target", "no_std"]) {
        (
            "Rust attributes",
            "https://doc.rust-lang.org/reference/attributes.html",
            "Attributes control conditional compilation, derives, crate behavior, and tools.",
        )
    } else if contains_any(&q, &["async fn", ".await"]) {
        (
            "Async and await expressions",
            "https://doc.rust-lang.org/reference/expressions/await-expr.html",
            "Async functions return futures; .await polls a future to completion.",
        )
    } else if contains_any(
        &q,
        &["unsafe rust", "raw pointer", "foreign function", "ffi"],
    ) {
        (
            "Unsafe Rust",
            "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html",
            "Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations.",
        )
    } else if contains_any(
        &q,
        &[
            "tuple struct",
            "named field struct",
            "enums with data",
            "impl block",
        ],
    ) {
        (
            "Structs, enums, and implementations",
            "https://doc.rust-lang.org/reference/items.html",
            "Rust item syntax defines structs, enums, and inherent impl blocks.",
        )
    } else if q.contains("raw identifier") {
        (
            "Identifiers",
            "https://doc.rust-lang.org/reference/identifiers.html",
            "Raw identifiers such as r#type use a keyword as an identifier.",
        )
    } else {
        return None;
    };
    Some(Hit::new(route.0, route.1).snippet(route.2))
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn api_alias(query: &str) -> Option<&'static str> {
    let q = query.trim().to_ascii_lowercase();
    let words: Vec<_> = q
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|word| !word.is_empty())
        .collect();
    let has = |choices: &[&str]| words.iter().any(|word| choices.contains(word));
    if has(&["parse", "parsing"]) && has(&["socket"]) && has(&["address", "addresses"]) {
        return Some("std::net::SocketAddr");
    }
    if has(&["mutex", "mutexes"]) {
        return Some("std::sync::Mutex");
    }

    let aliases: &[(&[&str], &str)] = &[
        (&["print a line", "print line"], "std::println"),
        (
            &["read file to string", "read a file to string"],
            "std::fs::read_to_string",
        ),
        (&["write bytes to a file", "write file"], "std::fs::write"),
        (&["canonicalize a path"], "std::fs::canonicalize"),
        (
            &[
                "create directories recursively",
                "create directory recursively",
            ],
            "std::fs::create_dir_all",
        ),
        (&["spawn a thread"], "std::thread::spawn"),
        (
            &["run command and capture output"],
            "std::process::Command::output",
        ),
        (&["command line arguments"], "std::env::args"),
        (&["environment variables"], "std::env::var"),
        (&["measure elapsed time"], "std::time::Instant::elapsed"),
        (&["parse integer"], "std::str::parse"),
        (&["sort vectors", "sort vector"], "std::slice::sort"),
        (&["hash sets"], "std::collections::HashSet"),
        (&["binary heaps"], "std::collections::BinaryHeap"),
        (&["mpsc channels"], "std::sync::mpsc"),
    ];
    aliases.iter().find_map(|(phrases, path)| {
        phrases
            .iter()
            .any(|phrase| q.contains(phrase))
            .then_some(*path)
    })
}

fn looks_like_item(query: &str) -> bool {
    let bare = query.trim_matches(|ch: char| ch == '!' || ch == '<' || ch == '>');
    !bare.contains(char::is_whitespace)
        && bare.chars().any(|ch| ch.is_ascii_alphabetic())
        && (bare.contains('_') || bare.chars().next().is_some_and(char::is_uppercase))
}

pub fn resolve(client: &Client, root: &str, path: &str) -> Option<Page> {
    if let Some(page) = probe_item(client, root, path) {
        return Some(page);
    }
    let parts: Vec<&str> = path.split("::").filter(|part| !part.is_empty()).collect();
    if parts.len() < 2 {
        return None;
    }
    let child = parts[parts.len() - 1];
    let parent = parts[..parts.len() - 1].join("::");
    let mut page = probe_item(client, root, &parent)?;
    let fragment = find_fragment(&page.html, child)?;
    page.url = format!("{}#{fragment}", page.url);
    page.title = format!("{}::{child}", parts[parts.len() - 2]);
    page.html.clear();
    Some(page)
}

pub struct Page {
    pub url: String,
    pub title: String,
    pub snippet: String,
    html: String,
}

fn page_hit(page: &Page) -> Hit {
    Hit::new(&page.title, &page.url).snippet(&page.snippet)
}

fn enrich_api_hit(query: &str, hit: &mut Hit) {
    let q = query.trim().to_ascii_lowercase();
    let context = if q.contains("environment") && q.contains("variable") {
        Some("Use std::env::var for one value and std::env::vars to iterate over all variables.")
    } else if q.contains("command") && q.contains("argument") {
        Some("Use std::env::args for Unicode arguments and args_os when values may not be valid Unicode.")
    } else if q.contains("parse") && q.contains("integer") {
        Some("Select an integer type, for example text.parse::<i32>(); failures return ParseIntError.")
    } else if q.contains("parse") && q.contains("socket") && q.contains("address") {
        Some("SocketAddr implements FromStr, so an IP address and port can be parsed with str::parse.")
    } else if q.contains("asref") && q.contains("path") {
        Some("An AsRef<Path> bound accepts values that can be cheaply borrowed as a Path.")
    } else {
        None
    };
    if let Some(context) = context {
        hit.snippet = context.to_string();
    }
}

fn probe_item(client: &Client, root: &str, path: &str) -> Option<Page> {
    let ranked = candidate_urls(root, path);
    let url = first_live(client, ranked)?;
    let html = client.get_text(&url, APP_UA, &[]).ok()?;
    let (title, snippet) = page_text(&html);
    let title = if title.is_empty() {
        path.to_string()
    } else {
        title
    };
    Some(Page {
        url,
        title,
        snippet,
        html,
    })
}

pub fn candidate_urls(root: &str, path: &str) -> Vec<(u8, String)> {
    let parts: Vec<&str> = path.split("::").filter(|part| !part.is_empty()).collect();
    if parts.is_empty() {
        return Vec::new();
    }
    let item = parts[parts.len() - 1];
    let parent = parts[..parts.len() - 1].join("/");
    let prefix = if parent.is_empty() {
        root.trim_end_matches('/').to_string()
    } else {
        format!("{}/{}", root.trim_end_matches('/'), parent)
    };
    let type_name = item.chars().next().is_some_and(|ch| ch.is_uppercase());
    let mut urls = Vec::new();
    for kind in ITEM_KINDS {
        urls.push((
            kind_rank(type_name, kind),
            format!("{prefix}/{kind}.{item}.html"),
        ));
    }
    // Prefer a real item, then the module page, then primitive pages that share a name.
    urls.push((30, format!("{prefix}/{item}/index.html")));
    urls
}

fn kind_rank(type_name: bool, kind: &str) -> u8 {
    let order: &[&str] = if type_name {
        &[
            "struct",
            "enum",
            "trait",
            "derive",
            "union",
            "type",
            "macro",
            "primitive",
            "fn",
            "constant",
            "keyword",
        ]
    } else {
        &[
            "fn",
            "macro",
            "constant",
            "type",
            "primitive",
            "keyword",
            "struct",
            "enum",
            "trait",
            "derive",
            "union",
        ]
    };
    order
        .iter()
        .position(|name| *name == kind)
        .map(|index| index as u8)
        .unwrap_or(20)
}

fn first_live(client: &Client, mut ranked: Vec<(u8, String)>) -> Option<String> {
    ranked.sort_by_key(|(rank, _)| *rank);
    for (_, url) in ranked {
        if let Some(live) = cached_probe(&url, Instant::now()) {
            if live {
                return Some(url);
            }
            continue;
        }
        let live = probe_url(&client.agent, &url);
        cache_probe(url.clone(), live, Instant::now());
        if live {
            return Some(url);
        }
    }
    None
}

fn probe_url(agent: &ureq::Agent, url: &str) -> bool {
    match agent.request("HEAD", url).set("User-Agent", APP_UA).call() {
        Ok(resp) => (200..300).contains(&resp.status()),
        Err(ureq::Error::Status(code, _)) => (200..300).contains(&code),
        Err(_) => false,
    }
}

fn cached_probe(url: &str, now: Instant) -> Option<bool> {
    let mut cache = PROBE_CACHE.lock().ok()?;
    let key = url.to_string();
    let entry = cache.get(&key)?;
    if let Some(live) = fresh_probe(entry, now) {
        Some(live)
    } else {
        cache.remove(&key);
        None
    }
}

fn fresh_probe(entry: ProbeCacheEntry, now: Instant) -> Option<bool> {
    let ttl = if entry.live {
        POSITIVE_PROBE_TTL
    } else {
        NEGATIVE_PROBE_TTL
    };
    (now.saturating_duration_since(entry.checked_at) <= ttl).then_some(entry.live)
}

fn cache_probe(url: String, live: bool, checked_at: Instant) {
    if let Ok(mut cache) = PROBE_CACHE.lock() {
        cache.insert(url, ProbeCacheEntry { live, checked_at });
    }
}

#[cfg(test)]
enum ProbeSelection<'a> {
    Pending,
    Found(&'a str),
    NotFound,
}

#[cfg(test)]
fn select_ranked<'a>(ranked: &'a [(u8, String)], results: &[Option<bool>]) -> ProbeSelection<'a> {
    for ((_, url), result) in ranked.iter().zip(results) {
        match result {
            Some(true) => return ProbeSelection::Found(url),
            Some(false) => {}
            None => return ProbeSelection::Pending,
        }
    }
    ProbeSelection::NotFound
}

fn find_fragment(html: &str, name: &str) -> Option<String> {
    for kind in FRAGMENT_KINDS {
        let id = format!("id=\"{kind}.{name}\"");
        if html.contains(&id) {
            return Some(format!("{kind}.{name}"));
        }
    }
    None
}

fn page_text(html: &str) -> (String, String) {
    static TITLE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)<title>(.*?)</title>").unwrap());
    static DESC_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"(?is)<meta[^>]*name=["']description["'][^>]*content=["']([^"']*)["']|(?is)<meta[^>]*content=["']([^"']*)["'][^>]*name=["']description["']"#,
        )
        .unwrap()
    });
    let title = TITLE_RE
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| collapse_ws(&html_unescape(&strip_tags(m.as_str()))))
        .unwrap_or_default();
    let snippet = DESC_RE
        .captures(html)
        .and_then(|cap| cap.get(1).or_else(|| cap.get(2)))
        .map(|m| truncate(&collapse_ws(&html_unescape(m.as_str())), 320))
        .unwrap_or_default();
    (title, snippet)
}

pub fn normalize_std_path(query: &str) -> String {
    let path = strip_generics(query.trim().trim_matches(':').trim_end_matches('!'));
    let mut parts: Vec<&str> = path.split("::").filter(|part| !part.is_empty()).collect();
    let has_crate = matches!(
        parts.first(),
        Some(&"std") | Some(&"core") | Some(&"alloc") | Some(&"proc_macro") | Some(&"test")
    );
    if !has_crate {
        parts.insert(0, "std");
    }

    // Rustdoc URLs include the defining module. Users commonly omit it when
    // writing Type::method, so supply it from a compact standard-library index.
    let symbol_index = 1;
    if parts.len() > symbol_index {
        if let Some(module) = standard_module(parts[symbol_index]) {
            if module != parts[symbol_index] {
                parts.insert(symbol_index, module);
            }
        }
    }
    parts.join("::")
}

fn strip_generics(path: &str) -> String {
    let mut depth = 0_u32;
    path.chars()
        .filter(|ch| match ch {
            '<' => {
                depth += 1;
                false
            }
            '>' => {
                depth = depth.saturating_sub(1);
                false
            }
            _ => depth == 0,
        })
        .collect()
}

fn standard_module(item: &str) -> Option<&'static str> {
    Some(match item {
        "Vec" => "vec",
        "VecDeque" => "collections",
        "HashMap" | "HashSet" | "BTreeMap" | "BTreeSet" | "BinaryHeap" => "collections",
        "String" => "string",
        "Option" => "option",
        "Result" => "result",
        "Iterator" | "IntoIterator" | "ExactSizeIterator" => "iter",
        "From" | "Into" | "TryFrom" | "TryInto" | "AsRef" => "convert",
        "Cow" => "borrow",
        "Box" => "boxed",
        "Rc" => "rc",
        "Arc" | "Mutex" | "RwLock" | "OnceLock" => "sync",
        "RefCell" | "Cell" => "cell",
        "TcpListener" | "TcpStream" | "SocketAddr" => "net",
        "Duration" | "Instant" | "SystemTime" => "time",
        "Command" => "process",
        "Path" | "PathBuf" => "path",
        "AtomicUsize" | "Ordering" => "sync::atomic",
        "Drop" => "ops",
        "Default" => "default",
        "Clone" | "Copy" => "clone",
        _ => return None,
    })
}

fn std_search_url(query: &str) -> String {
    format!(
        "https://doc.rust-lang.org/std/?search={}",
        encode_query(query)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_exact_paths_outrank_surrounding_rust_topics() {
        for query in [
            "safe fast standard library tutorial std::ffi::OsStr::as_encoded_bytes",
            "FFI guide with std::ffi::OsStr::as_encoded_bytes examples",
            "unsafe tutorial std::ffi::OsStr::as_encoded_bytes now",
        ] {
            assert_eq!(
                positive_embedded_identifier(query).as_deref(),
                Some("std::ffi::OsStr::as_encoded_bytes"),
                "{query}"
            );
            // These prose cues otherwise trigger generic language pages.
            assert!(direct_task_hit(query).is_some(), "{query}");
        }

        let query = "read file to string, but use std::ffi::OsStr::as_encoded_bytes";
        let embedded = positive_embedded_identifier(query);
        assert_eq!(
            select_api_query(query, embedded),
            "std::ffi::OsStr::as_encoded_bytes"
        );
    }

    #[test]
    fn negative_exact_paths_do_not_override_positive_rust_intent() {
        assert_eq!(
            positive_embedded_identifier(
                "foreign function interface not std::ffi::OsStr::as_encoded_bytes"
            ),
            None
        );
        assert_eq!(
            positive_embedded_identifier(
                "std::ffi::OsStr::as_encoded_bytes not generic FFI tutorial"
            )
            .as_deref(),
            Some("std::ffi::OsStr::as_encoded_bytes")
        );
        assert_eq!(
            positive_embedded_identifier(
                "foreign function interface without std::ffi::OsStr::as_encoded_bytes"
            ),
            None
        );
    }

    #[test]
    fn compositional_task_hits_use_focused_authoritative_pages() {
        for (query, expected) in [
            ("keep successful iterator values", "filter_map"),
            ("turn Option into Result with a lazy error", "ok_or_else"),
            ("remove suffix not prefix from string slice", "strip_suffix"),
            ("split a slice once", "split_once"),
            ("checked addition not wrapping addition", "checked_add"),
            ("rotate a slice left", "rotate_left"),
            ("fold iterator values", "Iterator::fold"),
            ("macro_rules!", "macro_rules"),
            ("no_std", "no_std"),
        ] {
            let hit = direct_task_hit(query).unwrap_or_else(|| panic!("{query}"));
            assert!(
                hit.title.contains(expected) || hit.url.contains(expected),
                "{query}"
            );
        }
        assert!(direct_task_hit("remove prefix from a URL").is_none());
    }

    #[test]
    fn builds_struct_and_module_urls() {
        let urls = candidate_urls("https://doc.rust-lang.org", "std::vec::Vec");
        assert!(urls
            .iter()
            .any(|(_, url)| url == "https://doc.rust-lang.org/std/vec/struct.Vec.html"));
        assert!(urls
            .iter()
            .any(|(_, url)| url == "https://doc.rust-lang.org/std/vec/Vec/index.html"));
    }

    #[test]
    fn module_page_uses_index() {
        let urls = candidate_urls("https://doc.rust-lang.org", "std::vec");
        assert!(urls
            .iter()
            .any(|(_, url)| url == "https://doc.rust-lang.org/std/vec/index.html"));
    }

    #[test]
    fn prefixes_std_when_the_crate_is_missing() {
        assert_eq!(normalize_std_path("vec::Vec"), "std::vec::Vec");
        assert_eq!(normalize_std_path("core::mem::drop"), "core::mem::drop");
    }

    #[test]
    fn finds_method_fragments() {
        let html = r#"<h4 id="method.push">push</h4>"#;
        assert_eq!(find_fragment(html, "push").as_deref(), Some("method.push"));
    }

    #[test]
    fn expands_common_type_method_paths() {
        assert_eq!(normalize_std_path("Vec::push"), "std::vec::Vec::push");
        assert_eq!(
            normalize_std_path("HashMap::entry"),
            "std::collections::HashMap::entry"
        );
        assert_eq!(
            normalize_std_path("Option<T>::and_then"),
            "std::option::Option::and_then"
        );
        assert_eq!(normalize_std_path("f64::total_cmp"), "std::f64::total_cmp");
    }

    #[test]
    fn primitive_pages_rank_before_same_named_modules() {
        let urls = candidate_urls("https://doc.rust-lang.org", "std::str");
        let primitive = urls
            .iter()
            .find(|(_, url)| url.ends_with("/primitive.str.html"))
            .unwrap();
        let module = urls
            .iter()
            .find(|(_, url)| url.ends_with("/str/index.html"))
            .unwrap();
        assert!(primitive.0 < module.0);
    }

    #[test]
    fn routes_language_and_tooling_topics_to_their_own_sources() {
        assert!(guide_hit("borrow checker").unwrap().url.contains("/book/"));
        assert!(guide_hit("let else").unwrap().url.contains("/reference/"));
        assert!(guide_hit("Cargo features").unwrap().url.contains("/cargo/"));
        assert!(guide_hit("intra-doc links")
            .unwrap()
            .url
            .contains("/rustdoc/"));
        assert!(guide_hit("Rust 2024 edition")
            .unwrap()
            .url
            .contains("edition-guide"));
    }

    #[test]
    fn maps_task_phrases_to_canonical_apis() {
        assert_eq!(
            api_alias("read file to string"),
            Some("std::fs::read_to_string")
        );
        assert_eq!(
            api_alias("create directories recursively"),
            Some("std::fs::create_dir_all")
        );
        assert_eq!(api_alias("spawn a thread"), Some("std::thread::spawn"));
        assert_eq!(
            api_alias("parsing a socket address"),
            Some("std::net::SocketAddr")
        );
        assert_eq!(api_alias("poisoned mutexes"), Some("std::sync::Mutex"));
    }

    #[test]
    fn compositional_language_hits_include_distinguishing_syntax() {
        for (query, detail) in [
            ("mutable borrow", "&mut T"),
            ("struct destructuring pattern", "Point { x, y }"),
            ("raw pointers", "*const T"),
            ("foreign function interface", "extern \"C\""),
            ("cfg for target_os", "target_os"),
        ] {
            let hit = direct_task_hit(query).unwrap_or_else(|| panic!("{query}"));
            assert!(hit.snippet.contains(detail), "{query}: {}", hit.snippet);
        }
    }

    #[test]
    fn exact_api_context_surfaces_related_variants() {
        let mut hit = Hit::new("API", "https://example.invalid").snippet("generic description");
        enrich_api_hit("command arguments", &mut hit);
        assert!(hit.snippet.contains("args_os"));

        enrich_api_hit("parse socket address", &mut hit);
        assert!(hit.snippet.contains("FromStr"));
    }
    #[test]
    fn bounded_cache_evicts_the_oldest_entry() {
        let mut cache = BoundedCache::new(2);
        cache.insert("first", true);
        cache.insert("second", false);
        cache.insert("third", true);

        assert_eq!(cache.get(&"first"), None);
        assert_eq!(cache.get(&"second"), Some(false));
        assert_eq!(cache.get(&"third"), Some(true));
    }

    #[test]
    fn bounded_cache_replacing_a_key_refreshes_its_order() {
        let mut cache = BoundedCache::new(2);
        cache.insert("first", true);
        cache.insert("second", false);
        cache.insert("first", false);
        cache.insert("third", true);

        assert_eq!(cache.get(&"first"), Some(false));
        assert_eq!(cache.get(&"second"), None);
    }

    #[test]
    fn positive_and_negative_probe_entries_have_bounded_lifetimes() {
        let checked_at = Instant::now();
        let positive = ProbeCacheEntry {
            live: true,
            checked_at,
        };
        let negative = ProbeCacheEntry {
            live: false,
            checked_at,
        };

        assert_eq!(fresh_probe(positive, checked_at), Some(true));
        assert_eq!(fresh_probe(negative, checked_at), Some(false));
        assert_eq!(
            fresh_probe(
                negative,
                checked_at + NEGATIVE_PROBE_TTL + Duration::from_secs(1)
            ),
            None
        );
        assert_eq!(
            fresh_probe(
                positive,
                checked_at + POSITIVE_PROBE_TTL + Duration::from_secs(1)
            ),
            None
        );
    }

    #[test]
    fn selection_waits_for_better_ranked_probe() {
        let ranked = vec![(0, "best".to_string()), (1, "next".to_string())];
        let pending = vec![None, Some(true)];
        assert!(matches!(
            select_ranked(&ranked, &pending),
            ProbeSelection::Pending
        ));

        let resolved = vec![Some(false), Some(true)];
        assert!(matches!(
            select_ranked(&ranked, &resolved),
            ProbeSelection::Found("next")
        ));
    }

    #[test]
    fn selection_returns_as_soon_as_best_candidate_is_live() {
        let ranked = vec![(0, "best".to_string()), (1, "next".to_string())];
        let results = vec![Some(true), None];
        assert!(matches!(
            select_ranked(&ranked, &results),
            ProbeSelection::Found("best")
        ));
    }
}
