# Go candidate quicksearch scores

## Evaluation

- Binary: `/tmp/qs-candidate/release/qs` (not the installed `qs`).
- Commands run: **100/100**.
- Per-command timeout: **20 seconds**.
- Rubric: the exact 0–5 practical developer rubric in `language_scores.md`.

## Summary

- **Average:** 3.71/5.
- **Median:** 5/5.
- **Distribution:** 0: 22, 1: 0, 2: 2, 3: 6, 4: 1, 5: 69.
- **Pass rate (score >=3):** 76/100 (76%).
- **Strong pass rate (score >=4):** 70/100 (70%).
- **Command errors:** 19/100. **Timeouts:** 0/100.

## Baseline comparison

The documented Go baseline is 3.61 average, 5 median, 69% at score >=3, 64% at score >=4, and distribution 0: 6, 1: 16, 2: 9, 3: 5, 4: 8, 5: 56. The candidate improves the average by **0.10**, score >=3 by **7 percentage points**, and score >=4 by **6 points**. The median is unchanged. It converts many weak results into first-ranked answers, but hard failures increase: **19 errors and 0 timeouts**, versus **4 errors and 2 timeouts** in the baseline.

## Results

| # | Command | Score | Observed top result or error | User assessment |
|---:|---|---:|---|---|
| 1 | `qs go arrays` | 5 | #1 Go array | The expected target ranks first and is immediately useful. |
| 2 | `qs go slices` | 5 | #1 Go slice | The expected target ranks first and is immediately useful. |
| 3 | `qs go append` | 5 | #1 Go built-in append | The expected target ranks first and is immediately useful. |
| 4 | `qs go copy builtin` | 5 | #1 builtin.copy() | The expected target ranks first and is immediately useful. |
| 5 | `qs go maps` | 5 | #1 Go map | The expected target ranks first and is immediately useful. |
| 6 | `qs go delete map key` | 5 | #1 delete | The expected target ranks first and is immediately useful. |
| 7 | `qs go clear map` | 2 | #1 sync.Map.Clear() | `sync.Map.Clear` is related, but it is not the built-in `clear` for ordinary maps. |
| 8 | `qs go range loops` | 5 | #1 Go Wiki: Range Clauses - The Go Programming Language | The expected target ranks first and is immediately useful. |
| 9 | `qs go variadic functions` | 5 | #1 Go by Example: Variadic Functions | The expected target ranks first and is immediately useful. |
| 10 | `qs go multiple return values` | 5 | #1 Go by Example: Multiple Return Values | The expected target ranks first and is immediately useful. |
| 11 | `qs go defer statement` | 3 | #1 ast.DeferStmt | The AST node confirms the syntax category, but it does not explain `defer` behavior. |
| 12 | `qs go panic and recover` | 5 | #1 Defer, Panic, and Recover - The Go Programming Language | The expected target ranks first and is immediately useful. |
| 13 | `qs go goroutines` | 5 | #1 Go goroutine | The expected target ranks first and is immediately useful. |
| 14 | `qs go buffered channels` | 5 | #1 Buffered Channel in Golang - GeeksforGeeks | The expected target ranks first and is immediately useful. |
| 15 | `qs go select statement` | 3 | #1 ast.SelectStmt | The AST node is relevant, but it does not explain selection or channel semantics. |
| 16 | `qs go close channel` | 5 | #1 Go by Example: Closing Channels | The expected target ranks first and is immediately useful. |
| 17 | `qs go interfaces` | 5 | #1 Go interface | The expected target ranks first and is immediately useful. |
| 18 | `qs go interface embedding` | 3 | #1 types.Interface.Embedded() | Compiler interface APIs are relevant, but embedding and type-set guidance is weak. |
| 19 | `qs go type assertions` | 2 | #1 runtime.TypeAssertionError | The runtime error type is related, but assertion syntax and behavior are absent. |
| 20 | `qs go type switches` | 3 | #1 ast.TypeSwitchStmt | The AST node shows the assertion form, but language-level guidance is weak. |
| 21 | `qs go struct tags` | 5 | #1 reflect.StructTag | The expected target ranks first and is immediately useful. |
| 22 | `qs go embedded fields` | 0 | #1 embed.FS | The result concerns embedded files, not embedded struct fields. |
| 23 | `qs go methods pointer receivers` | 0 | Error: no web results for go methods pointer receivers: https://lite.duckduckgo.com/lite/?q=go%20methods%20pointer%20receivers | The command failed and returned no usable result. |
| 24 | `qs go iota constants` | 0 | Error: no web results for go iota constants: https://lite.duckduckgo.com/lite/?q=go%20iota%20constants | The command failed and returned no usable result. |
| 25 | `qs go type parameters` | 0 | #1 dwarf.PtrType | The DWARF pointer type is unrelated to generic type parameters. |
| 26 | `qs go generic constraints` | 0 | Error: no web results for go generic constraints: https://lite.duckduckgo.com/lite/?q=go%20generic%20constraints | The command failed and returned no usable result. |
| 27 | `qs go comparable` | 5 | #1 builtin.comparable | The expected target ranks first and is immediately useful. |
| 28 | `qs go new` | 5 | #1 Go built-in new | The expected target ranks first and is immediately useful. |
| 29 | `qs go make` | 5 | #1 Go built-in make | The expected target ranks first and is immediately useful. |
| 30 | `qs go nil` | 3 | #1 types.Nil | The compiler representation identifies `nil`, but practical language semantics are weak. |
| 31 | `qs go fmt.Sprintf` | 5 | #1 fmt.Sprintf() | The expected target ranks first and is immediately useful. |
| 32 | `qs go fmt.Errorf` | 4 | #1 fmt.Errorf() | `fmt.Errorf` ranks first with useful formatting detail, but `%w` wrapping is omitted. |
| 33 | `qs go fmt.Fscan` | 5 | #1 fmt.Fscan() | The expected target ranks first and is immediately useful. |
| 34 | `qs go strings.Builder` | 5 | #1 strings.Builder | The expected target ranks first and is immediately useful. |
| 35 | `qs go strings.Fields` | 5 | #1 strings.Fields() | The expected target ranks first and is immediately useful. |
| 36 | `qs go strings.ReplaceAll` | 5 | #1 strings.ReplaceAll() | The expected target ranks first and is immediately useful. |
| 37 | `qs go bytes.Buffer` | 5 | #1 bytes.Buffer | The expected target ranks first and is immediately useful. |
| 38 | `qs go bytes.Equal` | 5 | #1 bytes.Equal() | The expected target ranks first and is immediately useful. |
| 39 | `qs go strconv.Atoi` | 5 | #1 strconv.Atoi() | The expected target ranks first and is immediately useful. |
| 40 | `qs go strconv.FormatInt` | 5 | #1 strconv.FormatInt() | The expected target ranks first and is immediately useful. |
| 41 | `qs go regexp.Compile` | 5 | #1 regexp.Compile() | The expected target ranks first and is immediately useful. |
| 42 | `qs go regexp named groups` | 0 | Error: no web results for go regexp named groups: https://lite.duckduckgo.com/lite/?q=go%20regexp%20named%20groups | The command failed and returned no usable result. |
| 43 | `qs go unicode.IsLetter` | 5 | #1 unicode.IsLetter() | The expected target ranks first and is immediately useful. |
| 44 | `qs go sort.Slice` | 5 | #1 sort.Slice() | The expected target ranks first and is immediately useful. |
| 45 | `qs go sort.Search` | 5 | #1 sort.Search() | The expected target ranks first and is immediately useful. |
| 46 | `qs go slices.Sort` | 5 | #1 slices.Sort() | The expected target ranks first and is immediately useful. |
| 47 | `qs go slices.Contains` | 5 | #1 slices.Contains() | The expected target ranks first and is immediately useful. |
| 48 | `qs go maps.Clone` | 5 | #1 maps.Clone() | The expected target ranks first and is immediately useful. |
| 49 | `qs go maps.Equal` | 5 | #1 maps.Equal() | The expected target ranks first and is immediately useful. |
| 50 | `qs go container/heap` | 5 | #1 container/heap | The expected target ranks first and is immediately useful. |
| 51 | `qs go container/list.New` | 0 | Error: no web results for go container/list.New: https://lite.duckduckgo.com/lite/?q=go%20container%2Flist.New | The command failed and returned no usable result. |
| 52 | `qs go io.Copy` | 5 | #1 io.Copy() | The expected target ranks first and is immediately useful. |
| 53 | `qs go io.ReadAll` | 5 | #1 io.ReadAll() | The expected target ranks first and is immediately useful. |
| 54 | `qs go io.Reader` | 5 | #1 io.Reader | The expected target ranks first and is immediately useful. |
| 55 | `qs go bufio.Scanner` | 5 | #1 bufio.Scanner | The expected target ranks first and is immediately useful. |
| 56 | `qs go bufio.NewWriter` | 5 | #1 bufio.NewWriter() | The expected target ranks first and is immediately useful. |
| 57 | `qs go os.ReadFile` | 5 | #1 os.ReadFile | The expected target ranks first and is immediately useful. |
| 58 | `qs go os.WriteFile` | 5 | #1 os.WriteFile | The expected target ranks first and is immediately useful. |
| 59 | `qs go os.OpenFile flags` | 0 | Error: no web results for go os.OpenFile flags: https://lite.duckduckgo.com/lite/?q=go%20os.OpenFile%20flags | The command failed and returned no usable result. |
| 60 | `qs go filepath.WalkDir` | 5 | #1 filepath.WalkDir | The expected target ranks first and is immediately useful. |
| 61 | `qs go filepath.Join` | 5 | #1 filepath.Join() | The expected target ranks first and is immediately useful. |
| 62 | `qs go file permissions` | 0 | #1 ast.File.Pos() | File-position APIs are unrelated to `fs.FileMode` and permission bits. |
| 63 | `qs go temporary directory` | 0 | Error: no web results for go temporary directory: https://lite.duckduckgo.com/lite/?q=go%20temporary%20directory | The command failed and returned no usable result. |
| 64 | `qs go embed files` | 5 | #1 embed.FS | The expected target ranks first and is immediately useful. |
| 65 | `qs go time.Parse layout` | 0 | Error: no web results for go time.Parse layout: https://lite.duckduckgo.com/lite/?q=go%20time.Parse%20layout | The command failed and returned no usable result. |
| 66 | `qs go time.NewTicker` | 5 | #1 time.NewTicker() | The expected target ranks first and is immediately useful. |
| 67 | `qs go time.AfterFunc` | 5 | #1 time.AfterFunc() | The expected target ranks first and is immediately useful. |
| 68 | `qs go context cancellation` | 0 | Error: no web results for go context cancellation: https://lite.duckduckgo.com/lite/?q=go%20context%20cancellation | The command failed and returned no usable result. |
| 69 | `qs go context.WithTimeout` | 5 | #1 context.WithTimeout() | The expected target ranks first and is immediately useful. |
| 70 | `qs go errors.Is` | 5 | #1 errors.Is() | The expected target ranks first and is immediately useful. |
| 71 | `qs go errors.As` | 5 | #1 errors.As() | The expected target ranks first and is immediately useful. |
| 72 | `qs go errors.Join` | 5 | #1 errors.Join() | The expected target ranks first and is immediately useful. |
| 73 | `qs go sync.WaitGroup` | 5 | #1 sync.WaitGroup | The expected target ranks first and is immediately useful. |
| 74 | `qs go mutexes` | 5 | #1 sync.Mutex | The expected target ranks first and is immediately useful. |
| 75 | `qs go sync.Once` | 5 | #1 sync.Once | The expected target ranks first and is immediately useful. |
| 76 | `qs go atomic counter` | 0 | Error: no web results for go atomic counter: https://lite.duckduckgo.com/lite/?q=go%20atomic%20counter | The command failed and returned no usable result. |
| 77 | `qs go worker pool` | 0 | Error: no web results for go worker pool: https://lite.duckduckgo.com/lite/?q=go%20worker%20pool | The command failed and returned no usable result. |
| 78 | `qs go race detector` | 0 | Error: no web results for go race detector: https://lite.duckduckgo.com/lite/?q=go%20race%20detector | The command failed and returned no usable result. |
| 79 | `qs go net.DialTimeout` | 5 | #1 net.DialTimeout() | The expected target ranks first and is immediately useful. |
| 80 | `qs go url.Parse` | 5 | #1 url.Parse() | The expected target ranks first and is immediately useful. |
| 81 | `qs go http client timeout` | 0 | Error: no web results for go http client timeout: https://lite.duckduckgo.com/lite/?q=go%20http%20client%20timeout | The command failed and returned no usable result. |
| 82 | `qs go http.NewRequestWithContext` | 5 | #1 http.NewRequestWithContext() | The expected target ranks first and is immediately useful. |
| 83 | `qs go http.HandleFunc` | 5 | #1 http.HandleFunc() | The expected target ranks first and is immediately useful. |
| 84 | `qs go http middleware` | 0 | Error: no web results for go http middleware: https://lite.duckduckgo.com/lite/?q=go%20http%20middleware | The command failed and returned no usable result. |
| 85 | `qs go httptest.NewServer` | 5 | #1 httptest.NewServer() | The expected target ranks first and is immediately useful. |
| 86 | `qs go json_marshal` | 5 | #1 json.Marshal() | The expected target ranks first and is immediately useful. |
| 87 | `qs go encoding/json.Unmarshal` | 0 | Error: no web results for go encoding/json.Unmarshal: https://lite.duckduckgo.com/lite/?q=go%20encoding%2Fjson.Unmarshal | The command failed and returned no usable result. |
| 88 | `qs go omit empty json field` | 5 | #1 json.OmitEmptyWithLegacySemantics() | The expected target ranks first and is immediately useful. |
| 89 | `qs go xml decoder` | 5 | #1 xml.Decoder | The expected target ranks first and is immediately useful. |
| 90 | `qs go base64.StdEncoding` | 0 | Error: no web results for go base64.StdEncoding: https://lite.duckduckgo.com/lite/?q=go%20base64.StdEncoding | The command failed and returned no usable result. |
| 91 | `qs go hex.DecodeString` | 5 | #1 hex.DecodeString() | The expected target ranks first and is immediately useful. |
| 92 | `qs go sha256.Sum256` | 5 | #1 sha256.Sum256() | The expected target ranks first and is immediately useful. |
| 93 | `qs go crypto/rand.Read` | 0 | Error: no web results for go crypto/rand.Read: https://lite.duckduckgo.com/lite/?q=go%20crypto%2Frand.Read | The command failed and returned no usable result. |
| 94 | `qs go sql.DB.QueryContext` | 5 | #1 sql.DB.QueryContext() | The expected target ranks first and is immediately useful. |
| 95 | `qs go flag.String` | 5 | #1 flag.String() | The expected target ranks first and is immediately useful. |
| 96 | `qs go log/slog.Logger` | 3 | #1 slog.Logger.Log() | Logger methods are relevant, but the requested `slog.Logger` type does not rank first. |
| 97 | `qs go testing.T.Run` | 5 | #1 testing.T.Run() | The expected target ranks first and is immediately useful. |
| 98 | `qs go table driven tests` | 0 | Error: no web results for go table driven tests: https://lite.duckduckgo.com/lite/?q=go%20table%20driven%20tests | The command failed and returned no usable result. |
| 99 | `qs go benchmark functions` | 0 | Error: no web results for go benchmark functions: https://lite.duckduckgo.com/lite/?q=go%20benchmark%20functions | The command failed and returned no usable result. |
| 100 | `qs go runtime.GOMAXPROCS` | 5 | #1 runtime.GOMAXPROCS() | The expected target ranks first and is immediately useful. |
