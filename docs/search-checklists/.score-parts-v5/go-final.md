# Go final quicksearch scores

## Evaluation

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **30 seconds**.
- Rubric: the 0–5 practical developer rubric in `language_scores.md`.

## Summary

- **Average:** 4.38/5.
- **Median:** 5/5.
- **Distribution:** 0: 3, 1: 1, 2: 7, 3: 9, 4: 4, 5: 76.
- **Pass rate (score >=3):** 89/100 (89%).
- **Strong pass rate (score >=4):** 80/100 (80%).
- **Command errors:** 0/100. **Timeouts:** 0/100.

## Comparison with `.score-parts-v5/go.md`

- **Previous v5:** 2.72 average, 2 median, distribution 0: 10, 1: 11, 2: 36, 3: 11, 4: 4, 5: 28, 43% at >=3, and 32% at >=4.
- **Final:** 4.38 average, 5 median, distribution 0: 3, 1: 1, 2: 7, 3: 9, 4: 4, 5: 76, 89% at >=3, and 80% at >=4.
- The average increases by **1.66**. The >=3 rate increases by **46 percentage points**, and the >=4 rate increases by **48 points**. The median rises from **2 to 5**.
- **Per-command change from v5:** **53 improved**, **47 unchanged**, and **0 regressed**.
- Both runs had **0 errors and 0 timeouts**.

## Results

