# C++ quicksearch score report

## Summary

- Queries run: **100/100**
- Average score: **3.87 / 5**
- Median score: **5**
- Pass rate (score >=3): **83% (83/100)**
- Strong pass rate (score >=4): **78% (78/100)**
- Command errors/timeouts: **0**
- Distribution: **0: 11, 1: 4, 2: 2, 3: 5, 4: 26, 5: 52**

Each checklist command was run separately through the installed `qs` from `PATH`. A 25-second process timeout was applied per command. Scoring uses the requested practical-developer rubric; it does not use the checklist's expected field as execution evidence.

## All results

| # | Command | Score | Observed top result/error | Assessment |
|---:|---|---:|---|---|
| 1 | `qs cpp vector` | **4** | #1 “vector” | Header is #1; the `std::vector` class page is #2 with a clear signature and definition. |
| 2 | `qs cpp unordered maps` | **0** | #1 “std::mdspan::mapping” | No map hit. All five results are `mdspan`/layout mapping pages; top is `std::mdspan::mapping`. |
| 3 | `qs cpp std::array` | **5** | #1 “std::array” | Exact class is #1 with its template signature and fixed-size-array definition. |
| 4 | `qs cpp forward lists` | **4** | #1 “forward_list” | Header is #1 and the exact class is #2; declarations provide useful context. |
| 5 | `qs cpp priority_queue` | **5** | #1 “std::priority_queue” | Exact adaptor is #1 with signature, behavior, and example context. |
| 6 | `qs cpp multisets` | **5** | #1 “std::multiset” | Exact class is the sole result and includes a useful template signature. |
| 7 | `qs cpp map::try_emplace` | **5** | #1 “std::map::try_emplace” | Exact member is #1; snippet and example cover emplacement behavior. |
| 8 | `qs cpp vector capacity` | **5** | #1 “std::vector::capacity” | Exact member is #1 with signature, reallocation meaning, and example. |
| 9 | `qs cpp span` | **4** | #1 “span” | Header is #1 and exact class is #2; useful declarations are immediate. |
| 10 | `qs cpp mdspan` | **4** | #1 “mdspan” | Header is #1 and exact class is #2; modern API is easy to reach. |
| 11 | `qs cpp tuples` | **4** | #1 “tuple” | Header is #1 and exact class is #2; declaration context is useful. |
| 12 | `qs cpp std::variant` | **5** | #1 “std::variant” | Exact class is #1 with a concise type-safe-union explanation. |
| 13 | `qs cpp optional value` | **2** | #1 “std::optional::value” | Returns `std::optional::value`, not the `std::optional` type; relevant but narrows the query incorrectly. |
| 14 | `qs cpp any_cast` | **5** | #1 “std::any_cast” | Exact cast API is #1 with headers and example context. |
| 15 | `qs cpp bitsets` | **4** | #1 “bitset” | Header is #1 and exact class is #2; useful declaration shown. |
| 16 | `qs cpp sort` | **5** | #1 “std::sort” | Exact algorithm is #1 and includes an example, though the leading snippet is a constraint fragment. |
| 17 | `qs cpp stable_sort` | **5** | #1 “std::stable_sort” | Exact algorithm is #1 with example context. |
| 18 | `qs cpp lower_bound` | **5** | #1 “std::lower_bound” | Exact algorithm is #1 with a clear behavioral summary and example. |
| 19 | `qs cpp ranges::filter_view` | **5** | #1 “std::ranges::filter_view” | Exact range view is #1 with deduction-guide and example context. |
| 20 | `qs cpp std::ranges::sort` | **5** | #1 “std::ranges::sort” | Exact ranges algorithm is #1 with definition and example. |
| 21 | `qs cpp transform_reduce` | **5** | #1 “std::transform_reduce” | Exact numeric algorithm is #1; example is present, but snippet starts with a constraint fragment. |
| 22 | `qs cpp partition point` | **5** | #1 “std::partition_point” | Exact algorithm is #1 with precondition and example. |
| 23 | `qs cpp nth_element` | **5** | #1 “std::nth_element” | Exact algorithm is #1 with examples; constrained counterpart is #2. |
| 24 | `qs cpp heap algorithms` | **3** | #1 “Algorithms library” | Only broad algorithm index pages appear. They are relevant, but no heap operation is surfaced. |
| 25 | `qs cpp set intersection` | **5** | #1 “std::set_intersection” | Exact algorithm is #1 with behavior and example. |
| 26 | `qs cpp accumulate` | **5** | #1 “std::accumulate” | Exact numeric algorithm is #1 with include and example context. |
| 27 | `qs cpp remove_if` | **4** | #1 “remove_if” | Container member `remove_if` pages occupy #1–2; expected `std::remove_if` is #3 and still usable. |
| 28 | `qs cpp binary search` | **5** | #1 “std::binary_search” | Exact algorithm is #1 with semantics and example. |
| 29 | `qs cpp shuffle range` | **4** | #1 “std::ranges::shuffle” | `std::ranges::shuffle` is #1 with an example. It is a useful modern counterpart, not the requested classic API. |
| 30 | `qs cpp unique_ptr` | **5** | #1 “std::unique_ptr” | Exact smart pointer is #1 with ownership explanation and full template signature. |
| 31 | `qs cpp make shared` | **5** | #1 “std::make_shared” | Exact factory is #1 with signature; irrelevant `shared_mutex` at #2 does not block it. |
| 32 | `qs cpp weak pointers` | **0** | #1 “hazard_pointer” | No `std::weak_ptr` hit. Top result is unrelated `hazard_pointer` and only two results are returned. |
| 33 | `qs cpp std::move` | **4** | #1 “std::move” | Top result is the algorithm `std::move`, not the cast utility; the utility overload is #2. |
| 34 | `qs cpp perfect forwarding` | **4** | #1 “forward_list” | Top result is unrelated `forward_list`; `std::forward` is #2 and provides the practical API. |
| 35 | `qs cpp allocator_traits` | **5** | #1 “std::allocator_traits” | Exact traits type is #1 with definition and signature. |
| 36 | `qs cpp polymorphic memory resource` | **4** | #1 “memory_resource” | Header is #1 and exact `std::pmr::memory_resource` is #2; namespace declarations are useful. |
| 37 | `qs cpp reference_wrapper` | **5** | #1 “std::reference_wrapper” | Exact wrapper is #1 with purpose, signature, and example. |
| 38 | `qs cpp type traits` | **5** | #1 “Type traits” | Exact library overview is #1 and the header is #2. |
| 39 | `qs cpp std::function` | **5** | #1 “std::function” | Exact callable wrapper is #1 with strong description and signature. |
| 40 | `qs cpp string views` | **4** | #1 “string_view” | Header is #1; `std::basic_string_view` is #2. Useful, but the common alias is not titled directly. |
| 41 | `qs cpp from_chars` | **5** | #1 “std::from_chars” | Exact conversion API is #1 with success semantics, include, and example. |
| 42 | `qs cpp format string` | **5** | #1 “std::format_string” | `std::format_string` is #1 with aliases and signature. This directly answers the format-string wording. |
| 43 | `qs cpp regex_match` | **5** | #1 “std::regex_match” | Exact regex function is #1 with include and example context. |
| 44 | `qs cpp getline` | **5** | #1 “std::getline” | Exact helper is #1; snippet highlights a common newline pitfall and includes an example. |
| 45 | `qs cpp stringstreams` | **5** | #1 “std::basic_stringstream” | Exact primary stream type is #1 with description and template signature. |
| 46 | `qs cpp filesystem path` | **5** | #1 “std::filesystem::path” | Exact path type is #1 with a clear semantic description. |
| 47 | `qs cpp recursive_directory_iterator` | **5** | #1 “std::filesystem::recursive_directory_iterator” | Exact recursive iterator is #1 with behavior, include, and sample namespace setup. |
| 48 | `qs cpp ifstream` | **5** | #1 “std::basic_ifstream” | Expected primary type is #1 with a clear description and template signature. |
| 49 | `qs cpp write binary file` | **1** | #1 “std::FILE” | No `ofstream` or binary-writing result. Top `std::FILE` is only tangentially related. |
| 50 | `qs cpp std::print` | **5** | #1 “std::print” | Exact C++23 API is #1 with include and example context. |
| 51 | `qs cpp source_location` | **4** | #1 “source_location” | Header is #1 and exact type is #2; declaration is immediately useful. |
| 52 | `qs cpp stacktrace` | **4** | #1 “stacktrace” | Header is #1 and `std::basic_stacktrace` is #2; useful but alias is not directly titled. |
| 53 | `qs cpp error codes` | **5** | #1 “std::error_code” | Exact error type is #1 with a clear platform-error explanation. |
| 54 | `qs cpp exception hierarchy` | **5** | #1 “std::exception - cppreference.com” | Web fallback ranks cppreference's `std::exception` page #1 and its snippet explains the hierarchy. |
| 55 | `qs cpp RAII` | **5** | #1 “RAII” | Exact language concept is #1 with a strong definition and code example. |
| 56 | `qs cpp move semantics` | **0** | #1 “std::contracts::evaluation_semantic” | No move-semantics page appears. The only result is unrelated `std::contracts::evaluation_semantic`. |
| 57 | `qs cpp copy elision` | **5** | #1 “Copy elision” | Exact language page is #1 with definition and example. |
| 58 | `qs cpp rule of five` | **5** | #1 “The rule of three/five/zero” | Exact rule-of-three/five/zero page is #1 with useful guidance and code. |
| 59 | `qs cpp structured bindings` | **5** | #1 “Structured binding declaration” | Exact language page is #1 with a direct definition and example. |
| 60 | `qs cpp lambda captures` | **5** | #1 “Lambda Capture Clause in C++ - GeeksforGeeks” | Top web result directly explains capture clauses; cppreference is #2 for authoritative detail. |
| 61 | `qs cpp generic lambdas` | **4** | #1 “Lambda expressions” | General lambda page is #1 and its generated call-operator snippet supports generic lambdas, but title is broad. |
| 62 | `qs cpp template specializations` | **5** | #1 “Partial template specialization” | Partial specialization is #1 and explicit specialization is #2, covering both requested forms. |
| 63 | `qs cpp concepts requires clause` | **5** | #1 “requires expression (since C++20) - cppreference.com” | Requires expression is #1 and constraints/concepts is #3; combined results answer the query well. |
| 64 | `qs cpp fold expressions` | **4** | #1 “Fold expressions” | Exact page is #1, but its snippet is empty, so the result gives little context before opening. |
| 65 | `qs cpp constexpr functions` | **0** | #1 “Functions” | No constexpr-specific hit. Top is the generic `Functions` page and the remaining hits are unrelated. |
| 66 | `qs cpp consteval` | **4** | #1 “keyword: consteval” | Keyword page is #1 with boilerplate only; useful `consteval specifier` page is #2 with a definition. |
| 67 | `qs cpp decltype(auto)` | **1** | #1 “keyword: auto” | Only `keyword: auto` appears. It does not explain or even name `decltype(auto)` in the result. |
| 68 | `qs cpp type deduction` | **3** | #1 “Template argument deduction” | Template argument deduction is #1, but the result set omits `auto`; only half the expected topic is covered. |
| 69 | `qs cpp range based for` | **5** | #1 “Range-based for loop” | Exact language feature is #1 with a practical definition and example. |
| 70 | `qs cpp coroutines` | **5** | #1 “Coroutines” | Exact language topic is #1 with definition, include, and a substantial example. |
| 71 | `qs cpp modules` | **5** | #1 “Modules” | Exact modules page is #1 with explanatory text and module syntax. |
| 72 | `qs cpp three way comparison` | **4** | #1 “Comparison operators” | Comparison-operators page is #1 and relevant to `<=>`, but the snippet shows older two-way comparison text. |
| 73 | `qs cpp threads` | **4** | #1 “thread” | Header is #1 and exact `std::thread` is #2; declarations are useful. |
| 74 | `qs cpp jthread stop token` | **5** | #1 “std::jthread::get_stop_token” | `std::jthread::get_stop_token` is #1 with exact signature and explains the shared stop state. |
| 75 | `qs cpp mutexes` | **4** | #1 “mutex” | Header is #1 and exact `std::mutex` is #2; family declarations add useful context. |
| 76 | `qs cpp condition variables` | **4** | #1 “condition_variable” | Header is #1 and exact class is #2; declarations include related condition-variable APIs. |
| 77 | `qs cpp atomic compare exchange` | **4** | #1 “std::atomic_compare_exchange_weak” | Free `atomic_compare_exchange_weak` is #1 and member weak overload is #2; strong form is not surfaced in titles. |
| 78 | `qs cpp memory ordering` | **5** | #1 “std::memory_order” | Exact memory-order type is #1 with all enum values shown. |
| 79 | `qs cpp futures` | **4** | #1 “future” | Header is #1 and exact `std::future` is #2; the error enum is useful context but delays the class. |
| 80 | `qs cpp async launch policies` | **0** | #1 “execution-policy” | No `std::async` or `std::launch`. The sole result is unrelated parallel-algorithm `execution-policy`. |
| 81 | `qs cpp counting_semaphore` | **5** | #1 “std::counting_semaphore” | Exact semaphore is #1 with purpose, signature, include, and example. |
| 82 | `qs cpp latches` | **4** | #1 “latch” | Header is #1 and exact `std::latch` is #2; declaration is useful. |
| 83 | `qs cpp barriers` | **4** | #1 “barrier” | Header is #1 and exact `std::barrier` is #2; declaration is useful. |
| 84 | `qs cpp duration_cast` | **5** | #1 “std::chrono::duration_cast” | Exact chrono conversion API is #1 with constraint, include, and example. |
| 85 | `qs cpp steady clock` | **5** | #1 “std::chrono::steady_clock” | Exact monotonic clock is #1 with a strong explanation; `now` is #2. |
| 86 | `qs cpp calendar dates` | **2** | #1 “Date and time library” | Top result is only the broad chrono index. No `year_month_day` appears, and later hits drift to locale/C time. |
| 87 | `qs cpp time zones` | **5** | #1 “std::chrono::time_zone” | Both expected APIs are ranked #1 and #2 with clear definitions. |
| 88 | `qs cpp complex numbers` | **0** | #1 “numbers” | No `std::complex`. Top is the unrelated `<numbers>` header, followed by error numbers. |
| 89 | `qs cpp gcd lcm` | **3** | #1 “std::lcm” | Only `std::lcm` is returned. It is useful for half the query, but `std::gcd` is absent. |
| 90 | `qs cpp random number engine` | **4** | #1 “named requirements: RandomNumberEngine” | The exact RandomNumberEngine requirement is #1 with a useful definition, but common engines such as `mt19937` are absent. |
| 91 | `qs cpp uniform_int_distribution` | **5** | #1 “std::uniform_int_distribution” | Exact distribution is #1 with signature, probability description, include, and example. |
| 92 | `qs cpp remove elements from vector` | **1** | #1 “vector” | Returns generic vector pages only. Nothing explains erase-remove or `std::erase_if`. |
| 93 | `qs cpp erase map key` | **0** | #1 “std::map::key_comp” | No erase API. Top `std::map::key_comp` is misleading; other hits are comparison functions. |
| 94 | `qs cpp read whole file` | **0** | #1 “std::FILE” | No whole-file reading method or `ifstream`. Results are the same unrelated file-category pages as the write query. |
| 95 | `qs cpp split string delimiter` | **3** | #1 “How to Split a String by a Delimiter in C++? - GeeksforGeeks” | Web results give practical delimiter-splitting methods, but none surface the expected `std::views::split` API. |
| 96 | `qs cpp convert string to integer` | **0** | #1 “Integer literal” | No conversion API. Top `Integer literal` and `numeric_limits::is_integer` do not answer the task. |
| 97 | `qs cpp sort custom objects` | **1** | #1 “Function objects” | No sorting result. `Function objects` is only weak comparator context; other results are generic object pages. |
| 98 | `qs cpp find duplicate values` | **0** | #1 “Value categories” | No duplicate-detection approach. Top `Value categories` is unrelated to the task. |
| 99 | `qs cpp measure elapsed time` | **0** | #1 “LC_TIME” | No elapsed-time technique. Top `LC_TIME` and legacy time formatting APIs are misleading. |
| 100 | `qs cpp visit variant` | **3** | #1 “std::variant::visit” | Top is the C++26 member `std::variant::visit`, not the expected nonmember `std::visit`; still relevant but version-sensitive. |

