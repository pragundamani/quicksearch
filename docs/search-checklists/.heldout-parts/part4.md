# Held-out manifest evaluation: entries 150–199

- Score distribution: **0: 20, 1: 1, 2: 0, 3: 4, 4: 0, 5: 25**
- Average: **2.76/5**
- Score >=3: **29/50 (58%)**
- Score >=4: **25/50 (50%)**
- Process errors: **18**
- 20-second command timeouts: **0**

| Global index | Command | Class | Label | Score | Observed top result/error | Candid assessment |
|---:|---|---|---|---:|---|---|
| 150 | `qs rfc 8949` | source-routing | positive | 5 | RFC 8949 — “Concise Binary Object Representation (CBOR)” | Exact RFC is first with title and summary. |
| 151 | `qs haskell System.Mem.performGC` | source-routing | positive | 5 | performGC :: IO () (System.Mem) | Exact Hoogle symbol is first with signature and module. |
| 152 | `qs gh python-trio/trio` | source-routing | positive | 5 | python-trio/trio — GitHub repository | Exact repository is first and immediately useful. |
| 153 | `qs man gofmt` | source-routing | positive | 0 | Error: no manual entries for gofmt | The intended local manual lookup failed. |
| 154 | `qs crate zerocopy` | source-routing | positive | 3 | zerocopy on docs.rs; crates.io search URL in header | Exact crate is first, but the result links to docs.rs rather than the requested crates.io package page. |
| 155 | `qs web cppreference execution control library` | source-routing | positive | 0 | Error: no web results | The explicit web route was selected, but it returned an error. |
| 156 | `qs so c strict aliasing effective type` | source-routing | positive | 5 | “Does C strict aliasing make untyped static memory pools impossible?” | Correct Stack Overflow route; the first result directly addresses strict aliasing. |
| 157 | `qs web haskell backpack signatures` | source-routing | positive | 0 | Error: no web results | The explicit web route was selected, but it returned an error. |
| 158 | `qs pip orjson` | source-routing | positive | 0 | Error: no PyPI packages matched orjson | The correct package exists, so the no-match error is a hard failure. |
| 159 | `qs gh golang/tools` | source-routing | positive | 5 | golang/tools — GitHub repository | Exact repository is first and immediately useful. |
| 160 | `qs std std::hint::black_box` | source-routing | positive | 5 | black_box in std::hint — Rust | Exact standard-library item is first with its official page. |
| 161 | `qs man clang-format` | source-routing | positive | 0 | Error: no manual entries for clang-format | The intended local manual lookup failed. |
| 162 | `qs wiki C standard library` | source-routing | positive | 5 | C standard library — Wikipedia | Exact Wikipedia article is first with useful context. |
| 163 | `qs so haskell quantified constraints` | source-routing | positive | 3 | “Bifunctors in Haskell vs in category theory” | The Stack Overflow route is correct, but the first results do not directly answer quantified constraints. |
| 164 | `qs web python free threading status` | source-routing | positive | 0 | Error: no web results | The explicit web route was selected, but it returned an error. |
| 165 | `qs rfc QUIC transport` | source-routing | positive | 3 | “HTTP/2 Semantics Using The QUIC Transport Protocol” (draft) | RFC title search is used, but only related drafts appear; RFC 9000 is absent. |
| 166 | `qs docs alloc::alloc::Layout` | source-routing | positive | 5 | Layout in alloc::alloc — Rust | Exact official alloc item is first with a useful summary. |
| 167 | `qs 9780143127741` | non-language-routing-control | negative | 5 | The Body Keeps the Score (ISBN 9780143127741) | The numeric input is safely treated as a product code and returns the matching book. |
| 168 | `qs 1912` | non-language-routing-control | negative | 5 | 1912 — DuckDuckGo instant answer | The year is correctly kept as a general web query rather than misrouted. |
| 169 | `qs item 012345678905` | non-language-routing-control | negative | 5 | Sphère plaisir (barcode 012345678905) | The explicit item route returns the exact barcode product. |
| 170 | `qs stock ASML` | non-language-routing-control | negative | 0 | Error: Open Food Facts request timed out | “stock” is misrouted to a product database and then errors; no finance result is returned. |
| 171 | `qs cve CVE-2024-3094` | non-language-routing-control | negative | 5 | CVE-2024-3094 — NVD | Exact NVD advisory is first with severity and description. |
| 172 | `qs nvd libwebp heap overflow` | non-language-routing-control | negative | 5 | CVE-2018-25009 — libwebp heap overflow | Correct NVD search with directly relevant libwebp overflow advisories. |
| 173 | `qs npm fast-glob` | non-language-routing-control | negative | 5 | fast-glob — npm | Exact npm package is first with description and version. |
| 174 | `qs gem faraday` | non-language-routing-control | negative | 5 | faraday — RubyGems | Exact RubyGem is first with description and version. |
| 175 | `qs hex bandit` | non-language-routing-control | negative | 5 | bandit — Hex | Exact Hex package is first with a useful description. |
| 176 | `qs maven org.flywaydb flyway-core` | non-language-routing-control | negative | 0 | Error: Maven Central request timed out | The Maven route is attempted, but the request errors. |
| 177 | `qs nuget Polly` | non-language-routing-control | negative | 3 | Polly metadata with github.com/App-vNext/Polly URL | Exact package metadata is first, but its URL is GitHub rather than the requested NuGet page. |
| 178 | `qs wiki Ada Lovelace` | non-language-routing-control | negative | 5 | Ada Lovelace — Wikipedia | Exact Wikipedia article is first with useful context. |
| 179 | `qs mdn AbortSignal.any` | non-language-routing-control | negative | 5 | AbortSignal: any() static method — MDN | Exact MDN method page is first with its behavior summarized. |
| 180 | `qs so git rerere` | non-language-routing-control | negative | 5 | “Are there any downsides to enabling git rerere?” | Correct Stack Overflow route with a directly useful first result. |
| 181 | `qs gh sharkdp/bat` | non-language-routing-control | negative | 5 | sharkdp/bat — GitHub repository | Exact repository is first and immediately useful. |
| 182 | `qs debian unattended upgrades` | non-language-routing-control | negative | 1 | RecentChanges — Debian Wiki | The Debian route is correct, but returned entries are generic and not about unattended upgrades. |
| 183 | `qs ubuntu netplan policy routing` | non-language-routing-control | negative | 0 | Error: Ubuntu Wiki returned HTTP 404 | The intended Ubuntu Wiki lookup ends in an HTTP error. |
| 184 | `qs fedora toolbox` | non-language-routing-control | negative | 5 | Architectures/RISC-V/Toolbox — Fedora Wiki | The Fedora Wiki route is preserved and returns a relevant Toolbox guide first. |
| 185 | `qs gentoo eselect repository` | non-language-routing-control | negative | 5 | Eselect/Repository/ja — Gentoo Wiki | The Gentoo Wiki route is preserved and returns a directly relevant Eselect/Repository page first. |
| 186 | `qs arch mkinitcpio unified kernel image` | non-language-routing-control | negative | 0 | Error: no Arch Wiki pages matched | The intended ArchWiki lookup returned no result. |
| 187 | `qs man 5 resolv.conf` | non-language-routing-control | negative | 0 | Error: no manual entries for “5 resolv.conf” | Section syntax is not handled; the lookup searches the literal combined name and errors. |
| 188 | `qs cargo metadata` | non-language-routing-control | negative | 5 | cargo-metadata(1) manual page | Exact cargo-metadata manual page is returned with synopsis and description. |
| 189 | `qs podman kube play` | non-language-routing-control | negative | 0 | Error: podman has no package info for play | The intended podman-kube-play manual lookup is misparsed and errors. |
| 190 | `qs dnf ripgrep` | non-language-routing-control | negative | 5 | ripgrep package information from DNF | Exact DNF package information is returned with version and description. |
| 191 | `qs apt policy` | non-language-routing-control | negative | 0 | Error: apt executable not found | The intended apt-policy manual lookup fails because apt is unavailable. |
| 192 | `qs brew leaves` | non-language-routing-control | negative | 0 | Homebrew formula “leaves” (disk-usage tool) | Exact trap: it returns an unrelated Homebrew package named leaves, not the brew-leaves manual page. |
| 193 | `qs flatpak remotes` | non-language-routing-control | negative | 5 | flatpak-remotes(1) manual page | Exact manual page is returned with synopsis and description. |
| 194 | `qs snap connections` | non-language-routing-control | negative | 0 | Error: snap has no package info for connections | The intended snap-connections manual lookup is misparsed and errors. |
| 195 | `qs web rust crate serde` | non-language-routing-control | negative | 0 | Error: no web results | The safe explicit web route is selected, but it returns an error. |
| 196 | `qs !aur paru-bin` | non-language-routing-control | negative | 0 | Error: no DuckDuckGo redirect for !aur | The bang is preserved, but redirect resolution fails, so no result is returned. |
| 197 | `qs cheat tar extract strip components` | non-language-routing-control | negative | 0 | cheat.sh 404 NOT FOUND page | The correct service is contacted, but the only output is an embedded 404 page. |
| 198 | `qs rfc structured field values HTTP` | non-language-routing-control | negative | 0 | Error: no RFCs matched | The RFC route is selected, but it returns no matching RFC. |
| 199 | `qs price of palladium per ounce` | non-language-routing-control | negative | 0 | Error: no web results | The general web route is selected, but it returns an error. |
