# C candidate-v3 quicksearch scores

## Summary

- Executable: `/tmp/qs-candidate-v3/release/qs` (not the installed `qs`).
- Commands run: **100/100** with a **20-second timeout per command**.
- Average: **4.33/5**.
- Median: **5/5**.
- Distribution: **0: 4, 1: 5, 2: 2, 3: 6, 4: 9, 5: 74**.
- Pass rate (score >=3): **89/100 (89%)**.
- Strong pass rate (score >=4): **83/100 (83%)**.
- Command errors: **0/100**. Timeouts: **0/100**.

## Comparisons

### Installed baseline

- Baseline: average **4.35**, median **5**, distribution **0: 5, 1: 6, 2: 1, 3: 3, 4: 7, 5: 78**, score >=3 **88%**, score >=4 **85%**, and errors **1/100**.
- Candidate-v3 change: average **-0.02**, median **unchanged**, score >=3 **+1 percentage point**, score >=4 **-2 percentage points**, and errors **-1**.

### Candidate-v2

- Candidate-v2: average **4.30**, median **5**, distribution **0: 5, 1: 6, 2: 1, 3: 6, 4: 6, 5: 76**, score >=3 **88%**, score >=4 **82%**, errors **0/100**, and timeouts **0/100**.
- Candidate-v3 change: average **+0.03**, median **unchanged**, score >=3 **+1 percentage point**, score >=4 **+1 percentage point**, errors **unchanged**, and timeouts **unchanged**.

## Results

