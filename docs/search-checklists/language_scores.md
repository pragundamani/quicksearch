# Quicksearch language scores

## Evaluation scope

Six independent evaluators ran all 600 checklist commands through the installed `qs` executable. Scores reflect the output an end user received, not the expected result written in each checklist. The newer threading-example source fix was not installed during this evaluation.

### Rubric

- **5** — Expected result is first and immediately useful.
- **4** — Expected result is in the top three with useful context.
- **3** — A relevant result exists, but ranking or detail is weak.
- **2** — Results are only partially relevant.
- **1** — Results are misleading or barely useful.
- **0** — Error or no relevant result.

## Validated candidate results

The final benchmark checkpoint uses candidate v3 for Python, C++, Go, and C, plus the same source-specific candidate for Rust and Haskell. Later safety guards only restrict ambiguous aliases, blend medium-confidence aliases, and prefer entity pages over header pages; focused regression and smoke tests cover those changes.

| Rank | Language | Baseline average | Candidate average | Score ≥3 | Score ≥4 | Errors |
|---:|---|---:|---:|---:|---:|---:|
| 1 | C | 4.35 | 4.33 | 89% | 83% | 0 |
| 2 | C++ | 3.87 | 4.28 | 92% | 85% | 0 |
| 3 | Python | 4.06 | 4.26 | 89% | 76% | 0 |
| 4 | Go | 3.61 | 4.04 | 82% | 74% | 0 |
| 5 | Rust | 1.78 | 3.52 | 74% | 46% | 0 |
| 6 | Haskell | 1.26 | 2.79 | 47% | 32% | 0 |

- **Candidate average:** 3.87/5 (baseline 3.16)
- **Candidate median:** 5/5 (baseline 4)
- **Practical pass rate (score ≥3):** 473/600, 78.8% (baseline 62.2%)
- **Strong-result rate (score ≥4):** 396/600, 66.0% (baseline 53.8%)
- **Combined distribution:** 0: 20, 1: 51, 2: 56, 3: 77, 4: 52, 5: 344
- **Command errors/timeouts:** 0

## Installed baseline results

| Rank | Language | Average | Median | Score ≥3 | Score ≥4 | Main user impression |
|---:|---|---:|---:|---:|---:|---|
| 1 | C | 4.35 | 5 | 88% | 85% | Excellent for exact APIs; task phrases still drift. |
| 2 | Python | 4.06 | 5 | 83% | 76% | Strong for named APIs; natural tasks are inconsistent. |
| 3 | C++ | 3.87 | 5 | 83% | 78% | Strong reference lookup with clear phrase-ranking gaps. |
| 4 | Go | 3.61 | 5 | 69% | 64% | Exact package APIs work; concepts and built-ins are weaker. |
| 5 | Rust | 1.78 | 1 | 28% | 2% | Reliable command execution, but most results are only search links. |
| 6 | Haskell | 1.26 | 0 | 22% | 18% | Frequent decoder/no-result failures make it unreliable. |

- **Overall average:** 3.16/5
- **Overall median:** 4/5
- **Practical pass rate (score ≥3):** 373/600 (62.2%)
- **Strong-result rate (score ≥4):** 323/600 (53.8%)
- **Combined distribution:** 0: 88, 1: 98, 2: 41, 3: 50, 4: 52, 5: 271

## Real-user assessment

As a developer, I would trust `qs` for direct C, Python, C++, and Go API names. Exact qualified identifiers are often fast, focused, and more useful than opening a browser. I would not yet trust it for broad task descriptions. Queries such as “read whole file,” “remove elements,” or “parse integer from string” often lock onto one generic word and return the wrong concept.

Rust currently feels like a launcher for a standard-library search page rather than a search tool. It rarely answers the question in the terminal. Haskell feels broken: valid searches can abort on JSON decoding, and many ordinary concepts return no result. These two sources need structural fixes before ranking polish.

Snippets and examples strongly affect perceived quality. A correct title without a signature, explanation, or local example often still feels unfinished. Conversely, C scores well because exact results usually provide enough evidence to act immediately.

## Cross-language findings

### What works

1. Exact qualified identifiers and distinctive API names.
2. Common standard-library entities in C, Python, C++, and Go.
3. Snake_case, CamelCase, and several spaced identifier variants.
4. Results with focused signatures, concise descriptions, and local examples.

### What fails

1. Natural-language task queries lack intent-to-API mapping.
2. Multiword queries can be dominated by one generic token.
3. Concept queries are often routed to the wrong corpus or entity type.
4. Web fallback can fail instead of retaining or retrying a useful candidate.
5. Source-specific failures dominate Rust and Haskell.
6. Python exposed a Unicode byte-boundary panic during snippet handling.

## Recommended priority

1. **Fix hard failures first.** Make Hoogle JSON decoding tolerant and fix Unicode-safe snippet slicing.
2. **Build source-aware routing.** Route Rust concepts, Cargo, rustdoc, and edition topics to their correct manuals. Route Go language constructs to the specification or official guides.
3. **Add phrase-aware intent matching.** Preserve complete phrases and require broader token coverage before ranking a result highly.
4. **Add common task aliases.** Map frequent developer wording to canonical APIs for file I/O, parsing, allocation, conversion, concurrency, and collection operations.
5. **Improve fallback behavior.** Retry normalized queries, prefer official documentation, and do not replace a weak local match with an empty web result.
6. **Keep snippets local and useful.** Prefer the matched signature, nearby explanation, and first relevant example without leaking from sibling sections.

## Full per-language reports

## Python

### Summary

- Commands run: **100/100** with the installed `qs` from `PATH`.
- Per-command timeout: **25 seconds**.
- Average: **4.06/5**.
- Median: **5/5**.
- Distribution: **0: 8**, **1: 4**, **2: 5**, **3: 7**, **4: 9**, **5: 67**.
- Pass rate (score >=3): **83/100 (83%)**.
- Strong pass rate (score >=4): **76/100 (76%)**.

### Results

| # | Command | Score | Observed top result or error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs python dictionaries` | 3 | dictionary glossary | Useful definition ranks first, but canonical dict/mapping API is absent. |
| 2 | `qs python list comprehensions` | 5 | list comprehension glossary | Exact concept ranks first with a clear explanation and example. |
| 3 | `qs python generator expressions` | 5 | generator expression glossary | Exact concept ranks first; tutorial detail is also second. |
| 4 | `qs python decorators` | 5 | decorator glossary | Immediately useful definition and @ syntax example rank first. |
| 5 | `qs python context managers` | 5 | context manager glossary | Exact protocol concept ranks first and directly mentions the with statement. |
| 6 | `qs python async iterators` | 4 | typing.AsyncIterator | Expected protocol is second as collections.abc.AsyncIterator with __aiter__/__anext__ context; deprecated typing alias ranks first. |
| 7 | `qs python structural pattern matching` | 3 | 10.5. String pattern matching | Top results confuse the query with regex; ast.Match appears fourth but match-statement/pattern docs are not surfaced. |
| 8 | `qs python positional only parameters` | 3 | genericalias.__parameters__ | First result is irrelevant; generic parameter glossary is second, but positional-only / syntax is not shown. |
| 9 | `qs python keyword only arguments` | 3 | keyword argument glossary | Related results exist, including ast.arguments at second, but keyword-only * syntax is missing. |
| 10 | `qs python exception chaining` | 5 | 8.5. Exception Chaining | Exact tutorial section ranks first with useful exception-context examples. |
| 11 | `qs python assignment expressions` | 5 | 6.12. Assignment expressions | Exact language-reference section ranks first and shows := usage. |
| 12 | `qs python function type annotations` | 4 | type.__annotations__ | Function annotation syntax is second with useful parameter and return details; first result is for types. |
| 13 | `qs python generic type aliases` | 5 | Type Annotation Types — Generic Alias, Union | Generic alias docs rank first, with type-alias results also in the top five. |
| 14 | `qs python dataclasses` | 5 | dataclasses | Exact standard-library module ranks first with a useful @dataclass example. |
| 15 | `qs python frozen dataclass` | 5 | Frozen instances (dataclasses) | Exact frozen=True behavior ranks first and is clearly explained. |
| 16 | `qs python enumerations` | 5 | enum — Support for enumerations | Official enum module documentation ranks first and the Enum HOWTO ranks third. |
| 17 | `qs python abstract base classes` | 3 | Collections Abstract Base Classes | Related ABC pages rank first, but the general abc module and ABC class are not surfaced. |
| 18 | `qs python method resolution order` | 5 | method resolution order | Exact glossary entry ranks first, followed by the detailed MRO HOWTO. |
| 19 | `qs python descriptor protocol` | 0 | Rust panic: byte index is not a char boundary | Command failed with rc 101 and returned no search results. |
| 20 | `qs python iterator protocol` | 2 | Iterator Protocol (C API) | Exact words rank first, but it is the C API page and does not expose the Python __iter__/__next__ protocol requested. |
| 21 | `qs python enumerate` | 5 | enumerate() | Exact built-in ranks first with signature behavior and examples. |
| 22 | `qs python zip` | 5 | zip() | Exact built-in ranks first with a concise parallel-iteration example. |
| 23 | `qs python isinstance` | 5 | isinstance() | Exact built-in predicate ranks first with useful semantics. |
| 24 | `qs python super` | 5 | super | Exact built-in ranks first with an inheritance example. |
| 25 | `qs python convert string to integer` | 2 | Integer Objects (C API) | Results discuss integer internals and is_integer(), but do not surface int() for string conversion. |
| 26 | `qs python memoryview` | 5 | memoryview | Exact built-in type is #1 with useful buffer-protocol context. |
| 27 | `qs python breakpoint` | 5 | breakpoint() | Exact built-in is #1 with clear debugger behavior. |
| 28 | `qs python str.removeprefix` | 5 | str.removeprefix() | Exact method is #1 with behavior and examples. |
| 29 | `qs python str.splitlines` | 5 | str.splitlines() | Exact method is #1 with behavior and examples. |
| 30 | `qs python bytes.fromhex` | 5 | bytes.fromhex() | Exact constructor is #1 with decoding details and example. |
| 31 | `qs python sort list by key` | 1 | key | Only generic key/glossary results; list.sort(key=...) and Sorting HOW TO are absent. |
| 32 | `qs python dict.setdefault` | 5 | dict.setdefault() | Exact mapping method is #1 with concise semantics. |
| 33 | `qs python merge dictionaries` | 1 | dictionary | Only generic dictionary entries; union operators are absent. |
| 34 | `qs python set symmetric_difference` | 5 | set.symmetric_difference() | Exact set method is #1. |
| 35 | `qs python frozenset` | 5 | frozenset | Exact immutable set type is #1 with useful context. |
| 36 | `qs python format specification mini language` | 1 | Python Language Services | Formal format-specification section is absent; results only match generic language terms. |
| 37 | `qs python f string debugging` | 0 | Debugging (doctest) | Results concern ordinary debugging and never surface self-documenting f-strings. |
| 38 | `qs python FileNotFoundError` | 5 | FileNotFoundError | Exact exception is #1 with meaning and ENOENT context. |
| 39 | `qs python ExceptionGroup` | 5 | ExceptionGroup | Exact exception container is #1. |
| 40 | `qs python property setter` | 5 | property.setter() | Exact setter decorator is #1 via a direct anchor. |
| 41 | `qs python pathlib.Path.read_text` | 5 | pathlib.Path.read_text() | Exact qualified method is #1 with example. |
| 42 | `qs python pathlib.Path.glob` | 5 | pathlib.Path.glob() | Exact qualified method is #1 with useful glob examples. |
| 43 | `qs python expand home directory` | 0 | os.O_DIRECTORY | No os.path.expanduser() result; returned directory-related but irrelevant APIs. |
| 44 | `qs python environment variables` | 2 | 1.2. Environment variables | Top result is relevant background, but os.environ is absent and the result concerns interpreter settings. |
| 45 | `qs python sys.argv` | 5 | sys.argv | Exact attribute is #1 with clear semantics. |
| 46 | `qs python sys.path` | 5 | sys.path | Exact attribute is #1 with clear module-search-path context. |
| 47 | `qs python argparse subcommands` | 4 | argparse - Python | Official argparse reference is #1 and useful, but add_subparsers() is not surfaced directly. |
| 48 | `qs python json.dumps` | 5 | json.dumps() | Exact serialization function is #1 with concise semantics. |
| 49 | `qs python json custom encoder` | 5 | json.JSONEncoder | Expected extension class is #1 and its default() override is shown. |
| 50 | `qs python csv.DictReader` | 5 | csv.DictReader | Exact class is #1 with useful behavior and example. |
| 51 | `qs python configparser` | 5 | configparser module | Exact expected module is #1 with a useful overview. |
| 52 | `qs python tomllib.load` | 5 | tomllib.load() | Exact expected API is #1 with input and return details. |
| 53 | `qs python pickle security` | 3 | Security Considerations | Relevant security page is #1, but the output omits the actual pickle warning and direct pickle result. |
| 54 | `qs python sqlite3 row factory` | 4 | sqlite3.Cursor.row_factory | Closely related cursor property is #1; expected Connection.row_factory is #2 with useful detail. |
| 55 | `qs python regular expression named groups` | 0 | os.getgroups() | Named-regex-group syntax is absent; results confuse regex groups with OS user groups. |
| 56 | `qs python re.escape` | 5 | re.escape() | Exact expected API is #1. |
| 57 | `qs python parse ISO date time` | 1 | xmlrpc.client.DateTime | Barely related ISO-date result; datetime.fromisoformat() is absent and ordinary parsing intent is not served. |
| 58 | `qs python timezone aware datetime` | 4 | datetime.timezone | Useful timezone API is #1 and expected aware/naive concept is #3. |
| 59 | `qs python zoneinfo.ZoneInfo` | 5 | zoneinfo.ZoneInfo | Exact expected class is #1 with useful IANA timezone context. |
| 60 | `qs python time.monotonic` | 5 | time.monotonic() | Exact expected clock is #1 with the key monotonic semantics. |
| 61 | `qs python collections.defaultdict` | 5 | collections.defaultdict | Exact expected type is #1 with a useful description. |
| 62 | `qs python collections.Counter.most_common` | 5 | collections.Counter.most_common() | Exact expected method is #1 with behavior and example. |
| 63 | `qs python collections.ChainMap` | 5 | collections.ChainMap | Exact expected type is #1 with a useful description. |
| 64 | `qs python heapq.nlargest` | 5 | heapq.nlargest() | Exact expected function is #1 with behavior and key details. |
| 65 | `qs python bisect.insort` | 5 | bisect.insort() | Exact expected insertion API is #1 with useful distinction from insort_left. |
| 66 | `qs python itertools.zip_longest` | 5 | itertools.zip_longest() | Exact expected function is #1 with description and implementation example. |
| 67 | `qs python itertools.groupby` | 5 | itertools.groupby() | Exact expected function is #1 with crucial consecutive/sorted grouping semantics. |
| 68 | `qs python functools.lru_cache` | 5 | functools.lru_cache() | Exact expected decorator is #1 with clear memoization context. |
| 69 | `qs python functools.partial` | 5 | functools.partial() | Exact expected callable-binding API is #1 with semantics and sample implementation. |
| 70 | `qs python operator.itemgetter` | 5 | operator.itemgetter() | Exact expected helper is #1 with behavior and example implementation. |
| 71 | `qs python math.isclose` | 5 | math.isclose() | Exact expected comparison function is #1 with concise behavior. |
| 72 | `qs python calculate median` | 5 | statistics.median() | Exact expected function is #1 with behavior and example. |
| 73 | `qs python random choices weights` | 4 | GeeksforGeeks weighted random choice | Expected random.choices behavior is covered at #1 and #2 with useful weights context, though official docs are absent. |
| 74 | `qs python secrets.token_urlsafe` | 5 | secrets.token_urlsafe() | Exact expected function is #1 with encoding and size context. |
| 75 | `qs python hashlib.sha256` | 5 | hashlib.sha256() | Exact expected constructor is #1. |
| 76 | `qs python uuid.uuid4` | 5 | uuid.uuid4() | Exact expected API is first with useful description. |
| 77 | `qs python base64 urlsafe encoding` | 2 | locale.getencoding() | Only generic encoding APIs appear; no URL-safe Base64 function. |
| 78 | `qs python build URL query string` | 0 | docstring | Results do not address building a URL query string or urllib.parse.urlencode(). |
| 79 | `qs python logging.basicConfig` | 5 | logging.basicConfig() | Exact expected function is first. |
| 80 | `qs python create temporary directory` | 5 | tempfile.TemporaryDirectory | Exact expected context manager is first. |
| 81 | `qs python copy directory tree` | 0 | xml.etree.ElementTree | Results confuse “tree” with XML and do not address directory copying. |
| 82 | `qs python deep copy object` | 2 | object.__copy__() | A shallow-copy hook is related, but copy.deepcopy() is absent and the result misses the task. |
| 83 | `qs python walk directory recursively` | 0 | Error: no web results for “python walk directory recursively” | No result was returned. |
| 84 | `qs python run command capture output` | 3 | subprocess.getoutput() | Top result can run a command and capture output, but the expected subprocess.run(capture_output=True) API is absent. |
| 85 | `qs python concurrent.futures.ThreadPoolExecutor` | 5 | concurrent.futures.ThreadPoolExecutor | Exact expected class is first with useful context. |
| 86 | `qs python multiprocessing Queue` | 5 | multiprocessing.Queue | Exact expected class is first with a clear description. |
| 87 | `qs python threading Lock` | 5 | threading.Lock | Exact expected synchronization API is first. |
| 88 | `qs python run async tasks concurrently` | 4 | test.support.threading_helper.run_concurrently(); “Running tasks concurrently” is #2 | The useful second result shows asyncio.gather() in a complete example, but an internal test helper outranks it. |
| 89 | `qs python asyncio timeout` | 4 | asyncio.Timeout; asyncio.timeout() is #2 | Expected API is second; the first result is closely related and even demonstrates asyncio.timeout(). |
| 90 | `qs python queue.PriorityQueue` | 5 | queue.PriorityQueue | Exact expected class is first with constructor details and an example. |
| 91 | `qs python socket.create_connection` | 5 | socket.create_connection() | Exact expected helper is first. |
| 92 | `qs python unittest.mock.patch` | 5 | unittest.mock.patch() | Exact expected API is first. |
| 93 | `qs python doctest` | 5 | doctest | Exact expected module is first. |
| 94 | `qs python warnings filterwarnings` | 5 | warnings.filterwarnings() | Exact expected function is first with useful behavior details. |
| 95 | `qs python importlib.resources.files` | 5 | importlib.resources.files() | Exact expected function is first. |
| 96 | `qs python inspect.signature` | 4 | inspect.Signature; inspect.signature() is #2 | Expected function is second; the first result has the same useful signature example. |
| 97 | `qs python trace memory allocations` | 0 | Error: no web results for “python trace memory allocations” | No result was returned. |
| 98 | `qs python typing.Protocol` | 5 | typing.Protocol | Exact expected class is first with structural-typing context. |
| 99 | `qs python typing.TypedDict` | 5 | typing.TypedDict | Exact expected class is first. |
| 100 | `qs python typing.Literal` | 5 | typing.Literal | Exact expected typing form is first. |

