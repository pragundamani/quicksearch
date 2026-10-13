# Held-out v11 candidate evaluation: manifest indexes 0–49

**Binary:** `/tmp/qs-gate/release/qs`  
**SHA256:** `8f05a6dcd581ef25fe5d2904b169cb3794a94056b19d687cf4646a6372cc096d`  
**Run:** Fresh process per query with `--json`; 20-second timeout.  
**Rubric:** 5 canonical/immediately useful first; 4 expected target top 3/useful; 3 relevant but weak; 2 partial; 1 misleading; 0 error/no relevant result.

## Summary

- Total: **243/250**
- Average: **4.86/5**
- Score ≥3: **50/50 (100%)**
- Score ≥4: **50/50 (100%)**
- Errors: **0**
- Timeouts: **0**
- Distribution: **0: 0, 1: 0, 2: 0, 3: 0, 4: 7, 5: 43**

## Scores 0–3, errors, and timeouts

None.

## Results

| Index | Class | Query | Expected | Top results | Score | Assessment |
|---:|---|---|---|---|---:|---|
| 0 | paraphrase | `qs python pair adjacent iterator items` | `itertools.pairwise()` | 1. itertools.pairwise | **5** | itertools.pairwise is the first result and exactly matches the requested adjacent pairing. |
| 1 | paraphrase | `qs go find mime type from a filename extension` | `mime.TypeByExtension` | 1. func AddExtensionType<br>2. func FindProcess<br>3. mime.TypeByExtension() | **4** | TypeByExtension is the third hit and directly answers the query. |
| 2 | paraphrase | `qs rust keep only successful iterator values` | `Iterator::map_while or filter_map` | 1. Iterator::filter_map | **5** | filter_map is the first and exact intent match. |
| 3 | paraphrase | `qs cpp exchange a value and return the old one` | `std::exchange` | 1. std::exchange<br>2. std::atomic<T>::exchange<br>3. std::extreme_value_distribution::a | **5** | std::exchange is first and exact. |
| 4 | paraphrase | `qs c duplicate part of a string into bounded buffer` | `strncpy` | 1. strncpy | **5** | strncpy is first and exact. |
| 5 | paraphrase | `qs haskell apply function to first tuple component` | `Data.Bifunctor.first` | 1. first :: Bifunctor p => (a -> b) -> p a c -> p b c | **5** | Data.Bifunctor.first is first with tuple examples. |
| 6 | paraphrase | `qs python find common path prefix by components` | `os.path.commonpath()` | 1. find_closest(x, y, halo=None, start=None)<br>2. Built-in Types<br>3. commonprefix | **4** | commonprefix is third, but its snippet explicitly recommends commonpath. |
| 7 | paraphrase | `qs go notify one waiting goroutine` | `sync.Cond.Signal` | 1. sync.Cond.Signal | **5** | sync.Cond.Signal is first and exact. |
| 8 | paraphrase | `qs rust turn option into result with lazy error` | `Option::ok_or_else` | 1. Option::ok_or_else | **5** | Option::ok_or_else is first and exact. |
| 9 | paraphrase | `qs cpp test whether range begins with another range` | `std::ranges::starts_with` | 1. std::basic_format_parse_context<br>2. std::ranges::starts_with<br>3. std::ranges::join_with_view::begin | **4** | starts_with is second and exact. |
| 10 | paraphrase | `qs c report current stream file position` | `ftell` | 1. File input/output<br>2. Standard library header <stdio.h><br>3. FILE | **5** | The first result snippet names ftell and its exact behavior. |
| 11 | paraphrase | `qs haskell attach an index to every traversable value` | `Data.Traversable.mapAccumL` | 1. mapAccumL :: Traversable t => (s -> a -> (s, b)) -> s -> t a -> (s, t b) | **5** | mapAccumL is first and supports indexed traversal via state. |
| 12 | paraphrase | `qs python execute cleanup when interpreter exits` | `atexit.register()` | 1. atexit.register | **5** | atexit.register is first and exact. |
| 13 | paraphrase | `qs go read exactly n bytes or fail` | `io.ReadFull` | 1. func (*Reader) ReadBytes<br>2. func ReadFull<br>3. bytes.SplitN() | **4** | ReadFull is second and its snippet states exact-length reads. |
| 14 | paraphrase | `qs rust replace matching slice prefix` | `slice::strip_prefix` | 1. slice::strip_prefix | **5** | slice::strip_prefix is first and exact. |
| 15 | paraphrase | `qs cpp invoke callable only when invocable` | `std::is_invocable and std::invoke` | 1. std::is_invocable, std::is_invocable_r, std::is_nothrow_invocable, std::is_nothrow_invocable_r<br>2. std::result_of, std::invoke_result<br>3. std::invoke, std::invoke_r | **5** | is_invocable is first and std::invoke is third. |
| 16 | paraphrase | `qs c clear error and eof indicators on stream` | `clearerr` | 1. clearerr<br>2. fgetc, getc<br>3. SIG_ERR | **5** | clearerr is first and exact. |
| 17 | paraphrase | `qs haskell combine maps and resolve duplicate keys` | `Data.Map.unionWith` | 1. unionWith :: Ord k => (a -> a -> a) -> Map k a -> Map k a -> Map k a<br>2. unionWith :: Ord k => (a -> a -> a) -> Map k a -> Map k a -> Map k a<br>3. unionWith :: (a -> a -> a) -> IntMap a -> IntMap a -> IntMap a | **5** | Data.Map unionWith is first and exactly combines maps while resolving duplicate keys. |
| 18 | paraphrase | `qs python wait for whichever future finishes first` | `concurrent.futures.wait(FIRST_COMPLETED)` | 1. Coroutines and tasks<br>2. concurrent.futures.FIRST_COMPLETED<br>3. wait | **4** | FIRST_COMPLETED results lead; concurrent.futures is second and wait is third. |
| 19 | paraphrase | `qs go write quoted go string literal` | `strconv.Quote` | 1. strconv.Quote | **5** | strconv.Quote is first and exact. |
| 20 | paraphrase | `qs rust split slice once on a predicate` | `slice::split_once` | 1. slice::split_once | **5** | slice::split_once is first and exact. |
| 21 | paraphrase | `qs cpp convert enum to underlying integer type` | `std::to_underlying` | 1. std::to_underlying<br>2. Enumeration declaration<br>3. Usual arithmetic conversions | **5** | std::to_underlying is first and exact. |
| 22 | paraphrase | `qs c push one character back onto input stream` | `ungetc` | 1. ungetc<br>2. ungetwc<br>3. int | **5** | ungetc is first and exact. |
| 23 | paraphrase | `qs haskell run action only when condition is true` | `Control.Monad.when` | 1. when :: Applicative f => Bool -> f () -> f ()<br>2. when :: Applicative f => Bool -> f () -> f ()<br>3. whenIoSubSystem :: IoSubSystem -> IO () -> IO () | **5** | when is first and exact. |
| 24 | paraphrase | `qs python iterate over calendar month rows` | `calendar.Calendar.monthdayscalendar()` | 1. calendar.monthcalendar | **5** | calendar.monthcalendar is first and directly returns a month's rows; it is a useful module-level equivalent for this intent. |
| 25 | paraphrase | `qs go return a reader limited to n bytes` | `io.LimitReader` | 1. io.LimitReader | **5** | io.LimitReader is first and exact. |
| 26 | paraphrase | `qs rust get two mutable slice elements safely` | `slice::get_many_mut` | 1. slice::get_disjoint_mut | **5** | get_disjoint_mut is first and is the current API for the requested safe borrows. |
| 27 | paraphrase | `qs cpp count consecutive equal groups in a range` | `std::ranges::chunk_by_view` | 1. std::ranges::chunk_by_view | **5** | std::ranges::chunk_by_view is first and exact. |
| 28 | paraphrase | `qs c transform string for locale collation` | `strxfrm` | 1. strcoll<br>2. wcscoll<br>3. tm | **4** | strxfrm is fourth; the first strcoll snippet explicitly points to it. |
| 29 | paraphrase | `qs haskell unfold a container from a seed` | `Data.List.unfoldr` | 1. unfoldr :: (b -> Maybe (a, b)) -> b -> [a]<br>2. unfoldr :: (a -> (b, Maybe a)) -> a -> NonEmpty b<br>3. unfoldr :: (b -> Maybe (a, b)) -> b -> Seq a | **5** | Data.List.unfoldr is first and exact. |
| 30 | paraphrase | `qs python preserve function metadata in decorator` | `functools.wraps()` | 1. functools.wraps | **5** | functools.wraps is first and exact. |
| 31 | paraphrase | `qs go mirror bytes read from a reader into a writer` | `io.TeeReader` | 1. io.TeeReader | **5** | io.TeeReader is first and exact. |
| 32 | paraphrase | `qs rust create an iterator that yields once` | `std::iter::once` | 1. Once<br>2. Async Iterators<br>3. Function once | **5** | Once is first with exact semantics; function once is third. |
| 33 | paraphrase | `qs cpp construct object in optional storage in place` | `std::optional::emplace` | 1. std::optional::emplace | **5** | std::optional::emplace is first and exact. |
| 34 | negative-contrast | `qs c locate last character not first character` | `strrchr, not strchr` | 1. strrchr<br>2. strstr<br>3. char | **5** | strrchr is first and exact. |
| 35 | negative-contrast | `qs haskell sequence effects and discard results not collect them` | `mapM_, not mapM` | 1. mapM_ :: (Foldable t, Monad m) => (a -> m b) -> t a -> m ()<br>2. mapM_ :: (Foldable t, Monad m) => (a -> m b) -> t a -> m ()<br>3. mapM_ :: (Foldable t, Monad m) => (a -> m b) -> t a -> m () | **5** | mapM_ is first and explicitly says results are ignored. |
| 36 | negative-contrast | `qs python shell style splitting not regular expression splitting` | `shlex.split(), not re.split()` | 1. shlex.split | **5** | shlex.split is first and exact. |
| 37 | negative-contrast | `qs go compare strings case insensitively not byte equality` | `strings.EqualFold, not bytes.Equal` | 1. strings.EqualFold | **5** | strings.EqualFold is first and exact. |
| 38 | negative-contrast | `qs rust remove suffix not prefix from string slice` | `str::strip_suffix, not strip_prefix` | 1. str::strip_suffix | **5** | str::strip_suffix is first and exact. |
| 39 | negative-contrast | `qs cpp copy backward not ordinary copy` | `std::copy_backward, not std::copy` | 1. std::ranges::copy_backward, std::ranges::copy_backward_result<br>2. std::copy_backward<br>3. std::ranges::move_backward, std::ranges::move_backward_result | **5** | The copy_backward family leads, with std::copy_backward second. |
| 40 | negative-contrast | `qs c write one character not a formatted string` | `fputc, not fprintf` | 1. fputc | **5** | fputc is first and exact. |
| 41 | negative-contrast | `qs haskell left fold with monadic effects not pure fold` | `Control.Monad.foldM, not foldl` | 1. foldM :: (Foldable t, Monad m) => (b -> a -> m b) -> b -> t a -> m b<br>2. foldM :: Monad m => (a -> b -> m a) -> a -> Stream m b -> m a<br>3. foldM :: forall m a b (v :: Type -> Type). Monad m => (a -> b -> m a) -> a -> Bundle m v b -> m a | **5** | foldM is first and explicitly contrasts with foldl. |
| 42 | negative-contrast | `qs python decimal arithmetic not binary float` | `decimal.Decimal, not float` | 1. decimal — Decimal fixed-point and floating-point arithmetic<br>2. Expressions<br>3. decimal | **5** | The decimal module is first and directly describes decimal arithmetic. |
| 43 | negative-contrast | `qs go split after separator not before it` | `strings.SplitAfter, not strings.Split` | 1. func SplitAfter<br>2. func SplitAfter<br>3. big.Int.Not() | **5** | strings.SplitAfter is first and exact. |
| 44 | negative-contrast | `qs rust weak reference not owning rc clone` | `std::rc::Weak, not Rc` | 1. Reference Cycles Can Leak Memory<br>2. Weak<br>3. Struct Weak | **5** | The first tutorial hit explains Weak for preventing Rc cycles; Weak docs follow. |
| 45 | negative-contrast | `qs cpp shared lock not exclusive mutex lock` | `std::shared_lock, not std::unique_lock` | 1. std::shared_lock<Mutex>::shared_lock<br>2. Standard library header <shared_mutex> (C++14)<br>3. std::shared_lock::mutex | **5** | std::shared_lock results fill the top positions. |
| 46 | negative-contrast | `qs c rename a file not remove it` | `rename, not remove` | 1. rename<br>2. Standard library header <stdio.h><br>3. FE_INEXACT | **5** | rename is first and exact. |
| 47 | negative-contrast | `qs haskell catch only io errors not every exception` | `System.IO.Error.catchIOError, not Control.Exception.catch` | 1. catchIOError :: IO a -> (IOError -> IO a) -> IO a | **5** | catchIOError is the sole hit and exactly limits handling to IOError. |
| 48 | negative-contrast | `qs python compare sequences in constant time not normal equality` | `hmac.compare_digest(), not ==` | 1. hmac.compare_digest | **5** | hmac.compare_digest is first and exact. |
| 49 | negative-contrast | `qs go lexical path cleanup not filesystem symlink resolution` | `filepath.Clean, not EvalSymlinks` | 1. type Cleanup 1.24<br>2. func (*B) Cleanup 1.14<br>3. Package path | **4** | Clean is fourth and exactly describes lexical cleanup; earlier hits are noisy. |
