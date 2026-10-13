# Held-out final evaluation: manifest entries 50–99

**Binary:** `/tmp/qs-gate/release/qs`  
**Run:** Fresh process per query with `--json`; 30-second timeout; concurrency limited to 5.  
**Scoring:** 5 expected first/useful; 4 expected top 3/useful; 3 relevant but weak rank/detail; 2 partial; 1 barely useful or misleading; 0 no relevant result/error.

## Results

| ID | Class | Query | Expected | Top results (rank order) | Score | Assessment |
|---:|---|---|---|---|---:|---|
| 50 | negative-contrast | `qs rust checked addition not wrapping addition` | checked_add, not wrapping_add | 1. How to Write Tests<br>2. Test Organization<br>3. Unsafe Rust | **2** | Expected `checked_add` is absent; only `add_with_overflow` appears at rank 4. Rejected `wrapping_add` is not in the top 5. |
| 51 | negative-contrast | `qs cpp midpoint without overflow not simple average` | std::midpoint, not (a+b)/2 | 1. std::midpoint<br>2. std::lerp<br>3. FE_OVERFLOW | **5** | Exact expected API is rank 1. The rejected simple-average expression is not a result. |
| 52 | negative-contrast | `qs c find first matching span not substring` | strpbrk, not strstr | 1. memchr<br>2. strspn<br>3. Type-generic math | **2** | Expected `strpbrk` is absent; related `memchr`, `strspn`, `wcspbrk`, and `strcspn` appear. Rejected `strstr` is not in the top 5. |
| 53 | negative-contrast | `qs haskell strict bytestring not lazy bytestring` | Data.ByteString, not Data.ByteString.Lazy | 1. Bang patterns and Strict Haskell<br>2. Building secure systems (restricted IO Monads)<br>3. Loosening restrictions on class contexts | **0** | No useful strict `Data.ByteString` result. Results are unrelated GHC Guide pages. Rejected lazy module is not in the top 5. |
| 54 | negative-contrast | `qs python cryptographic random integer not random.randrange` | secrets.randbelow(), not random.randrange() | 1. random.randrange() | **0** | Failure: rejected `random.randrange()` is the only result; `secrets.randbelow()` is absent. |
| 55 | negative-contrast | `qs go escape html text not url query data` | html.EscapeString, not url.QueryEscape | 1. func EscapeText 1.1<br>2. func (*Encoder) SetEscapeHTML 1.7<br>3. url.QueryEscape() | **1** | Expected `html.EscapeString` is absent. Rank 1 is XML `EscapeText`; rejected `url.QueryEscape()` is rank 3. |
| 56 | negative-contrast | `qs rust saturating multiplication not checked multiplication` | saturating_mul, not checked_mul | 1. Struct Saturating<br>2. Function saturating_add<br>3. Function saturating_sub | **3** | Rank 1 `Saturating` is relevant but does not directly return `saturating_mul`; the exact expected method is absent. Rejected `checked_mul` is not in the top 5. |
| 57 | negative-contrast | `qs cpp terminate handler not unexpected exception handler` | std::set_terminate, not exception matching | 1. std::terminate_handler<br>2. std::set_terminate<br>3. std::get_terminate | **4** | `std::set_terminate` is rank 2; rank 1 is the directly related handler type. Rejected `unexpected_handler` is rank 5, not top. |
| 58 | negative-contrast | `qs c convert wide string to multibyte not reverse direction` | wcsrtombs, not mbsrtowcs | 1. Null-terminated multibyte strings<br>2. Null-terminated wide strings<br>3. mbtowc | **2** | Expected `wcsrtombs` is rank 5. Rejected reverse conversion `mbsrtowcs` ranks above it at 4. |
| 59 | negative-contrast | `qs haskell deep evaluation not weak head normal form` | Control.DeepSeq.force, not seq | 1. <span class="section-number">6.4.20. </span>Arbitrary-rank polymorphism | **0** | No useful deepseq/`force` result. The only result is unrelated rank-polymorphism documentation; rejected `seq` is not a result. |
| 60 | negative-contrast | `qs python open compressed gzip file not zip archive` | gzip.open(), not zipfile.ZipFile | 1. gzip.open(filename, mode='rb', compresslevel=6, encoding=None, errors=None, newline=None)<br>2. fileinput.hook_compressed(filename, mode, *, encoding=None, errors=None)<br>3. Open | **5** | Exact expected API is rank 1. Rejected `zipfile.ZipFile` is not in the top 5. |
| 61 | negative-contrast | `qs go stable slice sort not unstable sort` | sort.SliceStable, not sort.Slice | 1. func SortStableFunc 1.21<br>2. func SliceStable 1.8<br>3. sort.SliceStable() | **4** | `sort.SliceStable` is rank 2 (and its direct anchor rank 3); rank 1 is a useful stable slices alternative. Rejected unstable `sort.Slice` is not a result. |
| 62 | negative-contrast | `qs rust abort process not unwind panic` | std::process::abort, not panic! | 1. Function abort<br>2. Function abort_immediate<br>3. Function abort | **5** | Exact expected API is rank 1. Rejected `panic!` is not in the top 5. |
| 63 | negative-contrast | `qs cpp inclusive scan not exclusive scan` | std::inclusive_scan, not std::exclusive_scan | 1. std::inclusive_scan<br>2. std::transform_inclusive_scan<br>3. std::exclusive_scan | **5** | Exact expected API is rank 1. Rejected `std::exclusive_scan` is rank 3, but not top. |
| 64 | negative-contrast | `qs c read one wide character not one byte character` | fgetwc, not fgetc | 1. File input/output<br>2. fgetwc, getwc<br>3. char | **4** | `fgetwc` is rank 2 after a general I/O page. Rejected `fgetc` is not in the top 5. |
| 65 | negative-contrast | `qs haskell text builder not string concatenation` | Data.Text.Lazy.Builder, not (++) | 1. Data type contexts<br>2. Inferred context for deriving clauses<br>3. Loosening restrictions on class contexts | **0** | No useful text-builder result. Results are unrelated GHC Guide pages; rejected `(++)` is not a result. |
| 66 | negative-contrast | `qs python parse email address not url address` | email.utils.parseaddr(), not urllib.parse | 1. email.headerregistry: Custom Header Objects<br>2. email.utils: Miscellaneous utilities<br>3. parse | **4** | The correct `email.utils` documentation is rank 2 and is useful for `parseaddr`. Rejected `urllib.parse` is not in the top 5. |
| 67 | negative-contrast | `qs go cryptographic subtle constant time compare not bytes compare` | subtle.ConstantTimeCompare, not bytes.Equal | 1. func ConstantTimeCompare<br>2. func (Time) Compare 1.20<br>3. subtle.ConstantTimeCompare() | **5** | Exact expected function is rank 1 (and direct anchor rank 3). Rejected `bytes.Equal` is absent. |
| 68 | surplus-token-adversarial | `qs rust docs please exact api lookup today std::mem::take examples stable` | std::mem::take | 1. take in std::mem - Rust<br>2. Search Rust standard library for std::mem::take | **5** | Exact official Rust API is rank 1. |
| 69 | surplus-token-adversarial | `qs cpp modern standard library reference quick answer std::ranges::iota_view usage` | std::ranges::iota_view | 1. std::ranges::iota_view<br>2. std::ranges::iota_view::end<br>3. std::ranges::iota_view::size | **5** | Exact C++ API is rank 1. |
| 70 | surplus-token-adversarial | `qs c standard library portable example urgently get environment variable getenv` | getenv | 1. getenv, getenv_s<br>2. Error handling<br>3. gets | **5** | Exact C API is rank 1. |
| 71 | surplus-token-adversarial | `qs haskell hoogle package docs simple example now Data.Set.fromList` | Data.Set.fromList | 1. fromList :: Ord a => [a] -> Set a<br>2. fromList :: Ord a => [a] -> Set a<br>3. fromList :: Ord a => [a] -> OSet a | **5** | Exact Hackage function is rank 1. |
| 72 | surplus-token-adversarial | `qs python official docs latest version quick example contextlib.ExitStack` | contextlib.ExitStack | 1. contextlib.ExitStack<br>2. contextlib.ExitStack.push()<br>3. contextlib.ExitStack.close() | **5** | Exact Python API is rank 1. |
| 73 | surplus-token-adversarial | `qs go package documentation current release example net.IP.IsPrivate` | net.IP.IsPrivate | 1. net.IP.IsPrivate() | **5** | Exact Go method is rank 1. |
| 74 | surplus-token-adversarial | `qs rust beginner tutorial standard docs show me slice rotate_left method now` | slice::rotate_left | 1. Function rotate_left<br>2. Programming a Guessing Game<br>3. Unsafe Rust | **2** | Rank 1 is the different Rust intrinsic `rotate_left`; the requested slice method is not directly returned. Later inherited-method pages are only partial. |
| 75 | surplus-token-adversarial | `qs cpp reference safe portable c++23 example std::byteswap integer please` | std::byteswap | 1. std::byteswap | **5** | Exact C++ API is rank 1. |
| 76 | surplus-token-adversarial | `qs c reference manual safe code sample portable convert time difftime seconds` | difftime | 1. Standard library header <time.h><br>2. difftime<br>3. time | **4** | Exact C API is rank 2 after `<time.h>`. |
| 77 | surplus-token-adversarial | `qs haskell latest base library documentation beginner example Data.Char.generalCategory` | Data.Char.generalCategory | 1. generalCategory :: Char -> GeneralCategory<br>2. data GeneralCategory<br>3. showbGeneralCategory :: GeneralCategory -> Builder | **5** | Exact Hackage function is rank 1. |
| 78 | surplus-token-adversarial | `qs python docs tutorial best practice code sample pathlib.PurePath.match` | PurePath.match() | 1. pathlib.PurePath.match()<br>2. pathlib.PurePath.full_match() | **5** | Exact Python method is rank 1. |
| 79 | surplus-token-adversarial | `qs go official reference beginner quick code example cmp.Or zero values` | cmp.Or | 1. cmp.Or()<br>2. cmp.Ordered | **5** | Exact Go API is rank 1. |
| 80 | surplus-token-adversarial | `qs rust standard library fast safe idiomatic example Vec::spare_capacity_mut docs` | Vec::spare_capacity_mut | 1. Vec::spare_capacity_mut<br>2. Search Rust standard library for Vec::spare_capacity_mut | **5** | Exact official Rust method is rank 1. |
| 81 | surplus-token-adversarial | `qs cpp standard official documentation tutorial std::ranges::fold_left sample code` | std::ranges::fold_left | 1. std::ranges::fold_left<br>2. std::ranges::fold_left_first<br>3. std::ranges::fold_left_with_iter | **5** | Exact C++ API is rank 1. |
| 82 | surplus-token-adversarial | `qs c language library specification easy tutorial sample code div integer quotient remainder` | div | 1. div, ldiv, lldiv, imaxdiv<br>2. Arithmetic operators<br>3. Common mathematical functions | **5** | Exact C function family is rank 1. |
| 83 | surplus-token-adversarial | `qs haskell package docs latest tutorial quick answer Data.Sequence.adjust examples` | Data.Sequence.adjust | 1. adjust :: (a -> a) -> Int -> Seq a -> Seq a<br>2. adjust :: (a -> a) -> Int -> Seq a -> Seq a<br>3. adjust' :: (a -> a) -> Int -> Seq a -> Seq a | **5** | Exact Hackage function is rank 1. |
| 84 | surplus-token-adversarial | `qs python standard library official guide simple example graphlib.TopologicalSorter` | graphlib.TopologicalSorter | 1. graphlib.TopologicalSorter<br>2. graphlib.TopologicalSorter.add()<br>3. graphlib.TopologicalSorter.done() | **5** | Exact Python class is rank 1. |
| 85 | surplus-token-adversarial | `qs go current stable standard package docs quick example math.Nextafter` | math.Nextafter | 1. math.Nextafter() | **5** | Exact Go function is rank 1. |
| 86 | surplus-token-adversarial | `qs rust official stable docs easy example std::path::Path::ancestors iterator` | Path::ancestors | 1. Path::ancestors<br>2. Search Rust standard library for std::path::Path::ancestors | **5** | Exact official Rust method is rank 1. |
| 87 | surplus-token-adversarial | `qs cpp latest standard reference concise example std::launder object pointer` | std::launder | 1. std::launder | **5** | Exact C++ API is rank 1. |
| 88 | surplus-token-adversarial | `qs c standard portable reference example localeconv monetary formatting fields` | localeconv | 1. lconv<br>2. localeconv<br>3. Standard library header <locale.h> | **4** | Exact C API is rank 2; rank 1 `lconv` is also directly useful for the requested fields. |
| 89 | surplus-token-adversarial | `qs haskell current base hoogle quick tutorial Control.Arrow.loop examples` | Control.Arrow.loop | 1. loop :: ArrowLoop a => a (b, d) (c, d) -> a b c<br>2. class Arrow a => ArrowLoop (a :: Type -> Type -> Type)<br>3. loop :: ArrowLoop a => a (b, d) (c, d) -> a b c | **5** | Exact Hackage function is rank 1. |
| 90 | surplus-token-adversarial | `qs python official stdlib current docs tiny example weakref.finalize object cleanup` | weakref.finalize | 1. weakref.finalize<br>2. weakref.finalize.alive<br>3. weakref.finalize.atexit | **5** | Exact Python API is rank 1. |
| 91 | surplus-token-adversarial | `qs go standard package reference practical sample testing.F.Cleanup fuzz docs` | testing.F.Cleanup | 1. testing.F.Cleanup() | **5** | Exact Go method is rank 1. |
| 92 | surplus-token-adversarial | `qs rust docs quick beginner correct idiomatic std::array::from_fn example` | std::array::from_fn | 1. from_fn in std::array - Rust<br>2. Search Rust standard library for std::array::from_fn | **5** | Exact official Rust function is rank 1. |
| 93 | surplus-token-adversarial | `qs cpp portable modern reference sample std::ranges::borrowed_range concept details` | std::ranges::borrowed_range | 1. std::ranges::borrowed_range | **5** | Exact C++ concept is rank 1. |
| 94 | surplus-token-adversarial | `qs c official standard library guide sample code nearbyint rounding mode` | nearbyint | 1. Implementation defined behavior control<br>2. Standard library header <tgmath.h> (C99)<br>3. nearbyint, nearbyintf, nearbyintl | **4** | Exact C API is rank 3, within the useful top three. |
| 95 | surplus-token-adversarial | `qs haskell hoogle official base docs concise example Data.Ord.clamp` | Data.Ord.clamp | 1. clamp :: Ord a => (a, a) -> a -> a<br>2. clamp :: FastMinMax x => x -> x<br>3. clamp :: Ord a => (a, a) -> a -> a | **5** | Exact Hackage function is rank 1. |
| 96 | surplus-token-adversarial | `qs python latest official manual quick practical example socket.socketpair` | socket.socketpair() | 1. socket.socketpair()<br>2. c.PyErr_SetString()<br>3. c.PyConfig_SetString() | **5** | Exact Python function is rank 1. |
| 97 | surplus-token-adversarial | `qs go docs current version best practice example log.Logger.Writer method` | log.Logger.Writer | 1. log.Logger.Writer() | **5** | Exact Go method is rank 1. |
| 98 | surplus-token-adversarial | `qs rust safe fast standard library tutorial std::ffi::OsStr::as_encoded_bytes` | OsStr::as_encoded_bytes | 1. OsStr::as_encoded_bytes<br>2. Search Rust standard library for std::ffi::OsStr::as_encoded_bytes | **5** | Exact official Rust method is rank 1. |
| 99 | surplus-token-adversarial | `qs cpp official c++ reference beginner sample std::unreachable_sentinel_t iterator` | std::unreachable_sentinel_t | 1. std::unreachable_sentinel_t | **5** | Exact C++ type is rank 1. |

## Summary

- Queries: **50**
- Total score: **205/250**
- Mean score: **4.10/5**
- Scores at least 4: **40/50 (80.0%)**
- Exact score distribution: **5: 33, 4: 7, 3: 1, 2: 4, 1: 1, 0: 4**
- Process errors: **0**
- Timeouts: **0**

### By class

| Class | Count | Mean | Score ≥4 |
|---|---:|---:|---:|
| negative-contrast | 18 | 2.83 | 9/18 (50.0%) |
| surplus-token-adversarial | 32 | 4.81 | 31/32 (96.9%) |

### Contrast checks

- A rejected target was rank 1 in **1/18** contrast queries: ID 54 (`random.randrange()`).
- Rejected targets also appeared in the top 3 for ID 55 (`url.QueryEscape`, rank 3) and ID 63 (`std::exclusive_scan`, rank 3).
- Other rejected targets that appeared were ID 58 (`mbsrtowcs`, rank 4) and ID 57 (`unexpected_handler`, rank 5).
- The expected target outranked the rejected target in IDs 57 and 63. It did not in IDs 54, 55, and 58.

Raw captured JSON and timing data: `part2-raw.json`.
