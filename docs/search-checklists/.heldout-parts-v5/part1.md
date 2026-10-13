# Held-out quicksearch evaluation: manifest entries 0–49

Candidate: `/tmp/qs-gate/release/qs`  
Manifest: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/heldout_manifest.json`  
Timeout: 30 seconds per command  
Execution rule: only the leading `qs` in each manifest query was replaced with the candidate path.

## Summary

- Rows: **50**
- Total score: **116/250**
- Average: **2.32/5**
- Score distribution: **0: 13, 1: 9, 2: 5, 3: 4, 4: 10, 5: 9**
- Score ≥3: **23/50 (46%)**
- Score ≥4: **19/50 (38%)**
- Nonzero command errors: **0**
- Timeouts: **0**

## Per-row results

| Index | Language | Class | Label | Manifest query | Expected | Score | Exit | Evaluation |
|---:|---|---|---|---|---|---:|---:|---|
| 0 | python | paraphrase | positive | `qs python pair adjacent iterator items` | `itertools.pairwise()` | **5** | 0 | Exact target is result 1. |
| 1 | go | paraphrase | positive | `qs go find mime type from a filename extension` | `mime.TypeByExtension` | **4** | 0 | Exact target is result 3. |
| 2 | rust | paraphrase | positive | `qs rust keep only successful iterator values` | `Iterator::map_while or filter_map` | **2** | 0 | Only broad iterator material; neither `map_while` nor `filter_map` appears. |
| 3 | cpp | paraphrase | positive | `qs cpp exchange a value and return the old one` | `std::exchange` | **4** | 0 | `std::exchange` is result 2; atomic exchange is result 1. |
| 4 | c | paraphrase | positive | `qs c duplicate part of a string into bounded buffer` | `strncpy` | **0** | 0 | No `strncpy`; results concern formatted I/O. |
| 5 | haskell | paraphrase | positive | `qs haskell apply function to first tuple component` | `Data.Bifunctor.first` | **0** | 0 | No `Data.Bifunctor.first`; GHC-guide results are unrelated. |
| 6 | python | paraphrase | positive | `qs python find common path prefix by components` | `os.path.commonpath()` | **4** | 0 | Result 3 documents `commonprefix` as deprecated in favor of `os.path.commonpath()`. |
| 7 | go | paraphrase | positive | `qs go notify one waiting goroutine` | `sync.Cond.Signal` | **0** | 0 | No `sync.Cond.Signal`; results are unrelated synchronization/runtime entries. |
| 8 | rust | paraphrase | positive | `qs rust turn option into result with lazy error` | `Option::ok_or_else` | **2** | 0 | Broad Option/Result material only; no `ok_or_else`. |
| 9 | cpp | paraphrase | positive | `qs cpp test whether range begins with another range` | `std::ranges::starts_with` | **4** | 0 | Exact target is result 2. |
| 10 | c | paraphrase | positive | `qs c report current stream file position` | `ftell` | **3** | 0 | No `ftell`; `fgetpos` is result 2 and is a relevant alternative, but detail/ranking is weak. |
| 11 | haskell | paraphrase | positive | `qs haskell attach an index to every traversable value` | `Data.Traversable.mapAccumL` | **1** | 0 | Only broad Traversable/index material; no `mapAccumL`. |
| 12 | python | paraphrase | positive | `qs python execute cleanup when interpreter exits` | `atexit.register()` | **1** | 0 | Cleanup-related results appear, but no `atexit.register` and no direct interpreter-exit solution. |
| 13 | go | paraphrase | positive | `qs go read exactly n bytes or fail` | `io.ReadFull` | **5** | 0 | Exact target is result 1. |
| 14 | rust | paraphrase | positive | `qs rust replace matching slice prefix` | `slice::strip_prefix` | **1** | 0 | Only broad slice/pattern results; no `strip_prefix`. |
| 15 | cpp | paraphrase | positive | `qs cpp invoke callable only when invocable` | `std::is_invocable and std::invoke` | **4** | 0 | `is_invocable` is result 1 and `invoke` is result 3. |
| 16 | c | paraphrase | positive | `qs c clear error and eof indicators on stream` | `clearerr` | **5** | 0 | Exact target is result 1. |
| 17 | haskell | paraphrase | positive | `qs haskell combine maps and resolve duplicate keys` | `Data.Map.unionWith` | **0** | 0 | No `Data.Map.unionWith`; GHC-guide results are unrelated. |
| 18 | python | paraphrase | positive | `qs python wait for whichever future finishes first` | `concurrent.futures.wait(FIRST_COMPLETED)` | **4** | 0 | `concurrent.futures.FIRST_COMPLETED` is result 2 and `wait` is result 3; asyncio appears first. |
| 19 | go | paraphrase | positive | `qs go write quoted go string literal` | `strconv.Quote` | **1** | 0 | No `strconv.Quote`; `GoStringer` is only marginally related. |
| 20 | rust | paraphrase | positive | `qs rust split slice once on a predicate` | `slice::split_once` | **1** | 0 | Only broad slice results; no `split_once`. |
| 21 | cpp | paraphrase | positive | `qs cpp convert enum to underlying integer type` | `std::to_underlying` | **5** | 0 | Exact target is result 1. |
| 22 | c | paraphrase | positive | `qs c push one character back onto input stream` | `ungetc` | **5** | 0 | Exact target is result 1. |
| 23 | haskell | paraphrase | positive | `qs haskell run action only when condition is true` | `Control.Monad.when` | **0** | 0 | No `Control.Monad.when`; results are unrelated. |
| 24 | python | paraphrase | positive | `qs python iterate over calendar month rows` | `calendar.Calendar.monthdayscalendar()` | **4** | 0 | `calendar.monthcalendar` is result 1 and immediately useful, though not the requested instance method spelling. |
| 25 | go | paraphrase | positive | `qs go return a reader limited to n bytes` | `io.LimitReader` | **3** | 0 | `io.LimitedReader` is result 1; useful concept/detail, but `io.LimitReader` is absent. |
| 26 | rust | paraphrase | positive | `qs rust get two mutable slice elements safely` | `slice::get_many_mut` | **1** | 0 | Only general slice/vector results; no `get_many_mut`. |
| 27 | cpp | paraphrase | positive | `qs cpp count consecutive equal groups in a range` | `std::ranges::chunk_by_view` | **2** | 0 | `std::unique` is result 1 and relates to consecutive equal groups, but `chunk_by_view` is absent and counting is not shown. |
| 28 | c | paraphrase | positive | `qs c transform string for locale collation` | `strxfrm` | **4** | 0 | Result 1 (`strcoll`) links/describes `strxfrm`; direct `strxfrm` entry is result 4. |
| 29 | haskell | paraphrase | positive | `qs haskell unfold a container from a seed` | `Data.List.unfoldr` | **0** | 0 | No `Data.List.unfoldr`; results are unrelated. |
| 30 | python | paraphrase | positive | `qs python preserve function metadata in decorator` | `functools.wraps()` | **1** | 0 | Metadata/decorator-adjacent results only; no `functools.wraps`. |
| 31 | go | paraphrase | positive | `qs go mirror bytes read from a reader into a writer` | `io.TeeReader` | **1** | 0 | Reader results are generic; no `io.TeeReader` or mirroring solution. |
| 32 | rust | paraphrase | positive | `qs rust create an iterator that yields once` | `std::iter::once` | **1** | 0 | Broad iterator results only; no `std::iter::once`. |
| 33 | cpp | paraphrase | positive | `qs cpp construct object in optional storage in place` | `std::optional::emplace` | **2** | 0 | Optional construction and `in_place` results are relevant, but no `optional::emplace`. |
| 34 | c | negative-contrast | negative | `qs c locate last character not first character` | `strrchr, not strchr` | **5** | 0 | `strrchr` is result 1; rejected `strchr` is result 4, not top. |
| 35 | haskell | negative-contrast | negative | `qs haskell sequence effects and discard results not collect them` | `mapM_, not mapM` | **0** | 0 | No `mapM_`; rejected `mapM` is also absent/not top. Results are irrelevant. |
| 36 | python | negative-contrast | negative | `qs python shell style splitting not regular expression splitting` | `shlex.split(), not re.split()` | **0** | 0 | No `shlex.split`; rejected `re.split` is absent/not top. Results are irrelevant. |
| 37 | go | negative-contrast | negative | `qs go compare strings case insensitively not byte equality` | `strings.EqualFold, not bytes.Equal` | **3** | 0 | A case-insensitive sorting example is result 2, but no `strings.EqualFold`; rejected `bytes.Equal` is absent/not top (`bytes.Compare` is result 3). |
| 38 | rust | negative-contrast | negative | `qs rust remove suffix not prefix from string slice` | `str::strip_suffix, not strip_prefix` | **0** | 0 | Only an unrelated Unsafe Rust page; rejected `strip_prefix` is absent/not top, but target is also absent. |
| 39 | cpp | negative-contrast | negative | `qs cpp copy backward not ordinary copy` | `std::copy_backward, not std::copy` | **4** | 0 | `std::ranges::copy_backward` is result 2 and `std::copy_backward` result 4; rejected ordinary `std::copy` is absent/not top. |
| 40 | c | negative-contrast | negative | `qs c write one character not a formatted string` | `fputc, not fprintf` | **2** | 0 | `fputc` appears only inside the result-2 `rename` example; rejected `fprintf` is absent/not top. |
| 41 | haskell | negative-contrast | negative | `qs haskell left fold with monadic effects not pure fold` | `Control.Monad.foldM, not foldl` | **0** | 0 | No `foldM`; rejected `foldl` is absent/not top. Results are irrelevant. |
| 42 | python | negative-contrast | negative | `qs python decimal arithmetic not binary float` | `decimal.Decimal, not float` | **5** | 0 | The `decimal` module is result 1 and directly contrasts decimal with float; rejected float equality/arithmetic is not top. |
| 43 | go | negative-contrast | negative | `qs go split after separator not before it` | `strings.SplitAfter, not strings.Split` | **4** | 0 | `strings.SplitAfter` is result 2; rejected `strings.Split` is absent/not top (unrelated `filepath.Split` is result 1). |
| 44 | rust | negative-contrast | negative | `qs rust weak reference not owning rc clone` | `std::rc::Weak, not Rc` | **0** | 0 | No `std::rc::Weak`; rejected owning `Rc` is absent/not top. Results are generic references. |
| 45 | cpp | negative-contrast | negative | `qs cpp shared lock not exclusive mutex lock` | `std::shared_lock, not std::unique_lock` | **5** | 0 | `std::shared_lock` constructor is result 1; rejected `std::unique_lock` is absent/not top. |
| 46 | c | negative-contrast | negative | `qs c rename a file not remove it` | `rename, not remove` | **5** | 0 | `rename` is result 1; rejected `remove` is absent/not top. |
| 47 | haskell | negative-contrast | negative | `qs haskell catch only io errors not every exception` | `System.IO.Error.catchIOError, not Control.Exception.catch` | **0** | 0 | No `catchIOError`; rejected generic `Control.Exception.catch` is absent/not top. Results are unrelated. |
| 48 | python | negative-contrast | negative | `qs python compare sequences in constant time not normal equality` | `hmac.compare_digest(), not ==` | **0** | 0 | No `hmac.compare_digest`; rejected normal equality is absent/not top. Results are irrelevant. |
| 49 | go | negative-contrast | negative | `qs go lexical path cleanup not filesystem symlink resolution` | `filepath.Clean, not EvalSymlinks` | **3** | 0 | `filepath.Clean` is result 4 with the correct purely lexical description; rejected `EvalSymlinks` is absent/not top. |

## Scoring scale

- **5:** Expected result first or immediately useful.
- **4:** Expected result in the top three or otherwise useful.
- **3:** Relevant, but ranking or detail is weak.
- **2:** Partial result.
- **1:** Misleading or barely useful.
- **0:** Error or no relevant result.

For negative contrasts, the evaluation also checked that the rejected target was not the top result.
