# Held-out evaluation v9 — part 3 (indexes 100–149)

- Queries: 50
- Mean score: 4.70/5
- Distribution: 5: 39, 4: 7, 3: 4, 2: 0, 1: 0, 0: 0
- Command errors/timeouts: 0

## Key failures and weaker results

- **139, 147 (3/5):** The web provider returned only a generic DuckDuckGo fallback link, with no fetched result or direct target.
- **144 (3/5):** Correct cheat.sh route, but no exact sheet was available; output only links to a generic related search.
- **145 (3/5):** Correct GitHub search, but the canonical unordered-containers repository was absent.
- **103, 121 (4/5):** Exact Go API pages ranked third.
- **127 (4/5):** Correct `crypto/ecdh` package ranked first, but no direct `X25519` hit appeared.
- **131 (4/5):** `foldMap1` ranked first under `Data.Foldable1`, not the expected `Data.Foldable` path.
- **135 (4/5):** pkg.go.dev returned useful `x/sync` subpackages, not the module root.
- **136 (4/5):** Correct camino crate ranked first, but its hit linked to docs.rs instead of crates.io.
- **146 (4/5):** Relevant MDN WebAssembly API page ranked second behind the Python glossary.
