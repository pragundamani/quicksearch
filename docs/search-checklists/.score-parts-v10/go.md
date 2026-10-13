# Go v10 final-candidate evaluation

- Candidate: `/tmp/qs-gate/release/qs`
- SHA256: `a048a86983a8877eea4240c5a24e4e9119e901ceaeff44bc8d13e15911f8cabd`
- Checklist: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/go.md`
- Commands: **100/100**
- Successful JSON responses: **100/100**
- Timeouts/errors: **0**
- Score: **495/500 (99.0%)**
- Mean: **4.95/5.00**
- Distribution: **5: 95, 4: 5, 3: 0, 2: 0, 1: 0, 0: 0**

## Results below 5

| # | Query | Score | First result | Assessment |
|---:|---|---:|---|---|
| 12 | `qs go panic and recover` | 4 | `func panic` | `panic` is first and `recover` is discussed in the second result, but no direct `recover` result is returned. |
| 18 | `qs go interface embedding` | 4 | `interfaces` | The authoritative interface-types section is first and useful, but the title and snippet do not expose interface embedding or type sets. |
| 42 | `qs go regexp named groups` | 4 | `func (*Regexp) CapNames` | Named-capture APIs are the first three results, but the named-group pattern syntax is not shown directly. |
| 77 | `qs go worker pool` | 4 | `type Pool 1.3` | The useful authoritative worker-pool guide is third; unrelated `Pool` types occupy the first two positions. |
| 81 | `qs go http client timeout` | 4 | `Clients and Transports` | The `http.Client` type is third and its result includes the full `Timeout` field documentation, but the exact field is not surfaced directly. |
