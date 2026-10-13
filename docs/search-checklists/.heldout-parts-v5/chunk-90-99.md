# Held-out quicksearch evaluation: indices 90–99

Each command used `/tmp/qs-gate/release/qs` in place of only the leading `qs` and had a strict 30-second timeout.

| Index | Language | Class / label | Executed command | Expected | Score | Evidence / assessment |
|---:|---|---|---|---|---:|---|
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

## Summary

- Entries: 10
- Completed within timeout: 10
- Exit 0: 10
- Score counts: 5 = 9, 4 = 0, 3 = 1, 2 = 0, 1 = 0, 0 = 0
- Average score: **4.80 / 5.00**
- Labels: positive = 10, negative = 0. No negative-contrast rejection checks applied in this chunk.
