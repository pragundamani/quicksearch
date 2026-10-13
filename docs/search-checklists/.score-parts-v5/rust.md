# Rust candidate-v5 quicksearch scores

## Evaluation

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **30 seconds**.
- Rubric: the 0–5 practical developer rubric in `language_scores.md`.

## Summary

- **Average:** 3.66/5.
- **Median:** 3/5.
- **Distribution:** 0: 0, 1: 8, 2: 13, 3: 30, 4: 3, 5: 46.
- **Pass rate (score >=3):** 79/100 (79%).
- **Strong pass rate (score >=4):** 49/100 (49%).
- **Command errors:** 0/100. **Timeouts:** 0/100.

## Comparison

- No Rust per-command report exists in `.score-parts-v3`, so an exact v3 row-by-row comparison is unavailable.
- The documented validated Rust candidate checkpoint in `language_scores.md` (also recorded in `.score-parts-after/rust.md`) scored 3.52 average, 3 median, distribution 0: 0, 1: 14, 2: 12, 3: 28, 4: 0, 5: 46, 74% at >=3, and 46% at >=4, with 0 errors and 0 timeouts. Candidate-v5 improves the average by **0.14**, >=3 by **5 percentage points**, and >=4 by **3 points**. The median and error/timeout counts are unchanged.
- Compared with that checkpoint, web-backed results materially improve `mutexes`, `inclusive ranges`, and `derive PartialEq and Ord`; `AtomicUsize ordering` also becomes useful at #2. `macro_rules!` regresses to a generic search result, and `no_std` no longer gets the earlier attributes-page match.

## Results

