# Go quicksearch score report

## Summary

All 100 checklist commands were run through the installed `qs` executable. Each command had a 15-second timeout.

- **Average:** 3.61/5
- **Median:** 5/5
- **Distribution:** 0: 6, 1: 16, 2: 9, 3: 5, 4: 8, 5: 56
- **Pass rate >=3:** 69/100 (69%)
- **Pass rate >=4:** 64/100 (64%)

## Results

| Command | Score | Observed top result or error | Assessment |
|---|---:|---|---|
| `qs go arrays` | 2 | #1 types.Array | Wrong abstraction: `types.Array` is a compiler type object, not array syntax or semantics. |
| `qs go slices` | 3 | #1 slices | `slices` is useful for operations, but it does not explain the slice type itself. |
| `qs go append` | 3 | #1 fmt.Append() | The built-in appears only at #4; three unrelated `Append` APIs come first. |
| `qs go copy builtin` | 0 | Timeout after 15 s; no output | Timed out after 15 s with no output. |
| `qs go maps` | 3 | #1 maps | Useful `maps` package operations, but no map type language documentation. |
| `qs go delete map key` | 1 | #1 types.Map.Key() | No built-in `delete`; #1 is `types.Map.Key()` and is not a deletion API. |
| `qs go clear map` | 2 | #1 sync.Map.Clear() | `sync.Map.Clear()` is related, but it is not the built-in `clear` for ordinary maps. |
| `qs go range loops` | 1 | #1 testing.B.Loop() | Only `testing.B.Loop()` appears; it does not explain `for ... range`. |
| `qs go variadic functions` | 5 | #1 Go by Example: Variadic Functions | Exact tutorial is #1 and immediately explains variadic arguments with examples. |
| `qs go multiple return values` | 1 | #1 url.Values | Results are unrelated value containers such as `url.Values`; multiple returns are absent. |
| `qs go defer statement` | 5 | #1 Defer, Panic, and Recover - The Go Programming Language | Official Go defer/panic/recover article is #1 and directly useful. |
| `qs go panic and recover` | 4 | #1 builtin.recover() | `builtin.recover()` is #1 with good panic/defer context, but `panic` itself is missing. |
| `qs go goroutines` | 2 | #1 runtime.NumGoroutine() | Runtime goroutine inspection APIs are related, but there is no `go` statement guidance. |
| `qs go buffered channels` | 0 | Error 1: qs: no web results for go buffered channels: https://lite.duckduckgo.com/lite/?q=go%20buffered%20channels | Error: "no web results for go buffered channels". |
| `qs go select statement` | 0 | Error 1: qs: no web results for go select statement: https://lite.duckduckgo.com/lite/?q=go%20select%20statement | Error: "no web results for go select statement". |
| `qs go close channel` | 0 | Error 1: qs: no web results for go close channel: https://lite.duckduckgo.com/lite/?q=go%20close%20channel | Error: "no web results for go close channel". |
| `qs go interfaces` | 2 | #1 net.Interfaces() | Network interfaces dominate; `types.Interface` is only #5 and lacks language semantics. |
| `qs go interface embedding` | 3 | #1 types.Interface.Embedded() | Compiler APIs for embedded interface types are top-ranked; useful but no type-set explanation. |
| `qs go type assertions` | 2 | #1 runtime.TypeAssertionError | `runtime.TypeAssertionError` is related, but no assertion syntax or comma-ok example is shown. |
| `qs go type switches` | 4 | #1 ast.TypeSwitchStmt | `ast.TypeSwitchStmt` is #1 and exposes `y.(type)`, but this is AST detail rather than user guidance. |
| `qs go struct tags` | 1 | #1 types.HasherIgnoreTags | Results only discuss ignoring tags; struct-tag syntax and meaning are absent. |
| `qs go embedded fields` | 2 | #1 bytes.Fields() | `bytes.Fields` and `strings.Fields` are false matches; `ast.Field` at #3 has no embedding detail. |
| `qs go methods pointer receivers` | 1 | #1 gosym.Sym.ReceiverName() | Only `gosym.Sym.ReceiverName()` appears; pointer-receiver declaration semantics are absent. |
| `qs go iota constants` | 1 | #1 io constants | Generic package constants are returned; `iota` is not explained or shown. |
| `qs go type parameters` | 1 | #1 dsa.Parameters | Crypto `Parameters` types are false matches; generic type parameters are absent. |
| `qs go generic constraints` | 1 | #1 constraint.Expr | Build-tag `constraint.Expr` is the wrong kind of constraint. |
| `qs go comparable` | 5 | #1 builtin.comparable | Exact `builtin.comparable` is #1 with a clear constraint description. |
| `qs go new` | 1 | #1 v2.New() | Generic `New` functions crowd out the built-in; `builtin.new` is absent. |
| `qs go make` | 4 | #1 weak.Make() | `builtin.make()` is #3, but it has no displayed signature or usage context. |
| `qs go nil` | 4 | #1 types.Nil | `types.Nil` is #1 and identifies the predeclared value, though language semantics are thin. |
| `qs go fmt.Sprintf` | 5 | #1 fmt.Sprintf() | Exact function is #1 with signature and a short working example. |
| `qs go fmt.Errorf` | 4 | #1 fmt.Errorf() | Exact function is #1 with an example, but the key `%w` wrapping behavior is not shown. |
| `qs go fmt.Fscan` | 5 | #1 fmt.Fscan() | Exact function is #1 with full signature and scanning behavior. |
| `qs go strings.Builder` | 5 | #1 strings.Builder | Exact type is #1 with clear zero-value and usage context. |
| `qs go strings.Fields` | 5 | #1 strings.Fields() | Exact function is #1 with full signature and behavior. |
| `qs go strings.ReplaceAll` | 5 | #1 strings.ReplaceAll() | Exact function is #1 with full signature and edge-case context. |
| `qs go bytes.Buffer` | 5 | #1 bytes.Buffer | Exact type is #1 with clear purpose and zero-value behavior. |
| `qs go bytes.Equal` | 5 | #1 bytes.Equal() | Exact function is #1 with signature, semantics, and examples. |
| `qs go strconv.Atoi` | 5 | #1 strconv.Atoi() | Exact conversion helper is #1 with signature and example. |
| `qs go strconv.FormatInt` | 5 | #1 strconv.FormatInt() | Exact formatter is #1 with signature and base examples. |
| `qs go regexp.Compile` | 5 | #1 regexp.Compile() | Exact constructor is #1 with signature and useful `MustCompile` follow-up. |
| `qs go regexp named groups` | 1 | #1 os.Getgroups() | Group-related OS APIs are false matches; regexp named captures are absent. |
| `qs go unicode.IsLetter` | 5 | #1 unicode.IsLetter() | Exact classifier is #1 with signature and examples. |
| `qs go sort.Slice` | 5 | #1 sort.Slice() | Exact sorter is #1 with signature and a practical comparator example. |
| `qs go sort.Search` | 5 | #1 sort.Search() | Exact binary-search function is #1 with detailed contract and example. |
| `qs go slices.Sort` | 5 | #1 slices.Sort() | Exact generic sorter is #1 with signature and example. |
| `qs go slices.Contains` | 5 | #1 slices.Contains() | Exact membership helper is #1 with signature and example. |
| `qs go maps.Clone` | 5 | #1 maps.Clone() | Exact clone helper is #1 with a useful shallow-copy explanation and example. |
| `qs go maps.Equal` | 5 | #1 maps.Equal() | Exact equality helper is #1 with signature and example. |
| `qs go container/heap` | 0 | Timeout after 15 s; no output | Timed out after 15 s with no output. |
| `qs go container/list.New` | 5 | #1 list.New() | Exact constructor is #1 with its signature; unrelated `New` APIs follow. |
| `qs go io.Copy` | 5 | #1 io.Copy() | Exact copier is #1 with signature and a complete example. |
| `qs go io.ReadAll` | 5 | #1 io.ReadAll() | Exact reader helper is #1 with signature and EOF semantics. |
| `qs go io.Reader` | 5 | #1 io.Reader | Exact interface is #1 and includes the full `Read` method signature. |
| `qs go bufio.Scanner` | 5 | #1 bufio.Scanner | Exact type is #1 with a clear operational description. |
| `qs go bufio.NewWriter` | 5 | #1 bufio.NewWriter() | Exact constructor is #1 with signature and buffering behavior. |
| `qs go os.ReadFile` | 5 | #1 os.ReadFile() | Exact file helper is #1 with signature, error behavior, and example. |
| `qs go os.WriteFile` | 5 | #1 os.WriteFile() | Exact file helper is #1 with an immediately useful write example. |
| `qs go os.OpenFile flags` | 1 | #1 net.Flags | Flag APIs are false matches; `os.OpenFile` and its flags are absent. |
| `qs go filepath.WalkDir` | 5 | #1 filepath.WalkDir() | Exact walker is #1 with signature and traversal behavior. |
| `qs go filepath.Join` | 5 | #1 filepath.Join() | Exact path joiner is #1 with signature, platform semantics, and example. |
| `qs go file permissions` | 2 | #1 os.IsPermission() | `os.IsPermission()` concerns errors, not `fs.FileMode` permission bits. |
| `qs go temporary directory` | 1 | #1 pe.DataDirectory | PE directory structures are unrelated; `os.MkdirTemp` is absent. |
| `qs go embed files` | 3 | #1 embed.FS.ReadFile() | `embed.FS.ReadFile()` is relevant, but package setup and `//go:embed` are missing. |
| `qs go time.Parse layout` | 1 | #1 structs.HostLayout | Only unrelated `structs.HostLayout` is returned; `time.Parse` is absent. |
| `qs go time.NewTicker` | 5 | #1 time.NewTicker() | Exact ticker constructor is #1 with signature, timing behavior, and example. |
| `qs go time.AfterFunc` | 5 | #1 time.AfterFunc() | Exact timer helper is #1 with signature and cancellation context. |
| `qs go context cancellation` | 4 | #1 Go Context Tutorial: Cancellation and Deadlines | Top results are useful cancellation guides; exact `WithCancel` detail is not in the visible snippets. |
| `qs go context.WithTimeout` | 5 | #1 context.WithTimeout() | Exact timeout helper is #1 with signature and a cancellation example. |
| `qs go errors.Is` | 5 | #1 errors.Is() | Exact error matcher is #1 with signature, tree semantics, and example. |
| `qs go errors.As` | 5 | #1 errors.As() | Exact error extractor is #1 with signature and example. |
| `qs go errors.Join` | 5 | #1 errors.Join() | Exact joiner is #1 with signature and nil/format behavior. |
| `qs go sync.WaitGroup` | 5 | #1 sync.WaitGroup | Exact synchronization type is #1 with a current, useful example. |
| `qs go mutexes` | 5 | #1 sync.Mutex | `sync.Mutex` is #1 and clearly states zero-value behavior. |
| `qs go sync.Once` | 5 | #1 sync.Once | Exact once type is #1 with its `Do` method immediately below. |
| `qs go atomic counter` | 1 | #1 coverage.ClearCounters() | Coverage counters are false matches; `sync/atomic` operations are absent. |
| `qs go worker pool` | 1 | #1 sync.Pool | `sync.Pool` is not a goroutine worker pool; no worker pattern appears. |
| `qs go race detector` | 5 | #1 Data Race Detector - The Go Programming Language | Official Go race-detector guide is #1 with usage and examples. |
| `qs go net.DialTimeout` | 5 | #1 net.DialTimeout() | Exact dialer is #1 with signature and timeout semantics. |
| `qs go url.Parse` | 5 | #1 url.Parse() | Exact parser is #1 with signature and relative-URL context. |
| `qs go http client timeout` | 2 | #1 os.IsTimeout() | Timeout-related APIs appear, but `http.Client.Timeout` is absent. |
| `qs go http.NewRequestWithContext` | 5 | #1 http.NewRequestWithContext() | Exact request constructor is #1 with its full signature and context behavior. |
| `qs go http.HandleFunc` | 5 | #1 http.HandleFunc() | Exact handler registration API is #1 with signature and example. |
| `qs go http middleware` | 5 | #1 Making and Using HTTP Middleware in Go - Alex Edwards | Practical HTTP middleware guide is #1 and directly addresses the pattern. |
| `qs go httptest.NewServer` | 5 | #1 httptest.NewServer() | Exact test server is #1 with signature and startup behavior. |
| `qs go json_marshal` | 5 | #1 json.Marshal() | Underscore normalization works: `json.Marshal` is #1 with signature and tag example. |
| `qs go encoding/json.Unmarshal` | 5 | #1 json.Unmarshal() | Slash-qualified query resolves exactly to `json.Unmarshal` with signature and example. |
| `qs go omit empty json field` | 1 | #1 ast.Field | `ast.Field` is a false match; no `omitempty` option or JSON tag example appears. |
| `qs go xml decoder` | 5 | #1 xml.Decoder | `xml.Decoder` is #1 with useful parser configuration detail. |
| `qs go base64.StdEncoding` | 4 | #1 base64.Encoding | `base64.Encoding` is #1 and package result #2 has a `StdEncoding` round-trip example. |
| `qs go hex.DecodeString` | 5 | #1 hex.DecodeString() | Exact decoder is #1 with signature and a complete example. |
| `qs go sha256.Sum256` | 5 | #1 sha256.Sum256() | Exact SHA-256 helper is #1 with signature and the SHA3 alternative clearly separated. |
| `qs go crypto/rand.Read` | 0 | Timeout after 15 s; no output | Timed out after 15 s with no output. |
| `qs go sql.DB.QueryContext` | 5 | #1 sql.DB.QueryContext() | Exact database method is #1 with signature and a query example. |
| `qs go flag.String` | 5 | #1 flag.String() | Exact flag helper is #1 with signature and return-value behavior. |
| `qs go log/slog.Logger` | 4 | #1 slog.NewLogLogger() | Logger methods rank #2 and #3 with useful signatures, but the `slog.Logger` type page is missing. |
| `qs go testing.T.Run` | 5 | #1 testing.T.Run() | Exact subtest method is #1 with detailed execution semantics. |
| `qs go table driven tests` | 2 | #1 testing | Generic `testing` docs are #1, but no table-driven structure or example is shown. |
| `qs go benchmark functions` | 5 | #1 How to Write Benchmark Tests for Your Golang Functions | Benchmark tutorial is #1 with practical context; official `testing` docs also appear at #5. |
| `qs go runtime.GOMAXPROCS` | 5 | #1 runtime.GOMAXPROCS() | Exact runtime function is #1 with signature and behavior. |