### Strongest patterns

- Exact qualified identifiers are excellent. Dotted APIs such as `pathlib.Path.read_text`, `collections.Counter.most_common`, and `typing.TypedDict` usually rank first with useful context.
- Direct built-in and standard-library names are reliable. Examples include `enumerate`, `json.dumps`, `heapq.nlargest`, and `socket.create_connection`.
- Formal concept names often work when the indexed title is close to the query. Exception chaining, assignment expressions, dataclasses, and MRO are strong examples.
- Most successful results include enough behavior, signature detail, or example text to act without another search.

### Weakest patterns

- Natural task queries often fail semantic mapping. `sort list by key`, `merge dictionaries`, `expand home directory`, `build URL query string`, and `copy directory tree` miss their intended APIs.
- Generic words can dominate phrase intent. “groups” maps to OS groups, “tree” maps to XML, and “encoding” maps to locale APIs.
- Some concept searches favor internal or low-level pages. `iterator protocol` returns the C API, and `run async tasks concurrently` puts an internal `test.support` helper first.
- Two searches fell through to web lookup and returned no result: `walk directory recursively` and `trace memory allocations`.
- One query crashed. `descriptor protocol` returned rc 101 with `end byte index ... is not a char boundary` inside the Unicode character `’`.

### Prioritized improvements

1. **Fix Unicode-safe snippet slicing.** Slice on character boundaries, not raw byte offsets. Add a regression test with curly quotes so `descriptor protocol` cannot panic.
2. **Add task-to-API synonym expansion.** Map common verbs and phrases such as “expand home,” “copy directory tree,” “URL query string,” “merge dictionaries,” and “parse ISO date time” to canonical APIs.
3. **Use phrase and proximity scoring.** Keep multiword intent together. This should stop isolated tokens such as “group,” “tree,” “key,” and “encoding” from overwhelming the requested concept.
4. **Improve document-tier ranking and section anchors.** Prefer public Python APIs and language-reference sections over C API pages, `test.support`, generic module pages, and third-party web results. Show the matched signature or local example in the snippet.
5. **Make fallback retrieval resilient.** If local matching is weak or web search returns no results, retry normalized variants and known doc aliases before failing. Preserve a useful local candidate instead of returning an empty result.

### Failure evidence

- `qs python descriptor protocol`: rc 101; `end byte index 342814 is not a char boundary; it is inside ’`.
- `qs python walk directory recursively`: rc 1; `qs: no web results for python walk directory recursively`.
- `qs python trace memory allocations`: rc 1; `qs: no web results for python trace memory allocations`.
- `qs python regular expression named groups`: top result was `os.getgroups()`; the expected `(?P<name>...)` syntax did not appear.
- `qs python copy directory tree`: top result was `xml.etree.ElementTree`; `shutil.copytree()` did not appear.

## C++

### Summary

- Queries run: **100/100**
- Average score: **3.87 / 5**
- Median score: **5**
- Pass rate (score >=3): **83% (83/100)**
- Strong pass rate (score >=4): **78% (78/100)**
- Command errors/timeouts: **0**
- Distribution: **0: 11, 1: 4, 2: 2, 3: 5, 4: 26, 5: 52**

Each checklist command was run separately through the installed `qs` from `PATH`. A 25-second process timeout was applied per command. Scoring uses the requested practical-developer rubric; it does not use the checklist's expected field as execution evidence.

### All results

