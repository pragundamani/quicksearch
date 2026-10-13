# Held-out quicksearch evaluation: indices 80–89

Binary: `/tmp/qs-gate/release/qs`  
Each manifest command was run with only the leading `qs` replaced. Timeout: 30 seconds per command.

| Index | Language | Class / label | Command / query | Expected | Score | Evidence / assessment |
|---:|---|---|---|---|---:|---|
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

## Summary

- Entries: **10**
- Completed: **10**; timeouts: **0**; nonzero exits: **0**
- Labels: **10 positive**, **0 negative contrast**
- Scores: **5 × 9**, **4 × 1**, **3 × 0**, **2 × 0**, **1 × 0**, **0 × 0**
- Average: **4.90 / 5.00**