| # | Command | Score | Observed top result or error | User assessment |
|---:|---|---:|---|---|
| 1 | `qs rust ownership` | 5 | #1 Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | The expected target ranks first and is immediately useful. |
| 2 | `qs rust borrow checker` | 5 | #1 References and borrowing — Rust checks shared and mutable borrows so references remain valid. | The expected target ranks first and is immediately useful. |
| 3 | `qs rust lifetimes` | 2 | #1 References and borrowing — Rust checks shared and mutable borrows so references remain valid. | The results are only partially relevant and do not surface the requested target clearly. |
| 4 | `qs rust mutable references` | 3 | #1 References and borrowing — Rust checks shared and mutable borrows so references remain valid. | A relevant result exists, but ranking or detail is weak. |
| 5 | `qs rust move closures` | 2 | #1 Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | The results are only partially relevant and do not surface the requested target clearly. |
| 6 | `qs rust pattern matching` | 5 | #1 Patterns — Patterns destructure values in let bindings, if let, while let, and match. | The expected target ranks first and is immediately useful. |
| 7 | `qs rust struct destructuring` | 3 | #1 Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result exists, but ranking or detail is weak. |
| 8 | `qs rust if let` | 3 | #1 Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result exists, but ranking or detail is weak. |
| 9 | `qs rust let else` | 3 | #1 Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result exists, but ranking or detail is weak. |
| 10 | `qs rust while let` | 3 | #1 Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result exists, but ranking or detail is weak. |
| 11 | `qs rust traits` | 5 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The expected target ranks first and is immediately useful. |
| 12 | `qs rust dyn Trait` | 2 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The results are only partially relevant and do not surface the requested target clearly. |
| 13 | `qs rust associated types` | 5 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The expected target ranks first and is immediately useful. |
| 14 | `qs rust generic functions` | 2 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The results are only partially relevant and do not surface the requested target clearly. |
| 15 | `qs rust where clauses` | 2 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The results are only partially relevant and do not surface the requested target clearly. |
| 16 | `qs rust const generics` | 2 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The results are only partially relevant and do not surface the requested target clearly. |
| 17 | `qs rust modules` | 5 | #1 Modules and visibility — Modules organize items; use imports names and visibility controls access. | The expected target ranks first and is immediately useful. |
| 18 | `qs rust pub(crate)` | 3 | #1 Modules and visibility — Modules organize items; use imports names and visibility controls access. | A relevant result exists, but ranking or detail is weak. |
| 19 | `qs rust use declarations` | 5 | #1 Modules and visibility — Modules organize items; use imports names and visibility controls access. | The expected target ranks first and is immediately useful. |
| 20 | `qs rust external crates` | 2 | #1 Modules and visibility — Modules organize items; use imports names and visibility controls access. | The results are only partially relevant and do not surface the requested target clearly. |
| 21 | `qs rust attributes` | 5 | #1 Rust attributes — Attributes control conditional compilation, derives, crate behavior, and tools. | The expected target ranks first and is immediately useful. |
| 22 | `qs rust derive macros` | 2 | #1 Rust macros — Rust supports declarative macro_rules! macros and procedural macros. | The results are only partially relevant and do not surface the requested target clearly. |
| 23 | `qs rust macro_rules!` | 1 | #1 Search Rust standard library for macro_rules — generic std/core/alloc search link only. | The output is misleading or only barely useful, usually ending in a generic search link. |
| 24 | `qs rust procedural macros` | 5 | #1 Rust macros — Rust supports declarative macro_rules! macros and procedural macros. | The expected target ranks first and is immediately useful. |
| 25 | `qs rust async fn` | 5 | #1 Async and await expressions — Async functions return futures; .await polls a future to completion. | The expected target ranks first and is immediately useful. |
| 26 | `qs rust .await` | 5 | #1 Async and await expressions — Async functions return futures; .await polls a future to completion. | The expected target ranks first and is immediately useful. |
| 27 | `qs rust std::pin::Pin` | 5 | #1 Pin in std::pin - Rust — A pointer which pins its pointee in place. | The expected target ranks first and is immediately useful. |
| 28 | `qs rust Send and Sync` | 2 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The results are only partially relevant and do not surface the requested target clearly. |
| 29 | `qs rust unsafe Rust` | 5 | #1 Unsafe Rust — Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations. | The expected target ranks first and is immediately useful. |
| 30 | `qs rust raw pointers` | 5 | #1 Unsafe Rust — Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations. | The expected target ranks first and is immediately useful. |
| 31 | `qs rust foreign function interface` | 5 | #1 Unsafe Rust — Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations. | The expected target ranks first and is immediately useful. |
| 32 | `qs rust recoverable errors` | 5 | #1 Error handling — Result represents recoverable errors; the ? operator propagates them. | The expected target ranks first and is immediately useful. |
| 33 | `qs rust question mark operator` | 5 | #1 Error handling — Result represents recoverable errors; the ? operator propagates them. | The expected target ranks first and is immediately useful. |
| 34 | `qs rust std::panic::set_hook` | 5 | #1 set_hook in std::panic - Rust — Registers a custom panic hook, replacing the previously registered hook. | The expected target ranks first and is immediately useful. |
| 35 | `qs rust Result<T, E>` | 3 | #1 Using Result<T, E> in Tests; #3 Recoverable Errors with Result. | Useful `Result` material exists, but the enum reference is not ranked directly and detail is weak. |
| 36 | `qs rust Option::and_then` | 3 | #1 Option::and_then — The `Option` type. See the module level documentation for more. | A relevant result exists, but ranking or detail is weak. |
| 37 | `qs rust Vec::push` | 3 | #1 Vec::push — A contiguous growable array type, written as `Vec<T>`, short for ‘vector’. | A relevant result exists, but ranking or detail is weak. |
| 38 | `qs rust VecDeque` | 5 | #1 VecDeque in std::collections - Rust — A double-ended queue implemented with a growable ring buffer. | The expected target ranks first and is immediately useful. |
| 39 | `qs rust HashMap::entry` | 3 | #1 HashMap::entry — A hash map implemented with quadratic probing and SIMD lookup. | A relevant result exists, but ranking or detail is weak. |
| 40 | `qs rust BTreeMap` | 5 | #1 BTreeMap in std::collections - Rust — An ordered map based on a B-Tree. | The expected target ranks first and is immediately useful. |
| 41 | `qs rust hash sets` | 5 | #1 HashSet in std::collections - Rust — A hash set implemented as a `HashMap` where the value is `()`. | The expected target ranks first and is immediately useful. |
| 42 | `qs rust binary heaps` | 5 | #1 BinaryHeap in std::collections - Rust — A priority queue implemented with a binary heap. | The expected target ranks first and is immediately useful. |
| 43 | `qs rust String::from_utf8` | 3 | #1 String::from_utf8 — A UTF-8–encoded, growable string. | A relevant result exists, but ranking or detail is weak. |
| 44 | `qs rust str::split_whitespace` | 3 | #1 str::split_whitespace — String slices. | A relevant result exists, but ranking or detail is weak. |
| 45 | `qs rust format!` | 1 | #1 Input format — [input] [input .syntax] Lexer CHAR → [U+0000-U+D7FF U+E000-U+10FFFF] // a Unicode scalar value ASCII → [U+0000-U+007F] NUL → U+0000 EOF → ! CHAR // End of file or input Show Railroad CHAR U+0000-U+D7FF U+E000-U+10FFFF ASCII U+0000-U+007F NU | The output is misleading or only barely useful, usually ending in a generic search link. |
| 46 | `qs rust print a line` | 5 | #1 println in std - Rust — Prints to the standard output, with a newline. | The expected target ranks first and is immediately useful. |
| 47 | `qs rust Iterator::map` | 3 | #1 Iterator::map — A trait for dealing with iterators. | A relevant result exists, but ranking or detail is weak. |
| 48 | `qs rust collect iterator into vector` | 2 | #1 Iterator loops; iterator material appears below, but no focused `collect::<Vec<_>>()` result. | The results are only partially relevant and do not surface the requested target clearly. |
| 49 | `qs rust fold values` | 1 | #1 Copy values — Values that implement Copy that are moved into the closure are captured with the ImmBorrow mode. #![allow(unused)] fn main() { let x = [0; 1024]; let c = \|\| { let y = x; // x captured by ImmBorrow }; } [type .closure .async .input] | The output is misleading or only barely useful, usually ending in a generic search link. |
| 50 | `qs rust IntoIterator` | 5 | #1 IntoIterator in std::iter - Rust — Conversion into an `Iterator`. | The expected target ranks first and is immediately useful. |
| 51 | `qs rust ExactSizeIterator` | 5 | #1 ExactSizeIterator in std::iter - Rust — An iterator that knows its exact length. | The expected target ranks first and is immediately useful. |
| 52 | `qs rust From and Into` | 2 | #1 Traits and generics — Trait items, associated types, bounds, and implementations. | The results are only partially relevant and do not surface the requested target clearly. |
| 53 | `qs rust TryFrom` | 5 | #1 TryFrom in std::convert - Rust — Simple and safe type conversions that may fail in a controlled way under some circumstances. It is the reciprocal of `TryInto`. | The expected target ranks first and is immediately useful. |
| 54 | `qs rust AsRef<Path>` | 3 | #1 AsRef in std::convert - Rust — Used to do a cheap reference-to-reference conversion. | A relevant result exists, but ranking or detail is weak. |
| 55 | `qs rust Cow<'a, str>` | 1 | #1 str Layout — String slices are a UTF-8 representation of characters that have the same layout as slices of type [u8]. A reference &str has the same layout as a reference &[u8]. [layout .tuple] | The output is misleading or only barely useful, usually ending in a generic search link. |
| 56 | `qs rust Box<T>` | 5 | #1 Box in std::boxed - Rust — A pointer type that uniquely owns a heap allocation of type `T`. | The expected target ranks first and is immediately useful. |
| 57 | `qs rust reference counted pointer` | 3 | #1 Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | A relevant result exists, but ranking or detail is weak. |
| 58 | `qs rust atomic reference counting` | 3 | #1 Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | A relevant result exists, but ranking or detail is weak. |
| 59 | `qs rust RefCell::borrow_mut` | 3 | #1 RefCell::borrow_mut — A mutable memory location with dynamically checked borrow rules | A relevant result exists, but ranking or detail is weak. |
| 60 | `qs rust mutexes` | 5 | #1 Controlling Access with Mutexes — practical locking guidance; #2 Shared-State Concurrency. | The expected target ranks first and is immediately useful. |
| 61 | `qs rust RwLock` | 5 | #1 RwLock in std::sync - Rust — A reader-writer lock | The expected target ranks first and is immediately useful. |
| 62 | `qs rust OnceLock::get_or_init` | 3 | #1 OnceLock::get_or_init — A synchronization primitive which can nominally be written to only once. | A relevant result exists, but ranking or detail is weak. |
| 63 | `qs rust spawn a thread` | 5 | #1 spawn in std::thread - Rust — Spawns a new thread, returning a `JoinHandle` for it. | The expected target ranks first and is immediately useful. |
| 64 | `qs rust mpsc channels` | 5 | #1 std::sync::mpsc - Rust — Multi-producer, single-consumer FIFO queue communication primitives. | The expected target ranks first and is immediately useful. |
| 65 | `qs rust AtomicUsize ordering` | 4 | #1 ordering-comparison page; #2 Statics & generics includes `AtomicUsize` and `Ordering` code. | The requested atomic type and ordering use appear at #2 with useful code context. |
| 66 | `qs rust read file to string` | 5 | #1 read_to_string in std::fs - Rust — Reads the entire contents of a file into a string. | The expected target ranks first and is immediately useful. |
| 67 | `qs rust write bytes to a file` | 5 | #1 write in std::fs - Rust — Writes a slice as the entire contents of a file. | The expected target ranks first and is immediately useful. |
| 68 | `qs rust canonicalize a path` | 5 | #1 canonicalize in std::fs - Rust — Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved. | The expected target ranks first and is immediately useful. |
| 69 | `qs rust create directories recursively` | 5 | #1 create_dir_all in std::fs - Rust — Recursively create a directory and all of its parent components if they are missing. | The expected target ranks first and is immediately useful. |
| 70 | `qs rust run command and capture output` | 3 | #1 Command::output — A process builder, providing fine-grained control over how a new process should be spawned. | A relevant result exists, but ranking or detail is weak. |
| 71 | `qs rust environment variables` | 5 | #1 var in std::env - Rust — Fetches the environment variable `key` from the current process. | The expected target ranks first and is immediately useful. |
| 72 | `qs rust command line arguments` | 5 | #1 args in std::env - Rust — Returns the arguments that this program was started with (normally passed via the command line). | The expected target ranks first and is immediately useful. |
| 73 | `qs rust TcpListener::bind` | 3 | #1 TcpListener::bind — A TCP socket server, listening for connections. | A relevant result exists, but ranking or detail is weak. |
| 74 | `qs rust parse socket address` | 2 | #1 IpAddr parsing example; no focused `SocketAddr` result. | The results are only partially relevant and do not surface the requested target clearly. |
| 75 | `qs rust Duration::from_secs` | 3 | #1 Duration::from_secs — A `Duration` type to represent a span of time, typically used for system timeouts. | A relevant result exists, but ranking or detail is weak. |
| 76 | `qs rust measure elapsed time` | 3 | #1 Instant::elapsed — A measurement of a monotonically nondecreasing clock. Opaque and useful only with `Duration`. | A relevant result exists, but ranking or detail is weak. |
| 77 | `qs rust unix timestamp` | 1 | #1 unix/windows conditional-compilation page; no timestamp result. | The output is misleading or only barely useful, usually ending in a generic search link. |
| 78 | `qs rust sort vectors` | 3 | #1 slice::sort — A dynamically-sized view into a contiguous sequence, `[T]`. | A relevant result exists, but ranking or detail is weak. |
| 79 | `qs rust binary_search` | 1 | #1 generic Rust standard-library search for `binary_search`. | The output is misleading or only barely useful, usually ending in a generic search link. |
| 80 | `qs rust parse integer` | 3 | #1 str::parse — String slices. | A relevant result exists, but ranking or detail is weak. |
| 81 | `qs rust f64::total_cmp` | 3 | #1 f64::total_cmp — A 64-bit floating-point type (specifically, the “binary64” type defined in IEEE 754-2008). | A relevant result exists, but ranking or detail is weak. |
| 82 | `qs rust inclusive ranges` | 4 | #1 Matching Ranges of Values with `..=` — directly explains inclusive-range syntax. | The expected target is in the top three with useful context. |
| 83 | `qs rust chunks_exact_mut` | 1 | #1 generic Rust standard-library search for `chunks_exact_mut`. | The output is misleading or only barely useful, usually ending in a generic search link. |
| 84 | `qs rust fixed size arrays` | 3 | #1 unrelated fixed-size hash result; arrays appear at #4 and #5. | Array material exists only below the top three, so ranking and detail are weak. |
| 85 | `qs rust tuple structs` | 3 | #1 Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | A relevant result exists, but ranking or detail is weak. |
| 86 | `qs rust named field structs` | 3 | #1 Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | A relevant result exists, but ranking or detail is weak. |
| 87 | `qs rust enums with data` | 3 | #1 Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | A relevant result exists, but ranking or detail is weak. |
| 88 | `qs rust impl blocks` | 5 | #1 Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | The expected target ranks first and is immediately useful. |
| 89 | `qs rust Drop::drop` | 5 | #1 Drop::drop — Custom code within the destructor. | The expected target ranks first and is immediately useful. |
| 90 | `qs rust Default::default` | 5 | #1 Default::default — A trait for giving a type a useful default value. | The expected target ranks first and is immediately useful. |
| 91 | `qs rust Clone vs Copy` | 2 | #1 Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | The results are only partially relevant and do not surface the requested target clearly. |
| 92 | `qs rust derive PartialEq and Ord` | 4 | #1 PartialEq/Eq derivation; #2 PartialOrd/Ord derivation. | The expected target is in the top three with useful context. |
| 93 | `qs rust unit tests` | 5 | #1 Writing automated tests — Use #[test] functions with Rust's test harness. | The expected target ranks first and is immediately useful. |
| 94 | `qs rust documentation tests` | 5 | #1 Rustdoc documentation tests — Rustdoc compiles and runs Rust code blocks as documentation tests. | The expected target ranks first and is immediately useful. |
| 95 | `qs rust Cargo features` | 5 | #1 Cargo features and optional dependencies — Cargo feature flags enable conditional compilation and optional dependencies. | The expected target ranks first and is immediately useful. |
| 96 | `qs rust cfg target_os` | 3 | #1 Rust attributes — Attributes control conditional compilation, derives, crate behavior, and tools. | A relevant result exists, but ranking or detail is weak. |
| 97 | `qs rust no_std` | 1 | #1 generic Rust standard-library search for `no_std`. | The output is misleading or only barely useful, usually ending in a generic search link. |
| 98 | `qs rust intra-doc links` | 5 | #1 Rustdoc intra-doc links — Link to Rust items by path and let rustdoc check the link. | The expected target ranks first and is immediately useful. |
| 99 | `qs rust Rust 2024 edition` | 5 | #1 Rust 2024 Edition Guide — Migration notes and changes for the Rust 2024 edition. | The expected target ranks first and is immediately useful. |
| 100 | `qs rust raw identifiers` | 5 | #1 Identifiers — Raw identifiers such as r#type use a keyword as an identifier. | The expected target ranks first and is immediately useful. |
