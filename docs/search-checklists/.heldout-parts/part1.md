# Held-out evaluation, part 1 (indices 0–49)

Binary: `/tmp/qs-gate/release/qs`. Each manifest command was run unchanged except for replacing the leading `qs` executable. Timeout: 20 seconds.

| Index | Command | Class | Label | Score | Observed top result/error | Candid assessment |
|---:|---|---|---|---:|---|---|
| 0 | `qs python pair adjacent iterator items` | paraphrase | positive | 2 | c.PyIter_NextItem() | Expected API is absent; iterator results are only loosely related. |
| 1 | `qs go find mime type from a filename extension` | paraphrase | positive | 5 | mime.TypeByExtension() | Exact expected API is first with useful documentation. |
| 2 | `qs rust keep only successful iterator values` | paraphrase | positive | 2 | Search Rust standard library for keep only successful iterator values | Only a generic Rust std search link is returned; no target API is identified. |
| 3 | `qs cpp exchange a value and return the old one` | paraphrase | positive | 4 | std::extreme_value_distribution::a | Expected API is third and directly usable. |
| 4 | `qs c duplicate part of a string into bounded buffer` | paraphrase | positive | 3 | Copy a section of a string into buffer in C - Stack Overflow | Top web result addresses bounded substring copying, but does not surface `strncpy`. |
| 5 | `qs haskell apply function to first tuple component` | paraphrase | positive | 1 | Haskell language documentation — apply function to first tuple component | Only generic Haskell language documentation is returned. |
| 6 | `qs python find common path prefix by components` | paraphrase | positive | 2 | bytearray.find() | `os.path` appears fourth, but `commonpath` is absent. |
| 7 | `qs go notify one waiting goroutine` | paraphrase | positive | 2 | sync.Once | `sync.Cond.Wait` is fifth, but the requested `Signal` API is absent. |
| 8 | `qs rust turn option into result with lazy error` | paraphrase | positive | 2 | Search Rust standard library for turn option into result with lazy error | Only a generic Rust std search link is returned; no target API is identified. |
| 9 | `qs cpp test whether range begins with another range` | paraphrase | positive | 3 | std::ranges::join_with_view::begin | Expected API is fourth; relevant but ranked below the top three. |
| 10 | `qs c report current stream file position` | paraphrase | positive | 3 | c - Saving off the current read position so I can seek to it later ... | Results discuss saving stream position and include `ftell` fifth; useful but weakly ranked. |
| 11 | `qs haskell attach an index to every traversable value` | paraphrase | positive | 1 | Haskell language documentation — attach an index to every traversable value | Only generic Haskell language documentation is returned. |
| 12 | `qs python execute cleanup when interpreter exits` | paraphrase | positive | 1 | exit | Shutdown-related entries appear, but `atexit.register` is absent. |
| 13 | `qs go read exactly n bytes or fail` | paraphrase | positive | 1 | bytes.SplitN() | Results concern byte reading, but not exact-length `io.ReadFull` behavior. |
| 14 | `qs rust replace matching slice prefix` | paraphrase | positive | 2 | Search Rust standard library for replace matching slice prefix | Only a generic Rust std search link is returned; no target API is identified. |
| 15 | `qs cpp invoke callable only when invocable` | paraphrase | positive | 4 | std::invocable, std::regular_invocable - cppreference.com | `std::invoke` is third and the top result covers invocability, though not the exact trait. |
| 16 | `qs c clear error and eof indicators on stream` | paraphrase | positive | 5 | clearerr () function - C Library | Exact expected function is first with useful context. |
| 17 | `qs haskell combine maps and resolve duplicate keys` | paraphrase | positive | 1 | Haskell language documentation — combine maps and resolve duplicate keys | Only generic Haskell language documentation is returned. |
| 18 | `qs python wait for whichever future finishes first` | paraphrase | positive | 4 | asyncio.wait_for() | Expected `wait` API is second with useful context; `FIRST_COMPLETED` is not explicit in the visible snippet. |
| 19 | `qs go write quoted go string literal` | paraphrase | positive | 2 | fmt.GoStringer | Go-syntax output is loosely related, but `strconv.Quote` is absent. |
| 20 | `qs rust split slice once on a predicate` | paraphrase | positive | 2 | Search Rust standard library for split slice once on a predicate | Only a generic Rust std search link is returned; no target API is identified. |
| 21 | `qs cpp convert enum to underlying integer type` | paraphrase | positive | 5 | std::to_underlying | Exact expected API is first. |
| 22 | `qs c push one character back onto input stream` | paraphrase | positive | 2 | int | A generic file-I/O page is fifth; `ungetc` is absent. |
| 23 | `qs haskell run action only when condition is true` | paraphrase | positive | 1 | Haskell language documentation — run action only when condition is true | Only generic Haskell language documentation is returned. |
| 24 | `qs python iterate over calendar month rows` | paraphrase | positive | 3 | calendar.Month | Calendar results are relevant, but they format a month rather than return month rows. |
| 25 | `qs go return a reader limited to n bytes` | paraphrase | positive | 1 | bytes.Reader.WriteTo() | Reader-related results are returned, but `io.LimitReader` is absent. |
| 26 | `qs rust get two mutable slice elements safely` | paraphrase | positive | 2 | Search Rust standard library for get two mutable slice elements safely | Only a generic Rust std search link is returned; no target API is identified. |
| 27 | `qs cpp count consecutive equal groups in a range` | paraphrase | positive | 1 | std::in_range | Range results are unrelated to grouping consecutive equal elements. |
| 28 | `qs c transform string for locale collation` | paraphrase | positive | 3 | std::collate - cppreference.com | Results explain locale collation transforms, but the C `strxfrm` API is absent. |
| 29 | `qs haskell unfold a container from a seed` | paraphrase | positive | 1 | Haskell language documentation — unfold a container from a seed | Only generic Haskell language documentation is returned. |
| 30 | `qs python preserve function metadata in decorator` | paraphrase | positive | 2 | range (built-in function) | A decorator entry appears third, but metadata preservation and `wraps` are absent. |
| 31 | `qs go mirror bytes read from a reader into a writer` | paraphrase | positive | 2 | io.ByteReader | Reader/writer interfaces are relevant, but `io.TeeReader` is absent. |
| 32 | `qs rust create an iterator that yields once` | paraphrase | positive | 2 | Search Rust standard library for create an iterator that yields once | Only a generic Rust std search link is returned; no target API is identified. |
| 33 | `qs cpp construct object in optional storage in place` | paraphrase | positive | 4 | Why do std::optional constructors use std::in_place? | Top-three pages explain in-place optional construction and mention `emplace`; useful but indirect. |
| 34 | `qs c locate last character not first character` | negative-contrast | negative | 0 | TIMEOUT after 20 seconds | Timed out with no output. |
| 35 | `qs haskell sequence effects and discard results not collect them` | negative-contrast | negative | 1 | Haskell language documentation — sequence effects and discard results not collect them | The `mapM` trap is avoided, but no `mapM_` result is supplied. |
| 36 | `qs python shell style splitting not regular expression splitting` | negative-contrast | negative | 0 | tkinter.SE | The exact trap `re.split()` is returned third; intended `shlex.split()` is absent. |
| 37 | `qs go compare strings case insensitively not byte equality` | negative-contrast | negative | 1 | bytes.Compare() | The exact trap is avoided, but byte/string comparison results are wrong and `EqualFold` is absent. |
| 38 | `qs rust remove suffix not prefix from string slice` | negative-contrast | negative | 1 | Unsafe Rust | The prefix trap is avoided, but the sole Unsafe Rust result is irrelevant to `strip_suffix`. |
| 39 | `qs cpp copy backward not ordinary copy` | negative-contrast | negative | 5 | std::copy_backward | Exact intended API is first; ordinary `copy` is safely ranked fourth. |
| 40 | `qs c write one character not a formatted string` | negative-contrast | negative | 1 | char | `fprintf` is not a result, but `fputc` is absent and the results are not useful. |
| 41 | `qs haskell left fold with monadic effects not pure fold` | negative-contrast | negative | 1 | Haskell language documentation — left fold with monadic effects not pure fold | The pure-fold trap is avoided, but no `foldM` result is supplied. |
| 42 | `qs python decimal arithmetic not binary float` | negative-contrast | negative | 5 | Floating-point notes (decimal) | Decimal guidance is first and explicitly contrasts binary float. |
| 43 | `qs go split after separator not before it` | negative-contrast | negative | 3 | big.Int.Not() | `bytes.SplitAfter` is fourth and preserves separators, but the requested string API is absent. |
| 44 | `qs rust weak reference not owning rc clone` | negative-contrast | negative | 2 | Search Rust standard library for weak reference not owning rc clone | The owning `Rc` trap is avoided, but only a generic std search link is returned. |
| 45 | `qs cpp shared lock not exclusive mutex lock` | negative-contrast | negative | 0 | std::shared_mutex::lock - cppreference.com | Top result is `shared_mutex::lock`, explicitly exclusive—the contrast trap rather than `shared_lock`. |
| 46 | `qs c rename a file not remove it` | negative-contrast | negative | 5 | rename function in C - GeeksforGeeks | `rename` is first and clearly contrasted with removal. |
| 47 | `qs haskell catch only io errors not every exception` | negative-contrast | negative | 1 | Haskell language documentation — catch only io errors not every exception | Broad exception catching is avoided, but no `catchIOError` result is supplied. |
| 48 | `qs python compare sequences in constant time not normal equality` | negative-contrast | negative | 1 | ast.NotIn | Normal equality is avoided, but no constant-time comparison result is returned. |
| 49 | `qs go lexical path cleanup not filesystem symlink resolution` | negative-contrast | negative | 2 | path | Symlink resolution is avoided; the top `path` package is only broadly relevant and `filepath.Clean` is absent. |

## Summary

- Distribution: 0: 3, 1: 16, 2: 15, 3: 6, 4: 4, 5: 6
- Average: 2.20/5
- Score >=3: 16/50 (32%)
- Score >=4: 10/50 (20%)
- Command errors: 0
- Timeouts: 1
