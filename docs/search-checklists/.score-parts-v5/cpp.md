# C++ candidate-v5 quicksearch evaluation

## Summary

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Timeout: **30 seconds per command**.
- Average: **4.68/5**.
- Median: **5/5**.
- Distribution: **0: 0, 1: 0, 2: 2, 3: 7, 4: 12, 5: 79**.
- Practical pass rate (score >=3): **98/100 (98%)**.
- Strong-result rate (score >=4): **91/100 (91%)**.
- Command errors/timeouts: **0 errors, 0 timeouts**.

## Comparison with candidate-v3

- Candidate-v3: **4.28 average**, **5 median**, **92% score >=3**, **85% score >=4**, and **0 errors/timeouts**.
- Candidate-v5: **4.68 average**, **5 median**, **98% score >=3**, **91% score >=4**, and **0 errors/timeouts**.
- Change: **+0.40 average**, **+6 percentage points** at score >=3, and **+6 percentage points** at score >=4. The median and error/timeout counts are unchanged.
- Distribution changed from **0: 2, 1: 1, 2: 5, 3: 7, 4: 29, 5: 56** to **0: 0, 1: 0, 2: 2, 3: 7, 4: 12, 5: 79**.
- Per-query scores improved on **29** commands and regressed on **2** commands. The regressions are `perfect forwarding` (**4 -> 3**) and `exception hierarchy` (**4 -> 2**).

## Results

