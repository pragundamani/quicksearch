# C quicksearch score report

## Summary

I ran all 100 checklist commands through the installed `qs` executable. Each invocation had a 25-second timeout so one network problem could not stop the run. I scored only the observed output, not the checklist's expected field.

- **Average:** 4.35 / 5
- **Median:** 5
- **Distribution:** 0: 5, 1: 6, 2: 1, 3: 3, 4: 7, 5: 78
- **Pass rate (>=3):** 88 / 100 (88%)
- **Strong pass rate (>=4):** 85 / 100 (85%)
- **Command errors:** 1 / 100 (`qs c include guards`)

## All results

| # | Command | Score | Observed top result or error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs c arrays` | 5 | #1 `Array declaration` | Exact array declaration page is first; definition and runnable VLA/subscript example are immediately useful. |
| 2 | `qs c pointers` | 5 | #1 `Pointer declaration` | Exact pointer declaration page is first; the definition and function-pointer example provide strong context. |
| 3 | `qs c pointer arithmetic` | 3 | #1 `Arithmetic types` | `Arithmetic types` is first and `Arithmetic operators` second, but neither visible snippet explains pointer arithmetic. |
| 4 | `qs c structures` | 5 | #1 `C Structures (structs) - W3Schools` | The first web result directly teaches C structs and lists declaration, access, and examples. |
| 5 | `qs c unions` | 4 | #1 `union` | A thin `union` keyword stub is first; the useful `Union declaration` page is second with a clear definition and example. |
| 6 | `qs c enumerations` | 5 | #1 `Enumerations` | Exact enum language page is first with a definition and code example. |
| 7 | `qs c bit fields` | 5 | #1 `Bit-fields` | Exact `Bit-fields` page is first and clearly describes explicit widths and packing. |
| 8 | `qs c function pointers` | 5 | #1 `Pointer declaration` | Pointer declaration is first, and its displayed example is exactly a function-pointer declaration and call. |
| 9 | `qs c variadic functions` | 5 | #1 `Variadic functions` | Exact variadic-functions page is first; it identifies the feature and the output includes `<stdarg.h>` usage. |
| 10 | `qs c storage duration` | 4 | #1 `Static storage duration` | Static and thread duration pages rank first and second, but automatic and allocated duration are not surfaced. |
| 11 | `qs c language linkage` | 4 | #1 `Language linkage - cppreference.com` | The first hit is the C++ language-linkage page; the second hit correctly covers C internal, external, and no linkage. |
| 12 | `qs c type qualifiers` | 4 | #1 `const type qualifier` | `const`, `restrict`, and `volatile` occupy the top three; `_Atomic` is not surfaced. |
| 13 | `qs c restrict` | 3 | #1 `restrict` | The first result is an exact `restrict` keyword page, but its snippet is only a license notice and gives no qualifier guidance. |
| 14 | `qs c integer promotions` | 5 | #1 `Integer Promotions in C - GeeksforGeeks` | The first hit directly explains C integer promotions with a concrete arithmetic example. |
| 15 | `qs c usual arithmetic conversions` | 5 | #1 `Implicit conversions` | The first result is C implicit conversions, the correct home for the usual arithmetic conversions. |
| 16 | `qs c operator precedence` | 5 | #1 `C Operator Precedence` | The exact operator-precedence table is first and explicitly promises precedence and associativity. |
| 17 | `qs c compound literals` | 5 | #1 `Compound literals` | Exact compound-literals page is first with a precise definition. |
| 18 | `qs c designated initializers` | 4 | #1 `Designated Initializers in C - GeeksforGeeks` | The first snippet is generic site boilerplate; the second and third results give useful C99/GCC explanations. |
| 19 | `qs c generic selection` | 5 | #1 `Generic selection` | Exact generic-selection page is first and clearly explains compile-time type-based selection. |
| 20 | `qs c static assertions` | 5 | #1 `Static assertion` | Exact static-assertion page is first and shows both `static_assert` and `_Static_assert`. |
| 21 | `qs c flexible array member` | 1 | #1 `Member access operators` | Only `Member access operators` appears; the snippet and example do not mention flexible array members. |
| 22 | `qs c variable length arrays` | 5 | #1 `Array declaration` | Array declaration is first and is the correct page for variable-length arrays, with a VLA example. |
| 23 | `qs c alignment` | 5 | #1 `Objects and alignment` | Objects and alignment is first with `alignof` examples; `alignas` also appears in the next result's example. |
| 24 | `qs c object lifetime` | 5 | #1 `Lifetime` | Exact lifetime page is first and directly states C object-lifetime rules. |
| 25 | `qs c undefined behavior` | 5 | #1 `Undefined behavior` | Exact undefined-behavior page is first and gives the right abstract-machine context. |
| 26 | `qs c sequence points` | 0 | #1 `Floating-point environment` | The sole result is `Floating-point environment`, which is unrelated to sequence points or sequencing. |
| 27 | `qs c command line arguments` | 0 | #1 `Variadic arguments` | The sole result is `Variadic arguments`; it does not show `main`, `argc`, or `argv`. |
| 28 | `qs c preprocessing directives` | 4 | #1 `Diagnostic directives` | Several directive pages are returned; diagnostic is first and conditional compilation is second, but no category overview leads. |
| 29 | `qs c conditional compilation` | 3 | Knowledge card: `Conditional compilation` (no numbered results) | A knowledge-card explanation is relevant, but there are no result rows or concrete `#if`/`#ifdef` examples. |
| 30 | `qs c include guards` | 0 | Error: `qs: no web results for c include guards` | Command fails with `no web results for c include guards`; no usable result is returned. |
| 31 | `qs c variadic macros` | 5 | #1 `Variadic Macros (The C Preprocessor)` | The first GCC page is exact; the second snippet includes a practical `__VA_ARGS__` definition. |
| 32 | `qs c escape sequences` | 5 | #1 `Escape sequences` | Exact escape-sequences page is first and clearly describes string and character use. |
| 33 | `qs c string literals` | 5 | #1 `String literals` | Exact string-literals page is first with syntax/type context and an example. |
| 34 | `qs c implicit conversions` | 5 | #1 `Implicit conversions` | Exact implicit-conversions page is first and clearly defines the operation. |
| 35 | `qs c compound assignment operators` | 5 | #1 `Assignment operators` | Assignment operators is first and explicitly covers compound assignment, with code. |
| 36 | `qs c printf` | 5 | #1 `printf` | Exact `printf` reference is first with format-string details and examples. |
| 37 | `qs c snprintf` | 5 | #1 `snprintf` | Exact `snprintf` reference is first and distinguishes the bounded API. |
| 38 | `qs c scanf` | 5 | #1 `scanf` | Exact `scanf` reference is first with format-specifier context. |
| 39 | `qs c fopen` | 5 | #1 `fopen` | Exact `fopen` page is first with a complete open/read/error/close example, though the leading snippet starts at output text. |
| 40 | `qs c fclose` | 5 | #1 `fclose` | Exact `fclose` page is first and begins with the signature and behavior. |
| 41 | `qs c fgets` | 5 | #1 `fgets` | Exact `fgets` page is first with return semantics and a bounded line-reading loop. |
| 42 | `qs c fread` | 5 | #1 `fread` | Exact `fread` page is first with a complete binary-read example and error handling. |
| 43 | `qs c fwrite` | 5 | #1 `fwrite` | Exact `fwrite` page is first with a complete binary-write example. |
| 44 | `qs c fseek` | 5 | #1 `fseek` | Exact `fseek` page is first and displays the signature and origin macros. |
| 45 | `qs c perror` | 5 | #1 `perror` | Exact `perror` page is first with signature and purpose. |
| 46 | `qs c malloc` | 5 | #1 `malloc` | Exact `malloc` page is first with signature and allocation behavior. |
| 47 | `qs c calloc` | 5 | #1 `calloc` | Exact `calloc` page is first with signature and zero-initialized array semantics. |
| 48 | `qs c realloc` | 5 | #1 `realloc` | Exact `realloc` page is first with signature and resize behavior. |
| 49 | `qs c free` | 5 | #1 `free` | Exact `free` page is first with null handling and a useful allocation/deallocation example. |
| 50 | `qs c memcpy` | 5 | #1 `memcpy` | Exact `memcpy` page is first and explains efficient memory-to-memory copying. |
| 51 | `qs c memmove` | 5 | #1 `memmove` | Exact `memmove` page is first and discusses its temporary-buffer overlap semantics. |
| 52 | `qs c memset` | 5 | #1 `memset` | Exact `memset` page is first; the snippet adds a useful note about secure variants. |
| 53 | `qs c memcmp` | 5 | #1 `memcmp` | Exact `memcmp` page is first with the full comparison signature and purpose. |
| 54 | `qs c strlen` | 5 | #1 `strlen` | Exact `strlen` page is first with a clear example distinguishing the terminator. |
| 55 | `qs c strcmp` | 5 | #1 `strcmp` | Exact `strcmp` page is first with signature and lexicographic behavior. |
| 56 | `qs c strchr` | 5 | #1 `strchr` | Exact `strchr` page is first with a clear found/not-found return description. |
| 57 | `qs c strstr` | 5 | #1 `strstr` | Exact `strstr` page is first with a clear substring-result description. |
| 58 | `qs c strtok` | 5 | #1 `strtok` | Exact `strtok` page is first and warns that tokenization mutates the input. |
| 59 | `qs c qsort` | 5 | #1 `qsort` | Exact `qsort` page is first with comparator signature guidance and a safe full example. |
| 60 | `qs c bsearch` | 5 | #1 `bsearch` | Exact `bsearch` page is first with a complete comparator and lookup example. |
| 61 | `qs c strtol` | 5 | #1 `strtol` | Exact `strtol` page is first and explains parsing behavior. |
| 62 | `qs c strtod` | 5 | #1 `strtod` | Exact `strtod` page is first and explains whitespace and conversion behavior. |
| 63 | `qs c assert` | 5 | #1 `assert` | Exact `assert` page is first with macro expansion, `NDEBUG`, and example use. |
| 64 | `qs c setjmp` | 5 | #1 `setjmp` | Exact `setjmp` page is first and clearly states that it saves execution context. |
| 65 | `qs c signal` | 5 | #1 `signal` | Exact `signal` page is first with handler-installation semantics and an example. |
| 66 | `qs c strftime` | 5 | #1 `strftime` | Exact `strftime` page is first and describes conversion specifiers. |
| 67 | `qs c timespec_get` | 5 | #1 `timespec_get` | Exact `timespec_get` page is first with return semantics and a current-time example. |
| 68 | `qs c rand` | 5 | #1 `rand` | Exact `rand` page is first with seeding, range, and unbiased die-roll examples. |
| 69 | `qs c sqrt` | 5 | #1 `sqrt` | Exact `sqrt` page is first with domain-error guidance. |
| 70 | `qs c isnan` | 5 | #1 `isnan` | Exact `isnan` page is first and starts with the macro form and classification purpose. |
| 71 | `qs c isalpha` | 5 | #1 `isalpha` | Exact `isalpha` page is first with locale-aware classification context. |
| 72 | `qs c toupper` | 5 | #1 `toupper` | Exact `toupper` page is first with signature and conversion behavior. |
| 73 | `qs c setlocale` | 5 | #1 `setlocale` | Exact `setlocale` page is first with signature and locale-installation purpose. |
| 74 | `qs c mbrtowc` | 5 | #1 `mbrtowc` | Exact `mbrtowc` page is first with detailed semantics and a full UTF-8 conversion example. |
| 75 | `qs c thrd_create` | 5 | #1 `thrd_create` | Exact `thrd_create` page is first and states its successful result; detail is sparse but relevant. |
| 76 | `qs c mtx_lock` | 5 | #1 `mtx_lock` | Exact `mtx_lock` page is first and gives important recursive-lock behavior. |
| 77 | `qs c cnd_wait` | 5 | #1 `cnd_wait` | Exact `cnd_wait` page is first and gives the required mutex precondition. |
| 78 | `qs c call_once` | 5 | #1 `call_once` | Exact `call_once` page is first and usefully points to the POSIX equivalent. |
| 79 | `qs c atomic_fetch_add` | 5 | #1 `atomic_fetch_add` | Exact `atomic_fetch_add` page is first with type rules and a complete threaded example. |
| 80 | `qs c atomic_compare_exchange_strong` | 5 | #1 `atomic_compare_exchange_strong` | Exact strong compare-exchange page is first and exposes success/failure memory-model details. |
| 81 | `qs c memory_order_acquire` | 4 | #1 `std::memory_order - cppreference.com` | The first result is C++ `std::memory_order`; the correct C `memory_order` page is second and useful. |
| 82 | `qs c size_t` | 5 | #1 `size_t` | Exact `size_t` page is first with typedef and `sizeof` semantics. |
| 83 | `qs c ptrdiff_t` | 5 | #1 `ptrdiff_t` | Exact `ptrdiff_t` page is first with typedef and pointer-difference semantics. |
| 84 | `qs c va_list` | 5 | #1 `va_list` | Exact `va_list` page is first and clearly describes the variadic state object. |
| 85 | `qs c struct tm` | 5 | #1 `tm` | Exact `tm` structure page is first and identifies broken-down calendar components. |
| 86 | `qs c FILE` | 5 | #1 `FILE` | Exact `FILE` page is first and defines the stream object. |
| 87 | `qs c NULL` | 5 | #1 `NULL` | Exact `NULL` page is first and defines the implementation-defined null pointer macro. |
| 88 | `qs c errno` | 5 | #1 `errno` | Exact `errno` page is first and explains its macro form; related math errors are second. |
| 89 | `qs c stdatomic.h` | 5 | #1 `Standard library header <stdatomic.h>` | Exact `<stdatomic.h>` header page is first and identifies atomic/concurrency support. |
| 90 | `qs c complex numbers` | 5 | #1 `Complex number arithmetic` | Complex-number arithmetic is first and explicitly mentions complex types and `<complex.h>`. |
| 91 | `qs c read a file line by line` | 0 | #1 `#line directive` | The first and only hit is the preprocessor `#line` directive, not line-by-line file input. |
| 92 | `qs c copy overlapping memory` | 0 | #1 `Memory model` | Results are `Memory model` and atomic `memory_order`; `memmove` is absent. |
| 93 | `qs c allocate zero initialized array` | 1 | #1 `Array declaration` | Array declaration and initialization are returned, but no dynamic allocation or `calloc` appears. |
| 94 | `qs c parse integer from string` | 1 | #1 `String literals` | String literals and the strings-library index appear, but `strtol` and numeric parsing are absent. |
| 95 | `qs c sort array with comparator` | 5 | #1 `C Program to Sort an Array in Ascending Order - GeeksforGeeks` | The first hit describes sorting a C array with `qsort` and a comparator; the exact `qsort` article is second. |
| 96 | `qs c get current calendar time` | 5 | #1 `time` | Exact `time` page is first with `time_t`, error handling, and calendar conversion example. |
| 97 | `qs c format date and time` | 2 | #1 `Date and time utilities` | A date/time category and `time` rank first and second, but `strftime` and formatting instructions are absent. |
| 98 | `qs c remove a file` | 1 | #1 `FILE` | Only `FILE` and file scope appear; the `remove` function is absent. |
| 99 | `qs c create a temporary file` | 1 | #1 `FILE` | Only `FILE` and file scope appear; `tmpfile` is absent. |
| 100 | `qs c handle end of file` | 1 | #1 `FILE` | Only the `FILE` type and file scope appear; `EOF`, `feof`, and read-result handling are absent. |

