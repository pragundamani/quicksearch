# Go candidate-v5 quicksearch scores

## Evaluation

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **30 seconds**.
- Rubric: the 0–5 practical developer rubric in `language_scores.md`.

## Summary

- **Average:** 2.72/5.
- **Median:** 2/5.
- **Distribution:** 0: 10, 1: 11, 2: 36, 3: 11, 4: 4, 5: 28.
- **Pass rate (score >=3):** 43/100 (43%).
- **Strong pass rate (score >=4):** 32/100 (32%).
- **Command errors:** 0/100. **Timeouts:** 0/100.

## Comparison

- **Candidate-v3:** 4.04 average, 5 median, distribution 0: 10, 1: 2, 2: 6, 3: 8, 4: 4, 5: 70, 82% at >=3, and 74% at >=4. Candidate-v5 decreases the average by **1.32**, >=3 by **39 percentage points**, and >=4 by **42 points**. The median falls from **5 to 2**. Both runs had **0 errors and 0 timeouts**.
- **Per-command change from v3:** **12 improved**, **32 unchanged**, and **56 regressed**. The largest regression is on exact standard-library identifiers: many now return third-party `pkg.go.dev` search results instead of the matching standard-library entity.

## Results

| # | Command | Score | Observed top result or error | User assessment |
|---:|---|---:|---|---|
| 1 | `qs go arrays` | 1 | #1 github.com/apache/arrow-go/v18/arrow/array | Only third-party array packages appear; Go array types are not documented. |
| 2 | `qs go slices` | 5 | #1 slices | The slice-types specification is first and directly answers the query. |
| 3 | `qs go append` | 2 | #1 golang.org/x/tools/go/analysis/passes/appends | An append analyzer is related, but the built-in's signature and behavior are absent. |
| 4 | `qs go copy builtin` | 5 | #1 builtin.copy() | `builtin.copy` is first with its signature and behavior. |
| 5 | `qs go maps` | 5 | #1 maps | The map-types specification is first and directly useful. |
| 6 | `qs go delete map key` | 5 | #1 delete | The specification's map-deletion section is first, with `builtin.delete` also shown. |
| 7 | `qs go clear map` | 2 | #1 sync.Map.Clear() | `sync.Map.Clear` is related but is not the built-in `clear` for ordinary maps. |
| 8 | `qs go range loops` | 0 | #1 func (*B) Loop 1.24 | Loop and lookup results do not document the `range` clause. |
| 9 | `qs go variadic functions` | 5 | #1 variadic functions | The function-types specification directly documents variadic functions. |
| 10 | `qs go multiple return values` | 5 | #1 multiple return values | Effective Go's multiple-return section is first and directly useful. |
| 11 | `qs go defer statement` | 5 | #1 defer statements | The defer-statements specification is first and directly useful. |
| 12 | `qs go panic and recover` | 2 | #1 func panic | `panic` is documented, but `recover` and their paired control flow are missing. |
| 13 | `qs go goroutines` | 3 | #1 runtime | The runtime package is relevant to goroutines, but does not explain `go` statements. |
| 14 | `qs go buffered channels` | 5 | #1 buffered channels | The channel-types specification directly covers buffered channels. |
| 15 | `qs go select statement` | 5 | #1 select statements | The select-statements specification is first and directly useful. |
| 16 | `qs go close channel` | 5 | #1 func close | The built-in `close` documentation is first and directly useful. |
| 17 | `qs go interfaces` | 1 | #1 github.com/spf13/cobra | I/O packages mention interfaces, but the Go interface type feature is absent. |
| 18 | `qs go interface embedding` | 3 | #1 types.Interface.Embedded(); #2 EmbeddedType(); #3 EmbeddedTypes() | Compiler APIs for embedded interfaces are relevant, but language guidance is weak. |
| 19 | `qs go type assertions` | 5 | #1 type assertions | The type-assertions specification is first and directly useful. |
| 20 | `qs go type switches` | 5 | #1 type switches | The type-switches specification is first and directly useful. |
| 21 | `qs go struct tags` | 5 | #1 struct tags | The struct-tags specification is first and directly useful. |
| 22 | `qs go embedded fields` | 0 | #1 embed.FS | Results concern `embed.FS`, not embedded struct fields. |
| 23 | `qs go methods pointer receivers` | 1 | #1 reflect.Method; #4 types.MissingMethod() | A late `MissingMethod` result mentions pointer receivers, but receiver semantics are absent. |
| 24 | `qs go iota constants` | 3 | #1 internal/goarch constants; #4 builtin constants (`iota`) | The built-in constants result at rank 4 documents `iota`; ranking is weak. |
| 25 | `qs go type parameters` | 5 | #1 type parameters and generics | The generics specification section is first and directly useful. |
| 26 | `qs go generic constraints` | 2 | #1 func (*TypeParam) Constraint 1.18 | `TypeParam.Constraint` is related, but constraint interfaces and type sets are not explained. |
| 27 | `qs go comparable` | 0 | #1 cmp | Comparison packages appear, but the predeclared `comparable` constraint does not. |
| 28 | `qs go new` | 0 | #1 sigs.k8s.io/controller-runtime/pkg/log | Generic search results do not document the built-in `new`. |
| 29 | `qs go make` | 0 | #1 google.golang.org/grpc/credentials | Generic search results do not document the built-in `make`. |
| 30 | `qs go nil` | 1 | #1 golang.org/x/tools/go/analysis/passes/nilness | Nil-analysis packages are related, but the predeclared `nil` value is not documented. |
| 31 | `qs go fmt.Sprintf` | 2 | #1 sigs.k8s.io/external-dns/source | Third-party snippets use `fmt.Sprintf`, but its API documentation is absent. |
| 32 | `qs go fmt.Errorf` | 1 | #1 github.com/quintans/faults | Third-party error packages mention `fmt.Errorf`; `%w` behavior is absent. |
| 33 | `qs go fmt.Fscan` | 5 | #1 fmt.Fscan() | `fmt.Fscan` is first with useful API documentation. |
| 34 | `qs go strings.Builder` | 2 | #1 github.com/yylego/printgo | A wrapper describes `strings.Builder`, but the standard type documentation is absent. |
| 35 | `qs go strings.Fields` | 0 | #1 github.com/yangyin5127/sensitivewords | The lone package result does not document `strings.Fields`. |
| 36 | `qs go strings.ReplaceAll` | 2 | #1 github.com/github/gh-aw/pkg/linters/stringreplaceminusone | A linter explains when to use `strings.ReplaceAll`, but not its full API. |
| 37 | `qs go bytes.Buffer` | 1 | #1 github.com/beyondstorage/go-storage/v4/pkg/iowrap | Third-party buffer wrappers only mention `bytes.Buffer`; the standard type is absent. |
| 38 | `qs go bytes.Equal` | 2 | #1 github.com/xyproto/benchmarked | A linter explains a use for `bytes.Equal`, but the function documentation is absent. |
| 39 | `qs go strconv.Atoi` | 2 | #1 github.com/neutrino2211/go-result | Learning-package results mention `strconv.Atoi` without documenting the conversion API. |
| 40 | `qs go strconv.FormatInt` | 5 | #1 strconv.FormatInt() | `strconv.FormatInt` is first with a signature, description, and example. |
| 41 | `qs go regexp.Compile` | 2 | #1 gitlab.com/phpboyscout/go/regexutil | Wrappers and examples discuss `regexp.Compile`, but the standard constructor is absent. |
| 42 | `qs go regexp named groups` | 4 | #1 regexp/syntax CapNames; #2 regexp.SubexpNames | Named-capture APIs occupy the top three and provide useful context, though syntax is not shown. |
| 43 | `qs go unicode.IsLetter` | 2 | #1 github.com/LondonGophers/StudyGroup/workspaces/jlucktay/ch4/ex4_8 | Examples mention `unicode.IsLetter`, but the standard function documentation is absent. |
| 44 | `qs go sort.Slice` | 2 | #1 golang.org/x/tools/go/analysis/passes/sortslice | The top analyzer describes calls to `sort.Slice`, but not the sorting API itself. |
| 45 | `qs go sort.Search` | 2 | #1 github.com/albertocavalcante/go-analyzers/searchmigrate | An analyzer and examples mention `sort.Search`; the standard function is absent. |
| 46 | `qs go slices.Sort` | 2 | #1 github.com/TomTonic/rtcompare/cmd/rtcompare-example | A comparison example mentions `slices.Sort`, but the standard API is absent. |
| 47 | `qs go slices.Contains` | 5 | #1 slices.Contains() | `slices.Contains` is first with a signature, description, and example. |
| 48 | `qs go maps.Clone` | 5 | #1 maps.Clone() | `maps.Clone` is first with a signature, description, and example. |
| 49 | `qs go maps.Equal` | 5 | #1 maps.Equal() | `maps.Equal` is first with a signature, description, and example. |
| 50 | `qs go container/heap` | 5 | #1 container/heap | The `container/heap` package is first with complete, actionable example code. |
| 51 | `qs go container/list.New` | 4 | #1 container/list package; #3 list.New() | `list.New` is at rank 3 with its exact signature and description. |
| 52 | `qs go io.Copy` | 3 | #1 github.com/postmannen/go-learning/io/01-io.copy_WrapperAroundStdinStdout | The first result is a concrete `io.Copy` example, but not the standard API reference. |
| 53 | `qs go io.ReadAll` | 3 | #1 github.com/gomatic/yze-go-boundedread | The top analyzer explains an important `io.ReadAll` behavior, but omits the API reference. |
| 54 | `qs go io.Reader` | 2 | #1 bufio | `bufio` discusses wrapping `io.Reader`, but the interface itself is not documented. |
| 55 | `qs go bufio.Scanner` | 2 | #1 github.com/icza/backscanner | Related scanner tools appear, but `bufio.Scanner` documentation does not. |
| 56 | `qs go bufio.NewWriter` | 2 | #1 github.com/hhatto/gorst | A package example shows `bufio.NewWriter` usage, but the API reference is absent. |
| 57 | `qs go os.ReadFile` | 1 | #1 github.com/zew/go-questionnaire/pkg/cloudio | A wrapper says it emulates `os.ReadFile`, offering very little standard-API guidance. |
| 58 | `qs go os.WriteFile` | 2 | #1 github.com/zew/go-questionnaire/pkg/cloudio | Wrappers and analyzers mention `os.WriteFile`, but do not document the function. |
| 59 | `qs go os.OpenFile flags` | 5 | #1 os.OpenFile() | `os.OpenFile` is first with a useful flags example. |
| 60 | `qs go filepath.WalkDir` | 2 | #1 github.com/charlievieth/fastwalk | A faster alternative describes `filepath.WalkDir`'s purpose, but the API is absent. |
| 61 | `qs go filepath.Join` | 1 | #1 go-windows-su; #4 sanejoin comparison | Related path packages appear; only a late result briefly compares itself to `filepath.Join`. |
| 62 | `qs go file permissions` | 0 | #1 ast.File.Pos() | AST position APIs are unrelated to file permission bits. |
| 63 | `qs go temporary directory` | 3 | #1 ioutil.TempDir; #4 os.MkdirTemp | Deprecated `ioutil.TempDir` is first and `os.MkdirTemp` is rank 4; useful but poorly ranked. |
| 64 | `qs go embed files` | 5 | #1 embed.FS | `embed.FS` is first and its description directly mentions `//go:embed`. |
| 65 | `qs go time.Parse layout` | 5 | #1 time.Parse() | `time.Parse` is first with layout guidance and extensive examples. |
| 66 | `qs go time.NewTicker` | 2 | #1 github.com/mcpmini/mini/tools/clocklint | A linter mentions calls to `time.NewTicker`, but the function is not documented. |
| 67 | `qs go time.AfterFunc` | 2 | #1 github.com/ghettovoice/timeutil | Third-party timers compare themselves to `time.AfterFunc`, but the function is absent. |
| 68 | `qs go context cancellation` | 3 | #1 type Context 1.7 | `context.Context` explains cancellation signals, but `WithCancel` and `CancelFunc` are absent. |
| 69 | `qs go context.WithTimeout` | 2 | #1 github.com/vardius/shutdown | Several examples use `context.WithTimeout`, but its contract is not documented. |
| 70 | `qs go errors.Is` | 2 | #1 github.com/gomatic/go-error | Third-party packages explain matching with `errors.Is`, but the standard API is absent. |
| 71 | `qs go errors.As` | 3 | #1 golang.org/x/tools/go/analysis/passes/errorsas | The top analyzer explains `errors.As` argument requirements, but omits the standard API. |
| 72 | `qs go errors.Join` | 2 | #1 github.com/mrflynn/go-joinederror | A third-party utility discusses `errors.Join` values, but the function behavior is incomplete. |
| 73 | `qs go sync.WaitGroup` | 2 | #1 k8s.io/apimachinery/pkg/util/waitgroup | WaitGroup wrappers and an analyzer are relevant, but `sync.WaitGroup` itself is absent. |
| 74 | `qs go mutexes` | 1 | #1 github.com/juju/mutex | Only third-party mutex packages appear; `sync.Mutex` is not documented. |
| 75 | `qs go sync.Once` | 1 | #1 github.com/admpub/once | Only alternatives to `sync.Once` appear; the standard type is not documented. |
| 76 | `qs go atomic counter` | 3 | #1 coverage.ClearCounters; #3 sync/atomic | `sync/atomic` is rank 3 and relevant, but counter-operation guidance is weak. |
| 77 | `qs go worker pool` | 1 | #1 Example (Config) | Results concern `sync.Pool` and worker configuration, not the goroutine/channel pattern. |
| 78 | `qs go race detector` | 3 | #1 Package race | `runtime/race` is first, but practical race-detector usage and flags are missing. |
| 79 | `qs go net.DialTimeout` | 0 | #1 github.com/tomasen/fcgi_client | Unexplained third-party packages appear; `net.DialTimeout` is not documented. |
| 80 | `qs go url.Parse` | 2 | #1 github.com/ImVexed/fasturl | Examples and a wrapper show `url.Parse` use, but the standard API is absent. |
| 81 | `qs go http client timeout` | 4 | #1 HTTP 100 Continue example; #3 http.Client | `http.Client` is rank 3 and includes the exact `Timeout` field documentation. |
| 82 | `qs go http.NewRequestWithContext` | 0 | #1 gitee.com/FlyingOnion/httpclient | HTTP client libraries appear, but `http.NewRequestWithContext` is not documented. |
| 83 | `qs go http.HandleFunc` | 2 | #1 github.com/postmannen/go-learning/blueprints/01.1-first-serving-web-with-http.HandleFunc-and-anonymous-func-in-main | Examples show `http.HandleFunc` in use, but the function contract is absent. |
| 84 | `qs go http middleware` | 2 | #1 Constants | The `net/http` package is relevant, but no middleware or `Handler` pattern is shown. |
| 85 | `qs go httptest.NewServer` | 2 | #1 github.com/ghbvf/gocell/framework/pkg/testutil/nettest | A test helper mentions `httptest.NewServer`, but its behavior and signature are absent. |
| 86 | `qs go json_marshal` | 0 | #1 github.com/jpillora/velox | Unrelated JSON packages appear; `encoding/json.Marshal` is absent. |
| 87 | `qs go encoding/json.Unmarshal` | 2 | #1 JSON struct representation; #2 CustomMarshalJSON example | JSON documentation and an example use unmarshaling, but `json.Unmarshal` is not documented. |
| 88 | `qs go omit empty json field` | 5 | #1 json.OmitEmptyWithLegacySemantics() | The first result directly explains `omitempty` semantics. |
| 89 | `qs go xml decoder` | 5 | #1 xml.Decoder | `xml.Decoder` is first with detailed type documentation. |
| 90 | `qs go base64.StdEncoding` | 4 | #1 encoding/base64 package; #2 StdEncoding example | The base64 package and two examples use `StdEncoding` in the top results. |
| 91 | `qs go hex.DecodeString` | 2 | #1 github.com/leffss/go-admin/pkg/encryption | A third-party example explains hex decoding, but `hex.DecodeString` is not documented. |
| 92 | `qs go sha256.Sum256` | 5 | #1 sha256.Sum256() | `sha256.Sum256` is first with its exact signature and behavior. |
| 93 | `qs go crypto/rand.Read` | 2 | #1 github.com/ren-zxcyq/gorest/uuidGenerator | A package example shows `crypto/rand.Read` for random bytes, but no API contract. |
| 94 | `qs go sql.DB.QueryContext` | 5 | #1 sql.DB.QueryContext() | `sql.DB.QueryContext` is first with a full signature and example. |
| 95 | `qs go flag.String` | 2 | #1 github.com/chai2010/winsvc | A third-party example uses `flag.String`, but the standard function is absent. |
| 96 | `qs go log/slog.Logger` | 2 | #1 github.com/lesomnus/otx/log | Adapters describe `log/slog.Logger`, but the standard type documentation is absent. |
| 97 | `qs go testing.T.Run` | 2 | #1 github.com/picatz/flowstate/pkg/flowstate/v1/flowtest/flowtesting | A test harness explains use of `testing.T.Run`, but not the method itself. |
| 98 | `qs go table driven tests` | 2 | #1 html/template; #3 testing package | The testing package is only rank 3 and no table-driven pattern is shown. |
| 99 | `qs go benchmark functions` | 5 | #1 Benchmarks | The benchmark-functions section is first with the required signature and sample. |
| 100 | `qs go runtime.GOMAXPROCS` | 3 | #1 go.uber.org/automaxprocs/maxprocs | Top packages explain the purpose of `runtime.GOMAXPROCS`, but the function itself is absent. |
