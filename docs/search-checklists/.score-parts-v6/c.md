# C quicksearch score summary

- Queries: **100**
- Mean score: **4.63/5**
- Score distribution: **5: 83, 4: 5, 3: 8, 2: 2, 1: 0, 0: 2**
- Score >=3: **96/100**
- Score >=4: **88/100**
- Command/provider errors: **0**

## Failures (score <3)

- **0 — `qs c sequence points`**: No result covers sequencing or sequence points; `memory_order` is an unrelated first hit.
- **0 — `qs c variadic macros`**: Results confuse variadic preprocessor macros with variadic functions and `<stdarg.h>`; `__VA_ARGS__` is absent.
- **2 — `qs c designated initializers`**: Only an indirect mention appears in a rank-2 struct-initialization result; array designation is missing.
- **2 — `qs c include guards`**: `#pragma once` is first, but the standard include-guard pattern is not shown.

## Most important regressions

- Plural normalization fails badly for **structures**: the exact struct page is only rank 4.
- Natural task search for **read a file line by line** ranks `fread` twice before `fgets`.
- Natural task search for **sort array with comparator** ranks `bsearch` first and `qsort` only fourth.
- **copy overlapping memory** ranks unsafe-for-overlap `memcpy` before `memmove`.
- Broad concept coverage is incomplete for **storage duration**, **type qualifiers**, **alignment**, and **command line arguments**.
