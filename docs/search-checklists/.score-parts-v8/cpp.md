# C++ quicksearch v8 regression score

- Commands: **100**
- Successful JSON responses: **100**
- Mean score: **4.82/5**
- Total: **482/500**
- Distribution: **5=86, 4=10, 3=4, 2=0, 1=0, 0=0**

## Results below 5

- **3/5** — `optional value`: Results focus on optional value accessors; the std::optional type page is absent from the top five.
- **3/5** — `heap algorithms`: Top results are individual heap operations (push_heap and is_heap), but make_heap and a clear family overview are absent.
- **4/5** — `remove_if`: std::remove_if is third; the first two titles are ambiguous unqualified remove_if entries.
- **3/5** — `shuffle range`: std::ranges::shuffle is first and useful, but the expected std::shuffle page is absent.
- **4/5** — `perfect forwarding`: A relevant forwarding example is first and std::forward is second, but forwarding-reference guidance is indirect.
- **4/5** — `polymorphic memory resource`: The memory_resource header is second and std::pmr::memory_resource is third; the first result is allocator::resource.
- **4/5** — `decltype(auto)`: The decltype page is first and auto deduction is second, but decltype(auto) is not an exact result title.
- **4/5** — `type deduction`: Template argument deduction is first and auto-related context appears, but coverage of both expected topics is indirect.
- **4/5** — `atomic compare exchange`: Weak compare-exchange APIs are first and second; compare_exchange_strong is absent.
- **4/5** — `calendar dates`: Calendar operator and calendar library pages are top two, but year_month_day is not a direct top result.
- **4/5** — `time zones`: std::chrono::time_zone is first, but zoned_time is absent from the top five.
- **4/5** — `complex numbers`: The numerics overview is first and std::complex is second.
- **4/5** — `remove elements from vector`: std::erase_if is first, but the erase-remove idiom is not represented.
- **3/5** — `read whole file`: ifstream is first, but results do not show a whole-file iterator or buffer recipe.

## Key failures

- `optional value` did not return the `std::optional` type page; it returned only value-access members.
- `heap algorithms` omitted `std::make_heap` and did not provide a strong heap-family overview.
- `shuffle range` returned only `std::ranges::shuffle`, not the expected `std::shuffle`.
- `read whole file` found `std::ifstream` but did not show a whole-file recipe.

No command timed out or returned an error.
