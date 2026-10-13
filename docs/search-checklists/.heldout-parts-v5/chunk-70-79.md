# Held-out quicksearch evaluation: indices 70–79

Binary: `/tmp/qs-gate/release/qs`  
Timeout: 30 seconds per command  
All 10 commands exited successfully before the timeout.

| Index | Language | Class / label | Command / query | Expected | Score | Evidence / assessment |
|---:|---|---|---|---|---:|---|
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

## Chunk summary

- Entries: 10
- Labels: 10 positive, 0 negative
- Negative contrasts checked: none in this chunk
- Score counts: 5 → 8, 4 → 1, 3 → 0, 2 → 1, 1 → 0, 0 → 0
- Total score: 46 / 50
- Average score: **4.60 / 5.00**
