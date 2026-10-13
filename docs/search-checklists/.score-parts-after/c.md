# C candidate quicksearch scores

## Summary

- Executable: `/tmp/qs-candidate/release/qs` (not the installed `qs`).
- Commands run: **100/100** with a **20-second timeout per command**.
- Average: **3.93/5**.
- Median: **5/5**.
- Distribution: **0: 17, 1: 1, 2: 2, 3: 3, 4: 6, 5: 71**.
- Pass rate (score >=3): **80/100 (80%)**.
- Strong pass rate (score >=4): **77/100 (77%)**.
- Command errors: **17/100**. Timeouts: **0/100**. All errors were `no web results` failures.

## Baseline comparison

- Baseline: average **4.35**, median **5**, score >=3 **88%**, score >=4 **85%**, and command errors **1/100**.
- Candidate change: average **-0.42**, median **unchanged**, score >=3 **-8 percentage points**, score >=4 **-8 percentage points**, and errors **+16**.
- Baseline distribution was **0: 5, 1: 6, 2: 1, 3: 3, 4: 7, 5: 78**. The candidate has fewer weak nonzero results but many more hard failures.

## Results

| # | Command | Score | Observed top result or error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs c arrays` | 5 | #1 `Array declaration` | Exact array declaration page is first with useful type and example context. |
| 2 | `qs c pointers` | 5 | #1 `Pointer declaration` | Exact pointer declaration page is first with useful definition and examples. |
| 3 | `qs c pointer arithmetic` | 0 | Error: `qs: no web results for c pointer arithmetic: https://lite.duckduckgo.com/lite/?q=c%20pointer%20arithmetic` | The command failed, so pointer-arithmetic guidance is unavailable. |
| 4 | `qs c structures` | 1 | #1 `String literals` | Only string-literal and string-header pages appear; struct documentation is absent. |
| 5 | `qs c unions` | 3 | #1 `union` | The exact keyword appears first, but its snippet is only license text. |
| 6 | `qs c enumerations` | 5 | #1 `Enumerations` | Exact enumeration page is first with a clear definition. |
| 7 | `qs c bit fields` | 5 | #1 `Bit-fields` | Exact bit-fields page is first with packing details and an example. |
| 8 | `qs c function pointers` | 4 | Web card: `Function pointer` | The first web card explains function pointers and indirect calls, but gives no C syntax. |
| 9 | `qs c variadic functions` | 5 | #1 `Variadic functions` | Exact variadic-functions page is first and includes `<stdarg.h>` use. |
| 10 | `qs c storage duration` | 4 | #1 `Static storage duration` | Static and thread duration rank first, but automatic and allocated duration are missing. |
| 11 | `qs c language linkage` | 0 | Error: `qs: no web results for c language linkage: https://lite.duckduckgo.com/lite/?q=c%20language%20linkage` | The command failed, so C linkage rules are unavailable. |
| 12 | `qs c type qualifiers` | 4 | #1 `const type qualifier` | `const`, `restrict`, and `volatile` fill the top three; `_Atomic` is missing. |
| 13 | `qs c restrict` | 4 | #1 `restrict` | The keyword stub leads, but the second result gives useful qualifier guidance and code. |
| 14 | `qs c integer promotions` | 0 | Error: `qs: no web results for c integer promotions: https://lite.duckduckgo.com/lite/?q=c%20integer%20promotions` | The command failed, so integer-promotion rules are unavailable. |
| 15 | `qs c usual arithmetic conversions` | 0 | Error: `qs: no web results for c usual arithmetic conversions: https://lite.duckduckgo.com/lite/?q=c%20usual%20arithmetic%20conversions` | The command failed, so usual arithmetic conversions are unavailable. |
| 16 | `qs c operator precedence` | 5 | #1 `C Operator Precedence` | The exact precedence table ranks first with associativity context. |
| 17 | `qs c compound literals` | 5 | #1 `Compound literals` | Exact compound-literals documentation ranks first with a clear definition. |
| 18 | `qs c designated initializers` | 0 | Error: `qs: no web results for c designated initializers: https://lite.duckduckgo.com/lite/?q=c%20designated%20initializers` | The command failed, so designated-initializer guidance is unavailable. |
| 19 | `qs c generic selection` | 5 | #1 `Generic selection` | Exact generic-selection documentation ranks first with useful context. |
| 20 | `qs c static assertions` | 5 | #1 `Static assertion` | Exact static-assertion documentation ranks first and covers both spellings. |
| 21 | `qs c flexible array member` | 0 | Error: `qs: no web results for c flexible array member: https://lite.duckduckgo.com/lite/?q=c%20flexible%20array%20member` | The command failed, so flexible-array-member guidance is unavailable. |
| 22 | `qs c variable length arrays` | 0 | Error: `qs: no web results for c variable length arrays: https://lite.duckduckgo.com/lite/?q=c%20variable%20length%20arrays` | The command failed, so variable-length-array guidance is unavailable. |
| 23 | `qs c alignment` | 5 | #1 `Objects and alignment` | Objects and alignment ranks first; examples show both `alignof` and `alignas`. |
| 24 | `qs c object lifetime` | 2 | Web card: `Object lifetime` | The web card discusses object-oriented lifetime, not C object-lifetime rules. |
| 25 | `qs c undefined behavior` | 5 | #1 `Undefined behavior` | Exact undefined-behavior documentation ranks first with abstract-machine context. |
| 26 | `qs c sequence points` | 5 | Web card: `Sequence point` | The first web card directly defines C/C++ sequence points and newer terminology. |
| 27 | `qs c command line arguments` | 3 | #1 `argc` | `argc` links to the correct `main` page, but the snippet gives no `argv` detail. |
| 28 | `qs c preprocessing directives` | 0 | Error: `qs: no web results for c preprocessing directives: https://lite.duckduckgo.com/lite/?q=c%20preprocessing%20directives` | The command failed, so preprocessing-directive documentation is unavailable. |
| 29 | `qs c conditional compilation` | 3 | Web card: `Conditional compilation` | The web card is relevant, but it lacks concrete `#if` and `#ifdef` guidance. |
| 30 | `qs c include guards` | 0 | Error: `qs: no web results for c include guards: https://lite.duckduckgo.com/lite/?q=c%20include%20guards` | The command failed, so no include-guard guidance is returned. |
| 31 | `qs c variadic macros` | 4 | Web card: `Variadic macro` | The web card accurately defines C variadic macros, but omits `__VA_ARGS__` syntax. |
| 32 | `qs c escape sequences` | 5 | #1 `Escape sequences` | Exact escape-sequences documentation ranks first with clear context. |
| 33 | `qs c string literals` | 5 | #1 `String literals` | Exact string-literals documentation ranks first with syntax and example context. |
| 34 | `qs c implicit conversions` | 5 | #1 `Implicit conversions` | Exact implicit-conversions documentation ranks first with a clear definition. |
| 35 | `qs c compound assignment operators` | 0 | Error: `qs: no web results for c compound assignment operators: https://lite.duckduckgo.com/lite/?q=c%20compound%20assignment%20operators` | The command failed, so compound-assignment guidance is unavailable. |
| 36 | `qs c printf` | 5 | #1 `printf` | Exact `printf` reference ranks first with useful documentation context. |
| 37 | `qs c snprintf` | 5 | #1 `snprintf` | Exact `snprintf` reference ranks first with useful documentation context. |
| 38 | `qs c scanf` | 5 | #1 `scanf` | Exact `scanf` reference ranks first with useful documentation context. |
| 39 | `qs c fopen` | 5 | #1 `fopen` | Exact `fopen` reference ranks first with useful documentation context. |
| 40 | `qs c fclose` | 5 | #1 `fclose` | Exact `fclose` reference ranks first with useful documentation context. |
| 41 | `qs c fgets` | 5 | #1 `fgets` | Exact `fgets` reference ranks first with useful documentation context. |
| 42 | `qs c fread` | 5 | #1 `fread` | Exact `fread` reference ranks first with useful documentation context. |
| 43 | `qs c fwrite` | 5 | #1 `fwrite` | Exact `fwrite` reference ranks first with useful documentation context. |
| 44 | `qs c fseek` | 5 | #1 `fseek` | Exact `fseek` reference ranks first with useful documentation context. |
| 45 | `qs c perror` | 5 | #1 `perror` | Exact `perror` reference ranks first with useful documentation context. |
| 46 | `qs c malloc` | 5 | #1 `malloc` | Exact `malloc` reference ranks first with useful documentation context. |
| 47 | `qs c calloc` | 5 | #1 `calloc` | Exact `calloc` reference ranks first with useful documentation context. |
| 48 | `qs c realloc` | 5 | #1 `realloc` | Exact `realloc` reference ranks first with useful documentation context. |
| 49 | `qs c free` | 5 | #1 `free` | Exact `free` reference ranks first with useful documentation context. |
| 50 | `qs c memcpy` | 5 | #1 `memcpy` | Exact `memcpy` reference ranks first with useful documentation context. |
| 51 | `qs c memmove` | 5 | #1 `memmove` | Exact `memmove` reference ranks first with useful documentation context. |
| 52 | `qs c memset` | 5 | #1 `memset` | Exact `memset` reference ranks first with useful documentation context. |
| 53 | `qs c memcmp` | 5 | #1 `memcmp` | Exact `memcmp` reference ranks first with useful documentation context. |
| 54 | `qs c strlen` | 5 | #1 `strlen` | Exact `strlen` reference ranks first with useful documentation context. |
| 55 | `qs c strcmp` | 5 | #1 `strcmp` | Exact `strcmp` reference ranks first with useful documentation context. |
| 56 | `qs c strchr` | 5 | #1 `strchr` | Exact `strchr` reference ranks first with useful documentation context. |
| 57 | `qs c strstr` | 5 | #1 `strstr` | Exact `strstr` reference ranks first with useful documentation context. |
| 58 | `qs c strtok` | 5 | #1 `strtok` | Exact `strtok` reference ranks first with useful documentation context. |
| 59 | `qs c qsort` | 5 | #1 `qsort` | Exact `qsort` reference ranks first with useful documentation context. |
| 60 | `qs c bsearch` | 5 | #1 `bsearch` | Exact `bsearch` reference ranks first with useful documentation context. |
| 61 | `qs c strtol` | 5 | #1 `strtol` | Exact `strtol` reference ranks first with useful documentation context. |
| 62 | `qs c strtod` | 5 | #1 `strtod` | Exact `strtod` reference ranks first with useful documentation context. |
| 63 | `qs c assert` | 5 | #1 `assert` | Exact `assert` reference ranks first with useful documentation context. |
| 64 | `qs c setjmp` | 5 | #1 `setjmp` | Exact `setjmp` reference ranks first with useful documentation context. |
| 65 | `qs c signal` | 5 | #1 `signal` | Exact `signal` reference ranks first with useful documentation context. |
| 66 | `qs c strftime` | 5 | #1 `strftime` | Exact `strftime` reference ranks first with useful documentation context. |
| 67 | `qs c timespec_get` | 5 | #1 `timespec_get` | Exact `timespec_get` reference ranks first with useful documentation context. |
| 68 | `qs c rand` | 5 | #1 `rand` | Exact `rand` reference ranks first with useful documentation context. |
| 69 | `qs c sqrt` | 5 | #1 `sqrt` | Exact `sqrt` reference ranks first with useful documentation context. |
| 70 | `qs c isnan` | 5 | #1 `isnan` | Exact `isnan` reference ranks first with useful documentation context. |
| 71 | `qs c isalpha` | 5 | #1 `isalpha` | Exact `isalpha` reference ranks first with useful documentation context. |
| 72 | `qs c toupper` | 5 | #1 `toupper` | Exact `toupper` reference ranks first with useful documentation context. |
| 73 | `qs c setlocale` | 5 | #1 `setlocale` | Exact `setlocale` reference ranks first with useful documentation context. |
| 74 | `qs c mbrtowc` | 5 | #1 `mbrtowc` | Exact `mbrtowc` reference ranks first with useful documentation context. |
| 75 | `qs c thrd_create` | 5 | #1 `thrd_create` | Exact `thrd_create` reference ranks first with useful documentation context. |
| 76 | `qs c mtx_lock` | 5 | #1 `mtx_lock` | Exact `mtx_lock` reference ranks first with useful documentation context. |
| 77 | `qs c cnd_wait` | 5 | #1 `cnd_wait` | Exact `cnd_wait` reference ranks first with useful documentation context. |
| 78 | `qs c call_once` | 5 | #1 `call_once` | Exact `call_once` reference ranks first with useful documentation context. |
| 79 | `qs c atomic_fetch_add` | 5 | #1 `atomic_fetch_add` | Exact `atomic_fetch_add` reference ranks first with useful documentation context. |
| 80 | `qs c atomic_compare_exchange_strong` | 5 | #1 `atomic_compare_exchange_strong` | Exact `atomic_compare_exchange_strong` reference ranks first with useful documentation context. |
| 81 | `qs c memory_order_acquire` | 0 | Error: `qs: no web results for c memory_order_acquire: https://lite.duckduckgo.com/lite/?q=c%20memory_order_acquire` | The command failed, so acquire-order documentation is unavailable. |
| 82 | `qs c size_t` | 5 | #1 `size_t` | Exact `size_t` reference ranks first with useful documentation context. |
| 83 | `qs c ptrdiff_t` | 5 | #1 `ptrdiff_t` | Exact `ptrdiff_t` reference ranks first with useful documentation context. |
| 84 | `qs c va_list` | 5 | #1 `va_list` | Exact `va_list` reference ranks first with useful documentation context. |
| 85 | `qs c struct tm` | 0 | Error: `qs: no web results for c struct tm: https://lite.duckduckgo.com/lite/?q=c%20struct%20tm` | The command failed, so `struct tm` documentation is unavailable. |
| 86 | `qs c FILE` | 5 | #1 `FILE` | Exact `FILE` reference ranks first with useful documentation context. |
| 87 | `qs c NULL` | 5 | #1 `NULL` | Exact `NULL` reference ranks first with useful documentation context. |
| 88 | `qs c errno` | 5 | #1 `errno` | Exact `errno` reference ranks first with useful documentation context. |
| 89 | `qs c stdatomic.h` | 5 | #1 `Standard library header <stdatomic.h>` | Exact `Standard library header <stdatomic.h>` reference ranks first with useful documentation context. |
| 90 | `qs c complex numbers` | 5 | #1 `Complex number arithmetic` | Complex arithmetic ranks first and directly mentions complex types and `<complex.h>`. |
| 91 | `qs c read a file line by line` | 4 | #1 `fread` | `fread` is incorrectly first, but `fgets` and a line-reading loop appear third. |
| 92 | `qs c copy overlapping memory` | 0 | Error: `qs: no web results for c copy overlapping memory: https://lite.duckduckgo.com/lite/?q=c%20copy%20overlapping%20memory` | The command failed, so `memmove` guidance is unavailable. |
| 93 | `qs c allocate zero initialized array` | 0 | Error: `qs: no web results for c allocate zero initialized array: https://lite.duckduckgo.com/lite/?q=c%20allocate%20zero%20initialized%20array` | The command failed, so `calloc` guidance is unavailable. |
| 94 | `qs c parse integer from string` | 5 | #1 `strtol` | `strtol` ranks first, followed by detailed parsing and error-handling guidance. |
| 95 | `qs c sort array with comparator` | 0 | Error: `qs: no web results for c sort array with comparator: https://lite.duckduckgo.com/lite/?q=c%20sort%20array%20with%20comparator` | The command failed, so `qsort` and comparator guidance are unavailable. |
| 96 | `qs c get current calendar time` | 0 | Error: `qs: no web results for c get current calendar time: https://lite.duckduckgo.com/lite/?q=c%20get%20current%20calendar%20time` | The command failed, so current calendar-time guidance is unavailable. |
| 97 | `qs c format date and time` | 2 | #1 `Date and time utilities` | Only the date/time category appears; `strftime` and formatting instructions are absent. |
| 98 | `qs c remove a file` | 5 | #1 `remove` | `remove` ranks first, followed by its signature and a complete example. |
| 99 | `qs c create a temporary file` | 5 | #1 `tmpfile` | `tmpfile` ranks first, followed by useful behavior notes and an example. |
| 100 | `qs c handle end of file` | 0 | Error: `qs: no web results for c handle end of file: https://lite.duckduckgo.com/lite/?q=c%20handle%20end%20of%20file` | The command failed, so EOF and `feof` guidance is unavailable. |
