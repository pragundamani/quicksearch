# Go quicksearch score summary

- Queries: 100
- Mean score: **4.39/5**
- Score distribution: **5: 77, 4: 5, 3: 9, 2: 3, 1: 1, 0: 5**
- Score >=3: **91/100**
- Score >=4: **82/100**
- Command errors/timeouts: **0**

## Failures (score 0)

- `go range loops`: no language `range` clause; results start with `testing.B.Loop` and `big.Int.MulRange`.
- `go embedded fields`: returns the `embed` package instead of embedded struct fields.
- `go methods pointer receivers`: no method-declaration or pointer-receiver documentation.
- `go file permissions`: only unrelated `File.Pos` methods.
- `go worker pool`: returns `sync.Pool` and other object pools, not a goroutine/channel worker-pool pattern.

## Most important low-score regressions

- `go clear map` (2): returns only `sync.Map.Clear`; the built-in `clear` is absent.
- `go http middleware` (2): only a general `net/http` package hit at rank 3; no Handler middleware guidance.
- `go table driven tests` (2): general `testing` package only; no table-driven pattern.
- `go crypto/rand.Read` (1): sole result is an unrelated third-party package that mentions the API.
- Several conceptual queries are only weakly useful (3), notably `generic constraints`, `context cancellation`, `atomic counter`, and `race detector`.
