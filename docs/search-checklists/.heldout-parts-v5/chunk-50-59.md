# Held-out evaluation: manifest indices 50–59

Binary: `/tmp/qs-gate/release/qs`  
Timeout: 30 seconds per command  

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

## Chunk metrics

- Entries: **10** (indices 50–59)
- Commands completed: **10**; timeouts: **0**; nonzero exits: **0**
- Class / label count: **10 negative-contrast / negative**
- Score counts: **5: 1, 4: 1, 3: 3, 2: 1, 1: 0, 0: 4**
- Total score: **20/50**
- Average score: **2.00/5**