| # | Command | Score | Observed top result/error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs cpp vector` | **4** | #1 “vector” | Header is #1; the `std::vector` class page is #2 with a clear signature and definition. |
| 2 | `qs cpp unordered maps` | **0** | #1 “std::mdspan::mapping” | No map hit. All five results are `mdspan`/layout mapping pages; top is `std::mdspan::mapping`. |
| 3 | `qs cpp std::array` | **5** | #1 “std::array” | Exact class is #1 with its template signature and fixed-size-array definition. |
| 4 | `qs cpp forward lists` | **4** | #1 “forward_list” | Header is #1 and the exact class is #2; declarations provide useful context. |
| 5 | `qs cpp priority_queue` | **5** | #1 “std::priority_queue” | Exact adaptor is #1 with signature, behavior, and example context. |
| 6 | `qs cpp multisets` | **5** | #1 “std::multiset” | Exact class is the sole result and includes a useful template signature. |
| 7 | `qs cpp map::try_emplace` | **5** | #1 “std::map::try_emplace” | Exact member is #1; snippet and example cover emplacement behavior. |
| 8 | `qs cpp vector capacity` | **5** | #1 “std::vector::capacity” | Exact member is #1 with signature, reallocation meaning, and example. |
| 9 | `qs cpp span` | **4** | #1 “span” | Header is #1 and exact class is #2; useful declarations are immediate. |
| 10 | `qs cpp mdspan` | **4** | #1 “mdspan” | Header is #1 and exact class is #2; modern API is easy to reach. |
| 11 | `qs cpp tuples` | **4** | #1 “tuple” | Header is #1 and exact class is #2; declaration context is useful. |
| 12 | `qs cpp std::variant` | **5** | #1 “std::variant” | Exact class is #1 with a concise type-safe-union explanation. |
| 13 | `qs cpp optional value` | **2** | #1 “std::optional::value” | Returns `std::optional::value`, not the `std::optional` type; relevant but narrows the query incorrectly. |
| 14 | `qs cpp any_cast` | **5** | #1 “std::any_cast” | Exact cast API is #1 with headers and example context. |
| 15 | `qs cpp bitsets` | **4** | #1 “bitset” | Header is #1 and exact class is #2; useful declaration shown. |
| 16 | `qs cpp sort` | **5** | #1 “std::sort” | Exact algorithm is #1 and includes an example, though the leading snippet is a constraint fragment. |
| 17 | `qs cpp stable_sort` | **5** | #1 “std::stable_sort” | Exact algorithm is #1 with example context. |
| 18 | `qs cpp lower_bound` | **5** | #1 “std::lower_bound” | Exact algorithm is #1 with a clear behavioral summary and example. |
| 19 | `qs cpp ranges::filter_view` | **5** | #1 “std::ranges::filter_view” | Exact range view is #1 with deduction-guide and example context. |
| 20 | `qs cpp std::ranges::sort` | **5** | #1 “std::ranges::sort” | Exact ranges algorithm is #1 with definition and example. |
| 21 | `qs cpp transform_reduce` | **5** | #1 “std::transform_reduce” | Exact numeric algorithm is #1; example is present, but snippet starts with a constraint fragment. |
| 22 | `qs cpp partition point` | **5** | #1 “std::partition_point” | Exact algorithm is #1 with precondition and example. |
| 23 | `qs cpp nth_element` | **5** | #1 “std::nth_element” | Exact algorithm is #1 with examples; constrained counterpart is #2. |
| 24 | `qs cpp heap algorithms` | **3** | #1 “Algorithms library” | Only broad algorithm index pages appear. They are relevant, but no heap operation is surfaced. |
| 25 | `qs cpp set intersection` | **5** | #1 “std::set_intersection” | Exact algorithm is #1 with behavior and example. |
| 26 | `qs cpp accumulate` | **5** | #1 “std::accumulate” | Exact numeric algorithm is #1 with include and example context. |
| 27 | `qs cpp remove_if` | **4** | #1 “remove_if” | Container member `remove_if` pages occupy #1–2; expected `std::remove_if` is #3 and still usable. |
| 28 | `qs cpp binary search` | **5** | #1 “std::binary_search” | Exact algorithm is #1 with semantics and example. |
| 29 | `qs cpp shuffle range` | **4** | #1 “std::ranges::shuffle” | `std::ranges::shuffle` is #1 with an example. It is a useful modern counterpart, not the requested classic API. |
| 30 | `qs cpp unique_ptr` | **5** | #1 “std::unique_ptr” | Exact smart pointer is #1 with ownership explanation and full template signature. |
| 31 | `qs cpp make shared` | **5** | #1 “std::make_shared” | Exact factory is #1 with signature; irrelevant `shared_mutex` at #2 does not block it. |
| 32 | `qs cpp weak pointers` | **0** | #1 “hazard_pointer” | No `std::weak_ptr` hit. Top result is unrelated `hazard_pointer` and only two results are returned. |
| 33 | `qs cpp std::move` | **4** | #1 “std::move” | Top result is the algorithm `std::move`, not the cast utility; the utility overload is #2. |
| 34 | `qs cpp perfect forwarding` | **4** | #1 “forward_list” | Top result is unrelated `forward_list`; `std::forward` is #2 and provides the practical API. |
| 35 | `qs cpp allocator_traits` | **5** | #1 “std::allocator_traits” | Exact traits type is #1 with definition and signature. |
| 36 | `qs cpp polymorphic memory resource` | **4** | #1 “memory_resource” | Header is #1 and exact `std::pmr::memory_resource` is #2; namespace declarations are useful. |
| 37 | `qs cpp reference_wrapper` | **5** | #1 “std::reference_wrapper” | Exact wrapper is #1 with purpose, signature, and example. |
| 38 | `qs cpp type traits` | **5** | #1 “Type traits” | Exact library overview is #1 and the header is #2. |
| 39 | `qs cpp std::function` | **5** | #1 “std::function” | Exact callable wrapper is #1 with strong description and signature. |
| 40 | `qs cpp string views` | **4** | #1 “string_view” | Header is #1; `std::basic_string_view` is #2. Useful, but the common alias is not titled directly. |
| 41 | `qs cpp from_chars` | **5** | #1 “std::from_chars” | Exact conversion API is #1 with success semantics, include, and example. |
| 42 | `qs cpp format string` | **5** | #1 “std::format_string” | `std::format_string` is #1 with aliases and signature. This directly answers the format-string wording. |
| 43 | `qs cpp regex_match` | **5** | #1 “std::regex_match” | Exact regex function is #1 with include and example context. |
| 44 | `qs cpp getline` | **5** | #1 “std::getline” | Exact helper is #1; snippet highlights a common newline pitfall and includes an example. |
| 45 | `qs cpp stringstreams` | **5** | #1 “std::basic_stringstream” | Exact primary stream type is #1 with description and template signature. |
| 46 | `qs cpp filesystem path` | **5** | #1 “std::filesystem::path” | Exact path type is #1 with a clear semantic description. |
| 47 | `qs cpp recursive_directory_iterator` | **5** | #1 “std::filesystem::recursive_directory_iterator” | Exact recursive iterator is #1 with behavior, include, and sample namespace setup. |
| 48 | `qs cpp ifstream` | **5** | #1 “std::basic_ifstream” | Expected primary type is #1 with a clear description and template signature. |
| 49 | `qs cpp write binary file` | **1** | #1 “std::FILE” | No `ofstream` or binary-writing result. Top `std::FILE` is only tangentially related. |
| 50 | `qs cpp std::print` | **5** | #1 “std::print” | Exact C++23 API is #1 with include and example context. |
| 51 | `qs cpp source_location` | **4** | #1 “source_location” | Header is #1 and exact type is #2; declaration is immediately useful. |
| 52 | `qs cpp stacktrace` | **4** | #1 “stacktrace” | Header is #1 and `std::basic_stacktrace` is #2; useful but alias is not directly titled. |
| 53 | `qs cpp error codes` | **5** | #1 “std::error_code” | Exact error type is #1 with a clear platform-error explanation. |
| 54 | `qs cpp exception hierarchy` | **5** | #1 “std::exception - cppreference.com” | Web fallback ranks cppreference's `std::exception` page #1 and its snippet explains the hierarchy. |
| 55 | `qs cpp RAII` | **5** | #1 “RAII” | Exact language concept is #1 with a strong definition and code example. |
| 56 | `qs cpp move semantics` | **0** | #1 “std::contracts::evaluation_semantic” | No move-semantics page appears. The only result is unrelated `std::contracts::evaluation_semantic`. |
| 57 | `qs cpp copy elision` | **5** | #1 “Copy elision” | Exact language page is #1 with definition and example. |
| 58 | `qs cpp rule of five` | **5** | #1 “The rule of three/five/zero” | Exact rule-of-three/five/zero page is #1 with useful guidance and code. |
| 59 | `qs cpp structured bindings` | **5** | #1 “Structured binding declaration” | Exact language page is #1 with a direct definition and example. |
| 60 | `qs cpp lambda captures` | **5** | #1 “Lambda Capture Clause in C++ - GeeksforGeeks” | Top web result directly explains capture clauses; cppreference is #2 for authoritative detail. |
| 61 | `qs cpp generic lambdas` | **4** | #1 “Lambda expressions” | General lambda page is #1 and its generated call-operator snippet supports generic lambdas, but title is broad. |
| 62 | `qs cpp template specializations` | **5** | #1 “Partial template specialization” | Partial specialization is #1 and explicit specialization is #2, covering both requested forms. |
| 63 | `qs cpp concepts requires clause` | **5** | #1 “requires expression (since C++20) - cppreference.com” | Requires expression is #1 and constraints/concepts is #3; combined results answer the query well. |
| 64 | `qs cpp fold expressions` | **4** | #1 “Fold expressions” | Exact page is #1, but its snippet is empty, so the result gives little context before opening. |
| 65 | `qs cpp constexpr functions` | **0** | #1 “Functions” | No constexpr-specific hit. Top is the generic `Functions` page and the remaining hits are unrelated. |
| 66 | `qs cpp consteval` | **4** | #1 “keyword: consteval” | Keyword page is #1 with boilerplate only; useful `consteval specifier` page is #2 with a definition. |
| 67 | `qs cpp decltype(auto)` | **1** | #1 “keyword: auto” | Only `keyword: auto` appears. It does not explain or even name `decltype(auto)` in the result. |
| 68 | `qs cpp type deduction` | **3** | #1 “Template argument deduction” | Template argument deduction is #1, but the result set omits `auto`; only half the expected topic is covered. |
| 69 | `qs cpp range based for` | **5** | #1 “Range-based for loop” | Exact language feature is #1 with a practical definition and example. |
| 70 | `qs cpp coroutines` | **5** | #1 “Coroutines” | Exact language topic is #1 with definition, include, and a substantial example. |
| 71 | `qs cpp modules` | **5** | #1 “Modules” | Exact modules page is #1 with explanatory text and module syntax. |
| 72 | `qs cpp three way comparison` | **4** | #1 “Comparison operators” | Comparison-operators page is #1 and relevant to `<=>`, but the snippet shows older two-way comparison text. |
| 73 | `qs cpp threads` | **4** | #1 “thread” | Header is #1 and exact `std::thread` is #2; declarations are useful. |
| 74 | `qs cpp jthread stop token` | **5** | #1 “std::jthread::get_stop_token” | `std::jthread::get_stop_token` is #1 with exact signature and explains the shared stop state. |
| 75 | `qs cpp mutexes` | **4** | #1 “mutex” | Header is #1 and exact `std::mutex` is #2; family declarations add useful context. |
| 76 | `qs cpp condition variables` | **4** | #1 “condition_variable” | Header is #1 and exact class is #2; declarations include related condition-variable APIs. |
| 77 | `qs cpp atomic compare exchange` | **4** | #1 “std::atomic_compare_exchange_weak” | Free `atomic_compare_exchange_weak` is #1 and member weak overload is #2; strong form is not surfaced in titles. |
| 78 | `qs cpp memory ordering` | **5** | #1 “std::memory_order” | Exact memory-order type is #1 with all enum values shown. |
| 79 | `qs cpp futures` | **4** | #1 “future” | Header is #1 and exact `std::future` is #2; the error enum is useful context but delays the class. |
| 80 | `qs cpp async launch policies` | **0** | #1 “execution-policy” | No `std::async` or `std::launch`. The sole result is unrelated parallel-algorithm `execution-policy`. |
| 81 | `qs cpp counting_semaphore` | **5** | #1 “std::counting_semaphore” | Exact semaphore is #1 with purpose, signature, include, and example. |
| 82 | `qs cpp latches` | **4** | #1 “latch” | Header is #1 and exact `std::latch` is #2; declaration is useful. |
| 83 | `qs cpp barriers` | **4** | #1 “barrier” | Header is #1 and exact `std::barrier` is #2; declaration is useful. |
| 84 | `qs cpp duration_cast` | **5** | #1 “std::chrono::duration_cast” | Exact chrono conversion API is #1 with constraint, include, and example. |
| 85 | `qs cpp steady clock` | **5** | #1 “std::chrono::steady_clock” | Exact monotonic clock is #1 with a strong explanation; `now` is #2. |
| 86 | `qs cpp calendar dates` | **2** | #1 “Date and time library” | Top result is only the broad chrono index. No `year_month_day` appears, and later hits drift to locale/C time. |
| 87 | `qs cpp time zones` | **5** | #1 “std::chrono::time_zone” | Both expected APIs are ranked #1 and #2 with clear definitions. |
| 88 | `qs cpp complex numbers` | **0** | #1 “numbers” | No `std::complex`. Top is the unrelated `<numbers>` header, followed by error numbers. |
| 89 | `qs cpp gcd lcm` | **3** | #1 “std::lcm” | Only `std::lcm` is returned. It is useful for half the query, but `std::gcd` is absent. |
| 90 | `qs cpp random number engine` | **4** | #1 “named requirements: RandomNumberEngine” | The exact RandomNumberEngine requirement is #1 with a useful definition, but common engines such as `mt19937` are absent. |
| 91 | `qs cpp uniform_int_distribution` | **5** | #1 “std::uniform_int_distribution” | Exact distribution is #1 with signature, probability description, include, and example. |
| 92 | `qs cpp remove elements from vector` | **1** | #1 “vector” | Returns generic vector pages only. Nothing explains erase-remove or `std::erase_if`. |
| 93 | `qs cpp erase map key` | **0** | #1 “std::map::key_comp” | No erase API. Top `std::map::key_comp` is misleading; other hits are comparison functions. |
| 94 | `qs cpp read whole file` | **0** | #1 “std::FILE” | No whole-file reading method or `ifstream`. Results are the same unrelated file-category pages as the write query. |
| 95 | `qs cpp split string delimiter` | **3** | #1 “How to Split a String by a Delimiter in C++? - GeeksforGeeks” | Web results give practical delimiter-splitting methods, but none surface the expected `std::views::split` API. |
| 96 | `qs cpp convert string to integer` | **0** | #1 “Integer literal” | No conversion API. Top `Integer literal` and `numeric_limits::is_integer` do not answer the task. |
| 97 | `qs cpp sort custom objects` | **1** | #1 “Function objects” | No sorting result. `Function objects` is only weak comparator context; other results are generic object pages. |
| 98 | `qs cpp find duplicate values` | **0** | #1 “Value categories” | No duplicate-detection approach. Top `Value categories` is unrelated to the task. |
| 99 | `qs cpp measure elapsed time` | **0** | #1 “LC_TIME” | No elapsed-time technique. Top `LC_TIME` and legacy time formatting APIs are misleading. |
| 100 | `qs cpp visit variant` | **3** | #1 “std::variant::visit” | Top is the C++26 member `std::variant::visit`, not the expected nonmember `std::visit`; still relevant but version-sensitive. |

