# Final held-out evaluation: manifest indices 50–99

## Method

- Candidate: `/tmp/qs-gate/release/qs`
- Manifest: `docs/search-checklists/heldout_manifest.json`
- Range: zero-based indices **50–99**
- Commands: **50/50** run from current output, replacing only the leading `qs`
- Timeout: **30 seconds per command**
- Scoring: 5 = expected first/immediately useful; 4 = expected in top 3/useful; 3 = relevant but ranking/detail weak; 2 = partial; 1 = misleading/barely useful; 0 = error or no relevant result
- Negative contrasts: each rejected target was checked against the top result. Any violation is stated in the row evidence.

## Summary

- Total score: **199/250**
- Average: **3.98/5**
- Score ≥3: **42/50 (84%)**
- Score ≥4: **37/50 (74%)**
- Errors: **0**
- Timeouts: **0**
- Nonzero exits: **0**
- Distribution: **0: 6, 1: 0, 2: 2, 3: 5, 4: 5, 5: 32**

| Range | Average | ≥3 | ≥4 | Distribution (0–5) |
|---|---:|---:|---:|---|
| 50–59 | 2.00 | 5/10 | 2/10 | 0:4, 1:0, 2:1, 3:3, 4:1, 5:1 |
| 60–69 | 3.60 | 8/10 | 7/10 | 0:2, 1:0, 2:0, 3:1, 4:2, 5:5 |
| 70–79 | 4.60 | 9/10 | 9/10 | 0:0, 1:0, 2:1, 3:0, 4:1, 5:8 |
| 80–89 | 4.90 | 10/10 | 10/10 | 0:0, 1:0, 2:0, 3:0, 4:1, 5:9 |
| 90–99 | 4.80 | 10/10 | 9/10 | 0:0, 1:0, 2:0, 3:1, 4:0, 5:9 |

## Results

