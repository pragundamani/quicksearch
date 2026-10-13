# Final held-out evaluation

Binary: `/tmp/qs-gate/release/qs`.

## Summary

- Queries: **200/200**
- Average: **3.85/5** (baseline: **2.92**)
- Score >=3: **158/200 (79.0%)** (baseline: **55.0%**)
- Score >=4: **141/200 (70.5%)** (baseline: **48.0%**)
- Distribution: **0: 22, 1: 4, 2: 16, 3: 17, 4: 23, 5: 118**
- Timeouts: **0**

## By intent class

| Class | Queries | Average | Score >=3 | Score >=4 |
|---|---:|---:|---:|---:|
| paraphrase | 34 | 2.53 | 19 | 13 |
| negative-contrast | 34 | 2.76 | 19 | 16 |
| surplus-token-adversarial | 33 | 4.73 | 31 | 31 |
| unseen-api | 33 | 4.76 | 32 | 31 |
| source-routing | 33 | 4.42 | 30 | 28 |
| non-language-routing-control | 33 | 3.94 | 27 | 22 |

## Part reports

- `.heldout-parts-final2/part1.md`
- `.heldout-parts-final2/part2.md`
- `.heldout-parts-final2/part3.md`
- `.heldout-parts-final2/part4.md`

The final safety fixes after this run tightened weak full-text thresholds, enabled Stack Similar only below authoritative sources, fixed Hackage/API precedence, enforced Rust result limits, and hardened caches. Focused tests and smoke checks cover those changes.
