# Go v8 regression score

- Commands run: 100/100
- Successful JSON responses: 100/100
- Mean score: 4.70/5.00
- Total score: 470/500
- Score 5: 83
- Score 4: 7
- Score 3: 8
- Score 2: 1
- Score 1: 1
- Score 0: 0
- Scores 4-5: 90/100

## Non-5 results

- **4** `go delete map key` — Built-in delete is third with a useful signature; the first result is exact in name but has only a generic snippet.
- **3** `go panic and recover` — The first result documents panic, but recover has no direct result; only the panic/recover interaction appears in context.
- **3** `go interface embedding` — Results are go/types inspection methods, not the language-spec discussion of embedded interfaces and type sets.
- **3** `go generic constraints` — The first result is TypeParam.Constraint; the results do not explain constraint interfaces or type sets.
- **3** `go fmt.Errorf` — fmt.Errorf is first, but the output does not mention %w or error wrapping.
- **4** `go container/list.New` — The exact list.New result is third with its signature and purpose; rank 2 is the unrelated ring.New.
- **2** `go temporary directory` — The useful result is the older TempDir function at rank 2; os.MkdirTemp is absent and rank 1 is Error.Temporary.
- **3** `go context cancellation` — The first result explains Context and cancellation generally, but WithCancel and CancelFunc are absent.
- **4** `go atomic counter` — A useful atomic.Int64 counter example is second; the sync/atomic package result itself is fourth.
- **3** `go worker pool` — The worker-pool result is third, but its snippet is generic and the first two results are unrelated Pool APIs.
- **1** `go race detector` — The first result is the internal runtime/race package; no useful data-race detector or go test -race guidance appears.
- **4** `go http client timeout` — The full http.Client result is third and includes the Timeout field documentation.
- **3** `go http middleware` — The first result points to http.Handler, but gives no middleware pattern or implementation context.
- **4** `go encoding/json.Unmarshal` — The encoding/json package result is third and includes useful Unmarshal context; the first two are unrelated UnmarshalJSON methods.
- **4** `go base64.StdEncoding` — StdEncoding appears in a useful example at rank 2, but no direct StdEncoding result is returned.
- **4** `go crypto/rand.Read` — The cryptographically secure Read function is second with useful context; rank 1 is math/rand.Rand.Read.
- **3** `go log/slog.Logger` — The first result is Logger.Log rather than the Logger type; results give related methods but no Logger type overview.

## Key failures

- `go race detector` scored 1. It returned `Package race` first but no practical race-detector or `go test -race` guidance.
- `go temporary directory` scored 2. It missed `os.MkdirTemp`; rank 1 was `(*Error).Temporary` and rank 2 was the older `TempDir` API.
- Generic-constraint, interface-embedding, context-cancellation, HTTP-middleware, and `log/slog.Logger` queries returned related APIs instead of the requested concept or type overview.
