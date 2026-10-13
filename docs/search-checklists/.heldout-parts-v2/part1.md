# Held-out evaluation, part 1 (indices 0–49)

Binary: `/tmp/qs-gate/release/qs`. Each manifest command was run unchanged except for replacing the leading `qs` executable. Timeout: 30 seconds.

| Index | Command | Class | Label | Score | Observed top result/error | Candid assessment |
|---:|---|---|---|---:|---|---|
| 0 | `qs python pair adjacent iterator items` | paraphrase | positive | 2 | Expressions | Expected `itertools.pairwise` is absent; results only loosely concern paired items and iteration. |
| 1 | `qs go find mime type from a filename extension` | paraphrase | positive | 4 | Package runtime | Exact `mime.TypeByExtension` is third with its signature and behavior. |
| 2 | `qs rust keep only successful iterator values` | paraphrase | positive | 2 | Iterator loops | Iterator documentation is relevant, but neither `map_while` nor `filter_map` is identified. |
| 3 | `qs cpp exchange a value and return the old one` | paraphrase | positive | 1 | std::atomic<std::weak_ptr> | `std::exchange` is absent; atomic exchange-related pages do not answer the value-exchange request. |
| 4 | `qs c duplicate part of a string into bounded buffer` | paraphrase | positive | 2 | wprintf, fwprintf, swprintf, wprintf_s, fwprintf_s, swprintf_s, snwprintf_s | `strncpy` is absent; bounded formatted-output pages are only partially related. |
| 5 | `qs haskell apply function to first tuple component` | paraphrase | positive | 1 | Adding functional dependencies | Results concern GHC functional dependencies, not `Data.Bifunctor.first`. |
| 6 | `qs python find common path prefix by components` | paraphrase | positive | 4 | Using Python on Windows | The third result explicitly recommends `os.path.commonpath()` for path prefixes. |
| 7 | `qs go notify one waiting goroutine` | paraphrase | positive | 3 | Package syscall | `sync.Cond.Signal` appears with exact semantics in the fifth result, but ranking is weak. |
| 8 | `qs rust turn option into result with lazy error` | paraphrase | positive | 3 | Lazy boolean operators | The top three explain lazy evaluation, `Option`, and `Result`, but omit `ok_or_else`. |
| 9 | `qs cpp test whether range begins with another range` | paraphrase | positive | 1 | std::ranges::views::chunk_by, std::ranges::chunk_by_view | `std::ranges::starts_with` is absent; the returned range views do not perform the requested test. |
| 10 | `qs c report current stream file position` | paraphrase | positive | 4 | fsetpos | `fgetpos` is second and directly obtains the current stream position, though `ftell` is absent. |
| 11 | `qs haskell attach an index to every traversable value` | paraphrase | positive | 1 | Annotating values | Traversable-related results do not show `mapAccumL` or how to attach indices. |
| 12 | `qs python execute cleanup when interpreter exits` | paraphrase | positive | 1 | unittest — Unit testing framework | Shutdown cleanup via `atexit.register` is absent; teardown and asyncio cleanup pages are not a substitute. |
| 13 | `qs go read exactly n bytes or fail` | paraphrase | positive | 3 | Package binary | `io.ReadFull` and its exact read-or-error behavior appear fourth. |
| 14 | `qs rust replace matching slice prefix` | paraphrase | positive | 1 | Slice patterns | `strip_prefix` is absent; generic slice-pattern pages do not provide the operation. |
| 15 | `qs cpp invoke callable only when invocable` | paraphrase | positive | 5 | std::is_invocable, std::is_invocable_r, std::is_nothrow_invocable, std::is_nothrow_invocable_r | `std::is_invocable` is first and `std::invoke` is third, covering both requested parts. |
| 16 | `qs c clear error and eof indicators on stream` | paraphrase | positive | 5 | clearerr | Exact `clearerr` is first with the error/EOF reset behavior. |
| 17 | `qs haskell combine maps and resolve duplicate keys` | paraphrase | positive | 1 | Duplicate record fields | Results concern duplicate fields and unrelated GHC topics; `unionWith` is absent. |
| 18 | `qs python wait for whichever future finishes first` | paraphrase | positive | 4 | Coroutines and tasks | `concurrent.futures.wait` is second and `asyncio.wait` is first; the needed `FIRST_COMPLETED` detail is not visible. |
| 19 | `qs go write quoted go string literal` | paraphrase | positive | 2 | Package http | Go quoting results are loosely related, but `strconv.Quote` is absent. |
| 20 | `qs rust split slice once on a predicate` | paraphrase | positive | 2 | Predicate loops | Slice and predicate documentation is relevant, but `split_once` is absent. |
| 21 | `qs cpp convert enum to underlying integer type` | paraphrase | positive | 5 | std::to_underlying | Exact `std::to_underlying` is first. |
| 22 | `qs c push one character back onto input stream` | paraphrase | positive | 5 | ungetc | Exact `ungetc` is first with its push-back behavior. |
| 23 | `qs haskell run action only when condition is true` | paraphrase | positive | 1 | RTS options to produce runtime statistics | Results concern unrelated GHC options; `Control.Monad.when` is absent. |
| 24 | `qs python iterate over calendar month rows` | paraphrase | positive | 3 | calendar — General calendar-related functions | The calendar module is first and month formatting is third, but `monthdayscalendar` and row iteration are absent. |
| 25 | `qs go return a reader limited to n bytes` | paraphrase | positive | 1 | Package bytes | Reader-related pages are returned, but `io.LimitReader` is absent. |
| 26 | `qs rust get two mutable slice elements safely` | paraphrase | positive | 2 | Slice types | Slice and safe-access material is relevant, but `get_many_mut` is absent. |
| 27 | `qs cpp count consecutive equal groups in a range` | paraphrase | positive | 2 | std::unique | `std::unique` is first and discusses consecutive equal elements, but does not provide `chunk_by_view` or group counting. |
| 28 | `qs c transform string for locale collation` | paraphrase | positive | 4 | strcoll | The first result's snippet identifies and defines `strxfrm`; its dedicated page is fourth. |
| 29 | `qs haskell unfold a container from a seed` | paraphrase | positive | 1 | Building a package from Haskell source | Results are generic GHC material; `Data.List.unfoldr` is absent. |
| 30 | `qs python preserve function metadata in decorator` | paraphrase | positive | 2 | dataclasses — Data Classes | Function metadata appears in the third result, but decorator preservation and `functools.wraps` are absent. |
| 31 | `qs go mirror bytes read from a reader into a writer` | paraphrase | positive | 2 | Package bytes | Reader/writer pages are relevant and `io.Copy` appears fifth, but `io.TeeReader` is absent. |
| 32 | `qs rust create an iterator that yields once` | paraphrase | positive | 2 | Iterator loops | Iterator documentation is relevant, but `std::iter::once` is not identified. |
| 33 | `qs cpp construct object in optional storage in place` | paraphrase | positive | 4 | std::inplace_vector | `std::optional` is second and `std::in_place` is third, giving a useful in-place construction route though `emplace` is absent. |
| 34 | `qs c locate last character not first character` | negative-contrast | negative | 0 | strstr | `strrchr` is fifth, while rejected `strchr` is second; the contrast is ranked incorrectly. |
| 35 | `qs haskell sequence effects and discard results not collect them` | negative-contrast | negative | 1 | <span class="section-number">6.4.14. </span>Kind polymorphism | The `mapM` trap is avoided, but unrelated GHC pages do not identify `mapM_`. |
| 36 | `qs python shell style splitting not regular expression splitting` | negative-contrast | negative | 1 | Glossary | The `re.split` trap is avoided, but `shlex.split` is absent and results are only loosely shell-related. |
| 37 | `qs go compare strings case insensitively not byte equality` | negative-contrast | negative | 3 | Package strings | The strings package is first and case-folded equality is explained second via `bytes.EqualFold`; `strings.EqualFold` is not surfaced and `bytes.Equal` is not the top result. |
| 38 | `qs rust remove suffix not prefix from string slice` | negative-contrast | negative | 1 | Unsafe Rust | The `Rc` trap is avoided, but the sole Unsafe Rust result is irrelevant to `std::rc::Weak`. |
| 39 | `qs cpp copy backward not ordinary copy` | negative-contrast | negative | 4 | std::ranges::move_backward, std::ranges::move_backward_result | `std::ranges::copy_backward` is second and directly useful; ordinary `copy` is not a top result. |
| 40 | `qs c write one character not a formatted string` | negative-contrast | negative | 1 | Generic selection (since C11) | The `fprintf` trap is avoided, but no result identifies `fputc` and the returned pages are irrelevant. |
| 41 | `qs haskell left fold with monadic effects not pure fold` | negative-contrast | negative | 1 | Monad comprehensions | The pure `foldl` trap is avoided, but `foldM` is absent and results are irrelevant. |
| 42 | `qs python decimal arithmetic not binary float` | negative-contrast | negative | 5 | decimal — Decimal fixed-point and floating-point arithmetic | Decimal arithmetic is first and explicitly contrasts the ordinary `float` type. |
| 43 | `qs go split after separator not before it` | negative-contrast | negative | 0 | Package filepath | Rejected `strings.Split` is second while `strings.SplitAfter` is absent, so the contrast fails. |
| 44 | `qs rust weak reference not owning rc clone` | negative-contrast | negative | 1 | Weak keywords | The owning-`Rc` trap is avoided, but generic reference pages do not identify `std::rc::Weak`. |
| 45 | `qs cpp shared lock not exclusive mutex lock` | negative-contrast | negative | 5 | Standard library header <shared_mutex> (C++14) | The shared-mutex header is first and `std::shared_lock` construction is second; no exclusive-lock trap is top-ranked. |
| 46 | `qs c rename a file not remove it` | negative-contrast | negative | 5 | rename | Exact `rename` is first; `remove` only appears in a second-result comparison. |
| 47 | `qs haskell catch only io errors not every exception` | negative-contrast | negative | 1 | Limitations of deferred type errors | Broad exception catching is not top-ranked, but no result identifies `catchIOError`. |
| 48 | `qs python compare sequences in constant time not normal equality` | negative-contrast | negative | 1 | Expressions | Normal equality is avoided, but `hmac.compare_digest` is absent and the constant-time result concerns tuple copying. |
| 49 | `qs go lexical path cleanup not filesystem symlink resolution` | negative-contrast | negative | 3 | Package runtime | `filepath.Clean` appears fourth with exact lexical-cleanup semantics; `EvalSymlinks` is absent. |

## Summary

- Distribution: 0: 2, 1: 18, 2: 10, 3: 6, 4: 7, 5: 7
- Average: 2.38/5
- Score >=3: 20/50 (40%)
- Score >=4: 14/50 (28%)
- Command errors: 0
- Timeouts: 0
