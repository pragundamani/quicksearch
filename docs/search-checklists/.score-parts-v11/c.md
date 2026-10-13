# C quicksearch v11 evaluation

- Executable: `/tmp/qs-gate/release/qs`
- SHA256: `8f05a6dcd581ef25fe5d2904b169cb3794a94056b19d687cf4646a6372cc096d`
- Commands: **100**
- Timeout: **20 seconds per command**
- Exit-code failures: **0**
- Timeouts: **0**
- Score: **495/500 (99.0%)**
- Mean: **4.95/5**

## Score distribution

| Score | Count |
|---:|---:|
| 5 | 95 |
| 4 | 5 |
| 3 | 0 |
| 2 | 0 |
| 1 | 0 |
| 0 | 0 |

## Results below 5

| # | Query | Score | Top results | Assessment |
|---:|---|---:|---|---|
| 1 | `arrays` | 4 | `Array initialization`, `Array declaration`, `Null-terminated wide strings`, `Null-terminated byte strings` | Array declaration is second; the first result is useful array initialization, but subscripting is not surfaced. |
| 12 | `type qualifiers` | 4 | `const type qualifier`, `restrict type qualifier`, `volatile type qualifier` | const, restrict, and volatile qualifiers occupy the top three, but _Atomic is absent. |
| 91 | `read a file line by line` | 4 | `fread`, `fread`, `fgets` | fgets is third and useful; the first two fread results do not directly support line-by-line input. |
| 92 | `copy overlapping memory` | 4 | `memcpy, memcpy_s`, `memmove, memmove_s`, `va_copy`, `Analyzability` | memmove is second and useful, but memcpy is first even though overlap makes memcpy unsafe. |
| 100 | `handle end of file` | 4 | `fopen, fopen_s`, `feof`, `fclose`, `fgetc, getc` | feof is second and useful; fopen is first and does not directly explain EOF handling. |

## Errors and timeouts

None.