## Strongest patterns

- Exact package-qualified APIs are the clear strength. Queries such as `fmt.Sprintf`, `io.Copy`, `errors.Is`, `http.NewRequestWithContext`, and `testing.T.Run` return the expected API at #1 with signatures and useful context.
- CamelCase identifiers rank reliably. This holds across functions, methods, and types.
- Many exact API results include practical examples. File I/O, slice/map helpers, errors, sorting, HTTP, JSON, and hashing are especially good.
- Some input normalization works well. `json_marshal` resolves to `json.Marshal`, and `encoding/json.Unmarshal` resolves to the correct package API.
- Web fallback can be excellent for idioms. Variadic functions, defer, middleware, the race detector, and benchmarks return useful tutorials or official Go pages.

## Weakest patterns

- Language concepts are often treated as raw API tokens. Examples include arrays returning `types.Array`, interfaces returning `net.Interfaces`, and type switches returning `ast.TypeSwitchStmt`.
- Natural multiword tasks collide with incidental words. `embedded fields` returns `bytes.Fields`, `time.Parse layout` returns `structs.HostLayout`, and `worker pool` returns `sync.Pool`.
- Core built-ins rank poorly or disappear. `append` is #4, `make` is #3, while `new`, `delete`, `clear`, and `iota` do not return their expected built-in documentation.
- Several concept queries fail instead of reaching useful web or official documentation. Buffered channels, select statements, and closing channels report "no web results".
- Slash-qualified queries are unstable. `container/heap` and `crypto/rand.Read` timed out, while other slash forms worked.

## Prioritized improvements

1. **Add a Go language-construct index.** Map arrays, slices, maps, interfaces, type assertions, type switches, struct tags, receivers, generics, goroutines, channels, `range`, and `iota` to the Go specification or official tutorials before general API search.
2. **Give built-ins exact-match priority.** For bare identifiers such as `append`, `copy`, `delete`, `clear`, `new`, `make`, `nil`, `panic`, and `recover`, rank `builtin` entries ahead of package methods with the same token.
3. **Improve phrase-aware ranking.** Preserve units such as `worker pool`, `multiple return values`, `file permissions`, `named groups`, `table driven tests`, and `time.Parse layout` instead of matching isolated words.
4. **Use a robust official-web fallback.** If DevDocs has low-confidence results or no semantic match, retry against `go.dev`, `pkg.go.dev`, and high-quality tutorials. Do not turn a transient DuckDuckGo miss into an empty result.
5. **Harden package-path lookup and snippets.** Resolve slash/dot package identifiers directly, prevent the observed timeouts, and ensure exact entries show signatures or focused examples such as `%w`, `omitempty`, `Client.Timeout`, and `//go:embed`.
