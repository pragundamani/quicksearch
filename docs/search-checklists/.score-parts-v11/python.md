# Python production-candidate v11 regression

- Candidate: `/tmp/qs-gate/release/qs`
- SHA256: `8f05a6dcd581ef25fe5d2904b169cb3794a94056b19d687cf4646a6372cc096d`
- Checklist: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/python.md`
- Invocation: each checklist command with `--json` and a 20-second timeout
- Completed: **100/100**; errors: **0**; timeouts: **0**
- Score: **500/500 (5.00/5, 100.0%)**
- Distribution: **5=100, 4=0, 3=0, 2=0, 1=0, 0=0**

## Non-perfect results

None.

## Errors and timeouts

None.

## All results

| # | Command | Expected | Top results | Score | Assessment |
|---:|---|---|---|---:|---|
| 1 | `qs python dictionaries` | Mapping Types — `dict` | Mapping Types — dict | 5 | The first result, Mapping Types — dict, is canonical or immediately useful for the expected topic. |
| 2 | `qs python list comprehensions` | List displays and list comprehensions | list comprehension | 5 | The first result, list comprehension, is canonical or immediately useful for the expected topic. |
| 3 | `qs python generator expressions` | Generator expressions | Generator expressions | 5 | The first result, Generator expressions, is canonical or immediately useful for the expected topic. |
| 4 | `qs python decorators` | Function and class decorators | decorator; contextlib.ContextDecorator; contextlib.AsyncContextDecorator | 5 | The first result, decorator, is canonical or immediately useful for the expected topic. |
| 5 | `qs python context managers` | Context manager types and the `with` statement | context manager; Available Context Managers (warnings); typing.ContextManager | 5 | The first result, context manager, is canonical or immediately useful for the expected topic. |
| 6 | `qs python async iterators` | Asynchronous iterator protocol | typing.AsyncIterator; collections.abc.AsyncIterator; pyclbr.Function.is_async | 5 | The first result, typing.AsyncIterator, is canonical or immediately useful for the expected topic. |
| 7 | `qs python structural pattern matching` | `match` statements and patterns | Data model; Compound statements; Pattern | 5 | The first result, Data model, is canonical or immediately useful for the expected topic. |
| 8 | `qs python positional only parameters` | Positional-only parameters using `/` | More Control Flow Tools; class inspect.Signature(parameters=None, *, return_annotation=Signature.empty); parameters | 5 | The first result, More Control Flow Tools, is canonical or immediately useful for the expected topic. |
| 9 | `qs python keyword only arguments` | Keyword-only parameters using `*` | Keyword-only parameters | 5 | The first result, Keyword-only parameters, is canonical or immediately useful for the expected topic. |
| 10 | `qs python exception chaining` | Exception context and `raise ... from` | 8.5. Exception Chaining | 5 | The first result, 8.5. Exception Chaining, is canonical or immediately useful for the expected topic. |
| 11 | `qs python assignment expressions` | Assignment expressions using `:=` | 6.12. Assignment expressions; ast.Expression | 5 | The first result, 6.12. Assignment expressions, is canonical or immediately useful for the expected topic. |
| 12 | `qs python function type annotations` | Function annotations and `typing` | Function annotations | 5 | The first result, Function annotations, is canonical or immediately useful for the expected topic. |
| 13 | `qs python generic type aliases` | Generic aliases and type aliases | Generic type aliases: type statement and TypeAliasType | 5 | The first result, Generic type aliases: type statement and TypeAliasType, is canonical or immediately useful for the expected topic. |
| 14 | `qs python dataclasses` | The `dataclasses` module | dataclasses; Inheritance (dataclasses); Class variables (dataclasses) | 5 | The first result, dataclasses, is canonical or immediately useful for the expected topic. |
| 15 | `qs python frozen dataclass` | `@dataclass(frozen=True)` | Frozen instances (dataclasses); dataclasses.FrozenInstanceError | 5 | The first result, Frozen instances (dataclasses), is canonical or immediately useful for the expected topic. |
| 16 | `qs python enumerations` | The `enum` module and `Enum` | enum.Enum | 5 | The first result, enum.Enum, is canonical or immediately useful for the expected topic. |
| 17 | `qs python abstract base classes` | The `abc` module and `ABC` | abc.ABC and abstract base classes | 5 | The first result, abc.ABC and abstract base classes, is canonical or immediately useful for the expected topic. |
| 18 | `qs python method resolution order` | Class MRO and `type.mro()` | method resolution order; The Python 2.3 Method Resolution Order | 5 | The first result, method resolution order, is canonical or immediately useful for the expected topic. |
| 19 | `qs python descriptor protocol` | Descriptors and `__get__` / `__set__` | descriptor | 5 | The first result, descriptor, is canonical or immediately useful for the expected topic. |
| 20 | `qs python iterator protocol` | Iterator types, `__iter__`, and `__next__` | Iterator Protocol; iterator | 5 | The first result, Iterator Protocol, is canonical or immediately useful for the expected topic. |
| 21 | `qs python enumerate` | Built-in `enumerate()` | enumerate(); threading.enumerate() | 5 | The first result, enumerate(), is canonical or immediately useful for the expected topic. |
| 22 | `qs python zip` | Built-in `zip()` | zip(); zipapp; Examples (zipapp) | 5 | The first result, zip(), is canonical or immediately useful for the expected topic. |
| 23 | `qs python isinstance` | Built-in `isinstance()` | isinstance() | 5 | The first result, isinstance(), is canonical or immediately useful for the expected topic. |
| 24 | `qs python super` | Built-in `super()` | super; pyclbr.Class.super; mmap.MAP_ALIGNED_SUPER | 5 | The first result, super, is canonical or immediately useful for the expected topic. |
| 25 | `qs python convert string to integer` | Built-in `int()` | int; int; object.__int__() | 5 | The first result, int, is canonical or immediately useful for the expected topic. |
| 26 | `qs python memoryview` | Built-in `memoryview` type | memoryview; memoryview.obj; memoryview.ndim | 5 | The first result, memoryview, is canonical or immediately useful for the expected topic. |
| 27 | `qs python breakpoint` | Built-in `breakpoint()` | breakpoint(); bdb.Breakpoint; mmap.MAP_32BIT | 5 | The first result, breakpoint(), is canonical or immediately useful for the expected topic. |
| 28 | `qs python str.removeprefix` | `str.removeprefix()` | str.removeprefix() | 5 | The first result, str.removeprefix(), is canonical or immediately useful for the expected topic. |
| 29 | `qs python str.splitlines` | `str.splitlines()` | str.splitlines() | 5 | The first result, str.splitlines(), is canonical or immediately useful for the expected topic. |
| 30 | `qs python bytes.fromhex` | `bytes.fromhex()` | bytes.fromhex() | 5 | The first result, bytes.fromhex(), is canonical or immediately useful for the expected topic. |
| 31 | `qs python sort list by key` | `list.sort(key=...)` and the Sorting HOW TO | sorted key; pstats.SortKey; pstats.SortKey.NFL | 5 | The first result, sorted key, is canonical or immediately useful for the expected topic. |
| 32 | `qs python dict.setdefault` | `dict.setdefault()` | dict.setdefault(); c.PyDict_SetItem(); c.PyDict_SetDefault() | 5 | The first result, dict.setdefault(), is canonical or immediately useful for the expected topic. |
| 33 | `qs python merge dictionaries` | Dictionary union operators `\|` and `\|=` | Dictionary union operators | 5 | The first result, Dictionary union operators, is canonical or immediately useful for the expected topic. |
| 34 | `qs python set symmetric_difference` | `set.symmetric_difference()` | set.symmetric_difference(); set.symmetric_difference_update() | 5 | The first result, set.symmetric_difference(), is canonical or immediately useful for the expected topic. |
| 35 | `qs python frozenset` | Built-in `frozenset` type | frozenset; tkinter.font; frozenset.copy() | 5 | The first result, frozenset, is canonical or immediately useful for the expected topic. |
| 36 | `qs python format specification mini language` | Format specification mini-language | Format specification mini-language; __format__(format_spec, /); format | 5 | The first result, Format specification mini-language, is canonical or immediately useful for the expected topic. |
| 37 | `qs python f string debugging` | Self-documenting f-string expressions using `=` | Self-documenting f-string expressions | 5 | The first result, Self-documenting f-string expressions, is canonical or immediately useful for the expected topic. |
| 38 | `qs python FileNotFoundError` | Built-in `FileNotFoundError` exception | FileNotFoundError; ModuleNotFoundError; zoneinfo.ZoneInfoNotFoundError | 5 | The first result, FileNotFoundError, is canonical or immediately useful for the expected topic. |
| 39 | `qs python ExceptionGroup` | Built-in `ExceptionGroup` | ExceptionGroup; BaseExceptionGroup; BaseExceptionGroup.message | 5 | The first result, ExceptionGroup, is canonical or immediately useful for the expected topic. |
| 40 | `qs python property setter` | `property` and its `setter()` decorator | property.setter(); property; xml.sax.handler.property_xml_string | 5 | The first result, property.setter(), is canonical or immediately useful for the expected topic. |
| 41 | `qs python pathlib.Path.read_text` | `Path.read_text()` | pathlib.Path.read_text() | 5 | The first result, pathlib.Path.read_text(), is canonical or immediately useful for the expected topic. |
| 42 | `qs python pathlib.Path.glob` | `Path.glob()` | pathlib.Path.glob() | 5 | The first result, pathlib.Path.glob(), is canonical or immediately useful for the expected topic. |
| 43 | `qs python expand home directory` | `os.path.expanduser()` | os.path.expanduser | 5 | The first result, os.path.expanduser, is canonical or immediately useful for the expected topic. |
| 44 | `qs python environment variables` | `os.environ` | os.environ | 5 | The first result, os.environ, is canonical or immediately useful for the expected topic. |
| 45 | `qs python sys.argv` | `sys.argv` | sys.argv; sys.orig_argv; c.PySys_SetArgv() | 5 | The first result, sys.argv, is canonical or immediately useful for the expected topic. |
| 46 | `qs python sys.path` | `sys.path` | sys.path; sys.path_hooks; sys.path_importer_cache | 5 | The first result, sys.path, is canonical or immediately useful for the expected topic. |
| 47 | `qs python argparse subcommands` | `argparse` subparsers via `add_subparsers()` | argparse.add_subparsers | 5 | The first result, argparse.add_subparsers, is canonical or immediately useful for the expected topic. |
| 48 | `qs python json.dumps` | `json.dumps()` | json.dumps(); json.dump() | 5 | The first result, json.dumps(), is canonical or immediately useful for the expected topic. |
| 49 | `qs python json custom encoder` | `json.JSONEncoder` and `default()` | json.JSONEncoder and default() | 5 | The first result, json.JSONEncoder and default(), is canonical or immediately useful for the expected topic. |
| 50 | `qs python csv.DictReader` | `csv.DictReader` | csv.DictReader; csv.DictReader.fieldnames | 5 | The first result, csv.DictReader, is canonical or immediately useful for the expected topic. |
| 51 | `qs python configparser` | The `configparser` module | configparser; Exceptions (configparser); Quick Start (configparser) | 5 | The first result, configparser, is canonical or immediately useful for the expected topic. |
| 52 | `qs python tomllib.load` | `tomllib.load()` | tomllib.load(); tomllib.loads() | 5 | The first result, tomllib.load(), is canonical or immediately useful for the expected topic. |
| 53 | `qs python pickle security` | `pickle` security warning | pickle security warning | 5 | The first result, pickle security warning, is canonical or immediately useful for the expected topic. |
| 54 | `qs python sqlite3 row factory` | `Connection.row_factory` and `sqlite3.Row` | sqlite3.Cursor.row_factory; sqlite3.Connection.row_factory | 5 | The first result, sqlite3.Cursor.row_factory, is canonical or immediately useful for the expected topic. |
| 55 | `qs python regular expression named groups` | `re` named groups using `(?P<name>...)` | Regular expression named groups | 5 | The first result, Regular expression named groups, is canonical or immediately useful for the expected topic. |
| 56 | `qs python re.escape` | `re.escape()` | re.escape() | 5 | The first result, re.escape(), is canonical or immediately useful for the expected topic. |
| 57 | `qs python parse ISO date time` | `datetime.fromisoformat()` | datetime.fromisoformat | 5 | The first result, datetime.fromisoformat, is canonical or immediately useful for the expected topic. |
| 58 | `qs python timezone aware datetime` | Aware `datetime` objects and `tzinfo` | Timezone-aware datetime | 5 | The first result, Timezone-aware datetime, is canonical or immediately useful for the expected topic. |
| 59 | `qs python zoneinfo.ZoneInfo` | `zoneinfo.ZoneInfo` | zoneinfo.ZoneInfo; zoneinfo.ZoneInfo.key; zoneinfo.ZoneInfo.no_cache() | 5 | The first result, zoneinfo.ZoneInfo, is canonical or immediately useful for the expected topic. |
| 60 | `qs python time.monotonic` | `time.monotonic()` | time.monotonic(); time.monotonic_ns(); c.PyTime_Monotonic() | 5 | The first result, time.monotonic(), is canonical or immediately useful for the expected topic. |
| 61 | `qs python collections.defaultdict` | `collections.defaultdict` | collections.defaultdict; collections.defaultdict.__missing__(); collections.defaultdict.default_factory | 5 | The first result, collections.defaultdict, is canonical or immediately useful for the expected topic. |
| 62 | `qs python collections.Counter.most_common` | `Counter.most_common()` | collections.Counter.most_common() | 5 | The first result, collections.Counter.most_common(), is canonical or immediately useful for the expected topic. |
| 63 | `qs python collections.ChainMap` | `collections.ChainMap` | collections.ChainMap; collections.ChainMap.maps; collections.ChainMap.parents | 5 | The first result, collections.ChainMap, is canonical or immediately useful for the expected topic. |
| 64 | `qs python heapq.nlargest` | `heapq.nlargest()` | heapq.nlargest() | 5 | The first result, heapq.nlargest(), is canonical or immediately useful for the expected topic. |
| 65 | `qs python bisect.insort` | `bisect.insort()` | bisect.insort(); bisect.insort_left(); bisect.insort_right() | 5 | The first result, bisect.insort(), is canonical or immediately useful for the expected topic. |
| 66 | `qs python itertools.zip_longest` | `itertools.zip_longest()` | itertools.zip_longest() | 5 | The first result, itertools.zip_longest(), is canonical or immediately useful for the expected topic. |
| 67 | `qs python itertools.groupby` | `itertools.groupby()` | itertools.groupby() | 5 | The first result, itertools.groupby(), is canonical or immediately useful for the expected topic. |
| 68 | `qs python functools.lru_cache` | `functools.lru_cache()` | functools.lru_cache() | 5 | The first result, functools.lru_cache(), is canonical or immediately useful for the expected topic. |
| 69 | `qs python functools.partial` | `functools.partial()` | functools.partial(); functools.partial.args; functools.partial.func | 5 | The first result, functools.partial(), is canonical or immediately useful for the expected topic. |
| 70 | `qs python operator.itemgetter` | `operator.itemgetter()` | operator.itemgetter() | 5 | The first result, operator.itemgetter(), is canonical or immediately useful for the expected topic. |
| 71 | `qs python math.isclose` | `math.isclose()` | math.isclose() | 5 | The first result, math.isclose(), is canonical or immediately useful for the expected topic. |
| 72 | `qs python calculate median` | `statistics.median()` | statistics.median(data); Other Applications; median | 5 | The first result, statistics.median(data), is canonical or immediately useful for the expected topic. |
| 73 | `qs python random choices weights` | `random.choices(..., weights=...)` | random.choices(population, weights=None, *, cum_weights=None, k=1); statistics.fmean(data, weights=None); choices | 5 | The first result, random.choices(population, weights=None, *, cum_weights=None, k=1), is canonical or immediately useful for the expected topic. |
| 74 | `qs python secrets.token_urlsafe` | `secrets.token_urlsafe()` | secrets.token_urlsafe() | 5 | The first result, secrets.token_urlsafe(), is canonical or immediately useful for the expected topic. |
| 75 | `qs python hashlib.sha256` | `hashlib.sha256()` | hashlib.sha256() | 5 | The first result, hashlib.sha256(), is canonical or immediately useful for the expected topic. |
| 76 | `qs python uuid.uuid4` | `uuid.uuid4()` | uuid.uuid4() | 5 | The first result, uuid.uuid4(), is canonical or immediately useful for the expected topic. |
| 77 | `qs python base64 urlsafe encoding` | `base64.urlsafe_b64encode()` | base64.urlsafe_b64encode(s, *, padded=True); binascii.URLSAFE_BASE64_ALPHABET; base64 | 5 | The first result, base64.urlsafe_b64encode(s, *, padded=True), is canonical or immediately useful for the expected topic. |
| 78 | `qs python build URL query string` | `urllib.parse.urlencode()` | urllib.parse.urlencode | 5 | The first result, urllib.parse.urlencode, is canonical or immediately useful for the expected topic. |
| 79 | `qs python logging.basicConfig` | `logging.basicConfig()` | logging.basicConfig() | 5 | The first result, logging.basicConfig(), is canonical or immediately useful for the expected topic. |
| 80 | `qs python create temporary directory` | `tempfile.TemporaryDirectory` | class tempfile.TemporaryDirectory(suffix=None, prefix=None, dir=None, ignore_cleanup_errors=False, *, delete=True); os.symlink(src, dst, target_is_directory=False, *, dir_fd=None); create | 5 | The first result, class tempfile.TemporaryDirectory(suffix=None, prefix=None, dir=None, ignore_cleanup_errors=False, *, delete=True), is canonical or immediately useful for the expected topic. |
| 81 | `qs python copy directory tree` | `shutil.copytree()` | shutil.copytree(src, dst, symlinks=False, ignore=None, copy_function=copy2, ignore_dangling_symlinks=False, dirs_exist_ok=False); Path.copy(target, *, follow_symlinks=True, preserve_metadata=False); copy | 5 | The first result, shutil.copytree(src, dst, symlinks=False, ignore=None, copy_function=copy2, ignore_dangling_symlinks=False, dirs_exist_ok=False), is canonical or immediately useful for the expected topic. |
| 82 | `qs python deep copy object` | `copy.deepcopy()` | copy — Shallow and deep copy operations; TarInfo.replace(name=..., mtime=..., mode=..., linkname=..., uid=..., gid=..., uname=..., gname=..., deep=True); __deepcopy__ | 5 | The first result, copy — Shallow and deep copy operations, is canonical or immediately useful for the expected topic. |
| 83 | `qs python walk directory recursively` | `os.walk()` | os.walk; os.walk(); pathlib.Path.rglob() | 5 | The first result, os.walk, is canonical or immediately useful for the expected topic. |
| 84 | `qs python run command capture output` | `subprocess.run(..., capture_output=True)` | subprocess.run(args, *, stdin=None, input=None, stdout=None, stderr=None, capture_output=False, shell=False, cwd=None, timeout=None, check=False, encoding=None, errors=None, text=None, env=None, universal_newlines=None, **other_popen_kwargs); The run command; run | 5 | The first result, subprocess.run(args, *, stdin=None, input=None, stdout=None, stderr=None, capture_output=False, shell=False, cwd=None, timeout=None, check=False, encoding=None, errors=None, text=None, env=None, universal_newlines=None, **other_popen_kwargs), is canonical or immediately useful for the expected topic. |
| 85 | `qs python concurrent.futures.ThreadPoolExecutor` | `ThreadPoolExecutor` | concurrent.futures.ThreadPoolExecutor; ThreadPoolExecutor (concurrent.futures) | 5 | The first result, concurrent.futures.ThreadPoolExecutor, is canonical or immediately useful for the expected topic. |
| 86 | `qs python multiprocessing Queue` | `multiprocessing.Queue` | multiprocessing.Queue; multiprocessing; multiprocessing.Queue.get() | 5 | The first result, multiprocessing.Queue, is canonical or immediately useful for the expected topic. |
| 87 | `qs python threading Lock` | `threading.Lock` | threading.Lock; threading; threading.Lock.locked() | 5 | The first result, threading.Lock, is canonical or immediately useful for the expected topic. |
| 88 | `qs python run async tasks concurrently` | `asyncio.gather()` | Running tasks concurrently (Coroutines and tasks) | 5 | The first result, Running tasks concurrently (Coroutines and tasks), is canonical or immediately useful for the expected topic. |
| 89 | `qs python asyncio timeout` | `asyncio.timeout()` | asyncio.timeout() | 5 | The first result, asyncio.timeout(), is canonical or immediately useful for the expected topic. |
| 90 | `qs python queue.PriorityQueue` | `queue.PriorityQueue` | queue.PriorityQueue; Priority Queue (Queues) | 5 | The first result, queue.PriorityQueue, is canonical or immediately useful for the expected topic. |
| 91 | `qs python socket.create_connection` | `socket.create_connection()` | socket.create_connection() | 5 | The first result, socket.create_connection(), is canonical or immediately useful for the expected topic. |
| 92 | `qs python unittest.mock.patch` | `unittest.mock.patch()` | unittest.mock.patch(); unittest.mock.patch.dict(); unittest.mock.patch.object() | 5 | The first result, unittest.mock.patch(), is canonical or immediately useful for the expected topic. |
| 93 | `qs python doctest` | The `doctest` module | doctest; Soapbox (doctest); Debugging (doctest) | 5 | The first result, doctest, is canonical or immediately useful for the expected topic. |
| 94 | `qs python warnings filterwarnings` | `warnings.filterwarnings()` | warnings.filterwarnings(); warnings; sys.flags.context_aware_warnings | 5 | The first result, warnings.filterwarnings(), is canonical or immediately useful for the expected topic. |
| 95 | `qs python importlib.resources.files` | `importlib.resources.files()` | importlib.resources.files(); importlib.resources.as_file(); importlib.resources.abc.TraversableResources.files() | 5 | The first result, importlib.resources.files(), is canonical or immediately useful for the expected topic. |
| 96 | `qs python inspect.signature` | `inspect.signature()` | inspect.signature(); inspect.Signature; inspect.Signature.empty | 5 | The first result, inspect.signature(), is canonical or immediately useful for the expected topic. |
| 97 | `qs python trace memory allocations` | The `tracemalloc` module | tracemalloc — Trace memory allocations; Memory Management; Trace | 5 | The first result, tracemalloc — Trace memory allocations, is canonical or immediately useful for the expected topic. |
| 98 | `qs python typing.Protocol` | `typing.Protocol` | typing.Protocol; typing.is_protocol(); typing.get_protocol_members() | 5 | The first result, typing.Protocol, is canonical or immediately useful for the expected topic. |
| 99 | `qs python typing.TypedDict` | `typing.TypedDict` | typing.TypedDict; typing.TypedDict.__total__; typing.TypedDict.__closed__ | 5 | The first result, typing.TypedDict, is canonical or immediately useful for the expected topic. |
| 100 | `qs python typing.Literal` | `typing.Literal` | typing.Literal; typing.LiteralString | 5 | The first result, typing.Literal, is canonical or immediately useful for the expected topic. |
