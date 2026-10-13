# C quicksearch v12 evaluation

- Executable: `/tmp/qs-gate/release/qs`
- SHA256: `f1988ac17d46ea8bf5ab53e3e0dd173aee946ebc7aad4f44e33eecbc8ab987c7`
- Commands: **100**
- Timeout: **20 seconds per command**
- Exit-code failures: **0**
- Timeouts: **0**
- Score: **497/500 (99.4%)**
- Mean: **4.97/5**

## Score distribution

| Score | Count |
|---:|---:|
| 5 | 99 |
| 4 | 0 |
| 3 | 0 |
| 2 | 1 |
| 1 | 0 |
| 0 | 0 |

## Results at or below 4

| # | Query | Score | Top results | Assessment |
|---:|---|---:|---|---|
| 93 | `allocate zero initialized array` | 2 | `Array declaration and subscripting` | The array declaration page is only partially related; it does not surface calloc or zero-initialized allocation. |

## Errors and timeouts

None.
