# C++ production-candidate v11 regression

- Candidate: `/tmp/qs-gate/release/qs`
- SHA256: `8f05a6dcd581ef25fe5d2904b169cb3794a94056b19d687cf4646a6372cc096d` (**verified**)
- Checklist: `cpp.md`
- Commands: **100**
- Mode: `--json`
- Timeout: **20 seconds per command**
- Nonzero exits: **0**
- Timeouts: **0**
- Invalid JSON responses: **0**
- Score: **489/500 (97.8%)**
- Mean: **4.89/5**

## Score distribution

| Score | Meaning | Count |
|---:|---|---:|
| 5 | Canonical / immediately useful first | 89 |
| 4 | Expected target top 3 / useful | 11 |
| 3 | Relevant but weak | 0 |
| 2 | Partial | 0 |
| 1 | Misleading | 0 |
| 0 | Error / no relevant result | 0 |

## Non-5 results

| # | Command | Score | Assessment |
|---:|---|---:|---|
| 13 | `qs cpp optional value` | 4 | Highly useful optional member pages lead, but the canonical std::optional class page is not in the top three. |
| 24 | `qs cpp heap algorithms` | 4 | Top results are useful heap operations, but std::make_heap itself is not in the top three. |
| 27 | `qs cpp remove_if` | 4 | Exact std::remove_if algorithm page is third; container remove_if overloads rank above it. |
| 29 | `qs cpp shuffle range` | 4 | Only std::ranges::shuffle appears; it is closely useful but not the expected std::shuffle page. |
| 33 | `qs cpp std::move` | 4 | The utility std::move target is second; the algorithm with the same name ranks first. |
| 49 | `qs cpp write binary file` | 4 | std::ofstream documentation ranks first, but the results do not directly surface binary-mode guidance. |
| 66 | `qs cpp consteval` | 4 | Exact language specifier page is third; the first result is only a keyword stub. |
| 67 | `qs cpp decltype(auto)` | 4 | First result is the closely related decltype page, while the exact placeholder-type page is lower. |
| 77 | `qs cpp atomic compare exchange` | 4 | A valid compare-exchange free function is first; the expected atomic member appears second, but strong is not top-three. |
| 87 | `qs cpp time zones` | 4 | Canonical time_zone is first and useful, but zoned_time is absent from the top three. |
| 98 | `qs cpp find duplicate values` | 4 | std::adjacent_find is relevant and first, but only detects adjacent duplicates unless data is arranged first. |

## Errors and timeouts

None. All 100 commands exited successfully and returned valid JSON within 20 seconds.

Full results, hit data, timings, exit status, and scoring rationales are in `cpp.json`.
