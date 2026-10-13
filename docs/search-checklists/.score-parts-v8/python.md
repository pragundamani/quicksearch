# Python v8 regression evaluation

- Commands: 100
- Successful JSON responses: 100/100
- Score: 466/500 (93.2%)
- Mean: 4.66/5
- Distribution: 5: 84, 4: 8, 3: 2, 2: 3, 1: 2, 0: 1

## Key failures

- **0/5** `python regular expression named groups` — Expected: `re` named groups using `(?P<name>...)`. The only result is an unrelated glossary definition of expression; named regular-expression groups are absent.
- **1/5** `python function type annotations` — Expected: Function annotations and `typing`. Only a generic glossary definition of function appears; annotations and typing are absent.
- **1/5** `python argparse subcommands` — Expected: `argparse` subparsers via `add_subparsers()`. Only the generic argparse module page appears; subparsers/add_subparsers are absent.
- **2/5** `python generic type aliases` — Expected: Generic aliases and type aliases. GenericAlias is first, but type aliases and broader context are absent.
- **2/5** `python expand home directory` — Expected: `os.path.expanduser()`. Results mention Path.home and pathlib expansion, but omit the expected os.path.expanduser entry.
- **2/5** `python timezone aware datetime` — Expected: Aware `datetime` objects and `tzinfo`. The datetime module overview is only partial; it gives no aware-datetime or tzinfo context.
- **3/5** `python abstract base classes` — Expected: The `abc` module and `ABC`. The exact concept is first, but the expected abc module and ABC API are absent.
- **3/5** `python pickle security` — Expected: `pickle` security warning. The first result is a general security-considerations page and the third is a pickle page, but the pickle security warning is not surfaced in the output.
- **4/5** `python dictionaries` — Expected: Mapping Types — `dict`. The expected dict section is third; the first glossary hit is still useful.
- **4/5** `python generator expressions` — Expected: Generator expressions. The exact generator-expression entry is second, with useful context.
- **4/5** `python async iterators` — Expected: Asynchronous iterator protocol. The protocol is represented by AsyncIterator hits in the top two.
- **4/5** `python f string debugging` — Expected: Self-documenting f-string expressions using `=`. The f-string documentation is second; the first result is unrelated GDB debugging.
- **4/5** `python json custom encoder` — Expected: `json.JSONEncoder` and `default()`. The JSON module is first and an extending-JSONEncoder section is third, but the exact API is not surfaced.
- **4/5** `python sqlite3 row factory` — Expected: `Connection.row_factory` and `sqlite3.Row`. Connection.row_factory is second; the first Cursor.row_factory result also mentions sqlite3.Row.
- **4/5** `python asyncio timeout` — Expected: `asyncio.timeout()`. The expected asyncio.timeout() result is second, after the related asyncio.Timeout class.
- **4/5** `python inspect.signature` — Expected: `inspect.signature()`. The expected inspect.signature() result is second, after inspect.Signature.
