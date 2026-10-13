# Python candidate-v3 score report

## Summary

- Binary: `/tmp/qs-candidate-v3/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **20 seconds**.
- Average: **4.26/5**.
- Median: **5/5**.
- Distribution: **0: 4**, **1: 1**, **2: 6**, **3: 13**, **4: 6**, **5: 70**.
- Practical pass rate (score >=3): **89/100 (89%)**.
- Strong-result rate (score >=4): **76/100 (76%)**.
- Execution errors: **0**. Timeouts: **0**.

## Comparisons

- **Installed baseline (`.score-parts/python.md`):** 4.06 average, 5 median, distribution 0: 8, 1: 4, 2: 5, 3: 7, 4: 9, 5: 67, 83% >=3, and 76% >=4. Candidate-v3 changes are **+0.20 average points**, **no median change**, **+6 percentage points at >=3**, and **no change at >=4**. The baseline had **3 execution errors** and **0 timeouts**; candidate-v3 had neither.
- **Candidate-v2 (`.score-parts-final/python.md`):** 4.26 average, 5 median, distribution 0: 4, 1: 1, 2: 6, 3: 13, 4: 6, 5: 70, 89% >=3, and 76% >=4. Candidate-v3 has **no score change**. Both runs had **0 execution errors** and **0 timeouts**.

## Results

| # | Command | Score | Observed top result or error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs python dictionaries` | 3 | dictionary | A useful dictionary definition is first, but the canonical dict/mapping API is absent. |
| 2 | `qs python list comprehensions` | 5 | list comprehension | Expected result is first with useful context. |
| 3 | `qs python generator expressions` | 5 | generator expression | Expected result is first with useful context. |
| 4 | `qs python decorators` | 5 | decorator | Expected result is first with useful context. |
| 5 | `qs python context managers` | 5 | context manager | Expected result is first with useful context. |
| 6 | `qs python async iterators` | 4 | typing.AsyncIterator | The deprecated typing alias is first; collections.abc.AsyncIterator is second and useful. |
| 7 | `qs python structural pattern matching` | 3 | re.Pattern.match() | Regex results dominate; ast.Match appears fifth, so the intended language feature is weakly represented. |
| 8 | `qs python positional only parameters` | 3 | parameter | The parameter glossary is relevant, but positional-only `/` syntax is not shown. |
| 9 | `qs python keyword only arguments` | 3 | as (keyword) | A keyword-argument glossary result appears second, but keyword-only `*` syntax is absent. |
| 10 | `qs python exception chaining` | 5 | 8.5. Exception Chaining | Expected result is first with useful context. |
| 11 | `qs python assignment expressions` | 5 | 6.12. Assignment expressions | Expected result is first with useful context. |
| 12 | `qs python function type annotations` | 5 | function annotation | Expected result is first with useful context. |
| 13 | `qs python generic type aliases` | 5 | types.GenericAlias | Expected result is first with useful context. |
| 14 | `qs python dataclasses` | 5 | dataclasses | Expected result is first with useful context. |
| 15 | `qs python frozen dataclass` | 5 | Frozen instances (dataclasses) | Expected result is first with useful context. |
| 16 | `qs python enumerations` | 0 | curses.KEY_EOS | No enum module or Enum result appears. |
| 17 | `qs python abstract base classes` | 3 | abstract base class | The ABC concept is relevant, but the general abc module and ABC class are absent. |
| 18 | `qs python method resolution order` | 5 | method resolution order | Expected result is first with useful context. |
| 19 | `qs python descriptor protocol` | 4 | dir() | The descriptor glossary entry is second and directly explains __get__, __set__, and __delete__. |
| 20 | `qs python iterator protocol` | 2 | Iterator Protocol | The first result is the C API iterator protocol and gives little Python-level protocol detail. |
| 21 | `qs python enumerate` | 5 | enumerate() | Expected result is first with useful context. |
| 22 | `qs python zip` | 5 | zip() | Expected result is first with useful context. |
| 23 | `qs python isinstance` | 5 | isinstance() | Expected result is first with useful context. |
| 24 | `qs python super` | 5 | super | Expected result is first with useful context. |
| 25 | `qs python convert string to integer` | 5 | int | Expected result is first with useful context. |
| 26 | `qs python memoryview` | 5 | memoryview | Expected result is first with useful context. |
| 27 | `qs python breakpoint` | 5 | breakpoint() | Expected result is first with useful context. |
| 28 | `qs python str.removeprefix` | 5 | str.removeprefix() | Expected result is first with useful context. |
| 29 | `qs python str.splitlines` | 5 | str.splitlines() | Expected result is first with useful context. |
| 30 | `qs python bytes.fromhex` | 5 | bytes.fromhex() | Expected result is first with useful context. |
| 31 | `qs python sort list by key` | 3 | sorted key | The Sorting HOW TO key-functions section is first, but list.sort(key=...) is not directly surfaced. |
| 32 | `qs python dict.setdefault` | 5 | dict.setdefault() | Expected result is first with useful context. |
| 33 | `qs python merge dictionaries` | 3 | dict.update | dict.update is useful for merging, but the expected `\|` and `\|=` operators are absent. |
| 34 | `qs python set symmetric_difference` | 5 | set.symmetric_difference() | Expected result is first with useful context. |
| 35 | `qs python frozenset` | 5 | frozenset | Expected result is first with useful context. |
| 36 | `qs python format specification mini language` | 3 | format() | format() is first and points to the mini-language, but the detailed specification is not directly surfaced. |
| 37 | `qs python f string debugging` | 2 | f-string | Generic f-string entries appear, but self-documenting `=` expressions are not shown. |
| 38 | `qs python FileNotFoundError` | 5 | FileNotFoundError | Expected result is first with useful context. |
| 39 | `qs python ExceptionGroup` | 5 | ExceptionGroup | Expected result is first with useful context. |
| 40 | `qs python property setter` | 5 | property.setter() | Expected result is first with useful context. |
| 41 | `qs python pathlib.Path.read_text` | 5 | pathlib.Path.read_text() | Expected result is first with useful context. |
| 42 | `qs python pathlib.Path.glob` | 5 | pathlib.Path.glob() | Expected result is first with useful context. |
| 43 | `qs python expand home directory` | 3 | ~ (tilde) (home directory expansion) | The first result is the os.path home-expansion index entry, but expanduser() is not named or explained. |
| 44 | `qs python environment variables` | 2 | 1.2. Environment variables | General environment-variable documentation appears, but os.environ is absent. |
| 45 | `qs python sys.argv` | 5 | sys.argv | Expected result is first with useful context. |
| 46 | `qs python sys.path` | 5 | sys.path | Expected result is first with useful context. |
| 47 | `qs python argparse subcommands` | 2 | argparse | The argparse module is relevant, but add_subparsers() and subcommands are not surfaced. |
| 48 | `qs python json.dumps` | 5 | json.dumps() | Expected result is first with useful context. |
| 49 | `qs python json custom encoder` | 5 | json.JSONEncoder | Expected result is first with useful context. |
| 50 | `qs python csv.DictReader` | 5 | csv.DictReader | Expected result is first with useful context. |
| 51 | `qs python configparser` | 5 | configparser | Expected result is first with useful context. |
| 52 | `qs python tomllib.load` | 5 | tomllib.load() | Expected result is first with useful context. |
| 53 | `qs python pickle security` | 3 | pickle | The pickle module is first, but its security warning is not shown. |
| 54 | `qs python sqlite3 row factory` | 4 | sqlite3.Cursor.row_factory | Connection.row_factory is second; the first result is the closely related Cursor.row_factory. |
| 55 | `qs python regular expression named groups` | 5 | (? (in regular expressions) | The first regex result explicitly documents `(?P<name>...)` in its snippet. |
| 56 | `qs python re.escape` | 5 | re.escape() | Expected result is first with useful context. |
| 57 | `qs python parse ISO date time` | 0 | datetime.time | No fromisoformat result appears. |
| 58 | `qs python timezone aware datetime` | 2 | datetime.time | Timezone-related results appear, but aware datetime and tzinfo guidance is missing. |
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
| 72 | `qs python calculate median` | 3 | min() | statistics.median() is third, but it has no useful snippet and unrelated min() ranks first. |
| 73 | `qs python random choices weights` | 4 | random.choice() | random.choices() is second with useful context; random.choice() ranks first. |
| 74 | `qs python secrets.token_urlsafe` | 5 | secrets.token_urlsafe() | Expected result is first with useful context. |
| 75 | `qs python hashlib.sha256` | 5 | hashlib.sha256() | Expected result is first with useful context. |
| 76 | `qs python uuid.uuid4` | 5 | uuid.uuid4() | Expected result is first with useful context. |
| 77 | `qs python base64 urlsafe encoding` | 2 | base64.encode() | General Base64 results and urlsafe decoding appear, but urlsafe_b64encode() is absent. |
| 78 | `qs python build URL query string` | 0 | str | No urllib.parse or urlencode result appears. |
| 79 | `qs python logging.basicConfig` | 5 | logging.basicConfig() | Expected result is first with useful context. |
| 80 | `qs python create temporary directory` | 5 | tempfile.TemporaryDirectory | Expected result is first with useful context. |
| 81 | `qs python copy directory tree` | 1 | copy | Results focus on generic object copying; shutil.copytree() is absent. |
| 82 | `qs python deep copy object` | 3 | object.__copy__() | The copy module is second and mentions deep copies, but copy.deepcopy() is not surfaced. |
| 83 | `qs python walk directory recursively` | 5 | os.walk | Expected result is first with useful context. |
| 84 | `qs python run command capture output` | 0 | cmd | No subprocess.run or capture_output result appears. |
| 85 | `qs python concurrent.futures.ThreadPoolExecutor` | 5 | concurrent.futures.ThreadPoolExecutor | Expected result is first with useful context. |
| 86 | `qs python multiprocessing Queue` | 5 | multiprocessing.Queue | Expected result is first with useful context. |
| 87 | `qs python threading Lock` | 5 | threading.Lock | Expected result is first with useful context. |
| 88 | `qs python run async tasks concurrently` | 5 | Running tasks concurrently (Coroutines and tasks) | The exact concurrency section is first and includes a complete asyncio.gather() example. |
| 89 | `qs python asyncio timeout` | 4 | asyncio.Timeout | asyncio.timeout() is second behind the related asyncio.Timeout class. |
| 90 | `qs python queue.PriorityQueue` | 5 | queue.PriorityQueue | Expected result is first with useful context. |
| 91 | `qs python socket.create_connection` | 5 | socket.create_connection() | Expected result is first with useful context. |
| 92 | `qs python unittest.mock.patch` | 5 | unittest.mock.patch() | Expected result is first with useful context. |
| 93 | `qs python doctest` | 5 | doctest | Expected result is first with useful context. |
| 94 | `qs python warnings filterwarnings` | 5 | warnings.filterwarnings() | Expected result is first with useful context. |
| 95 | `qs python importlib.resources.files` | 5 | importlib.resources.files() | Expected result is first with useful context. |
| 96 | `qs python inspect.signature` | 4 | inspect.Signature | inspect.signature() is second behind the related inspect.Signature class. |
| 97 | `qs python trace memory allocations` | 3 | tracemalloc.get_traced_memory() | A tracemalloc function is first and relevant, but the module overview is absent. |
| 98 | `qs python typing.Protocol` | 5 | typing.Protocol | Expected result is first with useful context. |
| 99 | `qs python typing.TypedDict` | 5 | typing.TypedDict | Expected result is first with useful context. |
| 100 | `qs python typing.Literal` | 5 | typing.Literal | Expected result is first with useful context. |
