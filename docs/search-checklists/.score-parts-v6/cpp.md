# C++ search score summary

- Queries: **100**
- Mean score: **4.66 / 5**
- Score distribution: **5: 78, 4: 11, 3: 10, 2: 1, 1: 0, 0: 0**
- Score >=3: **99/100**
- Score >=4: **89/100**
- Command errors/timeouts: **0**
- Failures (score <=2): **1**

## Failures

- **2 — `qs cpp exception hierarchy`**: The top three results are unrelated filesystem exception sections. A generic exception page is fourth, and no standard exception hierarchy is returned.

## Most important regressions / weak results

- **3 — `qs cpp format string`**: Returns `std::format_string`, but omits `std::format`.
- **3 — `qs cpp move semantics`**: Favors the `move_constructible` concept and does not explain rvalue references or move construction/assignment.
- **3 — `qs cpp random number engine`**: Returns requirements pages but no concrete engine such as `std::mt19937`.
- **3 — `qs cpp remove elements from vector`**: Returns remove algorithms but omits `std::erase_if` and clear erase-remove guidance.
- **3 — `qs cpp read whole file`**: Finds `std::ifstream`, but provides no whole-file technique.
- **3 — `qs cpp split string delimiter`**: Ranks deduction guides above the expected `views::split` page; `stderr` is third.
- **3 — `qs cpp visit variant`**: Returns only the C++26 member `std::variant::visit`, not the common free `std::visit` API.
