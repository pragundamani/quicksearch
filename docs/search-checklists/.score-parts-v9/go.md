# Go v9 regression summary

- Candidate: `/tmp/qs-gate/release/qs`
- Checklist: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/go.md`
- Commands: **100**
- Successful executions: **100/100**
- Timeouts/errors: **0**
- Score: **484/500 (96.8%)**
- Mean: **4.84/5.00**
- Distribution: **5: 88, 4: 8, 3: 4, 2: 0, 1: 0, 0: 0**

## Results below 5

| # | Query | Score | First result | Assessment |
|---:|---|---:|---|---|
| 12 | `qs go panic and recover` | 4 | `func panic` | `panic` is first and `recover` appears in the second snippet, but there is no direct `recover` result. |
| 18 | `qs go interface embedding` | 4 | `interfaces` | The authoritative interface-types section is first and useful, but the result title/snippet does not expose embedding or type sets. |
| 30 | `qs go nil` | 3 | `types.Nil` | The first result is `go/types.Nil`, which describes the predeclared value, but the language-level `nil` documentation is not returned. |
| 42 | `qs go regexp named groups` | 4 | `func (*Regexp) CapNames` | Named-capture APIs (`CapNames` and `SubexpNames`) are in the top three, but the capture-group syntax itself is not shown directly. |
| 51 | `qs go container/list.New` | 4 | `Package list` | The exact `list.New` function is third; the package page is first. |
| 76 | `qs go atomic counter` | 3 | `func ClearCounters 1.20` | The `sync/atomic` package is only fourth, behind coverage, testing, and expvar results; no counter operation is surfaced directly. |
| 77 | `qs go worker pool` | 4 | `type Pool 1.3` | The exact worker-pool guide is third; unrelated `Pool` types occupy the first two positions. |
| 81 | `qs go http client timeout` | 4 | `Clients and Transports` | The `http.Client` type is third and the package overview is first, but the `Timeout` field is not surfaced directly. |
| 87 | `qs go encoding/json.Unmarshal` | 3 | `func (*Value) UnmarshalJSON 1.27` | The exact `json.Unmarshal` function is absent. The broad `encoding/json` package page is third, after unrelated `UnmarshalJSON` methods. |
| 90 | `qs go base64.StdEncoding` | 4 | `Package base64` | The base64 package page is first and examples use `StdEncoding`, but the exact exported variable is absent. |
| 93 | `qs go crypto/rand.Read` | 4 | `func (*Rand) Read 1.6` | The exact cryptographic `Read` documentation is second, behind `math/rand.(*Rand).Read`. |
| 96 | `qs go log/slog.Logger` | 3 | `slog.Logger.Log()` | Results contain `slog.Logger` methods, but the requested `Logger` type overview is absent and `NewLogLogger` is a different logger type. |

## Key weak results

- `qs go nil`: Returns `go/types.Nil`, not language-level documentation for the predeclared identifier.
- `qs go atomic counter`: The relevant `sync/atomic` package is fourth. No counter API is shown directly.
- `qs go encoding/json.Unmarshal`: The exact function is missing. The package page is third behind unrelated methods.
- `qs go log/slog.Logger`: Returns logger methods, but not the requested type overview.

No command failed, timed out, or scored below 3.

Full per-command outputs, scores, and rationales are in `go.json`.
