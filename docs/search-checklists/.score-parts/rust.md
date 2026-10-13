# Rust quicksearch score report

## Summary

- Queries executed: **100/100**
- Average score: **1.78/5**
- Median score: **1/5**
- Pass rate, score >=3: **28/100 (28%)**
- Strong pass rate, score >=4: **2/100 (2%)**
- Command errors/timeouts: **0**
- Distribution: **0: 0, 1: 54, 2: 18, 3: 26, 4: 0, 5: 2**

Each checklist command was run through the installed `qs` executable. Each process had a 15-second timeout. Scoring uses the result list printed by `qs`; it does not assume that a generic linked search page will produce the expected item after another interaction.

## All query results

| # | Command | Score | Observed top result/error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs rust ownership` | 1 | std search: ownership | It only echoes the query into std search; it does not surface Ownership, moves, and scope. or useful context. |
| 2 | `qs rust borrow checker` | 1 | std search: borrow checker | It only echoes the query into std search; it does not surface Borrowing rules and the borrow checker. or useful context. |
| 3 | `qs rust lifetimes` | 1 | std search: lifetimes | It only echoes the query into std search; it does not surface Lifetime annotations and lifetime elision. or useful context. |
| 4 | `qs rust mutable references` | 1 | std search: mutable references | It only echoes the query into std search; it does not surface Mutable borrows written as &mut T. or useful context. |
| 5 | `qs rust move closures` | 1 | std search: move closures | It only echoes the query into std search; it does not surface move closures and captured values. or useful context. |
| 6 | `qs rust pattern matching` | 1 | std search: pattern matching | It only echoes the query into std search; it does not surface Patterns and match expressions. or useful context. |
| 7 | `qs rust struct destructuring` | 1 | std search: struct destructuring | It only echoes the query into std search; it does not surface Destructuring patterns for structs. or useful context. |
| 8 | `qs rust if let` | 1 | std search: if let | It only echoes the query into std search; it does not surface if let conditional pattern matching. or useful context. |
| 9 | `qs rust let else` | 1 | std search: let else | It only echoes the query into std search; it does not surface let...else refutable pattern binding. or useful context. |
| 10 | `qs rust while let` | 1 | std search: while let | It only echoes the query into std search; it does not surface while let conditional loops. or useful context. |
| 11 | `qs rust traits` | 1 | std search: traits | It only echoes the query into std search; it does not surface Trait definitions and implementations. or useful context. |
| 12 | `qs rust dyn Trait` | 1 | std search: dyn Trait | It only echoes the query into std search; it does not surface Trait objects written as dyn Trait. or useful context. |
| 13 | `qs rust associated types` | 1 | std search: associated types | It only echoes the query into std search; it does not surface Trait associated types. or useful context. |
| 14 | `qs rust generic functions` | 1 | std search: generic functions | It only echoes the query into std search; it does not surface Generic type parameters on functions. or useful context. |
| 15 | `qs rust where clauses` | 1 | std search: where clauses | It only echoes the query into std search; it does not surface where clauses for generic bounds. or useful context. |
| 16 | `qs rust const generics` | 1 | std search: const generics | It only echoes the query into std search; it does not surface Const generic parameters. or useful context. |
| 17 | `qs rust modules` | 1 | std search: modules | It only echoes the query into std search; it does not surface The module system and mod items. or useful context. |
| 18 | `qs rust pub(crate)` | 1 | std search: pub(crate) | It only echoes the query into std search; it does not surface Crate-scoped visibility with pub(crate). or useful context. |
| 19 | `qs rust use declarations` | 1 | std search: use declarations | It only echoes the query into std search; it does not surface use declarations and imported names. or useful context. |
| 20 | `qs rust external crates` | 1 | std search: external crates | It only echoes the query into std search; it does not surface External crates and the extern prelude. or useful context. |
| 21 | `qs rust attributes` | 1 | std search: attributes | It only echoes the query into std search; it does not surface Outer and inner attributes. or useful context. |
| 22 | `qs rust derive macros` | 1 | std search: derive macros | It only echoes the query into std search; it does not surface The derive attribute and derivable traits. or useful context. |
| 23 | `qs rust macro_rules!` | 1 | std search: macro_rules! | It only echoes the query into std search; it does not surface Declarative macros defined with macro_rules!. or useful context. |
| 24 | `qs rust procedural macros` | 1 | std search: procedural macros | It only echoes the query into std search; it does not surface Function-like, derive, and attribute procedural macros. or useful context. |
| 25 | `qs rust async fn` | 1 | std search: async fn | It only echoes the query into std search; it does not surface Asynchronous functions and returned futures. or useful context. |
| 26 | `qs rust .await` | 1 | std search: .await | It only echoes the query into std search; it does not surface The postfix .await expression. or useful context. |
| 27 | `qs rust std::pin::Pin` | 5 | Pin in std::pin - Rust | Exact Pin page is first, with a clear one-line guarantee. Immediately useful. |
| 28 | `qs rust Send and Sync` | 1 | std search: Send and Sync | It only echoes the query into std search; it does not surface The Send and Sync marker traits. or useful context. |
| 29 | `qs rust unsafe Rust` | 1 | std search: unsafe Rust | It only echoes the query into std search; it does not surface Unsafe operations and unsafe blocks. or useful context. |
| 30 | `qs rust raw pointers` | 1 | std search: raw pointers | It only echoes the query into std search; it does not surface Raw pointer types *const T and *mut T. or useful context. |
| 31 | `qs rust foreign function interface` | 1 | std search: foreign function interface | It only echoes the query into std search; it does not surface FFI with extern blocks and ABI strings. or useful context. |
| 32 | `qs rust recoverable errors` | 1 | std search: recoverable errors | It only echoes the query into std search; it does not surface Recoverable error handling with Result. or useful context. |
| 33 | `qs rust question mark operator` | 1 | std search: question mark operator | It only echoes the query into std search; it does not surface The ? error-propagation operator. or useful context. |
| 34 | `qs rust std::panic::set_hook` | 5 | set_hook in std::panic - Rust | Exact set_hook function page is first, with a clear action summary. Immediately useful. |
| 35 | `qs rust Result<T, E>` | 3 | std search: Result<T, E> | The std search preserves enough exact API terms to find The Result<T, E> enum., but the top row is generic and has no signature or example. |
| 36 | `qs rust Option::and_then` | 3 | Standard library search | The std search preserves enough exact API terms to find Option::and_then combinator., but the top row is generic and has no signature or example. |
| 37 | `qs rust Vec::push` | 3 | Standard library search | The std search preserves enough exact API terms to find Vec::push., but the top row is generic and has no signature or example. |
| 38 | `qs rust VecDeque` | 3 | std search: VecDeque | The std search preserves enough exact API terms to find std::collections::VecDeque., but the top row is generic and has no signature or example. |
| 39 | `qs rust HashMap::entry` | 3 | Standard library search | The std search preserves enough exact API terms to find HashMap::entry and the entry API., but the top row is generic and has no signature or example. |
| 40 | `qs rust BTreeMap` | 3 | std search: BTreeMap | The std search preserves enough exact API terms to find std::collections::BTreeMap., but the top row is generic and has no signature or example. |
| 41 | `qs rust hash sets` | 1 | std search: hash sets | It only echoes the query into std search; it does not surface std::collections::HashSet. or useful context. |
| 42 | `qs rust binary heaps` | 1 | std search: binary heaps | It only echoes the query into std search; it does not surface std::collections::BinaryHeap. or useful context. |
| 43 | `qs rust String::from_utf8` | 3 | Standard library search | The std search preserves enough exact API terms to find String::from_utf8 and FromUtf8Error., but the top row is generic and has no signature or example. |
| 44 | `qs rust str::split_whitespace` | 3 | std::str - Rust | The str module page is relevant, but split_whitespace is not the top page and no signature is shown. |
| 45 | `qs rust format!` | 3 | std search: format! | The std search preserves enough exact API terms to find The format! macro., but the top row is generic and has no signature or example. |
| 46 | `qs rust print a line` | 1 | std search: print a line | It only echoes the query into std search; it does not surface The println! macro. or useful context. |
| 47 | `qs rust Iterator::map` | 3 | Standard library search | The std search preserves enough exact API terms to find Iterator::map., but the top row is generic and has no signature or example. |
| 48 | `qs rust collect iterator into vector` | 1 | std search: collect iterator into vector | It only echoes the query into std search; it does not surface Iterator::collect with Vec type inference. or useful context. |
| 49 | `qs rust fold values` | 2 | std search: fold values | The std search has related keywords, but it does not identify Iterator::fold.; ranking and task guidance are weak. |
| 50 | `qs rust IntoIterator` | 3 | std search: IntoIterator | The std search preserves enough exact API terms to find The IntoIterator trait and into_iter., but the top row is generic and has no signature or example. |
| 51 | `qs rust ExactSizeIterator` | 3 | std search: ExactSizeIterator | The std search preserves enough exact API terms to find The ExactSizeIterator trait., but the top row is generic and has no signature or example. |
| 52 | `qs rust From and Into` | 2 | std search: From and Into | The std search has related keywords, but it does not identify The From and Into conversion traits.; ranking and task guidance are weak. |
| 53 | `qs rust TryFrom` | 3 | std search: TryFrom | The std search preserves enough exact API terms to find The TryFrom fallible conversion trait., but the top row is generic and has no signature or example. |
| 54 | `qs rust AsRef<Path>` | 3 | std search: AsRef<Path> | The std search preserves enough exact API terms to find AsRef<Path> bounds and borrowed conversions., but the top row is generic and has no signature or example. |
| 55 | `qs rust Cow<'a, str>` | 3 | std search: Cow<'a, str> | The std search preserves enough exact API terms to find std::borrow::Cow clone-on-write type., but the top row is generic and has no signature or example. |
| 56 | `qs rust Box<T>` | 3 | std search: Box<T> | The std search preserves enough exact API terms to find std::boxed::Box<T>., but the top row is generic and has no signature or example. |
| 57 | `qs rust reference counted pointer` | 2 | std search: reference counted pointer | The std search has related keywords, but it does not identify std::rc::Rc.; ranking and task guidance are weak. |
| 58 | `qs rust atomic reference counting` | 2 | std search: atomic reference counting | The std search has related keywords, but it does not identify std::sync::Arc.; ranking and task guidance are weak. |
| 59 | `qs rust RefCell::borrow_mut` | 3 | Standard library search | The std search preserves enough exact API terms to find RefCell::borrow_mut and runtime borrow checking., but the top row is generic and has no signature or example. |
| 60 | `qs rust mutexes` | 2 | std search: mutexes | The std search has related keywords, but it does not identify std::sync::Mutex.; ranking and task guidance are weak. |
| 61 | `qs rust RwLock` | 3 | std search: RwLock | The std search preserves enough exact API terms to find std::sync::RwLock., but the top row is generic and has no signature or example. |
| 62 | `qs rust OnceLock::get_or_init` | 3 | Standard library search | The std search preserves enough exact API terms to find OnceLock::get_or_init., but the top row is generic and has no signature or example. |
| 63 | `qs rust spawn a thread` | 2 | std search: spawn a thread | The std search has related keywords, but it does not identify std::thread::spawn.; ranking and task guidance are weak. |
| 64 | `qs rust mpsc channels` | 2 | std search: mpsc channels | The std search has related keywords, but it does not identify std::sync::mpsc channels.; ranking and task guidance are weak. |
| 65 | `qs rust AtomicUsize ordering` | 2 | std search: AtomicUsize ordering | The std search has related keywords, but it does not identify AtomicUsize operations and Ordering.; ranking and task guidance are weak. |
| 66 | `qs rust read file to string` | 1 | std search: read file to string | It only echoes the query into std search; it does not surface std::fs::read_to_string. or useful context. |
| 67 | `qs rust write bytes to a file` | 1 | std search: write bytes to a file | It only echoes the query into std search; it does not surface std::fs::write. or useful context. |
| 68 | `qs rust canonicalize a path` | 2 | std search: canonicalize a path | The std search has related keywords, but it does not identify std::fs::canonicalize.; ranking and task guidance are weak. |
| 69 | `qs rust create directories recursively` | 1 | std search: create directories recursively | It only echoes the query into std search; it does not surface std::fs::create_dir_all. or useful context. |
| 70 | `qs rust run command and capture output` | 1 | std search: run command and capture output | It only echoes the query into std search; it does not surface std::process::Command::output. or useful context. |
| 71 | `qs rust environment variables` | 2 | std search: environment variables | The std search has related keywords, but it does not identify std::env::var and vars.; ranking and task guidance are weak. |
| 72 | `qs rust command line arguments` | 2 | std search: command line arguments | The std search has related keywords, but it does not identify std::env::args and args_os.; ranking and task guidance are weak. |
| 73 | `qs rust TcpListener::bind` | 3 | Standard library search | The std search preserves enough exact API terms to find std::net::TcpListener::bind., but the top row is generic and has no signature or example. |
| 74 | `qs rust parse socket address` | 2 | std search: parse socket address | The std search has related keywords, but it does not identify std::net::SocketAddr and FromStr.; ranking and task guidance are weak. |
| 75 | `qs rust Duration::from_secs` | 3 | Standard library search | The std search preserves enough exact API terms to find std::time::Duration::from_secs., but the top row is generic and has no signature or example. |
| 76 | `qs rust measure elapsed time` | 2 | std search: measure elapsed time | The std search has related keywords, but it does not identify std::time::Instant::elapsed.; ranking and task guidance are weak. |
| 77 | `qs rust unix timestamp` | 2 | std search: unix timestamp | The std search has related keywords, but it does not identify SystemTime relative to UNIX_EPOCH.; ranking and task guidance are weak. |
| 78 | `qs rust sort vectors` | 2 | std search: sort vectors | The std search has related keywords, but it does not identify Slice sorting methods such as sort and sort_by.; ranking and task guidance are weak. |
| 79 | `qs rust binary_search` | 3 | std search: binary_search | The std search preserves enough exact API terms to find Slice method binary_search., but the top row is generic and has no signature or example. |
| 80 | `qs rust parse integer` | 2 | std search: parse integer | The std search has related keywords, but it does not identify str::parse with integer types and ParseIntError.; ranking and task guidance are weak. |
| 81 | `qs rust f64::total_cmp` | 3 | std::f64 - Rust | The f64 module page is related, but total_cmp is not surfaced and no method signature appears. |
| 82 | `qs rust inclusive ranges` | 2 | std search: inclusive ranges | The std search has related keywords, but it does not identify RangeInclusive and ..= syntax.; ranking and task guidance are weak. |
| 83 | `qs rust chunks_exact_mut` | 3 | std search: chunks_exact_mut | The std search preserves enough exact API terms to find Slice method chunks_exact_mut., but the top row is generic and has no signature or example. |
| 84 | `qs rust fixed size arrays` | 2 | std search: fixed size arrays | The std search has related keywords, but it does not identify Array types [T; N] and array operations.; ranking and task guidance are weak. |
| 85 | `qs rust tuple structs` | 1 | std search: tuple structs | It only echoes the query into std search; it does not surface Tuple struct declarations and construction. or useful context. |
| 86 | `qs rust named field structs` | 1 | std search: named field structs | It only echoes the query into std search; it does not surface Structs with named fields. or useful context. |
| 87 | `qs rust enums with data` | 1 | std search: enums with data | It only echoes the query into std search; it does not surface Enum variants containing fields. or useful context. |
| 88 | `qs rust impl blocks` | 1 | std search: impl blocks | It only echoes the query into std search; it does not surface Inherent implementations using impl. or useful context. |
| 89 | `qs rust Drop::drop` | 3 | Standard library search | The std search preserves enough exact API terms to find The Drop trait and drop method., but the top row is generic and has no signature or example. |
| 90 | `qs rust Default::default` | 3 | Standard library search | The std search preserves enough exact API terms to find The Default trait and default method., but the top row is generic and has no signature or example. |
| 91 | `qs rust Clone vs Copy` | 1 | std search: Clone vs Copy | It only echoes the query into std search; it does not surface Semantics of the Clone and Copy traits. or useful context. |
| 92 | `qs rust derive PartialEq and Ord` | 1 | std search: derive PartialEq and Ord | It only echoes the query into std search; it does not surface Derived equality and ordering traits. or useful context. |
| 93 | `qs rust unit tests` | 1 | std search: unit tests | It only echoes the query into std search; it does not surface #[test] functions and the test harness. or useful context. |
| 94 | `qs rust documentation tests` | 1 | std search: documentation tests | It only echoes the query into std search; it does not surface Rustdoc code examples run as doctests. or useful context. |
| 95 | `qs rust Cargo features` | 1 | std search: Cargo features | It only echoes the query into std search; it does not surface Cargo feature flags and optional dependencies. or useful context. |
| 96 | `qs rust cfg target_os` | 1 | std search: cfg target_os | It only echoes the query into std search; it does not surface Conditional compilation with #[cfg(target_os = ...)]. or useful context. |
| 97 | `qs rust no_std` | 1 | std search: no_std | It only echoes the query into std search; it does not surface The #![no_std] attribute and core. or useful context. |
| 98 | `qs rust intra-doc links` | 1 | std search: intra-doc links | It only echoes the query into std search; it does not surface Rustdoc intra-doc link syntax. or useful context. |
| 99 | `qs rust Rust 2024 edition` | 1 | std search: Rust 2024 edition | It only echoes the query into std search; it does not surface The Rust 2024 Edition Guide and migration changes. or useful context. |
| 100 | `qs rust raw identifiers` | 1 | std search: raw identifiers | It only echoes the query into std search; it does not surface Raw identifiers such as r#type. or useful context. |

