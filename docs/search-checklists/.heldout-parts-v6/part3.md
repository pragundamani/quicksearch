# Held-out evaluation: part 3 (indexes 100–149)

- Candidate: `/tmp/qs-gate/release/qs`
- Entries: 50
- Mean score: **4.70 / 5**
- Command errors or timeouts: **0**

## Score distribution

| Score | Count |
|---:|---:|
| 5 | 42 |
| 4 | 4 |
| 3 | 2 |
| 2 | 1 |
| 1 | 1 |
| 0 | 0 |

## Failures and weak results

- **100 — score 2:** `qs c reference standard code example quick answer feclearexcept floating environment` did not return `feclearexcept`. It returned nearby floating-point environment APIs.
- **144 — score 1:** `qs cheat awk field separator` returned only a placeholder that says the exact cheat.sh topic is unavailable. It gave no requested guidance.
- **127 — score 3:** `qs go crypto/ecdh.X25519` returned the `crypto/ecdh` package page first, but no visible hit named or described `X25519`.
- **145 — score 3:** `qs gh haskell-unordered-containers` returned related or packaging repositories. The canonical repository was not visible.

## Other ranking issues

- **103 — score 4:** Exact `netip.ParsePrefix` result was third.
- **121 — score 4:** Exact `zip.Writer.Copy` result was third.
- **135 — score 4:** Useful `golang.org/x/sync` subpackages appeared, but no root-module result was shown.
- **146 — score 4:** The WebAssembly JavaScript API guide was second; the first hit was a Python glossary page.