| # | Command | Score | Observed top result/error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs cpp vector` | **5** | #1 “std::vector” | “std::vector” ranks first and directly answers the query with useful documentation context. |
| 2 | `qs cpp unordered maps` | **5** | #1 “std::unordered_map” | “std::unordered_map” ranks first and directly answers the query with useful documentation context. |
| 3 | `qs cpp std::array` | **5** | #1 “std::array” | “std::array” ranks first and directly answers the query with useful documentation context. |
| 4 | `qs cpp forward lists` | **5** | #1 “std::forward_list” | “std::forward_list” ranks first and directly answers the query with useful documentation context. |
| 5 | `qs cpp priority_queue` | **5** | #1 “std::priority_queue” | “std::priority_queue” ranks first and directly answers the query with useful documentation context. |
| 6 | `qs cpp multisets` | **5** | #1 “std::multiset” | “std::multiset” ranks first and directly answers the query with useful documentation context. |
| 7 | `qs cpp map::try_emplace` | **5** | #1 “std::map::try_emplace” | “std::map::try_emplace” ranks first and directly answers the query with useful documentation context. |
| 8 | `qs cpp vector capacity` | **5** | #1 “std::vector::capacity” | “std::vector::capacity” ranks first and directly answers the query with useful documentation context. |
| 9 | `qs cpp span` | **5** | #1 “std::span” | “std::span” ranks first and directly answers the query with useful documentation context. |
| 10 | `qs cpp mdspan` | **5** | #1 “std::mdspan” | “std::mdspan” ranks first and directly answers the query with useful documentation context. |
| 11 | `qs cpp tuples` | **5** | #1 “std::tuple” | “std::tuple” ranks first and directly answers the query with useful documentation context. |
| 12 | `qs cpp std::variant` | **5** | #1 “std::variant” | “std::variant” ranks first and directly answers the query with useful documentation context. |
| 13 | `qs cpp optional value` | **2** | #1 “std::optional::value”; #2 “std::optional::value_or”; #3 “std::optional::has_value” | The query narrows to `std::optional::value` and misses the requested `std::optional` type page. |
| 14 | `qs cpp any_cast` | **5** | #1 “std::any_cast” | “std::any_cast” ranks first and directly answers the query with useful documentation context. |
| 15 | `qs cpp bitsets` | **5** | #1 “std::bitset” | “std::bitset” ranks first and directly answers the query with useful documentation context. |
| 16 | `qs cpp sort` | **5** | #1 “std::sort” | “std::sort” ranks first and directly answers the query with useful documentation context. |
| 17 | `qs cpp stable_sort` | **5** | #1 “std::stable_sort” | “std::stable_sort” ranks first and directly answers the query with useful documentation context. |
| 18 | `qs cpp lower_bound` | **5** | #1 “std::lower_bound” | “std::lower_bound” ranks first and directly answers the query with useful documentation context. |
| 19 | `qs cpp ranges::filter_view` | **5** | #1 “std::ranges::filter_view” | “std::ranges::filter_view” ranks first and directly answers the query with useful documentation context. |
| 20 | `qs cpp std::ranges::sort` | **5** | #1 “std::ranges::sort” | “std::ranges::sort” ranks first and directly answers the query with useful documentation context. |
| 21 | `qs cpp transform_reduce` | **5** | #1 “std::transform_reduce” | “std::transform_reduce” ranks first and directly answers the query with useful documentation context. |
| 22 | `qs cpp partition point` | **5** | #1 “std::partition_point” | “std::partition_point” ranks first and directly answers the query with useful documentation context. |
| 23 | `qs cpp nth_element` | **5** | #1 “std::nth_element” | “std::nth_element” ranks first and directly answers the query with useful documentation context. |
| 24 | `qs cpp heap algorithms` | **4** | #1 “std::push_heap”; #2 “std::is_heap”; #3 “Constrained algorithms” | `std::push_heap` ranks first and several heap operations follow, but `std::make_heap` is absent. |
| 25 | `qs cpp set intersection` | **5** | #1 “std::set_intersection” | “std::set_intersection” ranks first and directly answers the query with useful documentation context. |
| 26 | `qs cpp accumulate` | **5** | #1 “std::accumulate” | “std::accumulate” ranks first and directly answers the query with useful documentation context. |
| 27 | `qs cpp remove_if` | **4** | #1 “remove_if”; #2 “remove_if”; #3 “std::remove_if” | Container-member `remove_if` pages rank first and second; expected `std::remove_if` is third and usable. |
| 28 | `qs cpp binary search` | **5** | #1 “std::binary_search” | “std::binary_search” ranks first and directly answers the query with useful documentation context. |
| 29 | `qs cpp shuffle range` | **4** | #1 “std::ranges::shuffle” | `std::ranges::shuffle` ranks first with useful context, but the requested classic `std::shuffle` page is absent. |
| 30 | `qs cpp unique_ptr` | **5** | #1 “std::unique_ptr” | “std::unique_ptr” ranks first and directly answers the query with useful documentation context. |
| 31 | `qs cpp make shared` | **5** | #1 “std::make_shared” | “std::make_shared” ranks first and directly answers the query with useful documentation context. |
| 32 | `qs cpp weak pointers` | **5** | #1 “std::weak_ptr” | “std::weak_ptr” ranks first and directly answers the query with useful documentation context. |
| 33 | `qs cpp std::move` | **4** | #1 “std::move”; #2 “std::move”; #3 “std::move_backward” | The algorithm overload of `std::move` ranks first; the expected cast utility is second with a full explanation and example. |
| 34 | `qs cpp perfect forwarding` | **3** | #1 “Placeholder type specifiers (since C++11)”; #2 “decltype”; #3 “advance-fwd” | A relevant perfect-forwarding example ranks first, but the expected `std::forward` page falls to fifth. |
| 35 | `qs cpp allocator_traits` | **5** | #1 “std::allocator_traits” | “std::allocator_traits” ranks first and directly answers the query with useful documentation context. |
| 36 | `qs cpp polymorphic memory resource` | **4** | #1 “std::pmr::polymorphic_allocator<T>::polymorphic_allocator”; #2 “std::pmr::polymorphic_allocator”; #3 “std::pmr::memory_resource” | `std::pmr::memory_resource` is third after closely related polymorphic-allocator pages, so it remains easy to reach. |
| 37 | `qs cpp reference_wrapper` | **5** | #1 “std::reference_wrapper” | “std::reference_wrapper” ranks first and directly answers the query with useful documentation context. |
| 38 | `qs cpp type traits` | **5** | #1 “Type traits” | “Type traits” ranks first and directly answers the query with useful documentation context. |
| 39 | `qs cpp std::function` | **5** | #1 “std::function” | “std::function” ranks first and directly answers the query with useful documentation context. |
| 40 | `qs cpp string views` | **5** | #1 “std::basic_string_view” | “std::basic_string_view” ranks first and directly answers the query with useful documentation context. |
| 41 | `qs cpp from_chars` | **5** | #1 “std::from_chars” | “std::from_chars” ranks first and directly answers the query with useful documentation context. |
| 42 | `qs cpp format string` | **5** | #1 “std::format_string” | “std::format_string” ranks first and directly answers the query with useful documentation context. |
| 43 | `qs cpp regex_match` | **5** | #1 “std::regex_match” | “std::regex_match” ranks first and directly answers the query with useful documentation context. |
| 44 | `qs cpp getline` | **5** | #1 “std::getline” | “std::getline” ranks first and directly answers the query with useful documentation context. |
| 45 | `qs cpp stringstreams` | **5** | #1 “std::basic_stringstream” | “std::basic_stringstream” ranks first and directly answers the query with useful documentation context. |
| 46 | `qs cpp filesystem path` | **5** | #1 “std::filesystem::path” | “std::filesystem::path” ranks first and directly answers the query with useful documentation context. |
| 47 | `qs cpp recursive_directory_iterator` | **5** | #1 “std::filesystem::recursive_directory_iterator” | “std::filesystem::recursive_directory_iterator” ranks first and directly answers the query with useful documentation context. |
| 48 | `qs cpp ifstream` | **5** | #1 “std::basic_ifstream” | “std::basic_ifstream” ranks first and directly answers the query with useful documentation context. |
| 49 | `qs cpp write binary file` | **3** | #1 “std::ofstream”; #2 “std::basic_ofstream”; #3 “std::basic_ofstream::open” | `std::ofstream` ranks first, but the output does not show binary mode or a binary-write recipe. |
| 50 | `qs cpp std::print` | **5** | #1 “std::print” | “std::print” ranks first and directly answers the query with useful documentation context. |
| 51 | `qs cpp source_location` | **5** | #1 “std::source_location” | “std::source_location” ranks first and directly answers the query with useful documentation context. |
| 52 | `qs cpp stacktrace` | **5** | #1 “std::basic_stacktrace” | “std::basic_stacktrace” ranks first and directly answers the query with useful documentation context. |
| 53 | `qs cpp error codes` | **5** | #1 “std::error_code” | “std::error_code” ranks first and directly answers the query with useful documentation context. |
| 54 | `qs cpp exception hierarchy` | **2** | #1 “std::filesystem::recursive_directory_iterator::pop”; #2 “std::filesystem::recursive_directory_iterator::depth”; #3 “Exceptions” | Two unrelated filesystem exception fragments rank first; the language exception overview is third, and the standard exception hierarchy is not shown. |
| 55 | `qs cpp RAII` | **5** | #1 “RAII” | “RAII” ranks first and directly answers the query with useful documentation context. |
| 56 | `qs cpp move semantics` | **3** | #1 “std::move_constructible”; #2 “std::copy_constructible”; #3 “std::move” | `std::move_constructible` is relevant, but the results do not explain rvalue references, move construction, and move assignment as a whole. |
| 57 | `qs cpp copy elision` | **5** | #1 “Copy elision” | “Copy elision” ranks first and directly answers the query with useful documentation context. |
| 58 | `qs cpp rule of five` | **5** | #1 “The rule of three/five/zero” | “The rule of three/five/zero” ranks first and directly answers the query with useful documentation context. |
| 59 | `qs cpp structured bindings` | **5** | #1 “Structured binding declaration” | “Structured binding declaration” ranks first and directly answers the query with useful documentation context. |
| 60 | `qs cpp lambda captures` | **5** | #1 “Lambda expressions (since C++11)” | “Lambda expressions (since C++11)” ranks first and directly answers the query with useful documentation context. |
| 61 | `qs cpp generic lambdas` | **5** | #1 “Lambda expressions (since C++11)” | “Lambda expressions (since C++11)” ranks first and directly answers the query with useful documentation context. |
| 62 | `qs cpp template specializations` | **5** | #1 “Partial template specialization” | “Partial template specialization” ranks first and directly answers the query with useful documentation context. |
| 63 | `qs cpp concepts requires clause` | **5** | #1 “Constraints and concepts (since C++20)” | “Constraints and concepts (since C++20)” ranks first and directly answers the query with useful documentation context. |
| 64 | `qs cpp fold expressions` | **5** | #1 “Fold expressions” | “Fold expressions” ranks first and directly answers the query with useful documentation context. |
| 65 | `qs cpp constexpr functions` | **5** | #1 “constexpr” | “constexpr” ranks first and directly answers the query with useful documentation context. |
| 66 | `qs cpp consteval` | **4** | #1 “keyword: consteval”; #2 “std::cosl”; #3 “consteval specifier” | The `consteval` keyword ranks first, while the useful immediate-function/specifier page is only third. |
| 67 | `qs cpp decltype(auto)` | **3** | #1 “C++ keyword: decltype”; #2 “decltype”; #3 “keyword: auto” | The keyword result mentions `decltype(auto)` first, but the detailed placeholder-type page is fourth. |
| 68 | `qs cpp type deduction` | **3** | #1 “Class template argument deduction (CTAD) (since C++17)”; #2 “Class template argument deduction (CTAD) (since C++17)”; #3 “Type” | Class template argument deduction ranks first and is useful, but `auto` deduction is not covered. |
| 69 | `qs cpp range based for` | **5** | #1 “Range-based for loop” | “Range-based for loop” ranks first and directly answers the query with useful documentation context. |
| 70 | `qs cpp coroutines` | **5** | #1 “Coroutines” | “Coroutines” ranks first and directly answers the query with useful documentation context. |
| 71 | `qs cpp modules` | **5** | #1 “Modules” | “Modules” ranks first and directly answers the query with useful documentation context. |
| 72 | `qs cpp three way comparison` | **4** | #1 “std::three_way_comparable, std::three_way_comparable_with”; #2 “Standard library header <compare> (C++20)”; #3 “synth-three-way” | Three-way comparison facilities rank first with `<=>` context, but `std::strong_ordering` is not surfaced directly. |
| 73 | `qs cpp threads` | **5** | #1 “std::thread” | “std::thread” ranks first and directly answers the query with useful documentation context. |
| 74 | `qs cpp jthread stop token` | **5** | #1 “std::jthread::get_stop_token” | “std::jthread::get_stop_token” ranks first and directly answers the query with useful documentation context. |
| 75 | `qs cpp mutexes` | **5** | #1 “std::mutex” | “std::mutex” ranks first and directly answers the query with useful documentation context. |
| 76 | `qs cpp condition variables` | **5** | #1 “std::condition_variable” | “std::condition_variable” ranks first and directly answers the query with useful documentation context. |
| 77 | `qs cpp atomic compare exchange` | **4** | #1 “std::atomic_compare_exchange_weak”; #2 “std::atomic::compare_exchange_weak”; #3 “std::atomic_ref::compare_exchange_weak” | Weak compare-exchange APIs rank first and include member context, but the strong form is not surfaced in the visible results. |
| 78 | `qs cpp memory ordering` | **5** | #1 “std::memory_order” | “std::memory_order” ranks first and directly answers the query with useful documentation context. |
| 79 | `qs cpp futures` | **5** | #1 “std::future” | “std::future” ranks first and directly answers the query with useful documentation context. |
| 80 | `qs cpp async launch policies` | **5** | #1 “std::async” | “std::async” ranks first and directly answers the query with useful documentation context. |
| 81 | `qs cpp counting_semaphore` | **5** | #1 “std::counting_semaphore” | “std::counting_semaphore” ranks first and directly answers the query with useful documentation context. |
| 82 | `qs cpp latches` | **5** | #1 “std::latch” | “std::latch” ranks first and directly answers the query with useful documentation context. |
| 83 | `qs cpp barriers` | **5** | #1 “std::barrier” | “std::barrier” ranks first and directly answers the query with useful documentation context. |
| 84 | `qs cpp duration_cast` | **5** | #1 “std::chrono::duration_cast” | “std::chrono::duration_cast” ranks first and directly answers the query with useful documentation context. |
| 85 | `qs cpp steady clock` | **5** | #1 “std::chrono::steady_clock” | “std::chrono::steady_clock” ranks first and directly answers the query with useful documentation context. |
| 86 | `qs cpp calendar dates` | **4** | #1 “std::chrono::operator==,<=>(std::chrono::year_month_day)”; #2 “std::chrono::operator==,<=>(std::chrono::year_month_day_last)”; #3 “keyword: default” | `year_month_day` comparison APIs rank first and the chrono calendar overview is fifth; useful calendar coverage is present, but the primary type page is missing. |
| 87 | `qs cpp time zones` | **5** | #1 “std::chrono::time_zone” | “std::chrono::time_zone” ranks first and directly answers the query with useful documentation context. |
| 88 | `qs cpp complex numbers` | **5** | #1 “std::complex” | “std::complex” ranks first and directly answers the query with useful documentation context. |
| 89 | `qs cpp gcd lcm` | **5** | #1 “std::gcd” | “std::gcd” ranks first and directly answers the query with useful documentation context. |
| 90 | `qs cpp random number engine` | **4** | #1 “named requirements: RandomNumberEngine”; #2 “named requirements: RandomNumberEngineAdaptor” | The exact RandomNumberEngine requirement ranks first with a definition, but common engines such as `std::mt19937` are absent. |
| 91 | `qs cpp uniform_int_distribution` | **5** | #1 “std::uniform_int_distribution” | “std::uniform_int_distribution” ranks first and directly answers the query with useful documentation context. |
| 92 | `qs cpp remove elements from vector` | **3** | #1 “std::remove, std::remove_if”; #2 “std::ranges::remove_copy, std::ranges::remove_copy_if, std::ranges::remove_copy_result, std::ranges::remove_copy_if_result”; #3 “std::remove” | `std::remove`/`std::remove_if` ranks first, but `std::erase_if` and a complete erase-remove recipe are not surfaced. |
| 93 | `qs cpp erase map key` | **5** | #1 “std::map::erase” | “std::map::erase” ranks first and directly answers the query with useful documentation context. |
| 94 | `qs cpp read whole file` | **3** | #1 “std::ifstream”; #2 “std::basic_ifstream”; #3 “std::basic_ifstream::open” | `std::ifstream` ranks first, but no whole-file iterator or stream-buffer recipe is shown. |
| 95 | `qs cpp split string delimiter` | **4** | #1 “std::ranges::views::lazy_split, std::ranges::lazy_split_view”; #2 “deduction guides for std::ranges::split_view”; #3 “stderr” | `std::ranges::views::lazy_split` ranks first and `split_view` material appears in the top results; this is useful but not the exact requested `views::split` page. |
| 96 | `qs cpp convert string to integer` | **5** | #1 “std::stoi” | “std::stoi” ranks first and directly answers the query with useful documentation context. |
| 97 | `qs cpp sort custom objects` | **5** | #1 “std::sort” | “std::sort” ranks first and directly answers the query with useful documentation context. |
| 98 | `qs cpp find duplicate values` | **5** | #1 “std::adjacent_find” | “std::adjacent_find” ranks first and directly answers the query with useful documentation context. |
| 99 | `qs cpp measure elapsed time` | **5** | #1 “std::chrono::steady_clock” | “std::chrono::steady_clock” ranks first and directly answers the query with useful documentation context. |
| 100 | `qs cpp visit variant` | **4** | #1 “std::variant::visit” | The C++26 member `std::variant::visit` ranks first, but its visible example also shows the expected nonmember `std::visit` fallback. |

## Notes

- All commands exited with status 0.
- No command reached the 30-second timeout.
- Scores assess the terminal output shown to the user under the rubric in `language_scores.md`.