| # | Command | Score | Observed top result or error | User assessment |
|---:|---|---:|---|---|
| 1 | `qs go arrays` | 5 | #1 arrays | The first result directly matches Array types and is immediately useful. |
| 2 | `qs go slices` | 5 | #1 slices | The first result directly matches Slice types and operations and is immediately useful. |
| 3 | `qs go append` | 5 | #1 Go built-in append | The first result directly matches Built-in `append` and is immediately useful. |
| 4 | `qs go copy builtin` | 5 | #1 builtin.copy() | The first result directly matches Built-in `copy` and is immediately useful. |
| 5 | `qs go maps` | 5 | #1 maps | The first result directly matches Map types and is immediately useful. |
| 6 | `qs go delete map key` | 5 | #1 delete | The first result directly matches Built-in `delete` and is immediately useful. |
| 7 | `qs go clear map` | 2 | #1 sync.Map.Clear() | This is relevant to clearing a `sync.Map`, but it does not document the built-in `clear` for ordinary maps. |
| 8 | `qs go range loops` | 0 | #1 func (*B) Loop; #2 (*Int).MulRange; #5 (*Map).Range | The results do not document the language `range` clause. |
| 9 | `qs go variadic functions` | 5 | #1 variadic functions | The first result directly matches Variadic parameters and is immediately useful. |
| 10 | `qs go multiple return values` | 5 | #1 multiple return values | The first result directly matches Function declarations and multiple results and is immediately useful. |
| 11 | `qs go defer statement` | 5 | #1 defer statements | The first result directly matches `defer` statements and is immediately useful. |
| 12 | `qs go panic and recover` | 2 | #1 func panic; #2 runtime.PanicNilError | `panic` is documented, but `recover` and their paired control flow are missing. |
| 13 | `qs go goroutines` | 5 | #1 goroutines | The first result directly matches Goroutines and `go` statements and is immediately useful. |
| 14 | `qs go buffered channels` | 5 | #1 buffered channels | The first result directly matches Channel capacity and `make(chan T, n)` and is immediately useful. |
| 15 | `qs go select statement` | 5 | #1 select statements | The first result directly matches `select` statements and is immediately useful. |
| 16 | `qs go close channel` | 5 | #1 func close | The first result directly matches Built-in `close` and channel semantics and is immediately useful. |
| 17 | `qs go interfaces` | 5 | #1 interfaces | The first result directly matches Interface types and is immediately useful. |
| 18 | `qs go interface embedding` | 3 | #1 types.Interface.Embedded(); #2 EmbeddedType(); #3 EmbeddedTypes() | Compiler APIs for embedded interfaces are relevant, but language-level embedding and type-set guidance is weak. |
| 19 | `qs go type assertions` | 5 | #1 type assertions | The first result directly matches Type assertion expressions and is immediately useful. |
| 20 | `qs go type switches` | 5 | #1 type switches | The first result directly matches Type switch statements and is immediately useful. |
| 21 | `qs go struct tags` | 5 | #1 struct tags | The first result directly matches Struct field tags and is immediately useful. |
| 22 | `qs go embedded fields` | 0 | #1 embed.FS | The results concern embedded files, not embedded struct fields. |
| 23 | `qs go methods pointer receivers` | 2 | #1 types.MissingMethod(); #2 json.CallMethodsWithLegacySemantics | The snippets mention pointer receivers, but do not explain method declarations or receiver semantics. |
| 24 | `qs go iota constants` | 5 | #1 Constants | The first result directly matches Predeclared identifier `iota` and is immediately useful. |
| 25 | `qs go type parameters` | 5 | #1 type parameters and generics | The first result directly matches Generic type parameters and is immediately useful. |
| 26 | `qs go generic constraints` | 2 | #1 types.TypeParam.Constraint(); #2 go/build/constraint | The results are related to constraints, but do not explain constraint interfaces and type sets. |
| 27 | `qs go comparable` | 5 | #1 builtin.comparable | The first result directly matches Predeclared constraint `comparable` and is immediately useful. |
| 28 | `qs go new` | 5 | #1 Go built-in new | The first result directly matches Built-in `new` and is immediately useful. |
| 29 | `qs go make` | 5 | #1 Go built-in make | The first result directly matches Built-in `make` and is immediately useful. |
| 30 | `qs go nil` | 3 | #1 types.Nil; #2 uuid.Nil() | `types.Nil` represents the predeclared value, but the result is a compiler API rather than direct language documentation. |
| 31 | `qs go fmt.Sprintf` | 5 | #1 fmt.Sprintf() | The first result directly matches `fmt.Sprintf` and is immediately useful. |
| 32 | `qs go fmt.Errorf` | 3 | #1 fmt.Errorf() | The exact API is first, but the displayed documentation does not explain `%w` wrapping. |
| 33 | `qs go fmt.Fscan` | 5 | #1 fmt.Fscan() | The first result directly matches `fmt.Fscan` and is immediately useful. |
| 34 | `qs go strings.Builder` | 5 | #1 strings.Builder | The first result directly matches `strings.Builder` and is immediately useful. |
| 35 | `qs go strings.Fields` | 5 | #1 strings.Fields() | The first result directly matches `strings.Fields` and is immediately useful. |
| 36 | `qs go strings.ReplaceAll` | 5 | #1 strings.ReplaceAll() | The first result directly matches `strings.ReplaceAll` and is immediately useful. |
| 37 | `qs go bytes.Buffer` | 5 | #1 bytes.Buffer | The first result directly matches `bytes.Buffer` and is immediately useful. |
| 38 | `qs go bytes.Equal` | 5 | #1 bytes.Equal() | The first result directly matches `bytes.Equal` and is immediately useful. |
| 39 | `qs go strconv.Atoi` | 5 | #1 strconv.Atoi() | The first result directly matches `strconv.Atoi` and is immediately useful. |
| 40 | `qs go strconv.FormatInt` | 5 | #1 strconv.FormatInt() | The first result directly matches `strconv.FormatInt` and is immediately useful. |
| 41 | `qs go regexp.Compile` | 5 | #1 regexp.Compile() | The first result directly matches `regexp.Compile` and is immediately useful. |
| 42 | `qs go regexp named groups` | 4 | #1 syntax.Regexp.CapNames(); #2 regexp.SubexpNames() | Named-capture APIs lead the results and give useful context, though the pattern syntax itself is not shown. |
| 43 | `qs go unicode.IsLetter` | 5 | #1 unicode.IsLetter() | The first result directly matches `unicode.IsLetter` and is immediately useful. |
| 44 | `qs go sort.Slice` | 5 | #1 sort.Slice() | The first result directly matches `sort.Slice` and is immediately useful. |
| 45 | `qs go sort.Search` | 5 | #1 sort.Search() | The first result directly matches `sort.Search` and is immediately useful. |
| 46 | `qs go slices.Sort` | 5 | #1 slices.Sort() | The first result directly matches `slices.Sort` and is immediately useful. |
| 47 | `qs go slices.Contains` | 5 | #1 slices.Contains() | The first result directly matches `slices.Contains` and is immediately useful. |
| 48 | `qs go maps.Clone` | 5 | #1 maps.Clone() | The first result directly matches `maps.Clone` and is immediately useful. |
| 49 | `qs go maps.Equal` | 5 | #1 maps.Equal() | The first result directly matches `maps.Equal` and is immediately useful. |
| 50 | `qs go container/heap` | 5 | #1 container/heap | The first result directly matches Package `container/heap` and is immediately useful. |
| 51 | `qs go container/list.New` | 4 | #1 container/list package; #3 list.New() | The exact `list.New` function is rank 3 with its signature and description. |
| 52 | `qs go io.Copy` | 5 | #1 io.Copy() | The first result directly matches `io.Copy` and is immediately useful. |
| 53 | `qs go io.ReadAll` | 5 | #1 io.ReadAll() | The first result directly matches `io.ReadAll` and is immediately useful. |
| 54 | `qs go io.Reader` | 5 | #1 io.Reader | The first result directly matches `io.Reader` interface and is immediately useful. |
| 55 | `qs go bufio.Scanner` | 5 | #1 bufio.Scanner | The first result directly matches `bufio.Scanner` and is immediately useful. |
| 56 | `qs go bufio.NewWriter` | 5 | #1 bufio.NewWriter() | The first result directly matches `bufio.NewWriter` and is immediately useful. |
| 57 | `qs go os.ReadFile` | 5 | #1 os.ReadFile() | The first result directly matches `os.ReadFile` and is immediately useful. |
| 58 | `qs go os.WriteFile` | 5 | #1 os.WriteFile() | The first result directly matches `os.WriteFile` and is immediately useful. |
| 59 | `qs go os.OpenFile flags` | 5 | #1 os.OpenFile() | The first result directly matches `os.OpenFile` and open flags and is immediately useful. |
| 60 | `qs go filepath.WalkDir` | 5 | #1 filepath.WalkDir() | The first result directly matches `path/filepath.WalkDir` and is immediately useful. |
| 61 | `qs go filepath.Join` | 5 | #1 filepath.Join() | The first result directly matches `path/filepath.Join` and is immediately useful. |
| 62 | `qs go file permissions` | 0 | #1 ast.File.Pos(); #2 token.File.Pos() | Position APIs are unrelated to `fs.FileMode` permission bits. |
| 63 | `qs go temporary directory` | 3 | #1 net/url.Error.Temporary(); #2 ioutil.TempDir | The deprecated `ioutil.TempDir` is useful and near the top, but `os.MkdirTemp` is absent. |
| 64 | `qs go embed files` | 5 | #1 embed.FS | The first result directly matches Package `embed` and `//go:embed` and is immediately useful. |
| 65 | `qs go time.Parse layout` | 5 | #1 time.Parse() | The first result directly matches `time.Parse` and reference-time layouts and is immediately useful. |
| 66 | `qs go time.NewTicker` | 5 | #1 time.NewTicker() | The first result directly matches `time.NewTicker` and is immediately useful. |
| 67 | `qs go time.AfterFunc` | 5 | #1 time.AfterFunc() | The first result directly matches `time.AfterFunc` and is immediately useful. |
| 68 | `qs go context cancellation` | 3 | #1 context.Context; #2 exec.CommandContext() | The context interface explains cancellation signals, but `WithCancel` and `CancelFunc` are absent. |
| 69 | `qs go context.WithTimeout` | 5 | #1 context.WithTimeout() | The first result directly matches `context.WithTimeout` and is immediately useful. |
| 70 | `qs go errors.Is` | 5 | #1 errors.Is() | The first result directly matches `errors.Is` and is immediately useful. |
| 71 | `qs go errors.As` | 5 | #1 errors.As() | The first result directly matches `errors.As` and is immediately useful. |
| 72 | `qs go errors.Join` | 5 | #1 errors.Join() | The first result directly matches `errors.Join` and is immediately useful. |
| 73 | `qs go sync.WaitGroup` | 5 | #1 sync.WaitGroup | The first result directly matches `sync.WaitGroup` and is immediately useful. |
| 74 | `qs go mutexes` | 5 | #1 sync.Mutex | The first result directly matches `sync.Mutex` and is immediately useful. |
| 75 | `qs go sync.Once` | 5 | #1 sync.Once | The first result directly matches `sync.Once` and is immediately useful. |
| 76 | `qs go atomic counter` | 3 | #1 coverage.ClearCounters(); #4 sync/atomic package | The `sync/atomic` package is present, but ranking and counter-operation detail are weak. |
| 77 | `qs go worker pool` | 1 | #1 sync.Pool | The result is an object pool, not the goroutine/channel worker-pool pattern. |
| 78 | `qs go race detector` | 3 | #1 runtime/race package | The race package is relevant, but practical detector usage and flags are missing. |
| 79 | `qs go net.DialTimeout` | 5 | #1 net.DialTimeout() | The first result directly matches `net.DialTimeout` and is immediately useful. |
| 80 | `qs go url.Parse` | 5 | #1 url.Parse() | The first result directly matches `net/url.Parse` and is immediately useful. |
| 81 | `qs go http client timeout` | 4 | #1 Clients and Transports; #3 http.Client | `http.Client` is rank 3 and contains the relevant client fields, but `Timeout` is not focused first. |
| 82 | `qs go http.NewRequestWithContext` | 5 | #1 http.NewRequestWithContext() | The first result directly matches `net/http.NewRequestWithContext` and is immediately useful. |
| 83 | `qs go http.HandleFunc` | 5 | #1 http.HandleFunc() | The first result directly matches `net/http.HandleFunc` and is immediately useful. |
| 84 | `qs go http middleware` | 2 | #1 json/v2 ServeHTTP example; #3 net/http package | The HTTP package is relevant, but no clear `Handler` middleware pattern is shown. |
| 85 | `qs go httptest.NewServer` | 5 | #1 httptest.NewServer() | The first result directly matches `net/http/httptest.NewServer` and is immediately useful. |
| 86 | `qs go json_marshal` | 5 | #1 json.Marshal() | The first result directly matches `encoding/json.Marshal` and is immediately useful. |
| 87 | `qs go encoding/json.Unmarshal` | 3 | #1 jsontext.Value.UnmarshalJSON(); #3 encoding/json package | The standard JSON package is rank 3 and mentions unmarshaling, but `json.Unmarshal` itself is not shown. |
| 88 | `qs go omit empty json field` | 5 | #1 json.OmitEmptyWithLegacySemantics() | The first result directly matches JSON struct tag option `omitempty` and is immediately useful. |
| 89 | `qs go xml decoder` | 5 | #1 xml.Decoder | The first result directly matches `encoding/xml.Decoder` and is immediately useful. |
| 90 | `qs go base64.StdEncoding` | 4 | #1 encoding/base64 package; #2 StdEncoding example | The package and an exact `StdEncoding` example are in the top results, though the variable is not the first entity. |
| 91 | `qs go hex.DecodeString` | 5 | #1 hex.DecodeString() | The first result directly matches `encoding/hex.DecodeString` and is immediately useful. |
| 92 | `qs go sha256.Sum256` | 5 | #1 sha256.Sum256() | The first result directly matches `crypto/sha256.Sum256` and is immediately useful. |
| 93 | `qs go crypto/rand.Read` | 2 | #1 third-party uuidGenerator package | A third-party example mentions `crypto/rand.Read`, but its standard API contract is absent. |
| 94 | `qs go sql.DB.QueryContext` | 5 | #1 sql.DB.QueryContext() | The first result directly matches `database/sql.DB.QueryContext` and is immediately useful. |
| 95 | `qs go flag.String` | 5 | #1 flag.String() | The first result directly matches `flag.String` and is immediately useful. |
| 96 | `qs go log/slog.Logger` | 3 | #1 slog.Logger.Log(); #2 slog.NewLogLogger() | Standard `slog.Logger` methods are relevant, but the `Logger` type documentation itself is absent. |
| 97 | `qs go testing.T.Run` | 5 | #1 testing.T.Run() | The first result directly matches `testing.T.Run` and is immediately useful. |
| 98 | `qs go table driven tests` | 2 | #1 testing package; #2 crc32.Table | The testing package is first, but no table-driven testing pattern is shown. |
| 99 | `qs go benchmark functions` | 5 | #1 Benchmarks | The first result directly matches `testing.B` benchmark functions and is immediately useful. |
| 100 | `qs go runtime.GOMAXPROCS` | 5 | #1 runtime.GOMAXPROCS() | The first result directly matches `runtime.GOMAXPROCS` and is immediately useful. |