### Strongest patterns

- Exact qualified symbols and distinctive snake_case names are excellent. Examples include `std::array`, `map::try_emplace`, `ranges::filter_view`, `transform_reduce`, `recursive_directory_iterator`, and `uniform_int_distribution`.
- Most direct library lookups include useful signatures, headers, semantics, or examples. This makes 52 queries immediate score-5 answers.
- Spaced normalization works well for many APIs: `partition point`, `set intersection`, `make shared`, `steady clock`, and `range based for`.
- Language-topic pages rank well when the topic maps to a dedicated cppreference page: RAII, copy elision, rule of five, structured bindings, coroutines, and modules.
- Web fallback helps when the local index match is weak. `exception hierarchy`, `lambda captures`, and `concepts requires clause` returned useful cppreference or explanatory pages.

### Weakest patterns

- Plural normalization can cause severe token confusion. `unordered maps` returns `std::mdspan::mapping`; `weak pointers` returns `hazard_pointer`; `complex numbers` returns `<numbers>`.
- Natural task queries often degrade into individual-token matches. `erase map key` returns `std::map::key_comp`; `measure elapsed time` returns `LC_TIME`; `convert string to integer` returns `Integer literal`.
- Multi-term concepts are sometimes reduced to one generic term. `move semantics` returns `evaluation_semantic`; `constexpr functions` returns `Functions`; `async launch policies` returns algorithm `execution-policy`.
- File tasks do not route well. Both `write binary file` and `read whole file` return `std::FILE` first and never surface `ofstream`/`ifstream` techniques.
- Header pages frequently outrank their exact types. These remain usable, but they turn many potential 5s into 4s. Examples include `vector`, `span`, `thread`, `mutexes`, `latches`, and `barriers`.

### Prioritized improvements

1. **Add phrase and task aliases before token search.** Map common intents such as `convert string to integer` → `stoi`/`from_chars`, `measure elapsed time` → `steady_clock`, `erase map key` → `map::erase`, and file read/write phrases → stream APIs.
2. **Improve morphological normalization with symbol-aware safeguards.** Singularize `maps`, `pointers`, and `numbers` without letting substring matches such as `mapping`, `hazard_pointer`, or `<numbers>` dominate.
3. **Require multi-token coverage and penalize one-token drift.** This would suppress `evaluation_semantic` for `move semantics`, generic `Functions` for `constexpr functions`, and `execution-policy` for `async launch policies`.
4. **Rank entity pages above header pages when the query names an entity.** Prefer `std::vector`, `std::span`, `std::thread`, `std::mutex`, `std::latch`, and `std::barrier`; retain headers as the next result.
5. **Add concept-family and companion-symbol expansion.** Surface `make_heap` for heap algorithms, `year_month_day` for calendar dates, both `gcd` and `lcm`, common engines such as `mt19937`, erase-remove/`erase_if`, and the nonmember `std::visit`.

## Rust

### Summary

- Queries executed: **100/100**
- Average score: **1.78/5**
- Median score: **1/5**
- Pass rate, score >=3: **28/100 (28%)**
- Strong pass rate, score >=4: **2/100 (2%)**
- Command errors/timeouts: **0**
- Distribution: **0: 0, 1: 54, 2: 18, 3: 26, 4: 0, 5: 2**

Each checklist command was run through the installed `qs` executable. Each process had a 15-second timeout. Scoring uses the result list printed by `qs`; it does not assume that a generic linked search page will produce the expected item after another interaction.

### All query results

| # | Command | Score | Observed top result/error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs rust ownership` | 1 | std search: ownership | It only echoes the query into std search; it does not surface Ownership, moves, and scope. or useful context. |
| 2 | `qs rust borrow checker` | 1 | std search: borrow checker | It only echoes the query into std search; it does not surface Borrowing rules and the borrow checker. or useful context. |
| 3 | `qs rust lifetimes` | 1 | std search: lifetimes | It only echoes the query into std search; it does not surface Lifetime annotations and lifetime elision. or useful context. |
| 4 | `qs rust mutable references` | 1 | std search: mutable references | It only echoes the query into std search; it does not surface Mutable borrows written as &mut T. or useful context. |
| 5 | `qs rust move closures` | 1 | std search: move closures | It only echoes the query into std search; it does not surface move closures and captured values. or useful context. |
| 6 | `qs rust pattern matching` | 1 | std search: pattern matching | It only echoes the query into std search; it does not surface Patterns and match expressions. or useful context. |
| 7 | `qs rust struct destructuring` | 1 | std search: struct destructuring | It only echoes the query into std search; it does not surface Destructuring patterns for structs. or useful context. |
| 8 | `qs rust if let` | 1 | std search: if let | It only echoes the query into std search; it does not surface if let conditional pattern matching. or useful context. |
| 9 | `qs rust let else` | 1 | std search: let else | It only echoes the query into std search; it does not surface let...else refutable pattern binding. or useful context. |
| 10 | `qs rust while let` | 1 | std search: while let | It only echoes the query into std search; it does not surface while let conditional loops. or useful context. |
| 11 | `qs rust traits` | 1 | std search: traits | It only echoes the query into std search; it does not surface Trait definitions and implementations. or useful context. |
| 12 | `qs rust dyn Trait` | 1 | std search: dyn Trait | It only echoes the query into std search; it does not surface Trait objects written as dyn Trait. or useful context. |
| 13 | `qs rust associated types` | 1 | std search: associated types | It only echoes the query into std search; it does not surface Trait associated types. or useful context. |
| 14 | `qs rust generic functions` | 1 | std search: generic functions | It only echoes the query into std search; it does not surface Generic type parameters on functions. or useful context. |
| 15 | `qs rust where clauses` | 1 | std search: where clauses | It only echoes the query into std search; it does not surface where clauses for generic bounds. or useful context. |
| 16 | `qs rust const generics` | 1 | std search: const generics | It only echoes the query into std search; it does not surface Const generic parameters. or useful context. |
| 17 | `qs rust modules` | 1 | std search: modules | It only echoes the query into std search; it does not surface The module system and mod items. or useful context. |
| 18 | `qs rust pub(crate)` | 1 | std search: pub(crate) | It only echoes the query into std search; it does not surface Crate-scoped visibility with pub(crate). or useful context. |
| 19 | `qs rust use declarations` | 1 | std search: use declarations | It only echoes the query into std search; it does not surface use declarations and imported names. or useful context. |
| 20 | `qs rust external crates` | 1 | std search: external crates | It only echoes the query into std search; it does not surface External crates and the extern prelude. or useful context. |
| 21 | `qs rust attributes` | 1 | std search: attributes | It only echoes the query into std search; it does not surface Outer and inner attributes. or useful context. |
| 22 | `qs rust derive macros` | 1 | std search: derive macros | It only echoes the query into std search; it does not surface The derive attribute and derivable traits. or useful context. |
| 23 | `qs rust macro_rules!` | 1 | std search: macro_rules! | It only echoes the query into std search; it does not surface Declarative macros defined with macro_rules!. or useful context. |
| 24 | `qs rust procedural macros` | 1 | std search: procedural macros | It only echoes the query into std search; it does not surface Function-like, derive, and attribute procedural macros. or useful context. |
| 25 | `qs rust async fn` | 1 | std search: async fn | It only echoes the query into std search; it does not surface Asynchronous functions and returned futures. or useful context. |
| 26 | `qs rust .await` | 1 | std search: .await | It only echoes the query into std search; it does not surface The postfix .await expression. or useful context. |
| 27 | `qs rust std::pin::Pin` | 5 | Pin in std::pin - Rust | Exact Pin page is first, with a clear one-line guarantee. Immediately useful. |
| 28 | `qs rust Send and Sync` | 1 | std search: Send and Sync | It only echoes the query into std search; it does not surface The Send and Sync marker traits. or useful context. |
| 29 | `qs rust unsafe Rust` | 1 | std search: unsafe Rust | It only echoes the query into std search; it does not surface Unsafe operations and unsafe blocks. or useful context. |
| 30 | `qs rust raw pointers` | 1 | std search: raw pointers | It only echoes the query into std search; it does not surface Raw pointer types *const T and *mut T. or useful context. |
| 31 | `qs rust foreign function interface` | 1 | std search: foreign function interface | It only echoes the query into std search; it does not surface FFI with extern blocks and ABI strings. or useful context. |
| 32 | `qs rust recoverable errors` | 1 | std search: recoverable errors | It only echoes the query into std search; it does not surface Recoverable error handling with Result. or useful context. |
| 33 | `qs rust question mark operator` | 1 | std search: question mark operator | It only echoes the query into std search; it does not surface The ? error-propagation operator. or useful context. |
| 34 | `qs rust std::panic::set_hook` | 5 | set_hook in std::panic - Rust | Exact set_hook function page is first, with a clear action summary. Immediately useful. |
| 35 | `qs rust Result<T, E>` | 3 | std search: Result<T, E> | The std search preserves enough exact API terms to find The Result<T, E> enum., but the top row is generic and has no signature or example. |
| 36 | `qs rust Option::and_then` | 3 | Standard library search | The std search preserves enough exact API terms to find Option::and_then combinator., but the top row is generic and has no signature or example. |
| 37 | `qs rust Vec::push` | 3 | Standard library search | The std search preserves enough exact API terms to find Vec::push., but the top row is generic and has no signature or example. |
| 38 | `qs rust VecDeque` | 3 | std search: VecDeque | The std search preserves enough exact API terms to find std::collections::VecDeque., but the top row is generic and has no signature or example. |
| 39 | `qs rust HashMap::entry` | 3 | Standard library search | The std search preserves enough exact API terms to find HashMap::entry and the entry API., but the top row is generic and has no signature or example. |
| 40 | `qs rust BTreeMap` | 3 | std search: BTreeMap | The std search preserves enough exact API terms to find std::collections::BTreeMap., but the top row is generic and has no signature or example. |
| 41 | `qs rust hash sets` | 1 | std search: hash sets | It only echoes the query into std search; it does not surface std::collections::HashSet. or useful context. |
| 42 | `qs rust binary heaps` | 1 | std search: binary heaps | It only echoes the query into std search; it does not surface std::collections::BinaryHeap. or useful context. |
| 43 | `qs rust String::from_utf8` | 3 | Standard library search | The std search preserves enough exact API terms to find String::from_utf8 and FromUtf8Error., but the top row is generic and has no signature or example. |
| 44 | `qs rust str::split_whitespace` | 3 | std::str - Rust | The str module page is relevant, but split_whitespace is not the top page and no signature is shown. |
| 45 | `qs rust format!` | 3 | std search: format! | The std search preserves enough exact API terms to find The format! macro., but the top row is generic and has no signature or example. |
| 46 | `qs rust print a line` | 1 | std search: print a line | It only echoes the query into std search; it does not surface The println! macro. or useful context. |
| 47 | `qs rust Iterator::map` | 3 | Standard library search | The std search preserves enough exact API terms to find Iterator::map., but the top row is generic and has no signature or example. |
| 48 | `qs rust collect iterator into vector` | 1 | std search: collect iterator into vector | It only echoes the query into std search; it does not surface Iterator::collect with Vec type inference. or useful context. |
| 49 | `qs rust fold values` | 2 | std search: fold values | The std search has related keywords, but it does not identify Iterator::fold.; ranking and task guidance are weak. |
| 50 | `qs rust IntoIterator` | 3 | std search: IntoIterator | The std search preserves enough exact API terms to find The IntoIterator trait and into_iter., but the top row is generic and has no signature or example. |
| 51 | `qs rust ExactSizeIterator` | 3 | std search: ExactSizeIterator | The std search preserves enough exact API terms to find The ExactSizeIterator trait., but the top row is generic and has no signature or example. |
| 52 | `qs rust From and Into` | 2 | std search: From and Into | The std search has related keywords, but it does not identify The From and Into conversion traits.; ranking and task guidance are weak. |
| 53 | `qs rust TryFrom` | 3 | std search: TryFrom | The std search preserves enough exact API terms to find The TryFrom fallible conversion trait., but the top row is generic and has no signature or example. |
| 54 | `qs rust AsRef<Path>` | 3 | std search: AsRef<Path> | The std search preserves enough exact API terms to find AsRef<Path> bounds and borrowed conversions., but the top row is generic and has no signature or example. |
| 55 | `qs rust Cow<'a, str>` | 3 | std search: Cow<'a, str> | The std search preserves enough exact API terms to find std::borrow::Cow clone-on-write type., but the top row is generic and has no signature or example. |
| 56 | `qs rust Box<T>` | 3 | std search: Box<T> | The std search preserves enough exact API terms to find std::boxed::Box<T>., but the top row is generic and has no signature or example. |
| 57 | `qs rust reference counted pointer` | 2 | std search: reference counted pointer | The std search has related keywords, but it does not identify std::rc::Rc.; ranking and task guidance are weak. |
| 58 | `qs rust atomic reference counting` | 2 | std search: atomic reference counting | The std search has related keywords, but it does not identify std::sync::Arc.; ranking and task guidance are weak. |
| 59 | `qs rust RefCell::borrow_mut` | 3 | Standard library search | The std search preserves enough exact API terms to find RefCell::borrow_mut and runtime borrow checking., but the top row is generic and has no signature or example. |
| 60 | `qs rust mutexes` | 2 | std search: mutexes | The std search has related keywords, but it does not identify std::sync::Mutex.; ranking and task guidance are weak. |
| 61 | `qs rust RwLock` | 3 | std search: RwLock | The std search preserves enough exact API terms to find std::sync::RwLock., but the top row is generic and has no signature or example. |
| 62 | `qs rust OnceLock::get_or_init` | 3 | Standard library search | The std search preserves enough exact API terms to find OnceLock::get_or_init., but the top row is generic and has no signature or example. |
| 63 | `qs rust spawn a thread` | 2 | std search: spawn a thread | The std search has related keywords, but it does not identify std::thread::spawn.; ranking and task guidance are weak. |
| 64 | `qs rust mpsc channels` | 2 | std search: mpsc channels | The std search has related keywords, but it does not identify std::sync::mpsc channels.; ranking and task guidance are weak. |
| 65 | `qs rust AtomicUsize ordering` | 2 | std search: AtomicUsize ordering | The std search has related keywords, but it does not identify AtomicUsize operations and Ordering.; ranking and task guidance are weak. |
| 66 | `qs rust read file to string` | 1 | std search: read file to string | It only echoes the query into std search; it does not surface std::fs::read_to_string. or useful context. |
| 67 | `qs rust write bytes to a file` | 1 | std search: write bytes to a file | It only echoes the query into std search; it does not surface std::fs::write. or useful context. |
| 68 | `qs rust canonicalize a path` | 2 | std search: canonicalize a path | The std search has related keywords, but it does not identify std::fs::canonicalize.; ranking and task guidance are weak. |
| 69 | `qs rust create directories recursively` | 1 | std search: create directories recursively | It only echoes the query into std search; it does not surface std::fs::create_dir_all. or useful context. |
| 70 | `qs rust run command and capture output` | 1 | std search: run command and capture output | It only echoes the query into std search; it does not surface std::process::Command::output. or useful context. |
| 71 | `qs rust environment variables` | 2 | std search: environment variables | The std search has related keywords, but it does not identify std::env::var and vars.; ranking and task guidance are weak. |
| 72 | `qs rust command line arguments` | 2 | std search: command line arguments | The std search has related keywords, but it does not identify std::env::args and args_os.; ranking and task guidance are weak. |
| 73 | `qs rust TcpListener::bind` | 3 | Standard library search | The std search preserves enough exact API terms to find std::net::TcpListener::bind., but the top row is generic and has no signature or example. |
| 74 | `qs rust parse socket address` | 2 | std search: parse socket address | The std search has related keywords, but it does not identify std::net::SocketAddr and FromStr.; ranking and task guidance are weak. |
| 75 | `qs rust Duration::from_secs` | 3 | Standard library search | The std search preserves enough exact API terms to find std::time::Duration::from_secs., but the top row is generic and has no signature or example. |
| 76 | `qs rust measure elapsed time` | 2 | std search: measure elapsed time | The std search has related keywords, but it does not identify std::time::Instant::elapsed.; ranking and task guidance are weak. |
| 77 | `qs rust unix timestamp` | 2 | std search: unix timestamp | The std search has related keywords, but it does not identify SystemTime relative to UNIX_EPOCH.; ranking and task guidance are weak. |
| 78 | `qs rust sort vectors` | 2 | std search: sort vectors | The std search has related keywords, but it does not identify Slice sorting methods such as sort and sort_by.; ranking and task guidance are weak. |
| 79 | `qs rust binary_search` | 3 | std search: binary_search | The std search preserves enough exact API terms to find Slice method binary_search., but the top row is generic and has no signature or example. |
| 80 | `qs rust parse integer` | 2 | std search: parse integer | The std search has related keywords, but it does not identify str::parse with integer types and ParseIntError.; ranking and task guidance are weak. |
| 81 | `qs rust f64::total_cmp` | 3 | std::f64 - Rust | The f64 module page is related, but total_cmp is not surfaced and no method signature appears. |
| 82 | `qs rust inclusive ranges` | 2 | std search: inclusive ranges | The std search has related keywords, but it does not identify RangeInclusive and ..= syntax.; ranking and task guidance are weak. |
| 83 | `qs rust chunks_exact_mut` | 3 | std search: chunks_exact_mut | The std search preserves enough exact API terms to find Slice method chunks_exact_mut., but the top row is generic and has no signature or example. |
| 84 | `qs rust fixed size arrays` | 2 | std search: fixed size arrays | The std search has related keywords, but it does not identify Array types [T; N] and array operations.; ranking and task guidance are weak. |
| 85 | `qs rust tuple structs` | 1 | std search: tuple structs | It only echoes the query into std search; it does not surface Tuple struct declarations and construction. or useful context. |
| 86 | `qs rust named field structs` | 1 | std search: named field structs | It only echoes the query into std search; it does not surface Structs with named fields. or useful context. |
| 87 | `qs rust enums with data` | 1 | std search: enums with data | It only echoes the query into std search; it does not surface Enum variants containing fields. or useful context. |
| 88 | `qs rust impl blocks` | 1 | std search: impl blocks | It only echoes the query into std search; it does not surface Inherent implementations using impl. or useful context. |
| 89 | `qs rust Drop::drop` | 3 | Standard library search | The std search preserves enough exact API terms to find The Drop trait and drop method., but the top row is generic and has no signature or example. |
| 90 | `qs rust Default::default` | 3 | Standard library search | The std search preserves enough exact API terms to find The Default trait and default method., but the top row is generic and has no signature or example. |
| 91 | `qs rust Clone vs Copy` | 1 | std search: Clone vs Copy | It only echoes the query into std search; it does not surface Semantics of the Clone and Copy traits. or useful context. |
| 92 | `qs rust derive PartialEq and Ord` | 1 | std search: derive PartialEq and Ord | It only echoes the query into std search; it does not surface Derived equality and ordering traits. or useful context. |
| 93 | `qs rust unit tests` | 1 | std search: unit tests | It only echoes the query into std search; it does not surface #[test] functions and the test harness. or useful context. |
| 94 | `qs rust documentation tests` | 1 | std search: documentation tests | It only echoes the query into std search; it does not surface Rustdoc code examples run as doctests. or useful context. |
| 95 | `qs rust Cargo features` | 1 | std search: Cargo features | It only echoes the query into std search; it does not surface Cargo feature flags and optional dependencies. or useful context. |
| 96 | `qs rust cfg target_os` | 1 | std search: cfg target_os | It only echoes the query into std search; it does not surface Conditional compilation with #[cfg(target_os = ...)]. or useful context. |
| 97 | `qs rust no_std` | 1 | std search: no_std | It only echoes the query into std search; it does not surface The #![no_std] attribute and core. or useful context. |
| 98 | `qs rust intra-doc links` | 1 | std search: intra-doc links | It only echoes the query into std search; it does not surface Rustdoc intra-doc link syntax. or useful context. |
| 99 | `qs rust Rust 2024 edition` | 1 | std search: Rust 2024 edition | It only echoes the query into std search; it does not surface The Rust 2024 Edition Guide and migration changes. or useful context. |
| 100 | `qs rust raw identifiers` | 1 | std search: raw identifiers | It only echoes the query into std search; it does not surface Raw identifiers such as r#type. or useful context. |

