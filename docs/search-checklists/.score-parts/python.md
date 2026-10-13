# Python quicksearch score report

## Summary

- Commands run: **100/100** with the installed `qs` from `PATH`.
- Per-command timeout: **25 seconds**.
- Average: **4.06/5**.
- Median: **5/5**.
- Distribution: **0: 8**, **1: 4**, **2: 5**, **3: 7**, **4: 9**, **5: 67**.
- Pass rate (score >=3): **83/100 (83%)**.
- Strong pass rate (score >=4): **76/100 (76%)**.

## Results

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

## Strongest patterns

- Exact qualified identifiers are excellent. Dotted APIs such as `pathlib.Path.read_text`, `collections.Counter.most_common`, and `typing.TypedDict` usually rank first with useful context.
- Direct built-in and standard-library names are reliable. Examples include `enumerate`, `json.dumps`, `heapq.nlargest`, and `socket.create_connection`.
- Formal concept names often work when the indexed title is close to the query. Exception chaining, assignment expressions, dataclasses, and MRO are strong examples.
- Most successful results include enough behavior, signature detail, or example text to act without another search.

## Weakest patterns

- Natural task queries often fail semantic mapping. `sort list by key`, `merge dictionaries`, `expand home directory`, `build URL query string`, and `copy directory tree` miss their intended APIs.
- Generic words can dominate phrase intent. “groups” maps to OS groups, “tree” maps to XML, and “encoding” maps to locale APIs.
- Some concept searches favor internal or low-level pages. `iterator protocol` returns the C API, and `run async tasks concurrently` puts an internal `test.support` helper first.
- Two searches fell through to web lookup and returned no result: `walk directory recursively` and `trace memory allocations`.
- One query crashed. `descriptor protocol` returned rc 101 with `end byte index ... is not a char boundary` inside the Unicode character `’`.

## Prioritized improvements

1. **Fix Unicode-safe snippet slicing.** Slice on character boundaries, not raw byte offsets. Add a regression test with curly quotes so `descriptor protocol` cannot panic.
2. **Add task-to-API synonym expansion.** Map common verbs and phrases such as “expand home,” “copy directory tree,” “URL query string,” “merge dictionaries,” and “parse ISO date time” to canonical APIs.
3. **Use phrase and proximity scoring.** Keep multiword intent together. This should stop isolated tokens such as “group,” “tree,” “key,” and “encoding” from overwhelming the requested concept.
4. **Improve document-tier ranking and section anchors.** Prefer public Python APIs and language-reference sections over C API pages, `test.support`, generic module pages, and third-party web results. Show the matched signature or local example in the snippet.
5. **Make fallback retrieval resilient.** If local matching is weak or web search returns no results, retry normalized variants and known doc aliases before failing. Preserve a useful local candidate instead of returning an empty result.

## Failure evidence

- `qs python descriptor protocol`: rc 101; `end byte index 342814 is not a char boundary; it is inside ’`.
- `qs python walk directory recursively`: rc 1; `qs: no web results for python walk directory recursively`.
- `qs python trace memory allocations`: rc 1; `qs: no web results for python trace memory allocations`.
- `qs python regular expression named groups`: top result was `os.getgroups()`; the expected `(?P<name>...)` syntax did not appear.
- `qs python copy directory tree`: top result was `xml.etree.ElementTree`; `shutil.copytree()` did not appear.
