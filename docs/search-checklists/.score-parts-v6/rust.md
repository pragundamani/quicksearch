# Rust quicksearch score

- Mean: **4.30/5**
- Score distribution: **5: 64, 4: 15, 3: 10, 2: 10, 1: 0, 0: 1**
- Scores >=3: **89/100**
- Scores >=4: **79/100**
- Failures (score <3): **11/100**

## Failures

- `rust move closures` — **2**: The ownership page mentions moves but not move closures or captured values.
- `rust external crates` — **2**: The module overview does not mention external crates or the extern prelude.
- `rust macro_rules!` — **2**: The only hit is a standard-library search link, not documentation for `macro_rules!`.
- `rust Send and Sync` — **2**: The generic traits page does not mention either `Send` or `Sync`.
- `rust fold values` — **0**: All five hits are collection value iterators; none covers `Iterator::fold`.
- `rust From and Into` — **2**: The generic traits page does not mention the `From` or `Into` conversion traits.
- `rust AtomicUsize ordering` — **2**: The results lead with a deprecated constant and experimental intrinsics instead of normal `AtomicUsize` operations and `Ordering`.
- `rust binary_search` — **2**: The only hit is a generic standard-library search link rather than the slice `binary_search` method.
- `rust chunks_exact_mut` — **2**: The only hit is a generic standard-library search link rather than the `chunks_exact_mut` method.
- `rust Clone vs Copy` — **2**: The ownership overview does not explain the semantic difference between `Clone` and `Copy`.
- `rust no_std` — **2**: The only hit is a generic standard-library search link rather than documentation for `#![no_std]` and `core`.

## Most important regressions

- `rust fold values` returns no result about `Iterator::fold`; every hit is a collection values iterator.
- Exact syntax/API lookups `macro_rules!`, `binary_search`, `chunks_exact_mut`, and `no_std` degrade to a generic standard-library search link.
- `rust AtomicUsize ordering` leads with deprecated or experimental items instead of stable `AtomicUsize` and `Ordering` documentation.
- Core concept queries for move closures, external crates, `Send`/`Sync`, `From`/`Into`, and `Clone` vs `Copy` route to broad overview pages that omit the requested concept.
- Ranking is weak for inclusive ranges (`RangeInclusive` is fourth), mutexes (the stable mutex is third), and `format!` (second behind `format_args!`).