### Strongest patterns

- Fully qualified entries with explicit routing are excellent. `std::pin::Pin` returns **“Pin in std::pin - Rust”** first and describes the pinning guarantee. `std::panic::set_hook` returns the exact function and says it **“Registers a custom panic hook”**.
- Exact standard-library identifiers usually retain useful tokens. Queries such as `Vec::push`, `HashMap::entry`, `OnceLock::get_or_init`, and `Duration::from_secs` lead to a focused std search rather than losing the type or method name.
- Punctuation and casing are encoded safely. The run had no command errors for `pub(crate)`, `.await`, generic forms, lifetimes, macro exclamation marks, or snake_case names.

### Weakest patterns

- The dominant fallback is not a real ranked answer. In 54 cases the only top result was an echo such as **“std search: ownership”**, followed by **“Opens the Rust standard library search.”** This gives no ownership explanation, Book chapter, syntax, or example.
- Language-guide concepts are sent to the standard-library item search. `borrow checker`, `let else`, `procedural macros`, `unsafe Rust`, `tuple structs`, and `raw identifiers` need Reference or Book pages, not a std item-name search.
- Natural task phrases are not mapped to APIs. `print a line`, `read file to string`, `create directories recursively`, and `run command and capture output` do not surface `println!`, `read_to_string`, `create_dir_all`, or `Command::output`.
- Tooling and documentation topics use the wrong corpus. `Cargo features`, `intra-doc links`, and `Rust 2024 edition` all land in std search and show no relevant guide.
- Some identifier routing stops one level too early. `str::split_whitespace` ranks **“std::str - Rust”**, and `f64::total_cmp` ranks **“std::f64 - Rust”** rather than the requested method. Generic std-search rows also omit signatures and examples.

### Prioritized improvements

1. **Add a Rust intent router.** Send language syntax and semantics to the Rust Reference or Book, std APIs to rustdoc, Cargo topics to the Cargo Book, rustdoc topics to the rustdoc book, and edition queries to the Edition Guide.
2. **Resolve exact paths to exact rustdoc pages.** Support `Type::method`, primitive methods, traits, macros, and generic syntax. Preserve enough path context to return `str::split_whitespace` and `f64::total_cmp`, not their parent modules.
3. **Add task-to-API aliases.** Map common phrases such as “print a line,” “read file to string,” “create directories recursively,” and “spawn a thread” to the canonical macro, function, or method.
4. **Return useful result metadata.** Show the resolved signature, one-line documentation, stability/source context when useful, and a short example for task queries. Do not describe a search endpoint as if it were the answer.
5. **Add regression cases for compound and unusual syntax.** Cover `Send and Sync`, `From and Into`, `Clone vs Copy`, `Result<T, E>`, `Cow<'a, str>`, `.await`, `macro_rules!`, `cfg target_os`, and `no_std` with destination and rank assertions.

## Go

### Summary

All 100 checklist commands were run through the installed `qs` executable. Each command had a 15-second timeout.

- **Average:** 3.61/5
- **Median:** 5/5
- **Distribution:** 0: 6, 1: 16, 2: 9, 3: 5, 4: 8, 5: 56
- **Pass rate >=3:** 69/100 (69%)
- **Pass rate >=4:** 64/100 (64%)

### Results

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

### Strongest patterns

- Exact package-qualified APIs are the clear strength. Queries such as `fmt.Sprintf`, `io.Copy`, `errors.Is`, `http.NewRequestWithContext`, and `testing.T.Run` return the expected API at #1 with signatures and useful context.
- CamelCase identifiers rank reliably. This holds across functions, methods, and types.
- Many exact API results include practical examples. File I/O, slice/map helpers, errors, sorting, HTTP, JSON, and hashing are especially good.
- Some input normalization works well. `json_marshal` resolves to `json.Marshal`, and `encoding/json.Unmarshal` resolves to the correct package API.
- Web fallback can be excellent for idioms. Variadic functions, defer, middleware, the race detector, and benchmarks return useful tutorials or official Go pages.

### Weakest patterns

- Language concepts are often treated as raw API tokens. Examples include arrays returning `types.Array`, interfaces returning `net.Interfaces`, and type switches returning `ast.TypeSwitchStmt`.
- Natural multiword tasks collide with incidental words. `embedded fields` returns `bytes.Fields`, `time.Parse layout` returns `structs.HostLayout`, and `worker pool` returns `sync.Pool`.
- Core built-ins rank poorly or disappear. `append` is #4, `make` is #3, while `new`, `delete`, `clear`, and `iota` do not return their expected built-in documentation.
- Several concept queries fail instead of reaching useful web or official documentation. Buffered channels, select statements, and closing channels report "no web results".
- Slash-qualified queries are unstable. `container/heap` and `crypto/rand.Read` timed out, while other slash forms worked.

