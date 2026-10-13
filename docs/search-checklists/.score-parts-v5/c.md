# C candidate-v5 quicksearch scores

## Summary

- Executable: `/tmp/qs-gate/release/qs` (not the installed `qs`).
- Commands run: **100/100** with a **30-second timeout per command**.
- Average: **4.72/5**.
- Median: **5/5**.
- Distribution: **0: 0, 1: 1, 2: 2, 3: 4, 4: 10, 5: 83**.
- Pass rate (score >=3): **97/100 (97%)**.
- Strong pass rate (score >=4): **93/100 (93%)**.
- Command errors: **0/100**. Timeouts: **0/100**.

## Comparison to candidate-v3

- Candidate-v3: average **4.33**, median **5**, distribution **0: 4, 1: 5, 2: 2, 3: 6, 4: 9, 5: 74**, score >=3 **89%**, score >=4 **83%**, command errors **0/100**, and timeouts **0/100**.
- Candidate-v5 change: average **+0.39**, median **unchanged**, score >=3 **+8 percentage points**, score >=4 **+10 percentage points**, command errors **unchanged**, and timeouts **unchanged**.
- The largest gains are phrase routing for pointers, structures, promotions, task queries, and EOF handling. Regressions remain for sequence points and variadic macros.

## Results

| # | Command | Score | Observed top result or error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs c arrays` | 4 | #1 `Array initialization`; #2 `Array declaration` | Array initialization leads, but the declaration page is second with useful array-type and lifetime context. |
| 2 | `qs c pointers` | 5 | #1 `Pointer declaration` | `Pointer declaration` ranks first and provides immediately useful documentation for pointer types and indirection. |
| 3 | `qs c pointer arithmetic` | 5 | #1 `Arithmetic operators` | `Arithmetic operators` ranks first and provides immediately useful documentation for pointer arithmetic operators. |
| 4 | `qs c structures` | 5 | #1 `Struct declaration` | `Struct declaration` ranks first and provides immediately useful documentation for `struct` types. |
| 5 | `qs c unions` | 3 | #1 `union` | The exact keyword ranks first, but its result is only a license stub and gives no union-type guidance. |
| 6 | `qs c enumerations` | 5 | #1 `Enumerations` | `Enumerations` ranks first and provides immediately useful documentation for `enum` types and enumerators. |
| 7 | `qs c bit fields` | 5 | #1 `Bit-fields` | `Bit-fields` ranks first and provides immediately useful documentation for structure bit-fields. |
| 8 | `qs c function pointers` | 5 | #1 `Pointer declaration` | `Pointer declaration` ranks first and provides immediately useful documentation for pointer-to-function declarations and calls. |
| 9 | `qs c variadic functions` | 5 | #1 `Variadic functions` | `Variadic functions` ranks first and provides immediately useful documentation for variadic functions and `<stdarg.h>`. |
| 10 | `qs c storage duration` | 4 | #1 `Static storage duration`; #2 `Thread storage duration` | Static and thread storage duration rank first and second with useful context; automatic and allocated duration are absent. |
| 11 | `qs c language linkage` | 4 | #1 `External and tentative definitions`; #2 `Storage-class specifiers` | The first result covers external and internal linkage, and the second directly explains all recognized linkage categories. |
| 12 | `qs c type qualifiers` | 4 | #1 `const type qualifier`; #2 `restrict`; #3 `volatile` | `const`, `restrict`, and `volatile` fill the top three with useful detail; `_Atomic` is absent. |
| 13 | `qs c restrict` | 4 | #1 `restrict`; #2 `restrict type qualifier` | The keyword stub leads, while the detailed qualifier page ranks second with useful rules and code. |
| 14 | `qs c integer promotions` | 5 | #1 `Implicit conversions` | `Implicit conversions` ranks first and provides immediately useful documentation for integer promotion rules. |
| 15 | `qs c usual arithmetic conversions` | 3 | #1 `Arithmetic operators`; #4 `Implicit conversions` | The leading operator page only says that usual arithmetic conversions occur; the page with conversion rules is fourth. |
| 16 | `qs c operator precedence` | 5 | #1 `C Operator Precedence` | `C Operator Precedence` ranks first and provides immediately useful documentation for C operator precedence and associativity. |
| 17 | `qs c compound literals` | 5 | #1 `Compound literals` | `Compound literals` ranks first and provides immediately useful documentation for compound literal expressions. |
| 18 | `qs c designated initializers` | 4 | #1 `Struct and union initialization` | The leading initialization page gives useful designated-initializer context for structs and unions, but array designation is not visible. |
| 19 | `qs c generic selection` | 5 | #1 `Generic selection` | `Generic selection` ranks first and provides immediately useful documentation for `_Generic` generic selection. |
| 20 | `qs c static assertions` | 5 | #1 `Static assertion` | `Static assertion` ranks first and provides immediately useful documentation for `static_assert` / `_Static_assert`. |
| 21 | `qs c flexible array member` | 3 | #1 `Struct declaration`; #2 `Array declaration` | Relevant struct and array pages lead, but the shown snippets do not explain flexible array members. |
| 22 | `qs c variable length arrays` | 5 | #1 `Array declaration` | `Array declaration` ranks first and provides immediately useful documentation for variable-length array types. |
| 23 | `qs c alignment` | 5 | #1 `Objects and alignment` | `Objects and alignment` ranks first and provides immediately useful documentation for object alignment, `alignof`, and `alignas`. |
| 24 | `qs c object lifetime` | 5 | #1 `Lifetime` | `Lifetime` ranks first and provides immediately useful documentation for C object lifetime rules. |
| 25 | `qs c undefined behavior` | 5 | #1 `Undefined behavior` | `Undefined behavior` ranks first and provides immediately useful documentation for undefined behavior in the C abstract machine. |
| 26 | `qs c sequence points` | 2 | #1 `Lifetime` | The leading lifetime result only mentions the old term incidentally; sequencing rules and a focused sequence-point result are absent. |
| 27 | `qs c command line arguments` | 5 | #1 `argc` | `argc` ranks first and provides immediately useful documentation for `main` parameters `argc` and `argv`. |
| 28 | `qs c preprocessing directives` | 5 | #1 `Preprocessor` | `Preprocessor` ranks first and provides immediately useful documentation for C preprocessor directives. |
| 29 | `qs c conditional compilation` | 5 | #1 `Conditional inclusion` | `Conditional inclusion` ranks first and provides immediately useful documentation for `#if`, `#ifdef`, and related directives. |
| 30 | `qs c include guards` | 5 | #1 `Implementation defined behavior control` | `Implementation defined behavior control` ranks first and provides immediately useful documentation for header include-guard pattern. |
| 31 | `qs c variadic macros` | 1 | #1 `Variadic functions`; #2 `Variadic arguments` | Results concern variadic functions and `<stdarg.h>`, not variadic macro definitions or `__VA_ARGS__`. |
| 32 | `qs c escape sequences` | 5 | #1 `Escape sequences` | `Escape sequences` ranks first and provides immediately useful documentation for character and string escape sequences. |
| 33 | `qs c string literals` | 5 | #1 `String literals` | `String literals` ranks first and provides immediately useful documentation for string literal syntax and types. |
| 34 | `qs c implicit conversions` | 5 | #1 `Implicit conversions` | `Implicit conversions` ranks first and provides immediately useful documentation for implicit conversion rules. |
| 35 | `qs c compound assignment operators` | 5 | #1 `Assignment operators` | `Assignment operators` ranks first and provides immediately useful documentation for compound assignment expressions. |
| 36 | `qs c printf` | 5 | #1 `printf` | `printf` ranks first and provides immediately useful documentation for `printf` formatted standard output. |
| 37 | `qs c snprintf` | 5 | #1 `snprintf` | `snprintf` ranks first and provides immediately useful documentation for `snprintf` bounded formatted string output. |
| 38 | `qs c scanf` | 5 | #1 `scanf` | `scanf` ranks first and provides immediately useful documentation for `scanf` formatted standard input. |
| 39 | `qs c fopen` | 5 | #1 `fopen` | `fopen` ranks first and provides immediately useful documentation for `fopen` file-stream opening. |
| 40 | `qs c fclose` | 5 | #1 `fclose` | `fclose` ranks first and provides immediately useful documentation for `fclose` file-stream closing. |
| 41 | `qs c fgets` | 5 | #1 `fgets` | `fgets` ranks first and provides immediately useful documentation for `fgets` line or bounded string input. |
| 42 | `qs c fread` | 5 | #1 `fread` | `fread` ranks first and provides immediately useful documentation for `fread` binary stream input. |
| 43 | `qs c fwrite` | 5 | #1 `fwrite` | `fwrite` ranks first and provides immediately useful documentation for `fwrite` binary stream output. |
| 44 | `qs c fseek` | 5 | #1 `fseek` | `fseek` ranks first and provides immediately useful documentation for `fseek` stream positioning. |
| 45 | `qs c perror` | 5 | #1 `perror` | `perror` ranks first and provides immediately useful documentation for `perror` error-message output. |
| 46 | `qs c malloc` | 5 | #1 `malloc` | `malloc` ranks first and provides immediately useful documentation for `malloc` dynamic allocation. |
| 47 | `qs c calloc` | 5 | #1 `calloc` | `calloc` ranks first and provides immediately useful documentation for `calloc` zero-initialized allocation. |
| 48 | `qs c realloc` | 5 | #1 `realloc` | `realloc` ranks first and provides immediately useful documentation for `realloc` resizing allocated storage. |
| 49 | `qs c free` | 5 | #1 `free` | `free` ranks first and provides immediately useful documentation for `free` deallocation. |
| 50 | `qs c memcpy` | 5 | #1 `memcpy` | `memcpy` ranks first and provides immediately useful documentation for `memcpy` non-overlapping byte copy. |
| 51 | `qs c memmove` | 5 | #1 `memmove` | `memmove` ranks first and provides immediately useful documentation for `memmove` overlap-safe byte copy. |
| 52 | `qs c memset` | 5 | #1 `memset` | `memset` ranks first and provides immediately useful documentation for `memset` byte filling. |
| 53 | `qs c memcmp` | 5 | #1 `memcmp` | `memcmp` ranks first and provides immediately useful documentation for `memcmp` byte-sequence comparison. |
| 54 | `qs c strlen` | 5 | #1 `strlen` | `strlen` ranks first and provides immediately useful documentation for `strlen` string length. |
| 55 | `qs c strcmp` | 5 | #1 `strcmp` | `strcmp` ranks first and provides immediately useful documentation for `strcmp` lexicographic string comparison. |
| 56 | `qs c strchr` | 5 | #1 `strchr` | `strchr` ranks first and provides immediately useful documentation for `strchr` character search in a string. |
| 57 | `qs c strstr` | 5 | #1 `strstr` | `strstr` ranks first and provides immediately useful documentation for `strstr` substring search. |
| 58 | `qs c strtok` | 5 | #1 `strtok` | `strtok` ranks first and provides immediately useful documentation for `strtok` string tokenization. |
| 59 | `qs c qsort` | 5 | #1 `qsort` | `qsort` ranks first and provides immediately useful documentation for `qsort` generic array sorting. |
| 60 | `qs c bsearch` | 5 | #1 `bsearch` | `bsearch` ranks first and provides immediately useful documentation for `bsearch` generic binary search. |
| 61 | `qs c strtol` | 5 | #1 `strtol` | `strtol` ranks first and provides immediately useful documentation for `strtol` checked string-to-integer conversion. |
| 62 | `qs c strtod` | 5 | #1 `strtod` | `strtod` ranks first and provides immediately useful documentation for `strtod` string-to-floating conversion. |
| 63 | `qs c assert` | 5 | #1 `assert` | `assert` ranks first and provides immediately useful documentation for `assert` diagnostic macro. |
| 64 | `qs c setjmp` | 5 | #1 `setjmp` | `setjmp` ranks first and provides immediately useful documentation for `setjmp` nonlocal jump setup. |
| 65 | `qs c signal` | 5 | #1 `signal` | `signal` ranks first and provides immediately useful documentation for `signal` signal-handler installation. |
| 66 | `qs c strftime` | 5 | #1 `strftime` | `strftime` ranks first and provides immediately useful documentation for `strftime` formatted calendar time. |
| 67 | `qs c timespec_get` | 5 | #1 `timespec_get` | `timespec_get` ranks first and provides immediately useful documentation for `timespec_get` wall-clock time acquisition. |
| 68 | `qs c rand` | 5 | #1 `rand` | `rand` ranks first and provides immediately useful documentation for `rand` pseudo-random number generation. |
| 69 | `qs c sqrt` | 5 | #1 `sqrt` | `sqrt` ranks first and provides immediately useful documentation for `sqrt` square-root function. |
| 70 | `qs c isnan` | 5 | #1 `isnan` | `isnan` ranks first and provides immediately useful documentation for `isnan` floating-point classification macro. |
| 71 | `qs c isalpha` | 5 | #1 `isalpha` | `isalpha` ranks first and provides immediately useful documentation for `isalpha` character classification. |
| 72 | `qs c toupper` | 5 | #1 `toupper` | `toupper` ranks first and provides immediately useful documentation for `toupper` character case conversion. |
| 73 | `qs c setlocale` | 5 | #1 `setlocale` | `setlocale` ranks first and provides immediately useful documentation for `setlocale` locale configuration. |
| 74 | `qs c mbrtowc` | 5 | #1 `mbrtowc` | `mbrtowc` ranks first and provides immediately useful documentation for `mbrtowc` multibyte-to-wide conversion. |
| 75 | `qs c thrd_create` | 5 | #1 `thrd_create` | `thrd_create` ranks first and provides immediately useful documentation for `thrd_create` C thread creation. |
| 76 | `qs c mtx_lock` | 5 | #1 `mtx_lock` | `mtx_lock` ranks first and provides immediately useful documentation for `mtx_lock` mutex locking. |
| 77 | `qs c cnd_wait` | 5 | #1 `cnd_wait` | `cnd_wait` ranks first and provides immediately useful documentation for `cnd_wait` condition-variable waiting. |
| 78 | `qs c call_once` | 5 | #1 `call_once` | `call_once` ranks first and provides immediately useful documentation for `call_once` one-time initialization. |
| 79 | `qs c atomic_fetch_add` | 5 | #1 `atomic_fetch_add` | `atomic_fetch_add` ranks first and provides immediately useful documentation for `atomic_fetch_add` atomic read-modify-write. |
| 80 | `qs c atomic_compare_exchange_strong` | 5 | #1 `atomic_compare_exchange_strong` | `atomic_compare_exchange_strong` ranks first and provides immediately useful documentation for strong atomic compare-and-exchange. |
| 81 | `qs c memory_order_acquire` | 5 | #1 `memory_order` | `memory_order` ranks first and provides immediately useful documentation for acquire atomic memory ordering. |
| 82 | `qs c size_t` | 5 | #1 `size_t` | `size_t` ranks first and provides immediately useful documentation for `size_t` unsigned size type. |
| 83 | `qs c ptrdiff_t` | 5 | #1 `ptrdiff_t` | `ptrdiff_t` ranks first and provides immediately useful documentation for `ptrdiff_t` pointer-difference type. |
| 84 | `qs c va_list` | 5 | #1 `va_list` | `va_list` ranks first and provides immediately useful documentation for `va_list` variadic-argument state type. |
| 85 | `qs c struct tm` | 4 | #1 `<time.h>`; #3 `tm` | The time header leads and the exact `tm` structure ranks third, so the expected type is easy to reach but not first. |
| 86 | `qs c FILE` | 5 | #1 `FILE` | `FILE` ranks first and provides immediately useful documentation for `FILE` stream object type. |
| 87 | `qs c NULL` | 5 | #1 `NULL` | `NULL` ranks first and provides immediately useful documentation for null pointer macro `NULL`. |
| 88 | `qs c errno` | 5 | #1 `errno` | `errno` ranks first and provides immediately useful documentation for `errno` error indicator and error macros. |
| 89 | `qs c stdatomic.h` | 5 | #1 `Standard library header <stdatomic.h>` | `Standard library header <stdatomic.h>` ranks first and provides immediately useful documentation for the `<stdatomic.h>` atomic operations header. |
| 90 | `qs c complex numbers` | 5 | #1 `Complex number arithmetic` | `Complex number arithmetic` ranks first and provides immediately useful documentation for C complex arithmetic and `<complex.h>`. |
| 91 | `qs c read a file line by line` | 4 | #1 `fread`; #3 `fgets` | Two `fread` results lead, but `fgets` is third and provides the useful line-input API. |
| 92 | `qs c copy overlapping memory` | 4 | #1 `memcpy`; #2 `memmove` | `memcpy` misleadingly leads for overlapping storage, but the correct `memmove` result is second. |
| 93 | `qs c allocate zero initialized array` | 5 | #1 `calloc` | `calloc` ranks first and provides immediately useful documentation for zeroed allocation with `calloc`. |
| 94 | `qs c parse integer from string` | 5 | #1 `strtol` | `strtol` ranks first and provides immediately useful documentation for numeric parsing with `strtol`. |
| 95 | `qs c sort array with comparator` | 3 | #1 `bsearch`; #4 `qsort` | `bsearch` leads and `qsort` is only fourth, so the correct array-sort API exists with weak ranking. |
| 96 | `qs c get current calendar time` | 5 | #1 `timespec_get` | `timespec_get` ranks first and provides immediately useful documentation for `time` and calendar-time conversion. |
| 97 | `qs c format date and time` | 2 | #1 `Date and time utilities` | Only a broad date-and-time category is returned; `strftime` and formatting detail are absent. |
| 98 | `qs c remove a file` | 5 | #1 `remove` | `remove` ranks first and provides immediately useful documentation for `remove` file deletion. |
| 99 | `qs c create a temporary file` | 4 | #1 `tmpnam`; #2 `tmpfile` | `tmpnam` leads even though it only creates a name; the correct `tmpfile` API ranks second. |
| 100 | `qs c handle end of file` | 5 | #1 `feof` | `feof` ranks first and provides immediately useful documentation for EOF indication, `feof`, and stream read results. |