| # | Command | Score | Observed top result or error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs c arrays` | 5 | #1 `Array declaration` | `Array declaration` ranks first and provides useful documentation for the expected result. |
| 2 | `qs c pointers` | 4 | Web card: `Pointer (computer programming)` | The web card explains memory addresses and dereferencing, but it does not show C pointer declarations or types. |
| 3 | `qs c pointer arithmetic` | 3 | #1 `Complex number arithmetic` | Pointer and arithmetic pages appear, but the relevant operator page is fourth and no visible detail explains pointer arithmetic. |
| 4 | `qs c structures` | 1 | Web card: `Structure` | The web card describes structures in a general sense and does not cover C `struct` types. |
| 5 | `qs c unions` | 3 | #1 `union` | The exact `union` keyword is first, but the result contains only license text and no type guidance. |
| 6 | `qs c enumerations` | 5 | #1 `Enumerations` | `Enumerations` ranks first and provides useful documentation for the expected result. |
| 7 | `qs c bit fields` | 5 | #1 `Bit-fields` | `Bit-fields` ranks first and provides useful documentation for the expected result. |
| 8 | `qs c function pointers` | 4 | Web card: `Function pointer` | The web card explains function pointers and indirect calls, but it gives no C declaration syntax. |
| 9 | `qs c variadic functions` | 5 | #1 `Variadic functions` | `Variadic functions` ranks first and provides useful documentation for the expected result. |
| 10 | `qs c storage duration` | 4 | #1 `Static storage duration` | Static and thread storage duration rank first and second, but automatic and allocated duration are missing. |
| 11 | `qs c language linkage` | 1 | #1 `C language` | The generic C language page leads, but internal, external, and no-linkage rules are absent. |
| 12 | `qs c type qualifiers` | 4 | #1 `const type qualifier` | `const`, `restrict`, and `volatile` occupy the top three with useful context; `_Atomic` is missing. |
| 13 | `qs c restrict` | 4 | #1 `restrict` | The keyword stub leads, while the second result gives useful `restrict` rules and code. |
| 14 | `qs c integer promotions` | 1 | #1 `Integer constant` | Integer constants and fixed-width types appear, but integer-promotion rules do not. |
| 15 | `qs c usual arithmetic conversions` | 3 | #1 `cos` | Implicit conversions is third, but unrelated math results lead and the output does not explain the usual arithmetic conversions. |
| 16 | `qs c operator precedence` | 5 | #1 `C Operator Precedence` | `C Operator Precedence` ranks first and provides useful documentation for the expected result. |
| 17 | `qs c compound literals` | 5 | #1 `Compound literals` | `Compound literals` ranks first and provides useful documentation for the expected result. |
| 18 | `qs c designated initializers` | 0 | #1 `atomic_is_lock_free` | The results are unrelated to designated initialization. |
| 19 | `qs c generic selection` | 5 | #1 `Generic selection` | `Generic selection` ranks first and provides useful documentation for the expected result. |
| 20 | `qs c static assertions` | 5 | #1 `Static assertion` | `Static assertion` ranks first and provides useful documentation for the expected result. |
| 21 | `qs c flexible array member` | 1 | #1 `FILE` | FILE and floating-point results lead; Array declaration is fifth but flexible members are not explained. |
| 22 | `qs c variable length arrays` | 3 | #1 `Array declaration` | Array declaration leads and its example demonstrates a VLA, but the result does not directly explain variable-length array rules. |
| 23 | `qs c alignment` | 5 | #1 `Objects and alignment` | `Objects and alignment` ranks first and provides useful documentation for the expected result. |
| 24 | `qs c object lifetime` | 2 | Web card: `Object lifetime` | The web card discusses object-oriented lifetime rather than C object-lifetime rules. |
| 25 | `qs c undefined behavior` | 5 | #1 `Undefined behavior` | `Undefined behavior` ranks first and provides useful documentation for the expected result. |
| 26 | `qs c sequence points` | 5 | Web card: `Sequence point` | The web card directly defines C/C++ sequence points and notes the newer terminology. |
| 27 | `qs c command line arguments` | 3 | #1 `argc` | `argc` links to the correct `main` page, but the output gives no `argv` or parameter detail. |
| 28 | `qs c preprocessing directives` | 4 | #1 `div` | A wrong `div` result leads, but `#if`, `#line`, `#define`, and `#pragma` pages provide useful directive coverage. |
| 29 | `qs c conditional compilation` | 3 | Web card: `Conditional compilation` | The web card is relevant, but it lacks concrete `#if` and `#ifdef` guidance. |
| 30 | `qs c include guards` | 5 | #1 `#include directive` | The include page ranks first and shows the standard `#ifndef`/`#define` guard pattern. |
| 31 | `qs c variadic macros` | 4 | Web card: `Variadic macro` | The web card accurately defines C variadic macros, but it omits `__VA_ARGS__` syntax. |
| 32 | `qs c escape sequences` | 5 | #1 `Escape sequences` | `Escape sequences` ranks first and provides useful documentation for the expected result. |
| 33 | `qs c string literals` | 5 | #1 `String literals` | `String literals` ranks first and provides useful documentation for the expected result. |
| 34 | `qs c implicit conversions` | 5 | #1 `Implicit conversions` | `Implicit conversions` ranks first and provides useful documentation for the expected result. |
| 35 | `qs c compound assignment operators` | 5 | #1 `Assignment operators` | Assignment operators ranks first and directly covers compound assignment. |
| 36 | `qs c printf` | 5 | #1 `printf` | `printf` ranks first with format-string guidance, despite the displayed signature being for `snprintf`. |
| 37 | `qs c snprintf` | 5 | #1 `snprintf` | `snprintf` ranks first and provides useful documentation for the expected result. |
| 38 | `qs c scanf` | 5 | #1 `scanf` | `scanf` ranks first and provides useful documentation for the expected result. |
| 39 | `qs c fopen` | 5 | #1 `fopen` | `fopen` ranks first and provides useful documentation for the expected result. |
| 40 | `qs c fclose` | 5 | #1 `fclose` | `fclose` ranks first and provides useful documentation for the expected result. |
| 41 | `qs c fgets` | 5 | #1 `fgets` | `fgets` ranks first and provides useful documentation for the expected result. |
| 42 | `qs c fread` | 5 | #1 `fread` | `fread` ranks first and provides useful documentation for the expected result. |
| 43 | `qs c fwrite` | 5 | #1 `fwrite` | `fwrite` ranks first and provides useful documentation for the expected result. |
| 44 | `qs c fseek` | 5 | #1 `fseek` | `fseek` ranks first and provides useful documentation for the expected result. |
| 45 | `qs c perror` | 5 | #1 `perror` | `perror` ranks first and provides useful documentation for the expected result. |
| 46 | `qs c malloc` | 5 | #1 `malloc` | `malloc` ranks first and provides useful documentation for the expected result. |
| 47 | `qs c calloc` | 5 | #1 `calloc` | `calloc` ranks first and provides useful documentation for the expected result. |
| 48 | `qs c realloc` | 5 | #1 `realloc` | `realloc` ranks first and provides useful documentation for the expected result. |
| 49 | `qs c free` | 5 | #1 `free` | `free` ranks first and provides useful documentation for the expected result. |
| 50 | `qs c memcpy` | 5 | #1 `memcpy` | `memcpy` ranks first and provides useful documentation for the expected result. |
| 51 | `qs c memmove` | 5 | #1 `memmove` | `memmove` ranks first and provides useful documentation for the expected result. |
| 52 | `qs c memset` | 5 | #1 `memset` | `memset` ranks first and provides useful documentation for the expected result. |
| 53 | `qs c memcmp` | 5 | #1 `memcmp` | `memcmp` ranks first and provides useful documentation for the expected result. |
| 54 | `qs c strlen` | 5 | #1 `strlen` | `strlen` ranks first and provides useful documentation for the expected result. |
| 55 | `qs c strcmp` | 5 | #1 `strcmp` | `strcmp` ranks first and provides useful documentation for the expected result. |
| 56 | `qs c strchr` | 5 | #1 `strchr` | `strchr` ranks first and provides useful documentation for the expected result. |
| 57 | `qs c strstr` | 5 | #1 `strstr` | `strstr` ranks first and provides useful documentation for the expected result. |
| 58 | `qs c strtok` | 5 | #1 `strtok` | `strtok` ranks first and provides useful documentation for the expected result. |
| 59 | `qs c qsort` | 5 | #1 `qsort` | `qsort` ranks first and provides useful documentation for the expected result. |
| 60 | `qs c bsearch` | 5 | #1 `bsearch` | `bsearch` ranks first with a complete comparator-based example, despite a confusing deprecation snippet. |
| 61 | `qs c strtol` | 5 | #1 `strtol` | `strtol` ranks first and provides useful documentation for the expected result. |
| 62 | `qs c strtod` | 5 | #1 `strtod` | `strtod` ranks first and provides useful documentation for the expected result. |
| 63 | `qs c assert` | 5 | #1 `assert` | `assert` ranks first and provides useful documentation for the expected result. |
| 64 | `qs c setjmp` | 5 | #1 `setjmp` | `setjmp` ranks first and provides useful documentation for the expected result. |
| 65 | `qs c signal` | 5 | #1 `signal` | `signal` ranks first and provides useful documentation for the expected result. |
| 66 | `qs c strftime` | 5 | #1 `strftime` | `strftime` ranks first and provides useful documentation for the expected result. |
| 67 | `qs c timespec_get` | 5 | #1 `timespec_get` | `timespec_get` ranks first and provides useful documentation for the expected result. |
| 68 | `qs c rand` | 5 | #1 `rand` | `rand` ranks first and provides useful documentation for the expected result. |
| 69 | `qs c sqrt` | 5 | #1 `sqrt` | `sqrt` ranks first and provides useful documentation for the expected result. |
| 70 | `qs c isnan` | 5 | #1 `isnan` | `isnan` ranks first and provides useful documentation for the expected result. |
| 71 | `qs c isalpha` | 5 | #1 `isalpha` | `isalpha` ranks first and provides useful documentation for the expected result. |
| 72 | `qs c toupper` | 5 | #1 `toupper` | `toupper` ranks first and provides useful documentation for the expected result. |
| 73 | `qs c setlocale` | 5 | #1 `setlocale` | `setlocale` ranks first and provides useful documentation for the expected result. |
| 74 | `qs c mbrtowc` | 5 | #1 `mbrtowc` | `mbrtowc` ranks first and provides useful documentation for the expected result. |
| 75 | `qs c thrd_create` | 5 | #1 `thrd_create` | `thrd_create` ranks first and provides useful documentation for the expected result. |
| 76 | `qs c mtx_lock` | 5 | #1 `mtx_lock` | `mtx_lock` ranks first and provides useful documentation for the expected result. |
| 77 | `qs c cnd_wait` | 5 | #1 `cnd_wait` | `cnd_wait` ranks first and provides useful documentation for the expected result. |
| 78 | `qs c call_once` | 5 | #1 `call_once` | `call_once` ranks first and provides useful documentation for the expected result. |
| 79 | `qs c atomic_fetch_add` | 5 | #1 `atomic_fetch_add` | `atomic_fetch_add` ranks first and provides useful documentation for the expected result. |
| 80 | `qs c atomic_compare_exchange_strong` | 5 | #1 `atomic_compare_exchange_strong` | `atomic_compare_exchange_strong` ranks first and provides useful documentation for the expected result. |
| 81 | `qs c memory_order_acquire` | 5 | #1 `memory_order` | The `memory_order` page ranks first and its enum directly includes `memory_order_acquire`. |
| 82 | `qs c size_t` | 5 | #1 `size_t` | `size_t` ranks first and provides useful documentation for the expected result. |
| 83 | `qs c ptrdiff_t` | 5 | #1 `ptrdiff_t` | `ptrdiff_t` ranks first and provides useful documentation for the expected result. |
| 84 | `qs c va_list` | 5 | #1 `va_list` | `va_list` ranks first and provides useful documentation for the expected result. |
| 85 | `qs c struct tm` | 5 | #1 `tm` | `tm` ranks first with the exact calendar-components structure description. |
| 86 | `qs c FILE` | 5 | #1 `FILE` | `FILE` ranks first and provides useful documentation for the expected result. |
| 87 | `qs c NULL` | 5 | #1 `NULL` | `NULL` ranks first and provides useful documentation for the expected result. |
| 88 | `qs c errno` | 5 | #1 `errno` | `errno` ranks first and provides useful documentation for the expected result. |
| 89 | `qs c stdatomic.h` | 5 | #1 `Standard library header <stdatomic.h>` | `Standard library header <stdatomic.h>` ranks first and provides useful documentation for the expected result. |
| 90 | `qs c complex numbers` | 5 | #1 `Complex number arithmetic` | `Complex number arithmetic` ranks first and provides useful documentation for the expected result. |
| 91 | `qs c read a file line by line` | 4 | #1 `fread` | A misleading `fread` task card leads, but `fgets` is third with useful bounded line-input context. |
| 92 | `qs c copy overlapping memory` | 0 | #1 `va_copy` | The results discuss `va_copy` and the memory model; `memmove` is absent. |
| 93 | `qs c allocate zero initialized array` | 0 | #1 `FP_ZERO` | Zero classification and array pages appear, but `calloc` is absent. |
| 94 | `qs c parse integer from string` | 5 | #1 `strtol` | `strtol` ranks first through the task card, followed by the detailed parser reference. |
| 95 | `qs c sort array with comparator` | 0 | #1 `short` | The results are unrelated to sorting and do not include `qsort` or comparator guidance. |
| 96 | `qs c get current calendar time` | 4 | #1 `gets` | An unrelated `gets` result leads, but `time` is second with a useful current-calendar-time description. |
| 97 | `qs c format date and time` | 2 | #1 `Date and time utilities` | Only the broad date/time category appears; `strftime` and formatting guidance are absent. |
| 98 | `qs c remove a file` | 5 | #1 `remove` | `remove` ranks first through the task card, followed by the exact function and deletion semantics. |
| 99 | `qs c create a temporary file` | 5 | #1 `tmpfile` | `tmpfile` ranks first through the task card, followed by useful implementation detail. |
| 100 | `qs c handle end of file` | 1 | #1 `FILE` | FILE and generic error-related pages appear, but EOF, `feof`, and read-result handling are absent. |
