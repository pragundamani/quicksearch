# Python candidate score

## Run summary

- Binary: `/tmp/qs-candidate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **20 seconds**.
- Average: **3.62/5**.
- Median: **5/5**.
- Distribution: **0: 24**, **1: 0**, **2: 2**, **3: 4**, **4: 4**, **5: 66**.
- Practical pass rate (score >=3): **74/100 (74%)**.
- Strong-result rate (score >=4): **70/100 (70%)**.
- Execution errors: **23**. Timeouts: **0**.

## Baseline comparison

The baseline in `language_scores.md` used the installed `qs` and a 25-second timeout. It scored **4.06 average**, **5 median**, **0: 8, 1: 4, 2: 5, 3: 7, 4: 9, 5: 67**, **83% >=3**, and **76% >=4**. The candidate changes are **-0.44 average points**, **no median change**, **-9 percentage points at >=3**, and **-6 percentage points at >=4**. The baseline rows show three command failures; the candidate had 23 errors and no timeouts. The candidate improves some task aliases, but its many failed multiword searches dominate the result.

## Results

| # | Command | Score | Observed top result or error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs python dictionaries` | 3 | dictionary | Useful definition is first, but the canonical dict/mapping API is absent. |
| 2 | `qs python list comprehensions` | 5 | list comprehension | Expected result is first with useful context. |
| 3 | `qs python generator expressions` | 5 | generator expression | Expected result is first with useful context. |
| 4 | `qs python decorators` | 5 | decorator | Expected result is first with useful context. |
| 5 | `qs python context managers` | 5 | context manager | Expected result is first with useful context. |
| 6 | `qs python async iterators` | 4 | typing.AsyncIterator | Deprecated typing alias is first; collections.abc.AsyncIterator is second and useful. |
| 7 | `qs python structural pattern matching` | 0 | Error (rc 1): no web results | Command failed; no match-statement or pattern result. |
| 8 | `qs python positional only parameters` | 0 | Error (rc 1): no web results | Command failed; no positional-only parameter result. |
| 9 | `qs python keyword only arguments` | 0 | Error (rc 1): no web results | Command failed; no keyword-only parameter result. |
| 10 | `qs python exception chaining` | 5 | 8.5. Exception Chaining | Expected result is first with useful context. |
| 11 | `qs python assignment expressions` | 5 | 6.12. Assignment expressions | Expected result is first with useful context. |
| 12 | `qs python function type annotations` | 0 | Error (rc 1): no web results | Command failed; no function-annotation result. |
| 13 | `qs python generic type aliases` | 5 | types.GenericAlias | Expected result is first with useful context. |
| 14 | `qs python dataclasses` | 5 | dataclasses | Expected result is first with useful context. |
| 15 | `qs python frozen dataclass` | 5 | Frozen instances (dataclasses) | Expected result is first with useful context. |
| 16 | `qs python enumerations` | 0 | curses.KEY_EOS | Results are unrelated; enum and Enum are absent. |
| 17 | `qs python abstract base classes` | 3 | abstract base class | ABC concept is relevant, but the abc module and ABC class are absent. |
| 18 | `qs python method resolution order` | 5 | method resolution order | Expected result is first with useful context. |
| 19 | `qs python descriptor protocol` | 0 | Error (rc 1): no web results | Command failed; no descriptor-protocol result. |
| 20 | `qs python iterator protocol` | 2 | Iterator Protocol | Exact words lead to the C API, not the Python __iter__/__next__ protocol. |
| 21 | `qs python enumerate` | 5 | enumerate() | Expected result is first with useful context. |
| 22 | `qs python zip` | 5 | zip() | Expected result is first with useful context. |
| 23 | `qs python isinstance` | 5 | isinstance() | Expected result is first with useful context. |
| 24 | `qs python super` | 5 | super | Expected result is first with useful context. |
| 25 | `qs python convert string to integer` | 5 | int | int is first; the second result gives conversion examples. |
| 26 | `qs python memoryview` | 5 | memoryview | Expected result is first with useful context. |
| 27 | `qs python breakpoint` | 5 | breakpoint() | Expected result is first with useful context. |
| 28 | `qs python str.removeprefix` | 5 | str.removeprefix() | Expected result is first with useful context. |
| 29 | `qs python str.splitlines` | 5 | str.splitlines() | Expected result is first with useful context. |
| 30 | `qs python bytes.fromhex` | 5 | bytes.fromhex() | Expected result is first with useful context. |
| 31 | `qs python sort list by key` | 3 | sorted key | Direct Sorting HOWTO link is relevant, but terminal detail is weak and list.sort is absent. |
| 32 | `qs python dict.setdefault` | 5 | dict.setdefault() | Expected result is first with useful context. |
| 33 | `qs python merge dictionaries` | 3 | dict.update | dict.update is a useful merge method, but the requested union operators are absent. |
| 34 | `qs python set symmetric_difference` | 5 | set.symmetric_difference() | Expected result is first with useful context. |
| 35 | `qs python frozenset` | 5 | frozenset | Expected result is first with useful context. |
| 36 | `qs python format specification mini language` | 0 | Error (rc 1): no web results | Command failed; no format mini-language result. |
| 37 | `qs python f string debugging` | 0 | Error (rc 1): no web results | Command failed; no self-documenting f-string result. |
| 38 | `qs python FileNotFoundError` | 5 | FileNotFoundError | Expected result is first with useful context. |
| 39 | `qs python ExceptionGroup` | 5 | ExceptionGroup | Expected result is first with useful context. |
| 40 | `qs python property setter` | 5 | property.setter() | Expected result is first with useful context. |
| 41 | `qs python pathlib.Path.read_text` | 5 | pathlib.Path.read_text() | Expected result is first with useful context. |
| 42 | `qs python pathlib.Path.glob` | 5 | pathlib.Path.glob() | Expected result is first with useful context. |
| 43 | `qs python expand home directory` | 0 | Error (rc 1): no web results | Command failed; no os.path.expanduser() result. |
| 44 | `qs python environment variables` | 2 | 1.2. Environment variables | Interpreter environment settings are relevant background, but os.environ is absent. |
| 45 | `qs python sys.argv` | 5 | sys.argv | Expected result is first with useful context. |
| 46 | `qs python sys.path` | 5 | sys.path | Expected result is first with useful context. |
| 47 | `qs python argparse subcommands` | 0 | Error (rc 1): no web results | Command failed; no argparse subparser result. |
| 48 | `qs python json.dumps` | 5 | json.dumps() | Expected result is first with useful context. |
| 49 | `qs python json custom encoder` | 0 | Error (rc 1): no web results | Command failed; no JSONEncoder result. |
| 50 | `qs python csv.DictReader` | 5 | csv.DictReader | Expected result is first with useful context. |
| 51 | `qs python configparser` | 5 | configparser | Expected result is first with useful context. |
| 52 | `qs python tomllib.load` | 5 | tomllib.load() | Expected result is first with useful context. |
| 53 | `qs python pickle security` | 0 | Error (rc 1): no web results | Command failed; no pickle security warning. |
| 54 | `qs python sqlite3 row factory` | 4 | sqlite3.Cursor.row_factory | Cursor.row_factory is useful; Connection.row_factory is second with good detail. |
| 55 | `qs python regular expression named groups` | 0 | Error (rc 1): no web results | Command failed; no named-group syntax result. |
| 56 | `qs python re.escape` | 5 | re.escape() | Expected result is first with useful context. |
| 57 | `qs python parse ISO date time` | 0 | Error (rc 1): no web results | Command failed; no datetime.fromisoformat() result. |
| 58 | `qs python timezone aware datetime` | 0 | Error (rc 1): no web results | Command failed; no aware-datetime result. |
| 59 | `qs python zoneinfo.ZoneInfo` | 5 | zoneinfo.ZoneInfo | Expected result is first with useful context. |
| 60 | `qs python time.monotonic` | 5 | time.monotonic() | Expected result is first with useful context. |
| 61 | `qs python collections.defaultdict` | 5 | collections.defaultdict | Expected result is first with useful context. |
| 62 | `qs python collections.Counter.most_common` | 5 | collections.Counter.most_common() | Expected result is first with useful context. |
| 63 | `qs python collections.ChainMap` | 5 | collections.ChainMap | Expected result is first with useful context. |
| 64 | `qs python heapq.nlargest` | 5 | heapq.nlargest() | Expected result is first with useful context. |
| 65 | `qs python bisect.insort` | 5 | bisect.insort() | Expected result is first with useful context. |
| 66 | `qs python itertools.zip_longest` | 5 | itertools.zip_longest() | Expected result is first with useful context. |
| 67 | `qs python itertools.groupby` | 5 | itertools.groupby() | Expected result is first with useful context. |
| 68 | `qs python functools.lru_cache` | 5 | functools.lru_cache() | Expected result is first with useful context. |
| 69 | `qs python functools.partial` | 5 | functools.partial() | Expected result is first with useful context. |
| 70 | `qs python operator.itemgetter` | 5 | operator.itemgetter() | Expected result is first with useful context. |
| 71 | `qs python math.isclose` | 5 | math.isclose() | Expected result is first with useful context. |
| 72 | `qs python calculate median` | 0 | Error (rc 1): no web results | Command failed; no statistics.median() result. |
| 73 | `qs python random choices weights` | 0 | Error (rc 1): no web results | Command failed; no random.choices() weights result. |
| 74 | `qs python secrets.token_urlsafe` | 5 | secrets.token_urlsafe() | Expected result is first with useful context. |
| 75 | `qs python hashlib.sha256` | 5 | hashlib.sha256() | Expected result is first with useful context. |
| 76 | `qs python uuid.uuid4` | 5 | uuid.uuid4() | Expected result is first with useful context. |
| 77 | `qs python base64 urlsafe encoding` | 0 | Error (rc 1): no web results | Command failed; no urlsafe_b64encode() result. |
| 78 | `qs python build URL query string` | 0 | Error (rc 1): no web results | Command failed; no urllib.parse.urlencode() result. |
| 79 | `qs python logging.basicConfig` | 5 | logging.basicConfig() | Expected result is first with useful context. |
| 80 | `qs python create temporary directory` | 0 | Error (rc 1): no web results | Command failed; no TemporaryDirectory result. |
| 81 | `qs python copy directory tree` | 0 | Error (rc 1): no web results | Command failed; no shutil.copytree() result. |
| 82 | `qs python deep copy object` | 0 | Error (rc 1): no web results | Command failed; no copy.deepcopy() result. |
| 83 | `qs python walk directory recursively` | 5 | os.walk | os.walk is first; the second result gives an actionable recursive example. |
| 84 | `qs python run command capture output` | 0 | Error (rc 1): no web results | Command failed; no subprocess capture-output result. |
| 85 | `qs python concurrent.futures.ThreadPoolExecutor` | 5 | concurrent.futures.ThreadPoolExecutor | Expected result is first with useful context. |
| 86 | `qs python multiprocessing Queue` | 5 | multiprocessing.Queue | Expected result is first with useful context. |
| 87 | `qs python threading Lock` | 5 | threading.Lock | Expected result is first with useful context. |
| 88 | `qs python run async tasks concurrently` | 5 | Running tasks concurrently (Coroutines and tasks) | The exact concurrency section is first with a complete asyncio.gather() example. |
| 89 | `qs python asyncio timeout` | 4 | asyncio.Timeout | Closely related class is first; asyncio.timeout() is second with an actionable example. |
| 90 | `qs python queue.PriorityQueue` | 5 | queue.PriorityQueue | Expected result is first with useful context. |
| 91 | `qs python socket.create_connection` | 5 | socket.create_connection() | Expected result is first with useful context. |
| 92 | `qs python unittest.mock.patch` | 5 | unittest.mock.patch() | Expected result is first with useful context. |
| 93 | `qs python doctest` | 5 | doctest | Expected result is first with useful context. |
| 94 | `qs python warnings filterwarnings` | 5 | warnings.filterwarnings() | Expected result is first with useful context. |
| 95 | `qs python importlib.resources.files` | 5 | importlib.resources.files() | Expected result is first with useful context. |
| 96 | `qs python inspect.signature` | 4 | inspect.Signature | Closely related class is first; inspect.signature() is second with a useful example. |
| 97 | `qs python trace memory allocations` | 0 | Error (rc 1): no web results | Command failed; no tracemalloc result. |
| 98 | `qs python typing.Protocol` | 5 | typing.Protocol | Expected result is first with useful context. |
| 99 | `qs python typing.TypedDict` | 5 | typing.TypedDict | Expected result is first with useful context. |
| 100 | `qs python typing.Literal` | 5 | typing.Literal | Expected result is first with useful context. |