## Strongest patterns

- Fully qualified entries with explicit routing are excellent. `std::pin::Pin` returns **“Pin in std::pin - Rust”** first and describes the pinning guarantee. `std::panic::set_hook` returns the exact function and says it **“Registers a custom panic hook”**.
- Exact standard-library identifiers usually retain useful tokens. Queries such as `Vec::push`, `HashMap::entry`, `OnceLock::get_or_init`, and `Duration::from_secs` lead to a focused std search rather than losing the type or method name.
- Punctuation and casing are encoded safely. The run had no command errors for `pub(crate)`, `.await`, generic forms, lifetimes, macro exclamation marks, or snake_case names.

## Weakest patterns

- The dominant fallback is not a real ranked answer. In 54 cases the only top result was an echo such as **“std search: ownership”**, followed by **“Opens the Rust standard library search.”** This gives no ownership explanation, Book chapter, syntax, or example.
- Language-guide concepts are sent to the standard-library item search. `borrow checker`, `let else`, `procedural macros`, `unsafe Rust`, `tuple structs`, and `raw identifiers` need Reference or Book pages, not a std item-name search.
- Natural task phrases are not mapped to APIs. `print a line`, `read file to string`, `create directories recursively`, and `run command and capture output` do not surface `println!`, `read_to_string`, `create_dir_all`, or `Command::output`.
- Tooling and documentation topics use the wrong corpus. `Cargo features`, `intra-doc links`, and `Rust 2024 edition` all land in std search and show no relevant guide.
- Some identifier routing stops one level too early. `str::split_whitespace` ranks **“std::str - Rust”**, and `f64::total_cmp` ranks **“std::f64 - Rust”** rather than the requested method. Generic std-search rows also omit signatures and examples.

## Prioritized improvements

1. **Add a Rust intent router.** Send language syntax and semantics to the Rust Reference or Book, std APIs to rustdoc, Cargo topics to the Cargo Book, rustdoc topics to the rustdoc book, and edition queries to the Edition Guide.
2. **Resolve exact paths to exact rustdoc pages.** Support `Type::method`, primitive methods, traits, macros, and generic syntax. Preserve enough path context to return `str::split_whitespace` and `f64::total_cmp`, not their parent modules.
3. **Add task-to-API aliases.** Map common phrases such as “print a line,” “read file to string,” “create directories recursively,” and “spawn a thread” to the canonical macro, function, or method.
4. **Return useful result metadata.** Show the resolved signature, one-line documentation, stability/source context when useful, and a short example for task queries. Do not describe a search endpoint as if it were the answer.
5. **Add regression cases for compound and unusual syntax.** Cover `Send and Sync`, `From and Into`, `Clone vs Copy`, `Result<T, E>`, `Cow<'a, str>`, `.await`, `macro_rules!`, `cfg target_os`, and `no_std` with destination and rank assertions.
