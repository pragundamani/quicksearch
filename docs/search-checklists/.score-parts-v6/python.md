# Python search checklist score

- Queries: **100**
- Mean score: **4.56/5**
- Score distribution: **5: 77, 4: 10, 3: 7, 2: 5, 1: 0, 0: 1**
- Scores >=3: **94/100**
- Scores >=4: **87/100**
- Command errors/timeouts: **0**

## Failures (score <=2)

- **0 — `qs python enumerations`**: returned unrelated curses, decimal, and editor results; no `enum` or `Enum` result.
- **2 — `qs python keyword only arguments`**: returned ordinary keyword-argument docs, not keyword-only parameters or the `*` separator.
- **2 — `qs python merge dictionaries`**: returned `dict.update()` but omitted the expected `|` and `|=` operators.
- **2 — `qs python environment variables`**: returned startup environment-variable pages, not `os.environ`.
- **2 — `qs python parse ISO date time`**: ranked `time.strptime` first; `datetime.fromisoformat()` appeared only as a mention in the third snippet.
- **2 — `qs python build URL query string`**: returned `quote_plus`; `urllib.parse.urlencode()` was absent.

## Most important regressions

1. Morphological lookup fails completely for **enumerations → enum**.
2. Natural-language task queries confuse close APIs: **keyword-only vs keyword arguments**, **fromisoformat vs strptime**, and **urlencode vs quote_plus**.
3. Modern canonical APIs are displaced by older alternatives: dictionary union operators are replaced by `dict.update()`.
4. Broad concept searches can miss the expected object: **environment variables** does not expose `os.environ`.
5. Several otherwise relevant results have weak ranking or context, notably function annotations, generic type aliases, f-string debugging, pickle security, and `os.path.expanduser()`.