| Index | Language | Class / label | Command / query | Expected | Score | Evidence / assessment |
|---:|---|---|---|---|---:|---|
| 50 | rust | negative-contrast / negative | `qs rust checked addition not wrapping addition` | `checked_add`, not `wrapping_add` | 0 | No `checked_add` result; top five are unrelated. `wrapping_add` is not top. |
| 51 | cpp | negative-contrast / negative | `qs cpp midpoint without overflow not simple average` | `std::midpoint`, not `(a+b)/2` | 5 | `std::midpoint` is result 1 with its overflow-safe purpose; the rejected simple average is not top. |
| 52 | c | negative-contrast / negative | `qs c find first matching span not substring` | `strpbrk`, not `strstr` | 3 | Result 2 (`strspn`) mentions `strpbrk` and its correct purpose, but there is no direct `strpbrk` result. `strstr` is not top. |
| 53 | haskell | negative-contrast / negative | `qs haskell strict bytestring not lazy bytestring` | `Data.ByteString`, not `Data.ByteString.Lazy` | 0 | Results are unrelated GHC guide pages; neither desired module nor rejected lazy module appears. |
| 54 | python | negative-contrast / negative | `qs python cryptographic random integer not random.randrange` | `secrets.randbelow()`, not `random.randrange()` | 0 | Only result is the explicitly rejected `random.randrange()`; `secrets.randbelow()` is absent. |
| 55 | go | negative-contrast / negative | `qs go escape html text not url query data` | `html.EscapeString`, not `url.QueryEscape` | 3 | The relevant `Package html` is result 2, but `EscapeString` is not shown; rejected `url.QueryEscape` is result 3, not top. |
| 56 | rust | negative-contrast / negative | `qs rust saturating multiplication not checked multiplication` | `saturating_mul`, not `checked_mul` | 2 | Only a generic Rust std search link is returned; no concrete `saturating_mul` item. `checked_mul` is not a result. |
| 57 | cpp | negative-contrast / negative | `qs cpp terminate handler not unexpected exception handler` | `std::set_terminate`, not exception matching | 4 | `std::set_terminate` is result 2 and result 1 directly explains it. The rejected unexpected-handler item is result 5, not top. |
| 58 | c | negative-contrast / negative | `qs c convert wide string to multibyte not reverse direction` | `wcsrtombs`, not `mbsrtowcs` | 3 | Desired `wcsrtombs` is result 5; reverse `mbsrtowcs` ranks above it at 4, though it is not top. |
| 59 | haskell | negative-contrast / negative | `qs haskell deep evaluation not weak head normal form` | `Control.DeepSeq.force`, not `seq` | 0 | Only an unrelated rank-polymorphism page appears; no `Control.DeepSeq.force` result. Rejected `seq` is not top. |
| 60 | python | negative-contrast / negative | `qs python open compressed gzip file not zip archive` | `gzip.open(), not zipfile.ZipFile` | 5 | `gzip.open` is result 1 with the exact purpose. `zipfile.ZipFile` is not a top result. |
| 61 | go | negative-contrast / negative | `qs go stable slice sort not unstable sort` | `sort.SliceStable, not sort.Slice` | 4 | Modern `slices.SortStableFunc` is result 1; exact `sort.SliceStable` is results 2 and 3. Unstable `sort.Slice` is not a top result. |
| 62 | rust | negative-contrast / negative | `qs rust abort process not unwind panic` | `std::process::abort, not panic!` | 0 | No `std::process::abort` result appears. Results are mostly unrelated Rust Book/Reference pages; `panic!` is not promoted, but the target is missing. |
| 63 | cpp | negative-contrast / negative | `qs cpp inclusive scan not exclusive scan` | `std::inclusive_scan, not std::exclusive_scan` | 5 | Exact `std::inclusive_scan` is result 1. Rejected `std::exclusive_scan` is only result 3. |
| 64 | c | negative-contrast / negative | `qs c read one wide character not one byte character` | `fgetwc, not fgetc` | 5 | `fgetwc, getwc` is result 1 and its snippet directly says it reads the next wide character. `fgetc` is not a top result. |
| 65 | haskell | negative-contrast / negative | `qs haskell text builder not string concatenation` | `Data.Text.Lazy.Builder, not (++)` | 0 | Search is routed to the GHC User’s Guide and returns unrelated context pages. No `Data.Text.Lazy.Builder` result appears; `(++)` is also not promoted. |
| 66 | python | negative-contrast / negative | `qs python parse email address not url address` | `email.utils.parseaddr(), not urllib.parse` | 3 | Exact `email.utils.parseaddr` appears only at result 4. Earlier email results are related but less direct. `urllib.parse` is not a top result. |
| 67 | go | negative-contrast / negative | `qs go cryptographic subtle constant time compare not bytes compare` | `subtle.ConstantTimeCompare, not bytes.Equal` | 4 | Exact `subtle.ConstantTimeCompare()` is result 3 with a useful constant-time description. `bytes.Equal` is not a top result. |
| 68 | rust | surplus-token-adversarial / positive | `qs rust docs please exact api lookup today std::mem::take examples stable` | `std::mem::take` | 5 | Exact `take in std::mem` documentation is result 1 with a concise behavior summary. |
| 69 | cpp | surplus-token-adversarial / positive | `qs cpp modern standard library reference quick answer std::ranges::iota_view usage` | `std::ranges::iota_view` | 5 | Exact `std::ranges::iota_view` is result 1 and includes useful code examples. |
| 70 | C | `surplus-token-adversarial` / positive | `qs c standard library portable example urgently get environment variable getenv` | `getenv` | 5 | Rank 1 is **getenv, getenv_s**, with the exact C reference and signature. |
| 71 | Haskell | `surplus-token-adversarial` / positive | `qs haskell hoogle package docs simple example now Data.Set.fromList` | `Data.Set.fromList` | 5 | Rank 1 is `fromList` in `containers` / `Data.Set`, with its type and description. |
| 72 | Python | `surplus-token-adversarial` / positive | `qs python official docs latest version quick example contextlib.ExitStack` | `contextlib.ExitStack` | 5 | Rank 1 is `contextlib.ExitStack`, including a useful code example. |
| 73 | Go | `surplus-token-adversarial` / positive | `qs go package documentation current release example net.IP.IsPrivate` | `net.IP.IsPrivate` | 5 | Rank 1 is `net.IP.IsPrivate()`, with the API description and example. |
| 74 | Rust | `surplus-token-adversarial` / positive | `qs rust beginner tutorial standard docs show me slice rotate_left method now` | `slice::rotate_left` | 2 | The target is absent. Results cover slices generally, but none documents `rotate_left`; the best results are only partial context. |
| 75 | C++ | `surplus-token-adversarial` / positive | `qs cpp reference safe portable c++23 example std::byteswap integer please` | `std::byteswap` | 5 | Rank 1 is `std::byteswap`, with the C++23 signature and a full example. |
| 76 | C | `surplus-token-adversarial` / positive | `qs c reference manual safe code sample portable convert time difftime seconds` | `difftime` | 4 | Exact `difftime` page is rank 2; rank 1 is the useful `<time.h>` overview and explicitly lists `difftime`. |
| 77 | Haskell | `surplus-token-adversarial` / positive | `qs haskell latest base library documentation beginner example Data.Char.generalCategory` | `Data.Char.generalCategory` | 5 | Rank 1 is `generalCategory` in `base` / `Data.Char`, with type, description, and examples. |
| 78 | Python | `surplus-token-adversarial` / positive | `qs python docs tutorial best practice code sample pathlib.PurePath.match` | `PurePath.match()` | 5 | Rank 1 is `pathlib.PurePath.match()`, with exact behavior and examples. |
| 79 | Go | `surplus-token-adversarial` / positive | `qs go official reference beginner quick code example cmp.Or zero values` | `cmp.Or` | 5 | Rank 1 is `cmp.Or()`, with the generic signature, zero-value behavior, and example. |
| 80 | rust | surplus-token-adversarial / positive | `qs rust standard library fast safe idiomatic example Vec::spare_capacity_mut docs` | `Vec::spare_capacity_mut` | 5 | Exit 0. Exact `Vec::spare_capacity_mut` is result 1 with the Rust std method URL; result 2 is the std search. |
| 81 | cpp | surplus-token-adversarial / positive | `qs cpp standard official documentation tutorial std::ranges::fold_left sample code` | `std::ranges::fold_left` | 5 | Exit 0. Exact `std::ranges::fold_left` is result 1 with synopsis and a substantial code example. |
| 82 | c | surplus-token-adversarial / positive | `qs c language library specification easy tutorial sample code div integer quotient remainder` | `div` | 5 | Exit 0. `div, ldiv, lldiv, imaxdiv` is result 1; snippet states it computes quotient and remainder. |
| 83 | haskell | surplus-token-adversarial / positive | `qs haskell package docs latest tutorial quick answer Data.Sequence.adjust examples` | `Data.Sequence.adjust` | 5 | Exit 0. Exact `Data.Sequence.adjust` API is result 1 with type signature, behavior, and performance warning. |
| 84 | python | surplus-token-adversarial / positive | `qs python standard library official guide simple example graphlib.TopologicalSorter` | `graphlib.TopologicalSorter` | 5 | Exit 0. Exact `graphlib.TopologicalSorter` is result 1 with a clear description and runnable-style example. |
| 85 | go | surplus-token-adversarial / positive | `qs go current stable standard package docs quick example math.Nextafter` | `math.Nextafter` | 5 | Exit 0. Exact `math.Nextafter()` is the sole result, with signature, behavior, and edge cases. |
| 86 | rust | surplus-token-adversarial / positive | `qs rust official stable docs easy example std::path::Path::ancestors iterator` | `Path::ancestors` | 5 | Exit 0. Exact `Path::ancestors` is result 1 with the official Rust std method URL; description is generic but direct. |
| 87 | cpp | surplus-token-adversarial / positive | `qs cpp latest standard reference concise example std::launder object pointer` | `std::launder` | 5 | Exit 0. Exact `std::launder` is result 1 with signature, explanation, and a detailed code example. |
| 88 | c | surplus-token-adversarial / positive | `qs c standard portable reference example localeconv monetary formatting fields` | `localeconv` | 4 | Exit 0. Exact `localeconv` is result 2; result 1 is the closely related returned `lconv` fields, so useful but not exact-first. |
| 89 | haskell | surplus-token-adversarial / positive | `qs haskell current base hoogle quick tutorial Control.Arrow.loop examples` | `Control.Arrow.loop` | 5 | Exit 0. Exact `Control.Arrow.loop` API is result 1 from `base`, with signature and diagram; related `ArrowLoop` is second. |
| 90 | python | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs python official stdlib current docs tiny example weakref.finalize object cleanup` | `weakref.finalize` | 5 | Exit 0. `weakref.finalize` is result 1; definition directly explains garbage-collection cleanup. |
| 91 | go | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs go standard package reference practical sample testing.F.Cleanup fuzz docs` | `testing.F.Cleanup` | 5 | Exit 0. `testing.F.Cleanup()` is the only result and includes its cleanup-order behavior. |
| 92 | rust | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs rust docs quick beginner correct idiomatic std::array::from_fn example` | `std::array::from_fn` | 5 | Exit 0. `from_fn in std::array` is result 1 with the correct official Rust page and a useful summary. |
| 93 | cpp | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs cpp portable modern reference sample std::ranges::borrowed_range concept details` | `std::ranges::borrowed_range` | 5 | Exit 0. `std::ranges::borrowed_range` is result 1 with definition and a complete example. |
| 94 | c | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs c official standard library guide sample code nearbyint rounding mode` | `nearbyint` | 3 | Exit 0. `nearbyint, nearbyintf, nearbyintl` is relevant but ranks 4th, below rounding-mode and other rounding pages. |
| 95 | haskell | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs haskell hoogle official base docs concise example Data.Ord.clamp` | `Data.Ord.clamp` | 5 | Exit 0. Base `Data.Ord.clamp` is result 1 with signature, semantics, and definition. |
| 96 | python | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs python latest official manual quick practical example socket.socketpair` | `socket.socketpair()` | 5 | Exit 0. `socket.socketpair()` is result 1 with its behavior and defaults. |
| 97 | go | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs go docs current version best practice example log.Logger.Writer method` | `log.Logger.Writer` | 5 | Exit 0. `log.Logger.Writer()` is the only result and gives the exact return behavior. |
| 98 | rust | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs rust safe fast standard library tutorial std::ffi::OsStr::as_encoded_bytes` | `OsStr::as_encoded_bytes` | 5 | Exit 0. `OsStr::as_encoded_bytes` is result 1 on the official method page; snippet is brief but immediately useful. |
| 99 | cpp | surplus-token-adversarial / positive | `/tmp/qs-gate/release/qs cpp official c++ reference beginner sample std::unreachable_sentinel_t iterator` | `std::unreachable_sentinel_t` | 5 | Exit 0. `std::unreachable_sentinel_t` is result 1 with definition and a complete example. |
