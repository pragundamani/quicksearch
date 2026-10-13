# Rust candidate quicksearch evaluation

## Summary

- Candidate: `/tmp/qs-candidate/release/qs`.
- Commands run: **100/100**. Each command had a **20-second timeout**.
- Average: **3.52/5**.
- Median: **3/5**.
- Distribution: **0: 0, 1: 14, 2: 12, 3: 28, 4: 0, 5: 46**.
- Score >=3: **74/100 (74%)**.
- Score >=4: **46/100 (46%)**.
- Errors/timeouts: **0/0**.
- Baseline: average **1.78**, median **1**, score >=3 **28%**, score >=4 **2%**, errors/timeouts **0**.
- Change from baseline: average **+1.74**, median **+2**, score >=3 **+46 percentage points**, and score >=4 **+44 percentage points**. Errors/timeouts were unchanged.

Scores use the documented practical developer rubric: 5 means the expected result is first and immediately useful; 4 means it is in the top three with useful context; 3 means a relevant result exists but ranking or detail is weak; 2 means partial relevance; 1 means misleading or barely useful; 0 means an error or no relevant result.

## Results

| # | Command | Score | Observed top result/error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs rust ownership` | 5 | Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | The expected result ranks first with enough context to act. |
| 2 | `qs rust borrow checker` | 5 | References and borrowing — Rust checks shared and mutable borrows so references remain valid. | The expected result ranks first with enough context to act. |
| 3 | `qs rust lifetimes` | 2 | References and borrowing — Rust checks shared and mutable borrows so references remain valid. | The first result is related, but it does not surface the requested detail. |
| 4 | `qs rust mutable references` | 3 | References and borrowing — Rust checks shared and mutable borrows so references remain valid. | A relevant result ranks first, but the snippet omits key detail. |
| 5 | `qs rust move closures` | 2 | Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | The first result is related, but it does not surface the requested detail. |
| 6 | `qs rust pattern matching` | 5 | Patterns — Patterns destructure values in let bindings, if let, while let, and match. | The expected result ranks first with enough context to act. |
| 7 | `qs rust struct destructuring` | 3 | Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result ranks first, but the snippet omits key detail. |
| 8 | `qs rust if let` | 3 | Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result ranks first, but the snippet omits key detail. |
| 9 | `qs rust let else` | 3 | Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result ranks first, but the snippet omits key detail. |
| 10 | `qs rust while let` | 3 | Patterns — Patterns destructure values in let bindings, if let, while let, and match. | A relevant result ranks first, but the snippet omits key detail. |
| 11 | `qs rust traits` | 5 | Traits and generics — Trait items, associated types, bounds, and implementations. | The expected result ranks first with enough context to act. |
| 12 | `qs rust dyn Trait` | 2 | Traits and generics — Trait items, associated types, bounds, and implementations. | The first result is related, but it does not surface the requested detail. |
| 13 | `qs rust associated types` | 5 | Traits and generics — Trait items, associated types, bounds, and implementations. | The expected result ranks first with enough context to act. |
| 14 | `qs rust generic functions` | 2 | Traits and generics — Trait items, associated types, bounds, and implementations. | The first result is related, but it does not surface the requested detail. |
| 15 | `qs rust where clauses` | 2 | Traits and generics — Trait items, associated types, bounds, and implementations. | The first result is related, but it does not surface the requested detail. |
| 16 | `qs rust const generics` | 2 | Traits and generics — Trait items, associated types, bounds, and implementations. | The first result is related, but it does not surface the requested detail. |
| 17 | `qs rust modules` | 5 | Modules and visibility — Modules organize items; use imports names and visibility controls access. | The expected result ranks first with enough context to act. |
| 18 | `qs rust pub(crate)` | 3 | Modules and visibility — Modules organize items; use imports names and visibility controls access. | A relevant result ranks first, but the snippet omits key detail. |
| 19 | `qs rust use declarations` | 5 | Modules and visibility — Modules organize items; use imports names and visibility controls access. | The expected result ranks first with enough context to act. |
| 20 | `qs rust external crates` | 2 | Modules and visibility — Modules organize items; use imports names and visibility controls access. | The first result is related, but it does not surface the requested detail. |
| 21 | `qs rust attributes` | 5 | Rust attributes — Attributes control conditional compilation, derives, crate behavior, and tools. | The expected result ranks first with enough context to act. |
| 22 | `qs rust derive macros` | 2 | Rust macros — Rust supports declarative macro_rules! macros and procedural macros. | The first result is related, but it does not surface the requested detail. |
| 23 | `qs rust macro_rules!` | 5 | Rust macros — Rust supports declarative macro_rules! macros and procedural macros. | The expected result ranks first with enough context to act. |
| 24 | `qs rust procedural macros` | 5 | Rust macros — Rust supports declarative macro_rules! macros and procedural macros. | The expected result ranks first with enough context to act. |
| 25 | `qs rust async fn` | 5 | Async and await expressions — Async functions return futures; .await polls a future to completion. | The expected result ranks first with enough context to act. |
| 26 | `qs rust .await` | 5 | Async and await expressions — Async functions return futures; .await polls a future to completion. | The expected result ranks first with enough context to act. |
| 27 | `qs rust std::pin::Pin` | 5 | Pin in std::pin - Rust — A pointer which pins its pointee in place. | The expected result ranks first with enough context to act. |
| 28 | `qs rust Send and Sync` | 2 | Traits and generics — Trait items, associated types, bounds, and implementations. | The first result is related, but it does not surface the requested detail. |
| 29 | `qs rust unsafe Rust` | 5 | Unsafe Rust — Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations. | The expected result ranks first with enough context to act. |
| 30 | `qs rust raw pointers` | 5 | Unsafe Rust — Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations. | The expected result ranks first with enough context to act. |
| 31 | `qs rust foreign function interface` | 5 | Unsafe Rust — Unsafe blocks permit raw-pointer operations, FFI, and other unsafe operations. | The expected result ranks first with enough context to act. |
| 32 | `qs rust recoverable errors` | 5 | Error handling — Result represents recoverable errors; the ? operator propagates them. | The expected result ranks first with enough context to act. |
| 33 | `qs rust question mark operator` | 5 | Error handling — Result represents recoverable errors; the ? operator propagates them. | The expected result ranks first with enough context to act. |
| 34 | `qs rust std::panic::set_hook` | 5 | set_hook in std::panic - Rust — Registers a custom panic hook, replacing the previously registered hook. | The expected result ranks first with enough context to act. |
| 35 | `qs rust Result<T, E>` | 1 | Search Rust standard library for Result<T, E> — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 36 | `qs rust Option::and_then` | 3 | Option::and_then — The `Option` type. See the module level documentation for more. | A relevant result ranks first, but the snippet omits key detail. |
| 37 | `qs rust Vec::push` | 3 | Vec::push — A contiguous growable array type, written as `Vec<T>`, short for ‘vector’. | A relevant result ranks first, but the snippet omits key detail. |
| 38 | `qs rust VecDeque` | 5 | VecDeque in std::collections - Rust — A double-ended queue implemented with a growable ring buffer. | The expected result ranks first with enough context to act. |
| 39 | `qs rust HashMap::entry` | 3 | HashMap::entry — A hash map implemented with quadratic probing and SIMD lookup. | A relevant result ranks first, but the snippet omits key detail. |
| 40 | `qs rust BTreeMap` | 5 | BTreeMap in std::collections - Rust — An ordered map based on a B-Tree. | The expected result ranks first with enough context to act. |
| 41 | `qs rust hash sets` | 5 | HashSet in std::collections - Rust — A hash set implemented as a `HashMap` where the value is `()`. | The expected result ranks first with enough context to act. |
| 42 | `qs rust binary heaps` | 5 | BinaryHeap in std::collections - Rust — A priority queue implemented with a binary heap. | The expected result ranks first with enough context to act. |
| 43 | `qs rust String::from_utf8` | 3 | String::from_utf8 — A UTF-8–encoded, growable string. | A relevant result ranks first, but the snippet omits key detail. |
| 44 | `qs rust str::split_whitespace` | 3 | str::split_whitespace — String slices. | A relevant result ranks first, but the snippet omits key detail. |
| 45 | `qs rust format!` | 1 | Search Rust standard library for format — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 46 | `qs rust print a line` | 5 | println in std - Rust — Prints to the standard output, with a newline. | The expected result ranks first with enough context to act. |
| 47 | `qs rust Iterator::map` | 3 | Iterator::map — A trait for dealing with iterators. | A relevant result ranks first, but the snippet omits key detail. |
| 48 | `qs rust collect iterator into vector` | 1 | Search Rust standard library for collect iterator into vector — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 49 | `qs rust fold values` | 1 | Search Rust standard library for fold values — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 50 | `qs rust IntoIterator` | 5 | IntoIterator in std::iter - Rust — Conversion into an `Iterator`. | The expected result ranks first with enough context to act. |
| 51 | `qs rust ExactSizeIterator` | 5 | ExactSizeIterator in std::iter - Rust — An iterator that knows its exact length. | The expected result ranks first with enough context to act. |
| 52 | `qs rust From and Into` | 2 | Traits and generics — Trait items, associated types, bounds, and implementations. | The first result is related, but it does not surface the requested detail. |
| 53 | `qs rust TryFrom` | 5 | TryFrom in std::convert - Rust — Simple and safe type conversions that may fail in a controlled way under some circumstances. It is the reciprocal of `TryInto`. | The expected result ranks first with enough context to act. |
| 54 | `qs rust AsRef<Path>` | 3 | AsRef in std::convert - Rust — Used to do a cheap reference-to-reference conversion. | A relevant result ranks first, but the snippet omits key detail. |
| 55 | `qs rust Cow<'a, str>` | 1 | Search Rust standard library for Cow<'a, str> — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 56 | `qs rust Box<T>` | 5 | Box in std::boxed - Rust — A pointer type that uniquely owns a heap allocation of type `T`. | The expected result ranks first with enough context to act. |
| 57 | `qs rust reference counted pointer` | 3 | Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | A relevant result ranks first, but the snippet omits key detail. |
| 58 | `qs rust atomic reference counting` | 3 | Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | A relevant result ranks first, but the snippet omits key detail. |
| 59 | `qs rust RefCell::borrow_mut` | 3 | RefCell::borrow_mut — A mutable memory location with dynamically checked borrow rules | A relevant result ranks first, but the snippet omits key detail. |
| 60 | `qs rust mutexes` | 1 | Search Rust standard library for mutexes — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 61 | `qs rust RwLock` | 5 | RwLock in std::sync - Rust — A reader-writer lock | The expected result ranks first with enough context to act. |
| 62 | `qs rust OnceLock::get_or_init` | 3 | OnceLock::get_or_init — A synchronization primitive which can nominally be written to only once. | A relevant result ranks first, but the snippet omits key detail. |
| 63 | `qs rust spawn a thread` | 5 | spawn in std::thread - Rust — Spawns a new thread, returning a `JoinHandle` for it. | The expected result ranks first with enough context to act. |
| 64 | `qs rust mpsc channels` | 5 | std::sync::mpsc - Rust — Multi-producer, single-consumer FIFO queue communication primitives. | The expected result ranks first with enough context to act. |
| 65 | `qs rust AtomicUsize ordering` | 1 | Search Rust standard library for AtomicUsize ordering — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 66 | `qs rust read file to string` | 5 | read_to_string in std::fs - Rust — Reads the entire contents of a file into a string. | The expected result ranks first with enough context to act. |
| 67 | `qs rust write bytes to a file` | 5 | write in std::fs - Rust — Writes a slice as the entire contents of a file. | The expected result ranks first with enough context to act. |
| 68 | `qs rust canonicalize a path` | 5 | canonicalize in std::fs - Rust — Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved. | The expected result ranks first with enough context to act. |
| 69 | `qs rust create directories recursively` | 5 | create_dir_all in std::fs - Rust — Recursively create a directory and all of its parent components if they are missing. | The expected result ranks first with enough context to act. |
| 70 | `qs rust run command and capture output` | 3 | Command::output — A process builder, providing fine-grained control over how a new process should be spawned. | A relevant result ranks first, but the snippet omits key detail. |
| 71 | `qs rust environment variables` | 5 | var in std::env - Rust — Fetches the environment variable `key` from the current process. | The expected result ranks first with enough context to act. |
| 72 | `qs rust command line arguments` | 5 | args in std::env - Rust — Returns the arguments that this program was started with (normally passed via the command line). | The expected result ranks first with enough context to act. |
| 73 | `qs rust TcpListener::bind` | 3 | TcpListener::bind — A TCP socket server, listening for connections. | A relevant result ranks first, but the snippet omits key detail. |
| 74 | `qs rust parse socket address` | 1 | Search Rust standard library for parse socket address — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 75 | `qs rust Duration::from_secs` | 3 | Duration::from_secs — A `Duration` type to represent a span of time, typically used for system timeouts. | A relevant result ranks first, but the snippet omits key detail. |
| 76 | `qs rust measure elapsed time` | 3 | Instant::elapsed — A measurement of a monotonically nondecreasing clock. Opaque and useful only with `Duration`. | A relevant result ranks first, but the snippet omits key detail. |
| 77 | `qs rust unix timestamp` | 1 | Search Rust standard library for unix timestamp — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 78 | `qs rust sort vectors` | 3 | slice::sort — A dynamically-sized view into a contiguous sequence, `[T]`. | A relevant result ranks first, but the snippet omits key detail. |
| 79 | `qs rust binary_search` | 1 | Search Rust standard library for binary_search — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 80 | `qs rust parse integer` | 3 | str::parse — String slices. | A relevant result ranks first, but the snippet omits key detail. |
| 81 | `qs rust f64::total_cmp` | 3 | f64::total_cmp — A 64-bit floating-point type (specifically, the “binary64” type defined in IEEE 754-2008). | A relevant result ranks first, but the snippet omits key detail. |
| 82 | `qs rust inclusive ranges` | 1 | Search Rust standard library for inclusive ranges — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 83 | `qs rust chunks_exact_mut` | 1 | Search Rust standard library for chunks_exact_mut — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 84 | `qs rust fixed size arrays` | 1 | Search Rust standard library for fixed size arrays — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 85 | `qs rust tuple structs` | 3 | Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | A relevant result ranks first, but the snippet omits key detail. |
| 86 | `qs rust named field structs` | 3 | Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | A relevant result ranks first, but the snippet omits key detail. |
| 87 | `qs rust enums with data` | 3 | Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | A relevant result ranks first, but the snippet omits key detail. |
| 88 | `qs rust impl blocks` | 5 | Structs, enums, and implementations — Rust item syntax defines structs, enums, and inherent impl blocks. | The expected result ranks first with enough context to act. |
| 89 | `qs rust Drop::drop` | 5 | Drop::drop — Custom code within the destructor. | The expected result ranks first with enough context to act. |
| 90 | `qs rust Default::default` | 5 | Default::default — A trait for giving a type a useful default value. | The expected result ranks first with enough context to act. |
| 91 | `qs rust Clone vs Copy` | 2 | Understanding ownership — Ownership, moves, borrowing, slices, Rc, and Arc explained in context. | The first result is related, but it does not surface the requested detail. |
| 92 | `qs rust derive PartialEq and Ord` | 1 | Search Rust standard library for derive PartialEq and Ord — Searches matching items in std, core, and alloc. | This is only a generic search or a barely useful related page. |
| 93 | `qs rust unit tests` | 5 | Writing automated tests — Use #[test] functions with Rust's test harness. | The expected result ranks first with enough context to act. |
| 94 | `qs rust documentation tests` | 5 | Rustdoc documentation tests — Rustdoc compiles and runs Rust code blocks as documentation tests. | The expected result ranks first with enough context to act. |
| 95 | `qs rust Cargo features` | 5 | Cargo features and optional dependencies — Cargo feature flags enable conditional compilation and optional dependencies. | The expected result ranks first with enough context to act. |
| 96 | `qs rust cfg target_os` | 3 | Rust attributes — Attributes control conditional compilation, derives, crate behavior, and tools. | A relevant result ranks first, but the snippet omits key detail. |
| 97 | `qs rust no_std` | 2 | Rust attributes — Attributes control conditional compilation, derives, crate behavior, and tools. | The first result is related, but it does not surface the requested detail. |
| 98 | `qs rust intra-doc links` | 5 | Rustdoc intra-doc links — Link to Rust items by path and let rustdoc check the link. | The expected result ranks first with enough context to act. |
| 99 | `qs rust Rust 2024 edition` | 5 | Rust 2024 Edition Guide — Migration notes and changes for the Rust 2024 edition. | The expected result ranks first with enough context to act. |
| 100 | `qs rust raw identifiers` | 5 | Identifiers — Raw identifiers such as r#type use a keyword as an identifier. | The expected result ranks first with enough context to act. |