## Strongest patterns

- **Exact standard-library identifiers are excellent.** Queries from `printf` through the memory, string, numeric, thread, and atomic APIs almost always put the exact C reference first.
- **Exact language feature names work well.** Arrays, pointers, enums, bit-fields, generic selection, assertions, conversions, precedence, literals, alignment, lifetime, and undefined behavior lead to authoritative C pages.
- **API results often include actionable evidence.** The strongest outputs show signatures, return behavior, caveats, and runnable examples. `fread`, `qsort`, `bsearch`, `timespec_get`, and `mbrtowc` are especially useful.
- **Morphology is usually handled well.** Plurals such as `arrays`, `pointers`, `enumerations`, and `static assertions` map to singular reference concepts.
- **Web fallback can rescue missing index matches.** `structures`, `integer promotions`, `variadic macros`, and `sort array with comparator` return practical web results.

## Weakest patterns

- **Natural-language task queries often collapse onto one generic noun.** File tasks 98–100 all return `FILE`/file scope rather than `remove`, `tmpfile`, or EOF handling. Similar failures affect `calloc`, `strtol`, `memmove`, and `fgets` intent queries.
- **Important multiword concepts lose a discriminating token.** `command line arguments` becomes variadic arguments; `read a file line by line` becomes `#line`; `copy overlapping memory` becomes generic memory pages.
- **Some language concepts have bad or incomplete index coverage.** `sequence points` returns only `Floating-point environment`; `flexible array member` returns member-access operators.
- **Fallback behavior is inconsistent.** `conditional compilation` returns only a knowledge card, while `include guards` fails with `no web results`.
- **C/C++ source separation is imperfect.** `language linkage` and `memory_order_acquire` rank C++ references above the correct C result.

## Prioritized improvements

1. **Add intent-to-API aliases for common tasks.** Map `read ... line by line` to `fgets`, `copy overlapping memory` to `memmove`, `allocate zero initialized` to `calloc`, `parse integer` to `strtol`, `format date and time` to `strftime`, `remove a file` to `remove`, `temporary file` to `tmpfile`, and `end of file` to `EOF`/`feof`.
2. **Use phrase-aware matching before token scoring.** Preserve phrases such as `command line arguments`, `sequence points`, `flexible array member`, and `include guards` so generic matches for `arguments`, `line`, `member`, or `file` cannot dominate.
3. **Add curated C concept aliases and landing pages.** Explicitly index `argc argv`, sequencing/sequence points, all storage-duration kinds, `_Atomic` under type qualifiers, and header-guard patterns.
4. **Apply a strong C-domain boost and C++ penalty.** Rank `/c/` references above `/cpp/` or `std::` pages when the selected corpus is C, especially for linkage and atomic memory ordering.
5. **Harden and enrich web fallback.** Retry or switch source when DuckDuckGo returns no results, turn useful knowledge cards into normal ranked results, and prefer snippets that show C syntax or signatures over boilerplate such as the designated-initializer result.