### Prioritized improvements

1. **Add a Go language-construct index.** Map arrays, slices, maps, interfaces, type assertions, type switches, struct tags, receivers, generics, goroutines, channels, `range`, and `iota` to the Go specification or official tutorials before general API search.
2. **Give built-ins exact-match priority.** For bare identifiers such as `append`, `copy`, `delete`, `clear`, `new`, `make`, `nil`, `panic`, and `recover`, rank `builtin` entries ahead of package methods with the same token.
3. **Improve phrase-aware ranking.** Preserve units such as `worker pool`, `multiple return values`, `file permissions`, `named groups`, `table driven tests`, and `time.Parse layout` instead of matching isolated words.
4. **Use a robust official-web fallback.** If DevDocs has low-confidence results or no semantic match, retry against `go.dev`, `pkg.go.dev`, and high-quality tutorials. Do not turn a transient DuckDuckGo miss into an empty result.
5. **Harden package-path lookup and snippets.** Resolve slash/dot package identifiers directly, prevent the observed timeouts, and ensure exact entries show signatures or focused examples such as `%w`, `omitempty`, `Client.Timeout`, and `//go:embed`.

## C

### Summary

I ran all 100 checklist commands through the installed `qs` executable. Each invocation had a 25-second timeout so one network problem could not stop the run. I scored only the observed output, not the checklist's expected field.

- **Average:** 4.35 / 5
- **Median:** 5
- **Distribution:** 0: 5, 1: 6, 2: 1, 3: 3, 4: 7, 5: 78
- **Pass rate (>=3):** 88 / 100 (88%)
- **Strong pass rate (>=4):** 85 / 100 (85%)
- **Command errors:** 1 / 100 (`qs c include guards`)

### All results

| # | Command | Score | Observed top result or error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs c arrays` | 5 | #1 `Array declaration` | Exact array declaration page is first; definition and runnable VLA/subscript example are immediately useful. |
| 2 | `qs c pointers` | 5 | #1 `Pointer declaration` | Exact pointer declaration page is first; the definition and function-pointer example provide strong context. |
| 3 | `qs c pointer arithmetic` | 3 | #1 `Arithmetic types` | `Arithmetic types` is first and `Arithmetic operators` second, but neither visible snippet explains pointer arithmetic. |
| 4 | `qs c structures` | 5 | #1 `C Structures (structs) - W3Schools` | The first web result directly teaches C structs and lists declaration, access, and examples. |
| 5 | `qs c unions` | 4 | #1 `union` | A thin `union` keyword stub is first; the useful `Union declaration` page is second with a clear definition and example. |
| 6 | `qs c enumerations` | 5 | #1 `Enumerations` | Exact enum language page is first with a definition and code example. |
| 7 | `qs c bit fields` | 5 | #1 `Bit-fields` | Exact `Bit-fields` page is first and clearly describes explicit widths and packing. |
| 8 | `qs c function pointers` | 5 | #1 `Pointer declaration` | Pointer declaration is first, and its displayed example is exactly a function-pointer declaration and call. |
| 9 | `qs c variadic functions` | 5 | #1 `Variadic functions` | Exact variadic-functions page is first; it identifies the feature and the output includes `<stdarg.h>` usage. |
| 10 | `qs c storage duration` | 4 | #1 `Static storage duration` | Static and thread duration pages rank first and second, but automatic and allocated duration are not surfaced. |
| 11 | `qs c language linkage` | 4 | #1 `Language linkage - cppreference.com` | The first hit is the C++ language-linkage page; the second hit correctly covers C internal, external, and no linkage. |
| 12 | `qs c type qualifiers` | 4 | #1 `const type qualifier` | `const`, `restrict`, and `volatile` occupy the top three; `_Atomic` is not surfaced. |
| 13 | `qs c restrict` | 3 | #1 `restrict` | The first result is an exact `restrict` keyword page, but its snippet is only a license notice and gives no qualifier guidance. |
| 14 | `qs c integer promotions` | 5 | #1 `Integer Promotions in C - GeeksforGeeks` | The first hit directly explains C integer promotions with a concrete arithmetic example. |
| 15 | `qs c usual arithmetic conversions` | 5 | #1 `Implicit conversions` | The first result is C implicit conversions, the correct home for the usual arithmetic conversions. |
| 16 | `qs c operator precedence` | 5 | #1 `C Operator Precedence` | The exact operator-precedence table is first and explicitly promises precedence and associativity. |
| 17 | `qs c compound literals` | 5 | #1 `Compound literals` | Exact compound-literals page is first with a precise definition. |
| 18 | `qs c designated initializers` | 4 | #1 `Designated Initializers in C - GeeksforGeeks` | The first snippet is generic site boilerplate; the second and third results give useful C99/GCC explanations. |
| 19 | `qs c generic selection` | 5 | #1 `Generic selection` | Exact generic-selection page is first and clearly explains compile-time type-based selection. |
| 20 | `qs c static assertions` | 5 | #1 `Static assertion` | Exact static-assertion page is first and shows both `static_assert` and `_Static_assert`. |
| 21 | `qs c flexible array member` | 1 | #1 `Member access operators` | Only `Member access operators` appears; the snippet and example do not mention flexible array members. |
| 22 | `qs c variable length arrays` | 5 | #1 `Array declaration` | Array declaration is first and is the correct page for variable-length arrays, with a VLA example. |
| 23 | `qs c alignment` | 5 | #1 `Objects and alignment` | Objects and alignment is first with `alignof` examples; `alignas` also appears in the next result's example. |
| 24 | `qs c object lifetime` | 5 | #1 `Lifetime` | Exact lifetime page is first and directly states C object-lifetime rules. |
| 25 | `qs c undefined behavior` | 5 | #1 `Undefined behavior` | Exact undefined-behavior page is first and gives the right abstract-machine context. |
| 26 | `qs c sequence points` | 0 | #1 `Floating-point environment` | The sole result is `Floating-point environment`, which is unrelated to sequence points or sequencing. |
| 27 | `qs c command line arguments` | 0 | #1 `Variadic arguments` | The sole result is `Variadic arguments`; it does not show `main`, `argc`, or `argv`. |
| 28 | `qs c preprocessing directives` | 4 | #1 `Diagnostic directives` | Several directive pages are returned; diagnostic is first and conditional compilation is second, but no category overview leads. |
| 29 | `qs c conditional compilation` | 3 | Knowledge card: `Conditional compilation` (no numbered results) | A knowledge-card explanation is relevant, but there are no result rows or concrete `#if`/`#ifdef` examples. |
| 30 | `qs c include guards` | 0 | Error: `qs: no web results for c include guards` | Command fails with `no web results for c include guards`; no usable result is returned. |
| 31 | `qs c variadic macros` | 5 | #1 `Variadic Macros (The C Preprocessor)` | The first GCC page is exact; the second snippet includes a practical `__VA_ARGS__` definition. |
| 32 | `qs c escape sequences` | 5 | #1 `Escape sequences` | Exact escape-sequences page is first and clearly describes string and character use. |
| 33 | `qs c string literals` | 5 | #1 `String literals` | Exact string-literals page is first with syntax/type context and an example. |
| 34 | `qs c implicit conversions` | 5 | #1 `Implicit conversions` | Exact implicit-conversions page is first and clearly defines the operation. |
| 35 | `qs c compound assignment operators` | 5 | #1 `Assignment operators` | Assignment operators is first and explicitly covers compound assignment, with code. |
| 36 | `qs c printf` | 5 | #1 `printf` | Exact `printf` reference is first with format-string details and examples. |
| 37 | `qs c snprintf` | 5 | #1 `snprintf` | Exact `snprintf` reference is first and distinguishes the bounded API. |
| 38 | `qs c scanf` | 5 | #1 `scanf` | Exact `scanf` reference is first with format-specifier context. |
| 39 | `qs c fopen` | 5 | #1 `fopen` | Exact `fopen` page is first with a complete open/read/error/close example, though the leading snippet starts at output text. |
| 40 | `qs c fclose` | 5 | #1 `fclose` | Exact `fclose` page is first and begins with the signature and behavior. |
| 41 | `qs c fgets` | 5 | #1 `fgets` | Exact `fgets` page is first with return semantics and a bounded line-reading loop. |
| 42 | `qs c fread` | 5 | #1 `fread` | Exact `fread` page is first with a complete binary-read example and error handling. |
| 43 | `qs c fwrite` | 5 | #1 `fwrite` | Exact `fwrite` page is first with a complete binary-write example. |
| 44 | `qs c fseek` | 5 | #1 `fseek` | Exact `fseek` page is first and displays the signature and origin macros. |
| 45 | `qs c perror` | 5 | #1 `perror` | Exact `perror` page is first with signature and purpose. |
| 46 | `qs c malloc` | 5 | #1 `malloc` | Exact `malloc` page is first with signature and allocation behavior. |
| 47 | `qs c calloc` | 5 | #1 `calloc` | Exact `calloc` page is first with signature and zero-initialized array semantics. |
| 48 | `qs c realloc` | 5 | #1 `realloc` | Exact `realloc` page is first with signature and resize behavior. |
| 49 | `qs c free` | 5 | #1 `free` | Exact `free` page is first with null handling and a useful allocation/deallocation example. |
| 50 | `qs c memcpy` | 5 | #1 `memcpy` | Exact `memcpy` page is first and explains efficient memory-to-memory copying. |
| 51 | `qs c memmove` | 5 | #1 `memmove` | Exact `memmove` page is first and discusses its temporary-buffer overlap semantics. |
| 52 | `qs c memset` | 5 | #1 `memset` | Exact `memset` page is first; the snippet adds a useful note about secure variants. |
| 53 | `qs c memcmp` | 5 | #1 `memcmp` | Exact `memcmp` page is first with the full comparison signature and purpose. |
| 54 | `qs c strlen` | 5 | #1 `strlen` | Exact `strlen` page is first with a clear example distinguishing the terminator. |
| 55 | `qs c strcmp` | 5 | #1 `strcmp` | Exact `strcmp` page is first with signature and lexicographic behavior. |
| 56 | `qs c strchr` | 5 | #1 `strchr` | Exact `strchr` page is first with a clear found/not-found return description. |
| 57 | `qs c strstr` | 5 | #1 `strstr` | Exact `strstr` page is first with a clear substring-result description. |
| 58 | `qs c strtok` | 5 | #1 `strtok` | Exact `strtok` page is first and warns that tokenization mutates the input. |
| 59 | `qs c qsort` | 5 | #1 `qsort` | Exact `qsort` page is first with comparator signature guidance and a safe full example. |
| 60 | `qs c bsearch` | 5 | #1 `bsearch` | Exact `bsearch` page is first with a complete comparator and lookup example. |
| 61 | `qs c strtol` | 5 | #1 `strtol` | Exact `strtol` page is first and explains parsing behavior. |
| 62 | `qs c strtod` | 5 | #1 `strtod` | Exact `strtod` page is first and explains whitespace and conversion behavior. |
| 63 | `qs c assert` | 5 | #1 `assert` | Exact `assert` page is first with macro expansion, `NDEBUG`, and example use. |
| 64 | `qs c setjmp` | 5 | #1 `setjmp` | Exact `setjmp` page is first and clearly states that it saves execution context. |
| 65 | `qs c signal` | 5 | #1 `signal` | Exact `signal` page is first with handler-installation semantics and an example. |
| 66 | `qs c strftime` | 5 | #1 `strftime` | Exact `strftime` page is first and describes conversion specifiers. |
| 67 | `qs c timespec_get` | 5 | #1 `timespec_get` | Exact `timespec_get` page is first with return semantics and a current-time example. |
| 68 | `qs c rand` | 5 | #1 `rand` | Exact `rand` page is first with seeding, range, and unbiased die-roll examples. |
| 69 | `qs c sqrt` | 5 | #1 `sqrt` | Exact `sqrt` page is first with domain-error guidance. |
| 70 | `qs c isnan` | 5 | #1 `isnan` | Exact `isnan` page is first and starts with the macro form and classification purpose. |
| 71 | `qs c isalpha` | 5 | #1 `isalpha` | Exact `isalpha` page is first with locale-aware classification context. |
| 72 | `qs c toupper` | 5 | #1 `toupper` | Exact `toupper` page is first with signature and conversion behavior. |
| 73 | `qs c setlocale` | 5 | #1 `setlocale` | Exact `setlocale` page is first with signature and locale-installation purpose. |
| 74 | `qs c mbrtowc` | 5 | #1 `mbrtowc` | Exact `mbrtowc` page is first with detailed semantics and a full UTF-8 conversion example. |
| 75 | `qs c thrd_create` | 5 | #1 `thrd_create` | Exact `thrd_create` page is first and states its successful result; detail is sparse but relevant. |
| 76 | `qs c mtx_lock` | 5 | #1 `mtx_lock` | Exact `mtx_lock` page is first and gives important recursive-lock behavior. |
| 77 | `qs c cnd_wait` | 5 | #1 `cnd_wait` | Exact `cnd_wait` page is first and gives the required mutex precondition. |
| 78 | `qs c call_once` | 5 | #1 `call_once` | Exact `call_once` page is first and usefully points to the POSIX equivalent. |
| 79 | `qs c atomic_fetch_add` | 5 | #1 `atomic_fetch_add` | Exact `atomic_fetch_add` page is first with type rules and a complete threaded example. |
| 80 | `qs c atomic_compare_exchange_strong` | 5 | #1 `atomic_compare_exchange_strong` | Exact strong compare-exchange page is first and exposes success/failure memory-model details. |
| 81 | `qs c memory_order_acquire` | 4 | #1 `std::memory_order - cppreference.com` | The first result is C++ `std::memory_order`; the correct C `memory_order` page is second and useful. |
| 82 | `qs c size_t` | 5 | #1 `size_t` | Exact `size_t` page is first with typedef and `sizeof` semantics. |
| 83 | `qs c ptrdiff_t` | 5 | #1 `ptrdiff_t` | Exact `ptrdiff_t` page is first with typedef and pointer-difference semantics. |
| 84 | `qs c va_list` | 5 | #1 `va_list` | Exact `va_list` page is first and clearly describes the variadic state object. |
| 85 | `qs c struct tm` | 5 | #1 `tm` | Exact `tm` structure page is first and identifies broken-down calendar components. |
| 86 | `qs c FILE` | 5 | #1 `FILE` | Exact `FILE` page is first and defines the stream object. |
| 87 | `qs c NULL` | 5 | #1 `NULL` | Exact `NULL` page is first and defines the implementation-defined null pointer macro. |
| 88 | `qs c errno` | 5 | #1 `errno` | Exact `errno` page is first and explains its macro form; related math errors are second. |
| 89 | `qs c stdatomic.h` | 5 | #1 `Standard library header <stdatomic.h>` | Exact `<stdatomic.h>` header page is first and identifies atomic/concurrency support. |
| 90 | `qs c complex numbers` | 5 | #1 `Complex number arithmetic` | Complex-number arithmetic is first and explicitly mentions complex types and `<complex.h>`. |
| 91 | `qs c read a file line by line` | 0 | #1 `#line directive` | The first and only hit is the preprocessor `#line` directive, not line-by-line file input. |
| 92 | `qs c copy overlapping memory` | 0 | #1 `Memory model` | Results are `Memory model` and atomic `memory_order`; `memmove` is absent. |
| 93 | `qs c allocate zero initialized array` | 1 | #1 `Array declaration` | Array declaration and initialization are returned, but no dynamic allocation or `calloc` appears. |
| 94 | `qs c parse integer from string` | 1 | #1 `String literals` | String literals and the strings-library index appear, but `strtol` and numeric parsing are absent. |
| 95 | `qs c sort array with comparator` | 5 | #1 `C Program to Sort an Array in Ascending Order - GeeksforGeeks` | The first hit describes sorting a C array with `qsort` and a comparator; the exact `qsort` article is second. |
| 96 | `qs c get current calendar time` | 5 | #1 `time` | Exact `time` page is first with `time_t`, error handling, and calendar conversion example. |
| 97 | `qs c format date and time` | 2 | #1 `Date and time utilities` | A date/time category and `time` rank first and second, but `strftime` and formatting instructions are absent. |
| 98 | `qs c remove a file` | 1 | #1 `FILE` | Only `FILE` and file scope appear; the `remove` function is absent. |
| 99 | `qs c create a temporary file` | 1 | #1 `FILE` | Only `FILE` and file scope appear; `tmpfile` is absent. |
| 100 | `qs c handle end of file` | 1 | #1 `FILE` | Only the `FILE` type and file scope appear; `EOF`, `feof`, and read-result handling are absent. |

