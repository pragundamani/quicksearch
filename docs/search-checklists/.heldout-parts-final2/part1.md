# Final held-out evaluation: manifest entries 0–49

- Binary: `/tmp/qs-gate/release/qs`
- Manifest: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/heldout_manifest.json`
- Execution: each query was run fresh as `timeout 30s /tmp/qs-gate/release/qs …`.
- Process results: 50/50 exited 0; 0 timeouts; duration range 0.04–4.38s (mean 1.72s).
- Scoring: 5 expected first/useful; 4 expected top 3/useful; 3 relevant but weak rank/detail; 2 partial; 1 barely useful/misleading; 0 no relevant result/error.

## Summary

- Total: **129/250**
- Mean: **2.58/5**
- Score ≥4: **20/50 (40%)**
- Score ≥3: **28/50 (56%)**
- Positive/paraphrase mean (0–33): **2.53/5**
- Negative-contrast mean (34–49): **2.69/5**
- Distribution: **5=15, 4=5, 3=8, 2=4, 1=2, 0=16**

## Rows

| # | Class | Query | Expected | Top result | Assessment | Contrast check | Score |
|---:|---|---|---|---|---|---|---:|
| 0 | paraphrase | `qs python pair adjacent iterator items` | `itertools.pairwise()` | itertools.pairwise(iterable) | `itertools.pairwise(iterable)` is result 1 with the exact behavior. | — | **5** |
| 1 | paraphrase | `qs go find mime type from a filename extension` | `mime.TypeByExtension` | func AddExtensionType | `mime.TypeByExtension()` is result 3 and has the exact signature and description. | — | **4** |
| 2 | paraphrase | `qs rust keep only successful iterator values` | `Iterator::map_while or filter_map` | Storing Keys with Associated Values in Hash Maps | No `map_while` or `filter_map`; results are generic iterator/collection pages. | — | **0** |
| 3 | paraphrase | `qs cpp exchange a value and return the old one` | `std::exchange` | std::exchange | `std::exchange` is result 1 and says it returns the old value. | — | **5** |
| 4 | paraphrase | `qs c duplicate part of a string into bounded buffer` | `strncpy` | Error handling | No `strncpy`; results only discuss general bounds checking and formatted output. | — | **1** |
| 5 | paraphrase | `qs haskell apply function to first tuple component` | `Data.Bifunctor.first` | Adding functional dependencies | No `Data.Bifunctor.first`; GHC guide results are unrelated. | — | **0** |
| 6 | paraphrase | `qs python find common path prefix by components` | `os.path.commonpath()` | find_closest(x, y, halo=None, start=None) | Result 3 is `commonprefix`, but its snippet explicitly points to `os.path.commonpath()` as the safe path-prefix API. | — | **4** |
| 7 | paraphrase | `qs go notify one waiting goroutine` | `sync.Cond.Signal` | func SetGoroutineLabels 1.9 | No Signal result; result 5 is `Cond.Wait` and mentions wakeup by `Cond.Signal`. | — | **3** |
| 8 | paraphrase | `qs rust turn option into result with lazy error` | `Option::ok_or_else` | Implementations | Result 3 is the `Option` page section on Result fallback, but `ok_or_else` is not exposed in the result. | — | **3** |
| 9 | paraphrase | `qs cpp test whether range begins with another range` | `std::ranges::starts_with` | std::basic_format_parse_context | `std::ranges::starts_with` is result 2 with full signature. | — | **4** |
| 10 | paraphrase | `qs c report current stream file position` | `ftell` | File input/output | Result 1 is the file-I/O overview and explicitly identifies `ftell` as the current-position function. | — | **5** |
| 11 | paraphrase | `qs haskell attach an index to every traversable value` | `Data.Traversable.mapAccumL` | Annotating values | No `Data.Traversable.mapAccumL`; GHC guide results are unrelated. | — | **0** |
| 12 | paraphrase | `qs python execute cleanup when interpreter exits` | `atexit.register()` | cleanup() | No Python `atexit.register`; result 5 is the lower-level C API `PyUnstable_AtExit`. | — | **2** |
| 13 | paraphrase | `qs go read exactly n bytes or fail` | `io.ReadFull` | func (*Reader) ReadBytes | `io.ReadFull` is result 2 and exactly describes reading `len(buf)` bytes or returning an error. | — | **4** |
| 14 | paraphrase | `qs rust replace matching slice prefix` | `slice::strip_prefix` | Struct PrefixComponent | No `slice::strip_prefix`; results concern path prefixes, closures, and split iterators. | — | **0** |
| 15 | paraphrase | `qs cpp invoke callable only when invocable` | `std::is_invocable and std::invoke` | std::is_invocable, std::is_invocable_r, std::is_nothrow_invocable, std::is_nothrow_invocable_r | `std::is_invocable` is result 1 and `std::invoke` is result 3; together they answer the query. | — | **5** |
| 16 | paraphrase | `qs c clear error and eof indicators on stream` | `clearerr` | clearerr | `clearerr` is result 1 with the exact EOF/error reset behavior. | — | **5** |
| 17 | paraphrase | `qs haskell combine maps and resolve duplicate keys` | `Data.Map.unionWith` | Duplicate record fields | No `Data.Map.unionWith`; GHC guide results are unrelated. | — | **0** |
| 18 | paraphrase | `qs python wait for whichever future finishes first` | `concurrent.futures.wait(FIRST_COMPLETED)` | Coroutines and tasks | `concurrent.futures.FIRST_COMPLETED` is result 2 and `wait` is result 3; the needed pieces are in the top three. | — | **4** |
| 19 | paraphrase | `qs go write quoted go string literal` | `strconv.Quote` | func (*Writer) Write | No `strconv.Quote`; results are unrelated writer and formatter APIs. | — | **0** |
| 20 | paraphrase | `qs rust split slice once on a predicate` | `slice::split_once` | Struct Split | Result 1 is ordinary slice `Split`, not `split_once`; related but does not identify the requested API. | — | **2** |
| 21 | paraphrase | `qs cpp convert enum to underlying integer type` | `std::to_underlying` | std::to_underlying | `std::to_underlying` is result 1 with exact behavior. | — | **5** |
| 22 | paraphrase | `qs c push one character back onto input stream` | `ungetc` | ungetc | `ungetc` is result 1 with exact behavior. | — | **5** |
| 23 | paraphrase | `qs haskell run action only when condition is true` | `Control.Monad.when` | RTS options to produce runtime statistics | No `Control.Monad.when`; GHC guide results are unrelated. | — | **0** |
| 24 | paraphrase | `qs python iterate over calendar month rows` | `calendar.Calendar.monthdayscalendar()` | calendar.monthcalendar(year, month) | Result 1, `calendar.monthcalendar`, gives the desired month rows but is the module-level API, not the expected `Calendar` method. | — | **3** |
| 25 | paraphrase | `qs go return a reader limited to n bytes` | `io.LimitReader` | type LimitedReader | Result 1 is `io.LimitedReader`, the returned type; it explains the byte limit but omits the `io.LimitReader` constructor. | — | **3** |
| 26 | paraphrase | `qs rust get two mutable slice elements safely` | `slice::get_many_mut` | Primitive Type slice | Result 1 is the slice primitive documentation and is relevant, but the result snippet does not surface `get_many_mut`. | — | **3** |
| 27 | paraphrase | `qs cpp count consecutive equal groups in a range` | `std::ranges::chunk_by_view` | std::ranges::unique | No `chunk_by_view`; `ranges::unique` at result 1 is related to consecutive duplicates but does not produce/count groups. | — | **1** |
| 28 | paraphrase | `qs c transform string for locale collation` | `strxfrm` | strcoll | `strxfrm` is result 4; results 1-2 mention it as the locale transform. | — | **3** |
| 29 | paraphrase | `qs haskell unfold a container from a seed` | `Data.List.unfoldr` | Building a package from Haskell source | No `Data.List.unfoldr`; GHC guide results are unrelated. | — | **0** |
| 30 | paraphrase | `qs python preserve function metadata in decorator` | `functools.wraps()` | Path.copy_into(target_dir, *, follow_symlinks=True, preserve_metadata=False) | No `functools.wraps`; results discuss unrelated metadata and decorator topics. | — | **0** |
| 31 | paraphrase | `qs go mirror bytes read from a reader into a writer` | `io.TeeReader` | func (*Reader) ReadBytes | No `io.TeeReader`; results are generic reader APIs. | — | **0** |
| 32 | paraphrase | `qs rust create an iterator that yields once` | `std::iter::once` | Once | Result 1 is `Once` and directly says it yields exactly once; the `once` function is result 3. | — | **5** |
| 33 | paraphrase | `qs cpp construct object in optional storage in place` | `std::optional::emplace` | std::optional<T>::optional | No `optional::emplace`; top results cover optional construction/in-place construction only. | — | **2** |
| 34 | negative-contrast | `qs c locate last character not first character` | `strrchr, not strchr` | strrchr | `strrchr` is result 1 and `strchr` is only result 5. | PASS: rejected `strchr` rank 5. | **5** |
| 35 | negative-contrast | `qs haskell sequence effects and discard results not collect them` | `mapM_, not mapM` | <span class="section-number">6.4.14. </span>Kind polymorphism | Neither `mapM_` nor a useful effect-sequencing result appears; `mapM` is also absent. | PASS: rejected `mapM` absent. | **0** |
| 36 | negative-contrast | `qs python shell style splitting not regular expression splitting` | `shlex.split(), not re.split()` | IDLE — Python editor and shell | The `shlex` documentation is result 5 and explains shell compatibility; `re.split` does not appear. | PASS: rejected `re.split` absent. | **3** |
| 37 | negative-contrast | `qs go compare strings case insensitively not byte equality` | `strings.EqualFold, not bytes.Equal` | Example (CaseInsensitive) | No `strings.EqualFold`; result 1 shows a case-insensitive sorting workaround. `bytes.Equal` is only mentioned inside result 3's `bytes.Compare` example. | PASS*: rejected `bytes.Equal` is not a result title, but is mentioned in rank-3 snippet. | **2** |
| 38 | negative-contrast | `qs rust remove suffix not prefix from string slice` | `str::strip_suffix, not strip_prefix` | Unsafe Rust | Only an unrelated Unsafe Rust page appears; neither `strip_suffix` nor `strip_prefix` appears. | PASS: rejected `strip_prefix` absent. | **0** |
| 39 | negative-contrast | `qs cpp copy backward not ordinary copy` | `std::copy_backward, not std::copy` | std::ranges::copy_backward, std::ranges::copy_backward_result | `ranges::copy_backward` is result 1 and `std::copy_backward` is result 2; ordinary `std::copy` is not a result. | PASS: rejected `std::copy` absent as a result. | **5** |
| 40 | negative-contrast | `qs c write one character not a formatted string` | `fputc, not fprintf` | Objects and alignment | No `fputc`; results are unrelated character/string pages. `fprintf` is not a result. | PASS: rejected `fprintf` absent. | **0** |
| 41 | negative-contrast | `qs haskell left fold with monadic effects not pure fold` | `Control.Monad.foldM, not foldl` | Monad comprehensions | No `Control.Monad.foldM`; results are unrelated GHC guide pages. Pure `foldl` is also absent. | PASS: rejected `foldl` absent. | **0** |
| 42 | negative-contrast | `qs python decimal arithmetic not binary float` | `decimal.Decimal, not float` | decimal — Decimal fixed-point and floating-point arithmetic | The decimal module is result 1 and explicitly contrasts its arithmetic with `float`; no normal-float API is promoted. | PASS: rejected `float` is only contrasted by the correct rank-1 page. | **5** |
| 43 | negative-contrast | `qs go split after separator not before it` | `strings.SplitAfter, not strings.Split` | func SplitAfter | `strings.SplitAfter` is result 1. A different `filepath.Split` is result 4, not the rejected `strings.Split`. | PASS: rejected `strings.Split` absent. | **5** |
| 44 | negative-contrast | `qs rust weak reference not owning rc clone` | `std::rc::Weak, not Rc` | Reference Cycles Can Leak Memory | Result 1 explains preventing `Rc::clone` cycles with `Weak`; `std::rc::Weak` is directly represented by results 2-3. | PASS*: `Rc::clone` is discussed in rank-1 explanatory snippet only to steer to `Weak`; it is not the recommended result. | **5** |
| 45 | negative-contrast | `qs cpp shared lock not exclusive mutex lock` | `std::shared_lock, not std::unique_lock` | std::shared_lock<Mutex>::shared_lock | `std::shared_lock` is result 1; `std::unique_lock` does not appear. | PASS: rejected `std::unique_lock` absent. | **5** |
| 46 | negative-contrast | `qs c rename a file not remove it` | `rename, not remove` | rename | `rename` is result 1 with exact behavior; `remove` does not appear. | PASS: rejected `remove` absent. | **5** |
| 47 | negative-contrast | `qs haskell catch only io errors not every exception` | `System.IO.Error.catchIOError, not Control.Exception.catch` | Limitations of deferred type errors | No `catchIOError`; GHC guide results are unrelated. General `Control.Exception.catch` is also absent. | PASS: rejected `Control.Exception.catch` absent. | **0** |
| 48 | negative-contrast | `qs python compare sequences in constant time not normal equality` | `hmac.compare_digest(), not ==` | Expressions | No `hmac.compare_digest`; results are generic comparisons. Normal `==` is not promoted as a result. | PASS: rejected `==` absent as a result. | **0** |
| 49 | negative-contrast | `qs go lexical path cleanup not filesystem symlink resolution` | `filepath.Clean, not EvalSymlinks` | type Cleanup 1.24 | `filepath.Clean` is result 4 with the exact lexical-processing description; `EvalSymlinks` does not appear. | PASS: rejected `EvalSymlinks` absent. | **3** |