## Strongest patterns

- Exact qualified symbols and distinctive snake_case names are excellent. Examples include `std::array`, `map::try_emplace`, `ranges::filter_view`, `transform_reduce`, `recursive_directory_iterator`, and `uniform_int_distribution`.
- Most direct library lookups include useful signatures, headers, semantics, or examples. This makes 52 queries immediate score-5 answers.
- Spaced normalization works well for many APIs: `partition point`, `set intersection`, `make shared`, `steady clock`, and `range based for`.
- Language-topic pages rank well when the topic maps to a dedicated cppreference page: RAII, copy elision, rule of five, structured bindings, coroutines, and modules.
- Web fallback helps when the local index match is weak. `exception hierarchy`, `lambda captures`, and `concepts requires clause` returned useful cppreference or explanatory pages.

## Weakest patterns

- Plural normalization can cause severe token confusion. `unordered maps` returns `std::mdspan::mapping`; `weak pointers` returns `hazard_pointer`; `complex numbers` returns `<numbers>`.
- Natural task queries often degrade into individual-token matches. `erase map key` returns `std::map::key_comp`; `measure elapsed time` returns `LC_TIME`; `convert string to integer` returns `Integer literal`.
- Multi-term concepts are sometimes reduced to one generic term. `move semantics` returns `evaluation_semantic`; `constexpr functions` returns `Functions`; `async launch policies` returns algorithm `execution-policy`.
- File tasks do not route well. Both `write binary file` and `read whole file` return `std::FILE` first and never surface `ofstream`/`ifstream` techniques.
- Header pages frequently outrank their exact types. These remain usable, but they turn many potential 5s into 4s. Examples include `vector`, `span`, `thread`, `mutexes`, `latches`, and `barriers`.

## Prioritized improvements

1. **Add phrase and task aliases before token search.** Map common intents such as `convert string to integer` → `stoi`/`from_chars`, `measure elapsed time` → `steady_clock`, `erase map key` → `map::erase`, and file read/write phrases → stream APIs.
2. **Improve morphological normalization with symbol-aware safeguards.** Singularize `maps`, `pointers`, and `numbers` without letting substring matches such as `mapping`, `hazard_pointer`, or `<numbers>` dominate.
3. **Require multi-token coverage and penalize one-token drift.** This would suppress `evaluation_semantic` for `move semantics`, generic `Functions` for `constexpr functions`, and `execution-policy` for `async launch policies`.
4. **Rank entity pages above header pages when the query names an entity.** Prefer `std::vector`, `std::span`, `std::thread`, `std::mutex`, `std::latch`, and `std::barrier`; retain headers as the next result.
5. **Add concept-family and companion-symbol expansion.** Surface `make_heap` for heap algorithms, `year_month_day` for calendar dates, both `gcd` and `lcm`, common engines such as `mt19937`, erase-remove/`erase_if`, and the nonmember `std::visit`.