### Strongest patterns

- **Exact standard-library identifiers are excellent.** Queries from `printf` through the memory, string, numeric, thread, and atomic APIs almost always put the exact C reference first.
- **Exact language feature names work well.** Arrays, pointers, enums, bit-fields, generic selection, assertions, conversions, precedence, literals, alignment, lifetime, and undefined behavior lead to authoritative C pages.
- **API results often include actionable evidence.** The strongest outputs show signatures, return behavior, caveats, and runnable examples. `fread`, `qsort`, `bsearch`, `timespec_get`, and `mbrtowc` are especially useful.
- **Morphology is usually handled well.** Plurals such as `arrays`, `pointers`, `enumerations`, and `static assertions` map to singular reference concepts.
- **Web fallback can rescue missing index matches.** `structures`, `integer promotions`, `variadic macros`, and `sort array with comparator` return practical web results.

### Weakest patterns

- **Natural-language task queries often collapse onto one generic noun.** File tasks 98–100 all return `FILE`/file scope rather than `remove`, `tmpfile`, or EOF handling. Similar failures affect `calloc`, `strtol`, `memmove`, and `fgets` intent queries.
- **Important multiword concepts lose a discriminating token.** `command line arguments` becomes variadic arguments; `read a file line by line` becomes `#line`; `copy overlapping memory` becomes generic memory pages.
- **Some language concepts have bad or incomplete index coverage.** `sequence points` returns only `Floating-point environment`; `flexible array member` returns member-access operators.
- **Fallback behavior is inconsistent.** `conditional compilation` returns only a knowledge card, while `include guards` fails with `no web results`.
- **C/C++ source separation is imperfect.** `language linkage` and `memory_order_acquire` rank C++ references above the correct C result.

### Prioritized improvements

1. **Add intent-to-API aliases for common tasks.** Map `read ... line by line` to `fgets`, `copy overlapping memory` to `memmove`, `allocate zero initialized` to `calloc`, `parse integer` to `strtol`, `format date and time` to `strftime`, `remove a file` to `remove`, `temporary file` to `tmpfile`, and `end of file` to `EOF`/`feof`.
2. **Use phrase-aware matching before token scoring.** Preserve phrases such as `command line arguments`, `sequence points`, `flexible array member`, and `include guards` so generic matches for `arguments`, `line`, `member`, or `file` cannot dominate.
3. **Add curated C concept aliases and landing pages.** Explicitly index `argc argv`, sequencing/sequence points, all storage-duration kinds, `_Atomic` under type qualifiers, and header-guard patterns.
4. **Apply a strong C-domain boost and C++ penalty.** Rank `/c/` references above `/cpp/` or `std::` pages when the selected corpus is C, especially for linkage and atomic memory ordering.
5. **Harden and enrich web fallback.** Retry or switch source when DuckDuckGo returns no results, turn useful knowledge cards into normal ranked results, and prefer snippets that show C syntax or signatures over boilerplate such as the designated-initializer result.

## Haskell

### Summary

All 100 checklist commands were run through the installed `qs` command. Each command had a 25-second timeout. No command timed out. However, 57 commands exited with an application error: 35 reported `qs: no haskell results ...`, and 22 reported `qs: invalid JSON ... missing field name`.

- **Average:** 1.26 / 5
- **Median:** 0 / 5
- **Distribution:** 0: 58, 1: 14, 2: 6, 3: 4, 4: 2, 5: 16
- **Pass rate (>=3):** 22% (22/100)
- **Strong pass rate (>=4):** 18% (18/100)

### All queries

