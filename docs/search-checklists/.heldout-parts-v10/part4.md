# Held-out final-candidate evaluation: manifest indexes 150–199

**Binary:** `/tmp/qs-gate/release/qs`  
**SHA256:** `a048a86983a8877eea4240c5a24e4e9119e901ceaeff44bc8d13e15911f8cabd`  
**Run:** Fresh process per query with `--json`; 30-second timeout.  
**Rubric:** 5 expected first/immediately useful; 4 expected top 3/useful; 3 relevant but weak; 2 partial; 1 misleading; 0 error/no relevant result.

## Summary

- Total: **185/250**
- Average: **3.70/5**
- Score ≥3: **38/50 (76%)**
- Score ≥4: **35/50 (70%)**
- Errors: **7**
- Timeouts: **6**
- Distribution: **0: 7, 1: 1, 2: 4, 3: 3, 4: 8, 5: 27**

## Results

| Index | Class | Query | Expected | Top results | Score | Assessment |
|---:|---|---|---|---|---:|---|
| 150 | source-routing | `qs rfc 8949` | `RFC 8949` | 1. Concise Binary Object Representation (CBOR) | **5** | RFC 8949 is the first result, with the exact RFC Editor page and a useful summary. |
| 151 | source-routing | `qs haskell System.Mem.performGC` | `Hoogle results for System.Mem.performGC` | 1. performGC :: IO () | **5** | The first result is the exact performGC API and its Hackage documentation; it is immediately useful despite not being branded as Hoogle. |
| 152 | source-routing | `qs gh python-trio/trio` | `GitHub repository python-trio/trio` | 1. python-trio/trio<br>2. python-trio/trustme<br>3. florimondmanca/aiometer<br>4. python-trio/purerpc<br>5. libfuse/pyfuse3 | **5** | The requested python-trio/trio repository is the first result. |
| 153 | source-routing | `qs man gofmt` | `Local gofmt manual page` | 1. gofmt command documentation | **4** | The local manual was unavailable, but the first fallback is the official gofmt command documentation and remains useful. |
| 154 | source-routing | `qs crate zerocopy` | `crates.io page for zerocopy` | 1. zerocopy<br>2. musli-zerocopy<br>3. ix-schema-zerocopy<br>4. derive_aliases<br>5. derive_aliases_proc_macro | **5** | The requested zerocopy crate is first and links to its immediately useful docs.rs page, although the expected page was crates.io. |
| 155 | source-routing | `qs web cppreference execution control library` | `DuckDuckGo results for cppreference execution control library` | — | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |
| 156 | source-routing | `qs so c strict aliasing effective type` | `Stack Overflow search for C strict aliasing effective type` | 1. Does C strict aliasing make untyped static memory pools impossible?<br>2. Strict aliasing of first struct member through opaque pointer in C<br>3. What is the effective type of an object written by memset?<br>4. Is strict aliasing one-way?<br>5. Strict aliasing and memory locations | **5** | The Stack Overflow route returns several directly relevant strict-aliasing and effective-type questions; the first result is useful. |
| 157 | source-routing | `qs web haskell backpack signatures` | `DuckDuckGo results for Haskell Backpack signatures` | — | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |
| 158 | source-routing | `qs pip orjson` | `PyPI package page for orjson` | 1. Search PyPI for orjson | **4** | The only hit is the exact PyPI search for orjson rather than the package page; it is useful but requires one more selection. |
| 159 | source-routing | `qs gh golang/tools` | `GitHub repository golang/tools` | 1. golang/tools<br>2. fortio/fortio<br>3. cirosantilli/china-dictatorship<br>4. qax-os/goreporter<br>5. dreddsa5dies/goHackTools | **5** | The requested golang/tools repository is the first result. |
| 160 | source-routing | `qs std std::hint::black_box` | `Official Rust standard-library page for std::hint::black_box` | 1. black_box in std::hint - Rust<br>2. Search Rust standard library for std::hint::black_box | **5** | The exact official std::hint::black_box page is the first result. |
| 161 | source-routing | `qs man clang-format` | `Local clang-format manual page` | 1. ClangFormat documentation | **4** | The local manual was unavailable, but the official ClangFormat documentation is first and useful. |
| 162 | source-routing | `qs wiki C standard library` | `Wikipedia result for C standard library` | 1. C standard library<br>2. C++ Standard Library<br>3. Standard library<br>4. Standard Template Library<br>5. Boost (C++ libraries) | **5** | The exact Wikipedia article is the first result. |
| 163 | source-routing | `qs so haskell quantified constraints` | `Stack Overflow search for Haskell quantified constraints` | 1. Bifunctors in Haskell vs in category theory<br>2. Type variable introduction for existential types<br>3. Deriving `Eq` for a data type based on a type family in Haskell<br>4. Free monad and type constraints<br>5. Existential types in Haskell and generics in other languages | **3** | The Stack Overflow route works, but the returned questions are only weakly related to quantified constraints. |
| 164 | source-routing | `qs web python free threading status` | `DuckDuckGo results for Python free threading status` | — | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |
| 165 | source-routing | `qs rfc QUIC transport` | `RFC title search for QUIC transport` | 1. Applicability of the QUIC Transport Protocol<br>2. HTTP/2 Semantics Using The QUIC Transport Protocol<br>3. Applicability and Management of the QUIC Transport Protocol<br>4. Applicability of the QUIC Transport Protocol<br>5. Manageability of the QUIC Transport Protocol | **3** | The RFC search returns QUIC transport documents, but it omits the core QUIC Transport RFC 9000 and starts with an applicability RFC. |
| 166 | source-routing | `qs docs alloc::alloc::Layout` | `Official Rust alloc docs for alloc::alloc::Layout` | 1. Layout in alloc::alloc - Rust<br>2. Search Rust standard library for alloc::alloc::Layout | **5** | The exact official alloc::alloc::Layout page is the first result. |
| 167 | non-language-routing-control | `qs 9780143127741` | `Product lookup for barcode 9780143127741` | 1. The Body Keeps the Score - by Bessel Van Der Kolk (Paperback) | **5** | The barcode resolves directly to a product page and named product. |
| 168 | non-language-routing-control | `qs 1912` | `DuckDuckGo web search for 1912` | 1. Search DuckDuckGo for 1912 | **0** | The command exceeded the 30-second timeout. It emitted only a generic DuckDuckGo fallback before termination, so the run is an error. |
| 169 | non-language-routing-control | `qs item 012345678905` | `Product lookup for barcode 012345678905` | 1. Sphère plaisir | **5** | The barcode resolves directly to a product page and named product. |
| 170 | non-language-routing-control | `qs stock ASML` | `Stock lookup for ASML` | 1. Search products for ASML | **1** | The stock query is misrouted to an Open Food Facts product search for ASML, with no financial result. |
| 171 | non-language-routing-control | `qs cve CVE-2024-3094` | `NVD advisory for CVE-2024-3094` | 1. CVE-2024-3094 | **5** | The exact NVD advisory is the first result. |
| 172 | non-language-routing-control | `qs nvd libwebp heap overflow` | `NVD advisory search for libwebp heap overflow` | 1. CVE-2018-25009<br>2. CVE-2018-25010<br>3. CVE-2018-25011<br>4. CVE-2018-25012<br>5. CVE-2018-25013 | **5** | The NVD search returns directly relevant libwebp heap-overflow advisories, led by a specific critical CVE. |
| 173 | non-language-routing-control | `qs npm fast-glob` | `npm registry page for fast-glob` | 1. fast-glob<br>2. tinyglobby<br>3. @bluelovers/fast-glob<br>4. fdir<br>5. @neodx/glob | **5** | The exact npm package is the first result. |
| 174 | non-language-routing-control | `qs gem faraday` | `RubyGems page for faraday` | 1. faraday<br>2. faraday_middleware<br>3. faraday-retry<br>4. faraday-multipart<br>5. faraday-excon | **5** | The exact RubyGems package is the first result. |
| 175 | non-language-routing-control | `qs hex bandit` | `Hex package page for bandit` | 1. bandit<br>2. opentelemetry_bandit<br>3. lasso<br>4. francis<br>5. vancouver | **5** | The exact Hex package is the first result. |
| 176 | non-language-routing-control | `qs maven org.flywaydb flyway-core` | `Maven registry result for org.flywaydb:flyway-core` | — | **0** | The Maven lookup failed with a network read timeout and returned non-JSON error output. |
| 177 | non-language-routing-control | `qs nuget Polly` | `NuGet page for Polly` | 1. Polly<br>2. Polly.Core<br>3. AWSSDK.Polly<br>4. Polly.Extensions.Http<br>5. Microsoft.Extensions.Http.Polly | **4** | Polly is the first NuGet search result with useful package details, but its URL points to GitHub rather than the NuGet package page. |
| 178 | non-language-routing-control | `qs wiki Ada Lovelace` | `Wikipedia page for Ada Lovelace` | 1. Ada Lovelace<br>2. Ada Lovelace (microarchitecture)<br>3. Ada Lovelace Day<br>4. Ada Lovelace Award<br>5. Portrait of Ada Lovelace | **5** | The exact Wikipedia article is the first result. |
| 179 | non-language-routing-control | `qs mdn AbortSignal.any` | `MDN page for AbortSignal.any` | 1. AbortSignal: any() static method<br>2. AbortSignal<br>3. AbortSignal: abort event<br>4. AbortSignal: abort() static method<br>5. TaskSignal: any() static method | **5** | The exact MDN AbortSignal.any() page is the first result. |
| 180 | non-language-routing-control | `qs so git rerere` | `Stack Overflow search for git rerere` | 1. Are there any downsides to enabling git rerere?<br>2. What is git-rerere and how does it work?<br>3. Have git rerere automatically mark files as resolved?<br>4. Git rerere and resolution covering non-conflicting files<br>5. Can git rerere be applied retroactively? | **5** | The Stack Overflow results begin with directly useful git rerere questions. |
| 181 | non-language-routing-control | `qs gh sharkdp/bat` | `GitHub repository sharkdp/bat` | 1. sharkdp/bat<br>2. victor-gp/cmd-help-sublime-syntax<br>3. ttys3/syntect-assets<br>4. badloop/bat-rose-pine<br>5. teleivo/bat-dogrun | **5** | The requested sharkdp/bat repository is the first result. |
| 182 | non-language-routing-control | `qs debian unattended upgrades` | `Debian Wiki result for unattended upgrades` | 1. unattended upgrades | **4** | The Debian Wiki search is correctly scoped and useful, but it returns only the search page rather than a specific article. |
| 183 | non-language-routing-control | `qs ubuntu netplan policy routing` | `Ubuntu Wiki result for netplan policy routing` | 1. Search Ubuntu Wiki for netplan policy routing | **2** | The Ubuntu Wiki lookup falls back to a site-scoped DuckDuckGo search after an HTTP 404 and provides no specific result. |
| 184 | non-language-routing-control | `qs fedora toolbox` | `Fedora Wiki result for toolbox` | 1. Architectures/RISC-V/Toolbox | **3** | The only result is a Fedora RISC-V Toolbox guide, which is relevant but too narrow for the general Fedora Toolbox query. |
| 185 | non-language-routing-control | `qs gentoo eselect repository` | `Gentoo Wiki result for eselect repository` | 1. Eselect/Repository<br>2. Eselect/Repository/ta<br>3. Eselect/Repository/hu<br>4. Eselect/Repository/en<br>5. Eselect/Repository/ja | **5** | The exact Gentoo Eselect/Repository page is the first result. |
| 186 | non-language-routing-control | `qs arch mkinitcpio unified kernel image` | `ArchWiki result for unified kernel image` | 1. Search Arch Wiki for mkinitcpio unified kernel image | **2** | The ArchWiki lookup returns only a search link and reports that no matching page was found. |
| 187 | non-language-routing-control | `qs man 5 resolv.conf` | `Local resolv.conf manual section 5` | 1. resolv.conf (5) | **5** | The exact local resolv.conf(5) manual entry is the first result. |
| 188 | non-language-routing-control | `qs cargo metadata` | `cargo-metadata manual page` | 1. cargo-metadata (1) | **5** | The exact local cargo-metadata(1) manual entry is the first result. |
| 189 | non-language-routing-control | `qs podman kube play` | `podman-kube-play manual page` | 1. podman-kube-play (1) | **5** | The exact local podman-kube-play(1) manual entry is the first result. |
| 190 | non-language-routing-control | `qs dnf ripgrep` | `DNF package information for ripgrep` | — | **5** | The command returns complete local DNF package information for ripgrep. |
| 191 | non-language-routing-control | `qs apt policy` | `apt-policy manual page` | 1. apt-policy | **2** | The requested local manual is absent, and the fallback incorrectly links the general apt(8) page rather than a policy-specific manual. |
| 192 | non-language-routing-control | `qs brew leaves` | `brew-leaves manual page` | 1. brew-leaves | **4** | The local manual is absent, but the Homebrew manual is a useful fallback for the leaves command. |
| 193 | non-language-routing-control | `qs flatpak remotes` | `flatpak-remotes manual page` | 1. flatpak-remotes (1) | **5** | The exact local flatpak-remotes(1) manual entry is the first result. |
| 194 | non-language-routing-control | `qs snap connections` | `snap-connections manual page` | 1. snap-connections | **4** | The local manual is absent, but the Snap command reference is a useful fallback for connections. |
| 195 | non-language-routing-control | `qs web rust crate serde` | `DuckDuckGo results for rust crate serde` | — | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |
| 196 | non-language-routing-control | `qs !aur paru-bin` | `DuckDuckGo bang result for !aur paru-bin` | 1. Search AUR packages for paru-bin | **4** | The AUR search for paru-bin is directly useful, although it is not the expected DuckDuckGo bang result or a direct package page. |
| 197 | non-language-routing-control | `qs cheat tar extract strip components` | `cheat.sh result for tar extraction` | 1. Search cheat.sh for tar extract strip components | **2** | No exact cheat sheet is returned; the only hit is a related cheat.sh search link, so the result is partial. |
| 198 | non-language-routing-control | `qs rfc structured field values HTTP` | `RFC title search for Structured Field Values for HTTP` | 1. Structured Field Values for HTTP<br>2. Structured Field Values for HTTP<br>3. Structured Field Values for HTTP<br>4. Structured Field Values for HTTP | **5** | The current Structured Field Values for HTTP RFC is the first result, followed by the earlier RFC. |
| 199 | non-language-routing-control | `qs price of palladium per ounce` | `DuckDuckGo web search for palladium price` | — | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |

## Failures and low-scoring cases

- **155 (timeout, score 0):** The command timed out after 30 seconds and returned no JSON or relevant result.
- **157 (timeout, score 0):** The command timed out after 30 seconds and returned no JSON or relevant result.
- **164 (timeout, score 0):** The command timed out after 30 seconds and returned no JSON or relevant result.
- **168 (timeout, score 0):** The command exceeded the 30-second timeout. It emitted only a generic DuckDuckGo fallback before termination, so the run is an error.
- **170 (score 1, score 1):** The stock query is misrouted to an Open Food Facts product search for ASML, with no financial result.
- **176 (exit 1, score 0):** The Maven lookup failed with a network read timeout and returned non-JSON error output.
- **183 (score 2, score 2):** The Ubuntu Wiki lookup falls back to a site-scoped DuckDuckGo search after an HTTP 404 and provides no specific result.
- **186 (score 2, score 2):** The ArchWiki lookup returns only a search link and reports that no matching page was found.
- **191 (score 2, score 2):** The requested local manual is absent, and the fallback incorrectly links the general apt(8) page rather than a policy-specific manual.
- **195 (timeout, score 0):** The command timed out after 30 seconds and returned no JSON or relevant result.
- **197 (score 2, score 2):** No exact cheat sheet is returned; the only hit is a related cheat.sh search link, so the result is partial.
- **199 (timeout, score 0):** The command timed out after 30 seconds and returned no JSON or relevant result.
