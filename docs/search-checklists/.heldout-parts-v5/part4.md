# Quicksearch held-out final evaluation — part 4

Candidate: `/tmp/qs-gate/release/qs`  
Manifest rows: **150–199** (50 commands)  
Run rule: only the leading `qs` was replaced. Each command had a 30-second command timeout.

## Summary

- Average score: **3.88 / 5**
- Score distribution: **0: 3, 1: 1, 2: 9, 3: 4, 4: 2, 5: 31**
- Score >=3: **37 / 50 (74%)**
- Score >=4: **33 / 50 (66%)**
- Nonzero exit errors: **1** (row 176)
- Command-level timeouts (`timeout` exit 124): **0**
- Upstream/network timeouts reported by the candidate: **1** (row 176)

Negative-contrast note: the manifest has no separate `rejected_target` field. For rows labeled `negative`, I checked whether an unintended inferred route/result was ranked above the expected non-language target.

## Results

| Row | Label | Query | Expected | Current observed output | Contrast | Score |
|---:|:---:|---|---|---|:---:|---:|
| 150 | positive | `qs rfc 8949` | RFC 8949 | RFC 8949 and CBOR are first. | — | **5** |
| 151 | positive | `qs haskell System.Mem.performGC` | Hoogle results for System.Mem.performGC | Hoogle result `performGC` is first. | — | **5** |
| 152 | positive | `qs gh python-trio/trio` | GitHub repository python-trio/trio | `python-trio/trio` is first. | — | **5** |
| 153 | positive | `qs man gofmt` | Local gofmt manual page | No local entry; only Debian manual search fallback. | — | **2** |
| 154 | positive | `qs crate zerocopy` | crates.io page for zerocopy | `zerocopy` is first, but result URL is docs.rs rather than crates.io. | — | **4** |
| 155 | positive | `qs web cppreference execution control library` | DuckDuckGo results for cppreference execution control library | cppreference execution-control page is first. | — | **5** |
| 156 | positive | `qs so c strict aliasing effective type` | Stack Overflow search for C strict aliasing effective type | Relevant Stack Overflow strict-aliasing results. | — | **5** |
| 157 | positive | `qs web haskell backpack signatures` | DuckDuckGo results for Haskell Backpack signatures | Relevant Backpack/signature pages, with Stack docs first. | — | **5** |
| 158 | positive | `qs pip orjson` | PyPI package page for orjson | PyPI `orjson` is first. | — | **5** |
| 159 | positive | `qs gh golang/tools` | GitHub repository golang/tools | `golang/tools` is first. | — | **5** |
| 160 | positive | `qs std std::hint::black_box` | Official Rust standard-library page for std::hint::black_box | Official `std::hint::black_box` page is first. | — | **5** |
| 161 | positive | `qs man clang-format` | Local clang-format manual page | No local entry; only Debian manual search fallback. | — | **2** |
| 162 | positive | `qs wiki C standard library` | Wikipedia result for C standard library | Wikipedia C standard library is first. | — | **5** |
| 163 | positive | `qs so haskell quantified constraints` | Stack Overflow search for Haskell quantified constraints | Stack Overflow results are broadly about Haskell constraints, but none directly match quantified constraints. | — | **3** |
| 164 | positive | `qs web python free threading status` | DuckDuckGo results for Python free threading status | Official Python free-threading guide is first. | — | **5** |
| 165 | positive | `qs rfc QUIC transport` | RFC title search for QUIC transport | IETF search returns related QUIC drafts, but misses the core QUIC transport RFC. | — | **3** |
| 166 | positive | `qs docs alloc::alloc::Layout` | Official Rust alloc docs for alloc::alloc::Layout | Official alloc `Layout` page is first. | — | **5** |
| 167 | negative | `qs 9780143127741` | Product lookup for barcode 9780143127741 | Correct ISBN product is first; provider heading says `stock` but URL/result are product lookup. | Pass | **5** |
| 168 | negative | `qs 1912` | DuckDuckGo web search for 1912 | Web search for year 1912, with Wikipedia first. | Pass | **5** |
| 169 | negative | `qs item 012345678905` | Product lookup for barcode 012345678905 | Correct barcode product is first. | Pass | **5** |
| 170 | negative | `qs stock ASML` | Stock lookup for ASML | Misrouted to Open Food Facts product search; no ASML stock result. | Fail: unintended product route is top | **0** |
| 171 | negative | `qs cve CVE-2024-3094` | NVD advisory for CVE-2024-3094 | NVD CVE-2024-3094 is first. | Pass | **5** |
| 172 | negative | `qs nvd libwebp heap overflow` | NVD advisory search for libwebp heap overflow | Relevant NVD libwebp heap-overflow advisories. | Pass | **5** |
| 173 | negative | `qs npm fast-glob` | npm registry page for fast-glob | npm `fast-glob` is first. | Pass | **5** |
| 174 | negative | `qs gem faraday` | RubyGems page for faraday | RubyGems `faraday` is first. | Pass | **5** |
| 175 | negative | `qs hex bandit` | Hex package page for bandit | Hex `bandit` is first. | Pass | **5** |
| 176 | negative | `qs maven org.flywaydb flyway-core` | Maven registry result for org.flywaydb:flyway-core | Exited 1 after Maven upstream read timeout; no result. | N/A: error, no ranked result | **0** |
| 177 | negative | `qs nuget Polly` | NuGet page for Polly | Polly is first with package metadata, but item URL is GitHub rather than NuGet. | Pass | **4** |
| 178 | negative | `qs wiki Ada Lovelace` | Wikipedia page for Ada Lovelace | Wikipedia Ada Lovelace is first. | Pass | **5** |
| 179 | negative | `qs mdn AbortSignal.any` | MDN page for AbortSignal.any | Exact MDN `AbortSignal.any()` page is first. | Pass | **5** |
| 180 | negative | `qs so git rerere` | Stack Overflow search for git rerere | Strong Stack Overflow `git rerere` results. | Pass | **5** |
| 181 | negative | `qs gh sharkdp/bat` | GitHub repository sharkdp/bat | `sharkdp/bat` is first. | Pass | **5** |
| 182 | negative | `qs debian unattended upgrades` | Debian Wiki result for unattended upgrades | Only a Debian Wiki full-search link; no page details. | Pass | **3** |
| 183 | negative | `qs ubuntu netplan policy routing` | Ubuntu Wiki result for netplan policy routing | Ubuntu Wiki lookup returned HTTP 404; only a site-search fallback. | Pass | **2** |
| 184 | negative | `qs fedora toolbox` | Fedora Wiki result for toolbox | Only a niche RISC-V Toolbox result, not the general Fedora Toolbox target. | Pass | **1** |
| 185 | negative | `qs gentoo eselect repository` | Gentoo Wiki result for eselect repository | Exact English page is fifth; translations rank above it. | Pass | **3** |
| 186 | negative | `qs arch mkinitcpio unified kernel image` | ArchWiki result for unified kernel image | No matching ArchWiki page; only a search fallback. | Pass | **2** |
| 187 | negative | `qs man 5 resolv.conf` | Local resolv.conf manual section 5 | Local `resolv.conf(5)` content returned. | Pass | **5** |
| 188 | negative | `qs cargo metadata` | cargo-metadata manual page | Local `cargo-metadata(1)` content returned. | Pass | **5** |
| 189 | negative | `qs podman kube play` | podman-kube-play manual page | Local `podman-kube-play(1)` content returned. | Pass | **5** |
| 190 | negative | `qs dnf ripgrep` | DNF package information for ripgrep | Local DNF package details for ripgrep returned. | Pass | **5** |
| 191 | negative | `qs apt policy` | apt-policy manual page | No local `apt-policy`; fallback points to generic `apt(8)`. | Pass | **2** |
| 192 | negative | `qs brew leaves` | brew-leaves manual page | No local `brew-leaves`; fallback points to the generic Brew manpage. | Pass | **2** |
| 193 | negative | `qs flatpak remotes` | flatpak-remotes manual page | Local `flatpak-remotes(1)` content returned. | Pass | **5** |
| 194 | negative | `qs snap connections` | snap-connections manual page | No local `snap-connections`; fallback points to generic command reference. | Pass | **2** |
| 195 | negative | `qs web rust crate serde` | DuckDuckGo results for rust crate serde | Serde docs are first; crates.io is third. | Pass | **5** |
| 196 | negative | `qs !aur paru-bin` | DuckDuckGo bang result for !aur paru-bin | Bang did not redirect; only a DuckDuckGo bang link. | Pass | **2** |
| 197 | negative | `qs cheat tar extract strip components` | cheat.sh result for tar extraction | No exact cheat.sh sheet; only a related-search link. | Pass | **2** |
| 198 | negative | `qs rfc structured field values HTTP` | RFC title search for Structured Field Values for HTTP | No matching RFC; only a generic IETF document-search link. | Pass | **0** |
| 199 | negative | `qs price of palladium per ounce` | DuckDuckGo web search for palladium price | Useful palladium price pages are first. | Pass | **5** |

## Error detail

- Row 176, `qs maven org.flywaydb flyway-core`: exit 1 after about 15.5 seconds. Output: `Network Error: Error encountered in the status line: timed out reading response`.
- No command reached the 30-second wrapper timeout.