| # | Command | Score | Observed top result or error | Practical assessment |
|---:|---|---:|---|---|
| 1 | `qs haskell lists` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected List type, syntax, and `Data.List` result is unusable. |
| 2 | `qs haskell tuples` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Tuple types and tuple syntax result is unusable. |
| 3 | `qs haskell pattern matching` | 1 | `patternMatchingAllowed` (Agda) | Haskell syntax help is absent; the top results are Agda/compiler internals. |
| 4 | `qs haskell guards` | 1 | `guards :: ArrowIf ...` (hxt) | Misleading for language guards; only a source-AST helper appears at #3. |
| 5 | `qs haskell list comprehensions` | 0 | Error: `no haskell results for list comprehensions` | No result is shown for the expected List-comprehension syntax target. |
| 6 | `qs haskell type classes` | 1 | `typeEnvClasses` (GHC internal) | Related internals and extensions do not explain `class` or `instance`. |
| 7 | `qs haskell algebraic data types` | 1 | `matchingAlgebraicDataTypes` | Only a migration helper matches the words; no ADT declaration help. |
| 8 | `qs haskell newtypes` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `newtype` declarations result is unusable. |
| 9 | `qs haskell record syntax` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Record declarations, construction, and updates result is unusable. |
| 10 | `qs haskell deriving` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Derived instances and deriving strategies result is unusable. |
| 11 | `qs haskell modules` | 1 | `modules :: Config -> Maybe GlobPattern` | Results use “modules” as an identifier, not Haskell module syntax. |
| 12 | `qs haskell qualified imports` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Qualified module imports result is unusable. |
| 13 | `qs haskell infix operators` | 0 | `infixOperators` (JavaScript lexer) | Wrong language and no Haskell fixity declaration help. |
| 14 | `qs haskell strict evaluation` | 0 | Error: `no haskell results for strict evaluation` | No result is shown for the expected Strictness, `seq`, and bang patterns target. |
| 15 | `qs haskell thunks` | 1 | `ThunkSelector` (GHC internal) | Related runtime internals, but no useful explanation of lazy thunks. |
| 16 | `qs haskell tail recursion` | 0 | Error: `no haskell results for tail recursion` | No result is shown for the expected Tail-recursive function design target. |
| 17 | `qs haskell currying` | 1 | `class Currying ...` (Agda) | A niche Agda type-level API does not explain ordinary curried functions. |
| 18 | `qs haskell partial application` | 1 | `PartialApplication` (Servant type error) | The saturation message is related, but it does not teach partial application. |
| 19 | `qs haskell higher order functions` | 0 | Error: `no haskell results for higher order functions` | No result is shown for the expected Functions that accept or return functions target. |
| 20 | `qs haskell function composition` | 0 | Error: `no haskell results for function composition` | No result is shown for the expected The `(.)` operator target. |
| 21 | `qs haskell map` | 5 | `map :: (a -> b) -> [a] -> [b]` (Prelude) | Exact API at #1 with clear semantics and examples. |
| 22 | `qs haskell filter` | 5 | `filter :: (a -> Bool) -> [a] -> [a]` (Prelude) | Exact API at #1 with a useful signature and examples. |
| 23 | `qs haskell zipWith` | 5 | `zipWith :: ...` (Prelude) | Exact API at #1 with signature, behavior, and examples. |
| 24 | `qs haskell concat map` | 5 | `concatMap :: ...` (Prelude) | Exact API at #1; `Data.List` is #2 and the example is useful. |
| 25 | `qs haskell reverse` | 5 | `reverse :: [a] -> [a]` (Prelude) | Exact API at #1 with examples and a finite-list warning. |
| 26 | `qs haskell take while` | 5 | `takeWhile :: ...` (Prelude) | Exact API at #1 with behavior and laziness examples. |
| 27 | `qs haskell dropWhile` | 5 | `dropWhile :: ...` (Prelude) | Exact API at #1 with clear examples. |
| 28 | `qs haskell split list at index` | 0 | Error: `no haskell results for split list at index` | No result is shown for the expected `Prelude.splitAt` target. |
| 29 | `qs haskell partition list` | 2 | `partitionByList :: [Bool] -> [a] -> ...` | It partitions a list, but not by a predicate and not `Data.List.partition`. |
| 30 | `qs haskell element membership` | 0 | Error: `no haskell results for element membership` | No result is shown for the expected `elem` and `notElem` target. |
| 31 | `qs haskell association list lookup` | 0 | Error: `no haskell results for association list lookup` | No result is shown for the expected `Prelude.lookup` target. |
| 32 | `qs haskell maximum by` | 5 | `maximumBy :: ...` (Data.List) | Exact result at #1 with comparator example. |
| 33 | `qs haskell sort on field` | 1 | `ftCommonFieldSortable` (Redis) | Matches “field/sort” tokens but does not sort a Haskell list. |
| 34 | `qs haskell group adjacent values` | 0 | Error: `no haskell results for group adjacent values` | No result is shown for the expected `Data.List.group` / `groupBy` target. |
| 35 | `qs haskell remove duplicates` | 1 | `SortFlagsRemoveDuplicates` (GtkSource) | A flag named after the task is not the useful `nub` function. |
| 36 | `qs haskell foldr` | 5 | `foldr :: ...` (Prelude) | Exact API at #1 with a detailed fold explanation. |
| 37 | `qs haskell "foldl'"` | 5 | `foldl' :: ...` (Prelude) | Exact strict fold at #1; explains thunk avoidance. |
| 38 | `qs haskell running totals` | 0 | Error: `no haskell results for running totals` | No result is shown for the expected `Prelude.scanl` target. |
| 39 | `qs haskell infinite iteration` | 0 | Error: `no haskell results for infinite iteration` | No result is shown for the expected `Prelude.iterate` target. |
| 40 | `qs haskell repeat value n times` | 0 | Error: `no haskell results for repeat value n times` | No result is shown for the expected `Prelude.replicate` target. |
| 41 | `qs haskell Maybe` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Maybe` result is unusable. |
| 42 | `qs haskell default maybe value` | 0 | Error: `no haskell results for default maybe value` | No result is shown for the expected `Data.Maybe.fromMaybe` target. |
| 43 | `qs haskell map_maybe` | 0 | Error: `no haskell results for map_maybe` | No result is shown for the expected `Data.Maybe.mapMaybe` target. |
| 44 | `qs haskell discard Nothing values` | 0 | Error: `no haskell results for discard Nothing values` | No result is shown for the expected `Data.Maybe.catMaybes` target. |
| 45 | `qs haskell Either` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Either` result is unusable. |
| 46 | `qs haskell separate lefts and rights` | 0 | Error: `no haskell results for separate lefts and rights` | No result is shown for the expected `Data.Either.partitionEithers` target. |
| 47 | `qs haskell parse string safely` | 0 | Error: `no haskell results for parse string safely` | No result is shown for the expected `Text.Read.readMaybe` target. |
| 48 | `qs haskell convert value to string` | 0 | Error: `no haskell results for convert value to string` | No result is shown for the expected `Show` and `show` target. |
| 49 | `qs haskell ordering comparison` | 0 | Error: `no haskell results for ordering comparison` | No result is shown for the expected `Ord` and `compare` target. |
| 50 | `qs haskell enumerate range` | 0 | Error: `no haskell results for enumerate range` | No result is shown for the expected `Enum` and range syntax target. |
| 51 | `qs haskell Functor` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Functor` and `fmap` result is unusable. |
| 52 | `qs haskell applicatives` | 2 | `ApplicativeS` (selective) | Related class, but standard `Applicative`, `pure`, and `<*>` are missing. |
| 53 | `qs haskell monadic bind` | 0 | Error: `no haskell results for monadic bind` | No result is shown for the expected `Monad` and `(>>=)` target. |
| 54 | `qs haskell Alternative` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Control.Applicative.Alternative` result is unusable. |
| 55 | `qs haskell foldables` | 1 | `mapFoldableS` (conduit) | Name match only; no `Foldable` class documentation. |
| 56 | `qs haskell traverse` | 4 | `traverse :: ...` (Prelude) | Exact function at #1; requested `Data.Traversable` entry is #2 with good context. |
| 57 | `qs haskell Semigroup` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Semigroup` and `(<>)` result is unusable. |
| 58 | `qs haskell monoids` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Monoid`, `mempty`, and `mappend` result is unusable. |
| 59 | `qs haskell bimap` | 5 | `bimap :: ...` (Data.Bifunctor) | Exact module and function at #1 with examples. |
| 60 | `qs haskell monad transformers` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Transformer types and `lift` result is unusable. |
| 61 | `qs haskell Data.Map.Strict.lookup` | 5 | `lookup :: ...` (Data.Map.Strict) | Exact qualified target at #1 with signature and example. |
| 62 | `qs haskell insert map key` | 2 | `insertKeyMap` (hls-graph) | Related insertion shape, but standard `Data.Map.insert` is absent. |
| 63 | `qs haskell integer keyed maps` | 0 | Error: `no haskell results for integer keyed maps` | No result is shown for the expected `Data.IntMap` target. |
| 64 | `qs haskell set intersection` | 3 | `setIntersection` (GHC internal) | Usable intersection signatures appear, but not `Data.Set.intersection`. |
| 65 | `qs haskell hash maps` | 2 | `hashMapStrict` (aeson-combinators) | Shows `HashMap` in a decoder, but not the container type/module requested. |
| 66 | `qs haskell sequences` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Data.Sequence.Seq` result is unusable. |
| 67 | `qs haskell vectors` | 1 | `vectors` (gnuplot) | The plural identifier matches, but results do not expose `Data.Vector.Vector`. |
| 68 | `qs haskell byte strings` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Data.ByteString.ByteString` result is unusable. |
| 69 | `qs haskell Text` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Data.Text.Text` result is unusable. |
| 70 | `qs haskell split_on` | 0 | Error: `no haskell results for split_on` | No result is shown for the expected `Data.Text.splitOn` target. |
| 71 | `qs haskell replace substring` | 0 | Error: `no haskell results for replace substring` | No result is shown for the expected `Data.Text.replace` target. |
| 72 | `qs haskell encode utf8` | 3 | `utf8EncodeByteArray#`; `encodeUtf8` at #4 | The right function and module exist, but three low-level GHC APIs outrank it. |
| 73 | `qs haskell handle invalid Unicode` | 0 | Error: `no haskell results for handle invalid Unicode` | No result is shown for the expected `Data.Text.Encoding.decodeUtf8With` and `lenientDecode` target. |
| 74 | `qs haskell parse JSON` | 4 | `parseIndexedJSON`; `parseJSON` at #2 (aeson) | Strong Aeson result for `FromJSON`; `eitherDecode` is absent. |
| 75 | `qs haskell generic programming` | 0 | Error: `no haskell results for generic programming` | No result is shown for the expected `GHC.Generics.Generic` target. |
| 76 | `qs haskell lens view set` | 0 | Error: `no haskell results for lens view set` | No result is shown for the expected `Control.Lens.view`, `set`, and lens basics target. |
| 77 | `qs haskell putStrLn` | 5 | `putStrLn :: String -> IO ()` (Prelude) | Exact API at #1 with concise behavior. |
| 78 | `qs haskell readFile` | 5 | `readFile :: FilePath -> IO String` (Prelude) | Exact API at #1 with laziness and examples. |
| 79 | `qs haskell write text file` | 3 | `writeFileLText` / `writeFileText` (relude) | Useful text-file functions rank well, but standard `writeFile` is absent. |
| 80 | `qs haskell safe file handle` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `System.IO.withFile` result is unusable. |
| 81 | `qs haskell command line arguments` | 1 | GI `ApplicationCommandLine...` metadata | Framework metadata is not `System.Environment.getArgs`. |
| 82 | `qs haskell environment variables` | 2 | `expandEnvironmentVariables` (Agda) | Environment-related, but it does not read a variable via `lookupEnv`/`getEnv`. |
| 83 | `qs haskell list directory` | 5 | `listDirectory :: FilePath -> IO [FilePath]` | Exact `System.Directory` API at #1 with useful failure details. |
| 84 | `qs haskell check file exists` | 0 | Error: `no haskell results for check file exists` | No result is shown for the expected `System.Directory.doesFileExist` target. |
| 85 | `qs haskell file extension` | 3 | `fileExtension` (path) | A useful extension API ranks first, but standard `takeExtension` is missing. |
| 86 | `qs haskell temporary files` | 0 | Error: `no haskell results for temporary files` | No result is shown for the expected `System.IO.Temp` target. |
| 87 | `qs haskell exceptions` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Control.Exception` result is unusable. |
| 88 | `qs haskell bracket resource cleanup` | 0 | Error: `no haskell results for bracket resource cleanup` | No result is shown for the expected `Control.Exception.bracket` target. |
| 89 | `qs haskell async concurrency` | 0 | Error: `no haskell results for async concurrency` | No result is shown for the expected `Control.Concurrent.Async` target. |
| 90 | `qs haskell forkIO` | 5 | `forkIO :: IO () -> IO ThreadId` | Exact `Control.Concurrent` API at #1 with clear semantics. |
| 91 | `qs haskell MVar` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Control.Concurrent.MVar.MVar` result is unusable. |
| 92 | `qs haskell software transactional memory` | 0 | Error: `no haskell results for software transactional memory` | No result is shown for the expected `Control.Concurrent.STM` target. |
| 93 | `qs haskell operation timeout` | 1 | `CurlOperationTimeout` | A curl error code is not an operation-timeout function. |
| 94 | `qs haskell run external process` | 0 | Error: `no haskell results for run external process` | No result is shown for the expected `System.Process` / `readProcess` target. |
| 95 | `qs haskell random numbers` | 0 | Error: `no haskell results for random numbers` | No result is shown for the expected `System.Random` and `randomRIO` target. |
| 96 | `qs haskell HTTP client request` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Network.HTTP.Client` result is unusable. |
| 97 | `qs haskell QuickCheck properties` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Test.QuickCheck` and `property` result is unusable. |
| 98 | `qs haskell Hspec` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Test.Hspec` result is unusable. |
| 99 | `qs haskell GHCi commands` | 0 | Error: `no haskell results for GHCi commands` | No result is shown for the expected GHCi interactive commands target. |
| 100 | `qs haskell language extensions` | 2 | `languageExtensions` (GHC internal) | Related compiler data, but no `LANGUAGE` pragma or user-facing extension guide. |

### Strongest patterns

- Exact, case-sensitive function names are excellent. `map`, `filter`, `zipWith`, `foldr`, `foldl'`, `bimap`, `putStrLn`, `readFile`, and `forkIO` all put the useful API at #1 with a signature and often examples.
- Spaced forms of some camel-case names work well. `concat map` and `take while` resolve to `concatMap` and `takeWhile` at #1.
- Fully qualified lookup is the most reliable route. `Data.Map.Strict.lookup` returns the exact module, signature, explanation, and example at #1.
- A few task phrases succeed when they closely match an exported identifier. `list directory` finds `System.Directory.listDirectory` at #1.

### Weakest patterns

- Error handling is the largest failure. 22 commands abort on valid Hoogle payloads whose entries lack a `name` field. This breaks basic targets such as `Maybe`, `Either`, `Functor`, `Semigroup`, `MVar`, `Hspec`, and `exceptions`.
- Natural task queries often return nothing. Examples include `split list at index`, `remove duplicates` (which instead finds an unrelated flag), `default maybe value`, `parse string safely`, `check file exists`, and `run external process`.
- Language concepts rank poorly. `pattern matching` starts with Agda APIs, `infix operators` returns a JavaScript lexer symbol, and `modules` returns unrelated identifiers.
- Plurals and naming variants are fragile. `applicatives`, `foldables`, `vectors`, and `sequences` do not normalize to their standard Haskell types. `map_maybe` and `split_on` return no results.
- Common-library relevance loses to literal token overlap. Redis field metadata outranks `sortOn`; GHC internal set helpers outrank `Data.Set.intersection`; low-level UTF-8 functions outrank `Data.Text.Encoding.encodeUtf8`.

### Prioritized improvements

1. **Make the Hoogle JSON decoder tolerant.** Treat `name` as optional and derive a display label from `item` when needed. This directly fixes 22 hard failures.
2. **Add query normalization and intent aliases.** Normalize plural/singular forms, snake_case/camelCase, and common task phrases such as “remove duplicates” → `nub`, “repeat value n times” → `replicate`, and “check file exists” → `doesFileExist`.
3. **Boost canonical packages and modules.** Prefer `base`, `containers`, `text`, `bytestring`, `directory`, `filepath`, and other common packages over token matches in internal or niche modules.
4. **Use a concept-documentation fallback.** When Hoogle lacks an API match, return a Haskell Report, GHC User Guide, or trusted tutorial section for syntax and concepts such as guards, pattern matching, type classes, ADTs, modules, and language extensions.
5. **Add intent-aware reranking and regression tests.** Reward exact semantic targets and penalize cross-language/internal metadata. Cover every observed miss, including `sort on field`, `command line arguments`, `operation timeout`, `parse JSON`, and UTF-8 encoding.

## Final candidate checkpoint

The upgraded candidate was evaluated on all 600 frozen language commands after official-source and semantic full-text integration.

| Language | Average | Score >=3 | Score >=4 | Errors |
|---|---:|---:|---:|---:|
| C | 4.72 | 97% | 93% | 0 |
| C++ | 4.68 | 98% | 91% | 0 |
| Python | 4.57 | 96% | 85% | 0 |
| Go | 4.38 | 89% | 80% | 0 |
| Rust | 3.76 | 82% | 51% | 0 |
| Haskell | 2.76 | 48% | 35% | 0 |
| **Overall** | **4.15** | **85.0%** | **72.5%** | **0** |

The prior candidate checkpoint was 3.87 average, 78.8% at score >=3, and 66.0% at score >=4. Detailed reports are in `.score-parts-v5/`; the final Go and Rust reports are `go-final.md` and `rust-final.md`.
