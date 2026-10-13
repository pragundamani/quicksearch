# Held-out evaluation v8 — part 2 (indexes 50–99)

- Cases: 50
- Total score: 223/250
- Mean score: 4.46/5
- Score distribution: 5=39, 4=5, 3=2, 2=0, 1=2, 0=2
- Execution errors: 0

## By language

| Language | Cases | Mean |
|---|---:|---:|
| rust | 9 | 4.78 |
| cpp | 9 | 4.89 |
| c | 8 | 4.12 |
| haskell | 8 | 3.12 |
| python | 8 | 4.88 |
| go | 8 | 4.88 |

## By class

| Class | Cases | Mean |
|---|---:|---:|
| negative-contrast | 18 | 3.78 |
| surplus-token-adversarial | 32 | 4.84 |

## Key failures

- **53 (0):** Haskell strict `Data.ByteString` was absent. Results were unrelated `foldMap` and `ST` APIs.
- **65 (0):** `Data.Text.Lazy.Builder` was absent. Results focused on `String` and unrelated APIs.
- **59 (1):** `Control.DeepSeq.force` was absent. Results instead recommended weak-head evaluation (`evaluate` and `seq`), contrary to the query.
- **82 (1):** C `div` was absent. The only hit was floating-point `remainder`, which does not answer the request.
- **56 (3):** `saturating_mul` was absent. A general `Saturating` wrapper was the best partial match.
- **58 (3):** `wcsrtombs` was fifth, below broad results and the reverse-direction `mbsrtowcs`.

Exact targets appeared second for indexes 57, 61, 64, 66, and 77; each scored 4.
