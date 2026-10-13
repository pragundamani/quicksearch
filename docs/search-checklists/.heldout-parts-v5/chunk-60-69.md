# Held-out evaluation: indices 60–69

Binary: `/tmp/qs-gate/release/qs`  
Timeout: 30 seconds per command (all 10 completed with exit code 0)

| Index | Language | Class / label | Command / query | Expected | Score | Evidence / assessment |
|---:|---|---|---|---|---:|---|
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

## Chunk metrics

- Entries: 10
- Score total: 36/50
- Average: **3.60/5**
- Score counts: 5 = 5, 4 = 2, 3 = 1, 2 = 0, 1 = 0, 0 = 2
- Class counts: negative-contrast = 8, surplus-token-adversarial = 2
- Timeout count: 0
- Nonzero-exit count: 0
