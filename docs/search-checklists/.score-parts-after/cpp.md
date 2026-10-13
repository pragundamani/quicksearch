# C++ candidate quicksearch evaluation

## Summary

- Binary: `/tmp/qs-candidate/release/qs` (not the installed `qs`).
- Commands run: **100/100**.
- Timeout: **20 seconds per command**.
- Average: **3.69/5**.
- Median: **5/5**.
- Distribution: **0: 20, 1: 0, 2: 1, 3: 3, 4: 22, 5: 54**.
- Practical pass rate (score >=3): **79/100 (79%)**.
- Strong-result rate (score >=4): **76/100 (76%)**.
- Command errors/timeouts: **20 errors, 0 timeouts**.

## Baseline comparison

The documented C++ baseline is **3.87 average**, **5 median**, **83% score >=3**, **78% score >=4**, and **0 errors/timeouts**. The candidate is **0.18 points lower** on average, keeps the same median, is **4 percentage points lower** at score >=3, and **2 points lower** at score >=4. It improves several task/plural queries, but 20 failed web fallbacks erase much of that gain.

## Results

| # | Command | Score | Observed top result/error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs cpp vector` | **4** | #1 “vector” | Header is #1; the `std::vector` class page is #2 with a clear signature and definition. |
| 2 | `qs cpp unordered maps` | **4** | #1 “unordered_map” | Header is #1 and exact `std::unordered_map` is #2; the result is easy to use. |
| 3 | `qs cpp std::array` | **5** | #1 “std::array” | Exact class is #1 with its template signature and fixed-size-array definition. |
| 4 | `qs cpp forward lists` | **4** | #1 “forward_list” | Header is #1 and the exact class is #2; declarations provide useful context. |
| 5 | `qs cpp priority_queue` | **5** | #1 “std::priority_queue” | Exact adaptor is #1 with signature, behavior, and example context. |
| 6 | `qs cpp multisets` | **5** | #1 “std::multiset” | Exact class is #1 with a useful template signature and related set results. |
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
| 24 | `qs cpp heap algorithms` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 25 | `qs cpp set intersection` | **5** | #1 “std::set_intersection” | Exact algorithm is #1 with behavior and example. |
| 26 | `qs cpp accumulate` | **5** | #1 “std::accumulate” | Exact numeric algorithm is #1 with include and example context. |
| 27 | `qs cpp remove_if` | **4** | #1 “remove_if” | Container member `remove_if` pages occupy #1–2; expected `std::remove_if` is #3 and still usable. |
| 28 | `qs cpp binary search` | **5** | #1 “std::binary_search” | Exact algorithm is #1 with semantics and example. |
| 29 | `qs cpp shuffle range` | **4** | #1 “std::ranges::shuffle” | `std::ranges::shuffle` is #1 with an example. It is a useful modern counterpart, not the requested classic API. |
| 30 | `qs cpp unique_ptr` | **5** | #1 “std::unique_ptr” | Exact smart pointer is #1 with ownership explanation and full template signature. |
| 31 | `qs cpp make shared` | **5** | #1 “std::make_shared” | Exact factory is #1 with signature; irrelevant `shared_mutex` at #2 does not block it. |
| 32 | `qs cpp weak pointers` | **5** | #1 “std::weak_ptr” | Exact `std::weak_ptr` is #1 with its related APIs, fixing the earlier miss. |
| 33 | `qs cpp std::move` | **4** | #1 “std::move” | Top result is the algorithm `std::move`, not the cast utility; the utility overload is #2. |
| 34 | `qs cpp perfect forwarding` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 35 | `qs cpp allocator_traits` | **5** | #1 “std::allocator_traits” | Exact traits type is #1 with definition and signature. |
| 36 | `qs cpp polymorphic memory resource` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
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
| 49 | `qs cpp write binary file` | **3** | #1 “std::ofstream” | `std::ofstream` ranks first, but the output does not show binary mode or a write recipe. |
| 50 | `qs cpp std::print` | **5** | #1 “std::print” | Exact C++23 API is #1 with include and example context. |
| 51 | `qs cpp source_location` | **4** | #1 “source_location” | Header is #1 and exact type is #2; declaration is immediately useful. |
| 52 | `qs cpp stacktrace` | **4** | #1 “stacktrace” | Header is #1 and `std::basic_stacktrace` is #2; useful but alias is not directly titled. |
| 53 | `qs cpp error codes` | **5** | #1 “std::error_code” | Exact error type is #1 with a clear platform-error explanation. |
| 54 | `qs cpp exception hierarchy` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 55 | `qs cpp RAII` | **5** | #1 “RAII” | Exact language concept is #1 with a strong definition and code example. |
| 56 | `qs cpp move semantics` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 57 | `qs cpp copy elision` | **5** | #1 “Copy elision” | Exact language page is #1 with definition and example. |
| 58 | `qs cpp rule of five` | **5** | #1 “The rule of three/five/zero” | Exact rule-of-three/five/zero page is #1 with useful guidance and code. |
| 59 | `qs cpp structured bindings` | **5** | #1 “Structured binding declaration” | Exact language page is #1 with a direct definition and example. |
| 60 | `qs cpp lambda captures` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 61 | `qs cpp generic lambdas` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 62 | `qs cpp template specializations` | **5** | #1 “Partial template specialization” | Partial specialization is #1 and explicit specialization is #2, covering both requested forms. |
| 63 | `qs cpp concepts requires clause` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 64 | `qs cpp fold expressions` | **5** | #1 “Fold expressions” | Exact language page is #1 with a definition and useful fold examples. |
| 65 | `qs cpp constexpr functions` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 66 | `qs cpp consteval` | **4** | #1 “keyword: consteval” | Keyword page is #1 with boilerplate only; useful `consteval specifier` page is #2 with a definition. |
| 67 | `qs cpp decltype(auto)` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 68 | `qs cpp type deduction` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 69 | `qs cpp range based for` | **5** | #1 “Range-based for loop” | Exact language feature is #1 with a practical definition and example. |
| 70 | `qs cpp coroutines` | **5** | #1 “Coroutines” | Exact language topic is #1 with definition, include, and a substantial example. |
| 71 | `qs cpp modules` | **5** | #1 “Modules” | Exact modules page is #1 with explanatory text and module syntax. |
| 72 | `qs cpp three way comparison` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 73 | `qs cpp threads` | **4** | #1 “thread” | Header is #1 and exact `std::thread` is #2; declarations are useful. |
| 74 | `qs cpp jthread stop token` | **5** | #1 “std::jthread::get_stop_token” | `std::jthread::get_stop_token` is #1 with exact signature and explains the shared stop state. |
| 75 | `qs cpp mutexes` | **4** | #1 “mutex” | Header is #1 and exact `std::mutex` is #2; family declarations add useful context. |
| 76 | `qs cpp condition variables` | **4** | #1 “condition_variable” | Header is #1 and exact class is #2; declarations include related condition-variable APIs. |
| 77 | `qs cpp atomic compare exchange` | **4** | #1 “std::atomic_compare_exchange_weak” | Free `atomic_compare_exchange_weak` is #1 and member weak overload is #2; strong form is not surfaced in titles. |
| 78 | `qs cpp memory ordering` | **5** | #1 “std::memory_order” | Exact memory-order type is #1 with all enum values shown. |
| 79 | `qs cpp futures` | **4** | #1 “future” | Header is #1 and exact `std::future` is #2; the error enum is useful context but delays the class. |
| 80 | `qs cpp async launch policies` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 81 | `qs cpp counting_semaphore` | **5** | #1 “std::counting_semaphore” | Exact semaphore is #1 with purpose, signature, include, and example. |
| 82 | `qs cpp latches` | **4** | #1 “latch” | Header is #1 and exact `std::latch` is #2; declaration is useful. |
| 83 | `qs cpp barriers` | **4** | #1 “barrier” | Header is #1 and exact `std::barrier` is #2; declaration is useful. |
| 84 | `qs cpp duration_cast` | **5** | #1 “std::chrono::duration_cast” | Exact chrono conversion API is #1 with constraint, include, and example. |
| 85 | `qs cpp steady clock` | **5** | #1 “std::chrono::steady_clock” | Exact monotonic clock is #1 with a strong explanation; `now` is #2. |
| 86 | `qs cpp calendar dates` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 87 | `qs cpp time zones` | **5** | #1 “std::chrono::time_zone” | Both expected APIs are ranked #1 and #2 with clear definitions. |
| 88 | `qs cpp complex numbers` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 89 | `qs cpp gcd lcm` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 90 | `qs cpp random number engine` | **4** | #1 “named requirements: RandomNumberEngine” | The exact RandomNumberEngine requirement is #1 with a useful definition, but common engines such as `mt19937` are absent. |
| 91 | `qs cpp uniform_int_distribution` | **5** | #1 “std::uniform_int_distribution” | Exact distribution is #1 with signature, probability description, include, and example. |
| 92 | `qs cpp remove elements from vector` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 93 | `qs cpp erase map key` | **5** | #1 “std::map::erase” | Exact `std::map::erase` is #1 with a practical erasure example; unordered-map erase is also shown. |
| 94 | `qs cpp read whole file` | **3** | #1 “std::ifstream” | `std::ifstream` ranks first, but no whole-file iterator or buffer recipe is shown. |
| 95 | `qs cpp split string delimiter` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 96 | `qs cpp convert string to integer` | **5** | #1 “std::stoi” | `std::stoi` is #1 with a full conversion example; `std::from_chars` is also in the top results. |
| 97 | `qs cpp sort custom objects` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 98 | `qs cpp find duplicate values` | **0** | Error: no web results (exit 1) | Command failed with exit 1 after web fallback returned no results. |
| 99 | `qs cpp measure elapsed time` | **5** | #1 “std::chrono::steady_clock” | `std::chrono::steady_clock` is #1 and the `now()` example directly measures elapsed time. |
| 100 | `qs cpp visit variant` | **3** | #1 “std::variant::visit” | Top is the C++26 member `std::variant::visit`, not the expected nonmember `std::visit`; still relevant but version-sensitive. |
