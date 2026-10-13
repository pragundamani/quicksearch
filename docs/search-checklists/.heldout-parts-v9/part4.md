# Held-out evaluation v9 — part 4 (indexes 150–199)

- Candidate: `/tmp/qs-gate/release/qs`
- Mode: `--json`
- Timeout: 30 seconds per query
- Queries: manifest command strings, with the leading display token `qs` removed before invocation
- Cases: 50
- Total score: **185/250**
- Mean score: **3.70/5**
- Score 4–5: **33/50 (66%)**
- Score 3–5: **37/50 (74%)**
- Distribution: 5=27, 4=6, 3=4, 2=3, 1=8, 0=2

## By class

- `source-routing`: 17 cases, mean **3.88/5**
- `non-language-routing-control`: 33 cases, mean **3.61/5**

## Key failures

- **168 — `qs 1912` (0):** The web provider returned no hits.
- **176 — Maven Flyway lookup (0):** The Maven request timed out while reading the response.
- **155, 157, 164, 195, 199 (1):** Web searches degraded to one generic DuckDuckGo search link with no actual results.
- **183 (1):** Ubuntu Wiki returned only a DuckDuckGo site-search placeholder.
- **186 (1):** Arch Wiki returned only its generic search page.
- **197 (1):** cheat.sh returned only a search URL with no guidance.
- **163 (2):** Stack Overflow results did not visibly target quantified constraints.
- **182 (2):** Debian returned only a full-search URL without content.
- **184 (2):** Fedora returned only a RISC-V-specific Toolbox page.

## Notable partial results

- **153, 161 (4):** No local manual was found, but direct official online documentation was useful.
- **165 (4):** QUIC applicability was first, but the core QUIC transport RFC was absent.
- **177 (4):** Polly was exact, but the URL was GitHub rather than NuGet.
- **185 (3):** Gentoo returned only localized Eselect/Repository pages.
- **191, 192, 194 (3):** Remote or general documentation substituted for missing local manuals.

Detailed per-case evidence is in `part4.json`.
