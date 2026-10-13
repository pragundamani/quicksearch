# C quicksearch production-candidate v9 regression

- Executable: `/tmp/qs-gate/release/qs`
- Commands: **100**
- Timeout: **30 seconds per command**
- Exit-code failures: **0**
- Timeouts: **0**
- Score: **486/500 (97.2%)**
- Mean: **4.86/5**

## Score distribution

| Score | Count |
|---:|---:|
| 5 | 91 |
| 4 | 5 |
| 3 | 3 |
| 2 | 1 |
| 1 | 0 |
| 0 | 0 |

## Results below 5

| # | Query | Score | Top results | Assessment |
|---:|---|---:|---|---|
| 1 | `arrays` | 4 | `Array initialization`, `Array declaration`, `Null-terminated wide strings`, `Null-terminated byte strings` | Array declaration is second; the first result is useful array initialization. |
| 4 | `structures` | 3 | `Enumerations`, `Objects and alignment`, `String literals`, `Struct declaration` | The expected Struct declaration result is fourth; the first three are indirect. |
| 10 | `storage duration` | 2 | `Static storage duration`, `Thread storage duration` | Only static and thread storage duration results appear; automatic and allocated duration are absent. |
| 12 | `type qualifiers` | 4 | `const type qualifier`, `restrict type qualifier`, `volatile type qualifier` | const, restrict, and volatile qualifiers occupy the top three, but _Atomic is absent. |
| 91 | `read a file line by line` | 4 | `fread`, `fread`, `fgets` | fgets is third and useful; the first two fread results do not directly support line-by-line input. |
| 92 | `copy overlapping memory` | 4 | `memcpy, memcpy_s`, `memmove, memmove_s`, `va_copy`, `Analyzability` | memmove is second and useful, but memcpy is first even though overlap makes memcpy unsafe. |
| 95 | `sort array with comparator` | 3 | `bsearch, bsearch_s`, `Algorithms`, `short`, `qsort, qsort_s` | qsort is fourth; bsearch is first and is not a sorting function. |
| 97 | `format date and time` | 3 | `Date and time utilities` | Only a broad Date and time utilities page is returned, with no useful snippet or direct strftime result. |
| 100 | `handle end of file` | 4 | `fopen, fopen_s`, `feof`, `fclose`, `fgetc, getc` | feof is second and useful; fopen is first and does not directly explain EOF handling. |

## Key failures

- `storage duration` is the main coverage failure. It returns only static and thread duration. It omits automatic and allocated duration.
- `structures` ranks `Struct declaration` fourth, behind unrelated or indirect results.
- `sort array with comparator` ranks `bsearch` first and `qsort` fourth.
- `format date and time` returns only a broad category page. It does not return `strftime` directly.
- Two task queries put a potentially wrong operation before the right one: `copy overlapping memory` puts `memcpy` before `memmove`, and `read a file line by line` puts `fread` before `fgets`.

Detailed command output and judgments are in `c.json`.
