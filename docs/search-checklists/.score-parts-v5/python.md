# Python candidate-v5 score report

## Summary

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **30 seconds**.
- Average: **4.57/5**.
- Median: **5/5**.
- Distribution: **0: 1**, **1: 0**, **2: 3**, **3: 11**, **4: 7**, **5: 78**.
- Practical pass rate (score >=3): **96/100 (96%)**.
- Strong-result rate (score >=4): **85/100 (85%)**.
- Execution errors: **0**. Timeouts: **0**.

## Comparison to candidate-v3

- **Candidate-v3 (`.score-parts-v3/python.md`):** 4.26 average, 5 median, distribution 0: 4, 1: 1, 2: 6, 3: 13, 4: 6, 5: 70, 89% >=3, and 76% >=4.
- **Candidate-v5 change:** **+0.31 average points**, **no median change**, **+7 percentage points at >=3**, and **+9 percentage points at >=4**.
- The distribution changed by **-3 scores of 0**, **-1 score of 1**, **-3 scores of 2**, **-2 scores of 3**, **+1 score of 4**, and **+8 scores of 5**.
- Both runs had **0 execution errors** and **0 timeouts**.

## Results

| # | Command | Score | Observed top result or error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs python dictionaries` | 3 | dictionary | A useful dictionary definition is first, but the canonical dict/mapping API is absent. |
| 2 | `qs python list comprehensions` | 5 | list comprehension | Expected result is first with useful context. |
| 3 | `qs python generator expressions` | 5 | generator expression | Expected result is first with useful context. |
| 4 | `qs python decorators` | 5 | decorator | Expected result is first with useful context. |
| 5 | `qs python context managers` | 5 | context manager | Expected result is first with useful context. |
| 6 | `qs python async iterators` | 4 | typing.AsyncIterator | The deprecated typing alias is first; collections.abc.AsyncIterator is second and useful. |
| 7 | `qs python structural pattern matching` | 5 | Compound statements | The compound-statements page is first and its snippet directly explains the match statement and pattern syntax. |
| 8 | `qs python positional only parameters` | 5 | More Control Flow Tools | The control-flow page is first and its snippet explains positional-only parameters and the `/` separator. |
| 9 | `qs python keyword only arguments` | 3 | More Control Flow Tools | A keyword-arguments section is first, but it does not show the distinct keyword-only `*` syntax. |
| 10 | `qs python exception chaining` | 5 | 8.5. Exception Chaining | Expected result is first with useful context. |
| 11 | `qs python assignment expressions` | 5 | 6.12. Assignment expressions | Expected result is first with useful context. |
| 12 | `qs python function type annotations` | 3 | annotationlib.annotations_to_string(annotations) | Annotation results fill the top three, but none directly explains function type-annotation syntax. |
| 13 | `qs python generic type aliases` | 5 | types.GenericAlias | Expected result is first with useful context. |
| 14 | `qs python dataclasses` | 5 | dataclasses | Expected result is first with useful context. |
| 15 | `qs python frozen dataclass` | 5 | Frozen instances (dataclasses) | Expected result is first with useful context. |
| 16 | `qs python enumerations` | 0 | curses.KEY_EOS | No enum module or Enum result appears. |
| 17 | `qs python abstract base classes` | 3 | abstract base class | The ABC concept is relevant, but the general abc module and ABC class are absent. |
| 18 | `qs python method resolution order` | 5 | method resolution order | Expected result is first with useful context. |
| 19 | `qs python descriptor protocol` | 5 | Descriptor Guide | The Descriptor Guide is first and immediately lists `__get__`, `__set__`, and `__delete__`. |
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
| 33 | `qs python merge dictionaries` | 3 | dict.update | dict.update is useful for merging, but the expected `\\|` and `\\|=` operators are absent. |
| 34 | `qs python set symmetric_difference` | 5 | set.symmetric_difference() | Expected result is first with useful context. |
| 35 | `qs python frozenset` | 5 | frozenset | Expected result is first with useful context. |
| 36 | `qs python format specification mini language` | 5 | Format specification mini-language | The exact format specification mini-language section is first with a useful explanation. |
| 37 | `qs python f string debugging` | 3 | Debugging C API extensions and CPython Internals with GDB | The f-string tutorial is second, but the snippet does not expose the self-documenting `=` form. |
| 38 | `qs python FileNotFoundError` | 5 | FileNotFoundError | Expected result is first with useful context. |
| 39 | `qs python ExceptionGroup` | 5 | ExceptionGroup | Expected result is first with useful context. |
| 40 | `qs python property setter` | 5 | property.setter() | Expected result is first with useful context. |
| 41 | `qs python pathlib.Path.read_text` | 5 | pathlib.Path.read_text() | Expected result is first with useful context. |
| 42 | `qs python pathlib.Path.glob` | 5 | pathlib.Path.glob() | Expected result is first with useful context. |
| 43 | `qs python expand home directory` | 3 | Home scheme | Pathlib results mention `os.path.expanduser()` in the top three, but the expected function is not surfaced directly. |
| 44 | `qs python environment variables` | 2 | 1.2. Environment variables | General environment-variable documentation appears, but os.environ is absent. |
| 45 | `qs python sys.argv` | 5 | sys.argv | Expected result is first with useful context. |
| 46 | `qs python sys.path` | 5 | sys.path | Expected result is first with useful context. |
| 47 | `qs python argparse subcommands` | 3 | argparse — Parser for command-line options, arguments and subcommands | The argparse module is first and explicitly covers subcommands, but `add_subparsers()` is not shown. |
| 48 | `qs python json.dumps` | 5 | json.dumps() | Expected result is first with useful context. |
| 49 | `qs python json custom encoder` | 4 | json — JSON encoder and decoder | The JSON module is first, and a top-three result points to extending JSONEncoder, but the expected class and `default()` method are not directly shown. |
| 50 | `qs python csv.DictReader` | 5 | csv.DictReader | Expected result is first with useful context. |
| 51 | `qs python configparser` | 5 | configparser | Expected result is first with useful context. |
| 52 | `qs python tomllib.load` | 5 | tomllib.load() | Expected result is first with useful context. |
| 53 | `qs python pickle security` | 3 | Security Considerations | A general security-considerations page is first, but the pickle warning itself is not visible in the returned context. |
| 54 | `qs python sqlite3 row factory` | 4 | sqlite3.Cursor.row_factory | Connection.row_factory is second; the first result is the closely related Cursor.row_factory. |
| 55 | `qs python regular expression named groups` | 5 | Regular expression HOWTO | The Regular Expression HOWTO is first at its non-capturing and named-groups section. |
| 56 | `qs python re.escape` | 5 | re.escape() | Expected result is first with useful context. |
| 57 | `qs python parse ISO date time` | 3 | classmethod time.strptime(date_string, format) | `datetime.fromisoformat()` appears only in the third result snippet behind `time.strptime()` and XML-RPC results. |
| 58 | `qs python timezone aware datetime` | 4 | class datetime.timezone | `datetime.timezone` is first with useful `tzinfo` context, and the third result explains aware datetimes. |
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
| 72 | `qs python calculate median` | 5 | statistics.median(data) | `statistics.median()` is first with its signature and a useful explanation. |
| 73 | `qs python random choices weights` | 5 | random.choices(population, weights=None, *, cum_weights=None, k=1) | `random.choices()` is first and its signature and snippet explain weighted selection. |
| 74 | `qs python secrets.token_urlsafe` | 5 | secrets.token_urlsafe() | Expected result is first with useful context. |
| 75 | `qs python hashlib.sha256` | 5 | hashlib.sha256() | Expected result is first with useful context. |
| 76 | `qs python uuid.uuid4` | 5 | uuid.uuid4() | Expected result is first with useful context. |
| 77 | `qs python base64 urlsafe encoding` | 5 | base64.urlsafe_b64encode(s, *, padded=True) | `base64.urlsafe_b64encode()` is first with the exact signature and URL-safe behavior. |
| 78 | `qs python build URL query string` | 2 | urllib.parse.quote_plus(string, safe='', encoding=None, errors=None) | `quote_plus()` is related to query-string escaping, but `urllib.parse.urlencode()` is absent. |
| 79 | `qs python logging.basicConfig` | 5 | logging.basicConfig() | Expected result is first with useful context. |
| 80 | `qs python create temporary directory` | 5 | class tempfile.TemporaryDirectory(suffix=None, prefix=None, dir=None, ignore_cleanup_errors=False, *, delete=True) | Expected result is first with useful context. |
| 81 | `qs python copy directory tree` | 5 | shutil.copytree(src, dst, symlinks=False, ignore=None, copy_function=copy2, ignore_dangling_symlinks=False, dirs_exist_ok=False) | `shutil.copytree()` is first with its signature and recursive directory-copy behavior. |
| 82 | `qs python deep copy object` | 4 | copy — Shallow and deep copy operations | The copy module overview is first and discusses deep copies; `deepcopy()` is mentioned in the third result context. |
| 83 | `qs python walk directory recursively` | 5 | os.walk | Expected result is first with useful context. |
| 84 | `qs python run command capture output` | 5 | subprocess.run(args, *, stdin=None, input=None, stdout=None, stderr=None, capture_output=False, shell=False, cwd=None, timeout=None, check=False, encoding=None, errors=None, text=None, env=None, universal_newlines=None, **other_popen_kwargs) | `subprocess.run()` is first, and the signature directly includes `capture_output`. |
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
| 97 | `qs python trace memory allocations` | 5 | tracemalloc — Trace memory allocations | The `tracemalloc` module overview is first with a useful explanation of traced allocation data. |
| 98 | `qs python typing.Protocol` | 5 | typing.Protocol | Expected result is first with useful context. |
| 99 | `qs python typing.TypedDict` | 5 | typing.TypedDict | Expected result is first with useful context. |
| 100 | `qs python typing.Literal` | 5 | typing.Literal | Expected result is first with useful context. |
