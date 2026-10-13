# C++ final-candidate v10 evaluation

- Candidate: `/tmp/qs-gate/release/qs`
- SHA256: `a048a86983a8877eea4240c5a24e4e9119e901ceaeff44bc8d13e15911f8cabd`
- Checklist: `cpp.md`
- Commands: **100**
- Mode: `--json`
- Timeout: **30 seconds per command**
- Nonzero exits: **0**
- Timeouts: **0**
- Score: **489/500 (97.8%)**
- Mean: **4.89/5**

## Score distribution

| Score | Meaning | Count |
|---:|---|---:|
| 5 | First / immediately useful | 90 |
| 4 | Expected in top 3 / useful | 9 |
| 3 | Relevant but weak | 1 |
| 2 | Partial | 0 |
| 1 | Misleading | 0 |
| 0 | Error / no relevant result | 0 |

## Non-5 results

| # | Command | Score | Assessment |
|---:|---|---:|---|
| 13 | `qs cpp optional value` | 4 | The first three hits are useful std::optional value-access members, but the std::optional type page is not in the top five. |
| 27 | `qs cpp remove_if` | 4 | The exact std::remove_if algorithm is third; list and forward_list remove_if members rank first and second. |
| 33 | `qs cpp std::move` | 4 | The algorithm overload of std::move ranks first; the expected move utility is second. |
| 34 | `qs cpp perfect forwarding` | 4 | A related placeholder-type page ranks first and std::forward ranks second; the result set is useful but not ideally ordered. |
| 36 | `qs cpp polymorphic memory resource` | 4 | The memory-resource header is second and the exact std::pmr::memory_resource page is third, behind the allocator resource accessor. |
| 66 | `qs cpp consteval` | 4 | The exact keyword stub ranks first, but the substantive consteval specifier page is third. |
| 67 | `qs cpp decltype(auto)` | 4 | The decltype page ranks first and auto deduction ranks second, but no result is specifically titled decltype(auto). |
| 86 | `qs cpp calendar dates` | 4 | Calendar composition operators rank first and the chrono library calendar overview ranks second; year_month_day itself is not a top hit. |
| 88 | `qs cpp complex numbers` | 4 | The numerics overview ranks first and the exact std::complex page ranks second. |
| 94 | `qs cpp read whole file` | 3 | The results are ifstream class and member documentation, not a direct whole-file reading recipe using iterators or buffers. |

## Failures

None. All 100 commands completed successfully and returned valid JSON.

Full hit data, timing, exit status, score, and rationale for every command are in `cpp.json`.
