# Held-out evaluation, final2 part 4 (entries 150–199)

- Binary: `/tmp/qs-gate/release/qs`
- Manifest: `heldout_manifest.json`
- Runs: 50 fresh process invocations, each wrapped with `timeout 30s`.
- Outcomes: 49 exited successfully; entry 176 returned a Maven Central network read-timeout error before the outer limit.
- Scale: 0–5 as specified.

| Index | Query | Expected | Score | Observation | Contrast check |
|---:|---|---|---:|---|---|
| 150 | `qs rfc 8949` | RFC 8949 | 5 | Exact RFC 8949 and CBOR detail first. | — |
| 151 | `qs haskell System.Mem.performGC` | Hoogle results for System.Mem.performGC | 5 | Exact performGC API first with Hackage detail. | — |
| 152 | `qs gh python-trio/trio` | GitHub repository python-trio/trio | 5 | Exact repository first. | — |
| 153 | `qs man gofmt` | Local gofmt manual page | 2 | No local man page; only an online-manual search fallback. | — |
| 154 | `qs crate zerocopy` | crates.io page for zerocopy | 3 | Exact crate is first, but link is docs.rs rather than the requested crates.io page. | — |
| 155 | `qs web cppreference execution control library` | DuckDuckGo results for cppreference execution control library | 5 | Exact cppreference page first. | — |
| 156 | `qs so c strict aliasing effective type` | Stack Overflow search for C strict aliasing effective type | 5 | Correct Stack Overflow search with relevant results. | — |
| 157 | `qs web haskell backpack signatures` | DuckDuckGo results for Haskell Backpack signatures | 5 | Correct DuckDuckGo results with a useful Backpack guide first. | — |
| 158 | `qs pip orjson` | PyPI package page for orjson | 5 | Exact PyPI package first. | — |
| 159 | `qs gh golang/tools` | GitHub repository golang/tools | 5 | Exact repository first. | — |
| 160 | `qs std std::hint::black_box` | Official Rust standard-library page for std::hint::black_box | 5 | Exact official std item first. | — |
| 161 | `qs man clang-format` | Local clang-format manual page | 2 | No local man page; only an online-manual search fallback. | — |
| 162 | `qs wiki C standard library` | Wikipedia result for C standard library | 5 | Exact Wikipedia page first. | — |
| 163 | `qs so haskell quantified constraints` | Stack Overflow search for Haskell quantified constraints | 4 | Correct Stack Overflow search; results are related but weak for the exact feature. | — |
| 164 | `qs web python free threading status` | DuckDuckGo results for Python free threading status | 5 | Correct DuckDuckGo results with official Python free-threading docs first. | — |
| 165 | `qs rfc QUIC transport` | RFC title search for QUIC transport | 4 | Correct IETF title search with useful QUIC transport results, though not a definitive RFC first. | — |
| 166 | `qs docs alloc::alloc::Layout` | Official Rust alloc docs for alloc::alloc::Layout | 5 | Exact official alloc Layout page first. | — |
| 167 | `qs 9780143127741` | Product lookup for barcode 9780143127741 | 5 | Exact ISBN product first. | PASS — no language target at top |
| 168 | `qs 1912` | DuckDuckGo web search for 1912 | 5 | Correct web route and a useful 1912 instant answer. | PASS — no language target at top |
| 169 | `qs item 012345678905` | Product lookup for barcode 012345678905 | 5 | Exact barcode product first. | PASS — no language target at top |
| 170 | `qs stock ASML` | Stock lookup for ASML | 0 | Misroutes stock ticker ASML to product lookup and finds nothing. | PASS — no language target at top |
| 171 | `qs cve CVE-2024-3094` | NVD advisory for CVE-2024-3094 | 5 | Exact NVD advisory first. | PASS — no language target at top |
| 172 | `qs nvd libwebp heap overflow` | NVD advisory search for libwebp heap overflow | 5 | Correct NVD search with matching heap-overflow advisories first. | PASS — no language target at top |
| 173 | `qs npm fast-glob` | npm registry page for fast-glob | 5 | Exact npm package first. | PASS — no language target at top |
| 174 | `qs gem faraday` | RubyGems page for faraday | 5 | Exact RubyGems package first. | PASS — no language target at top |
| 175 | `qs hex bandit` | Hex package page for bandit | 5 | Exact Hex package first. | PASS — no language target at top |
| 176 | `qs maven org.flywaydb flyway-core` | Maven registry result for org.flywaydb:flyway-core | 0 | Fresh lookup failed with a Maven Central network read timeout; no result. | PASS — no language target at top |
| 177 | `qs nuget Polly` | NuGet page for Polly | 4 | Exact Polly package data first, but its result URL is GitHub rather than the NuGet page. | PASS — no language target at top |
| 178 | `qs wiki Ada Lovelace` | Wikipedia page for Ada Lovelace | 5 | Exact Wikipedia page first. | PASS — no language target at top |
| 179 | `qs mdn AbortSignal.any` | MDN page for AbortSignal.any | 5 | Exact MDN method first. | PASS — no language target at top |
| 180 | `qs so git rerere` | Stack Overflow search for git rerere | 5 | Highly relevant Stack Overflow result first. | PASS — no language target at top |
| 181 | `qs gh sharkdp/bat` | GitHub repository sharkdp/bat | 5 | Exact repository first. | PASS — no language target at top |
| 182 | `qs debian unattended upgrades` | Debian Wiki result for unattended upgrades | 3 | Correct Debian Wiki full-search target, but no concrete page/result detail. | PASS — no language target at top |
| 183 | `qs ubuntu netplan policy routing` | Ubuntu Wiki result for netplan policy routing | 2 | Ubuntu Wiki lookup returned HTTP 404; only a site-search fallback remains. | PASS — no language target at top |
| 184 | `qs fedora toolbox` | Fedora Wiki result for toolbox | 4 | A Fedora Wiki Toolbox result is first, but it is the RISC-V-specific page. | PASS — no language target at top |
| 185 | `qs gentoo eselect repository` | Gentoo Wiki result for eselect repository | 4 | Exact English Gentoo page is rank 3 after a translation and redirect. | PASS — no language target at top |
| 186 | `qs arch mkinitcpio unified kernel image` | ArchWiki result for unified kernel image | 2 | No matching ArchWiki page; only the site search fallback. | PASS — no language target at top |
| 187 | `qs man 5 resolv.conf` | Local resolv.conf manual section 5 | 5 | Exact local section-5 manual content. | PASS — no language target at top |
| 188 | `qs cargo metadata` | cargo-metadata manual page | 5 | Exact local cargo-metadata manual content. | PASS — no language target at top |
| 189 | `qs podman kube play` | podman-kube-play manual page | 5 | Exact local podman-kube-play manual content. | PASS — no language target at top |
| 190 | `qs dnf ripgrep` | DNF package information for ripgrep | 5 | Exact DNF package information. | PASS — no language target at top |
| 191 | `qs apt policy` | apt-policy manual page | 3 | No local apt-policy manual; fallback is the broader apt(8) page. | PASS — no language target at top |
| 192 | `qs brew leaves` | brew-leaves manual page | 3 | No local brew-leaves manual; fallback is the generic Homebrew manpage. | PASS — no language target at top |
| 193 | `qs flatpak remotes` | flatpak-remotes manual page | 5 | Exact local flatpak-remotes manual content. | PASS — no language target at top |
| 194 | `qs snap connections` | snap-connections manual page | 3 | No local snap-connections manual; fallback is the generic command reference. | PASS — no language target at top |
| 195 | `qs web rust crate serde` | DuckDuckGo results for rust crate serde | 5 | Correct web results; serde docs are first and crates.io is rank 3. | PASS — no language target at top |
| 196 | `qs !aur paru-bin` | DuckDuckGo bang result for !aur paru-bin | 3 | Correct !aur DuckDuckGo URL, but no redirect/live result was available. | PASS — no language target at top |
| 197 | `qs cheat tar extract strip components` | cheat.sh result for tar extraction | 2 | No exact cheat sheet; only a related-sheet search fallback. | PASS — no language target at top |
| 198 | `qs rfc structured field values HTTP` | RFC title search for Structured Field Values for HTTP | 2 | No RFC match returned; only an IETF document-search fallback. | PASS — no language target at top |
| 199 | `qs price of palladium per ounce` | DuckDuckGo web search for palladium price | 5 | Correct DuckDuckGo results with live palladium price pages first. | PASS — no language target at top |

## Summary

- Total: **205/250**
- Mean: **4.10/5**
- Score 5: **31/50 (62%)**
- Score ≥4: **36/50 (72%)**
- Score ≥3: **42/50 (84%)**
- Distribution: **5: 31, 4: 5, 3: 6, 2: 6, 1: 0, 0: 2**
- Contrast checks (entries 167–199): **33/33** kept the implied rejected language target out of the top position.
- Functional failure: `qs stock ASML` treated the equity ticker as a product query and returned no relevant result.
- Run error: the fresh Maven lookup for `org.flywaydb flyway-core` failed on a network read timeout.
- Most other weak scores were graceful fallbacks after local manuals or live web/wiki lookups were unavailable.
