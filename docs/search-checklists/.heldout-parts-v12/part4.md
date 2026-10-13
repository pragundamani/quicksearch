# Held-out v12 candidate evaluation: manifest indexes 150–199

**Binary:** `/tmp/qs-gate/release/qs`  
**SHA256:** `f1988ac17d46ea8bf5ab53e3e0dd173aee946ebc7aad4f44e33eecbc8ab987c7`  
**Run:** Fresh process per query with `--json`; 20-second timeout.  
**Rubric:** 5 canonical/immediately useful first; 4 expected target top 3/useful; 3 relevant but weak; 2 partial; 1 misleading; 0 error/no relevant result.

## Summary

- Total: **223/250**
- Average: **4.46/5**
- Score ≥3: **45/50 (90%)**
- Score ≥4: **44/50 (88%)**
- Errors: **0**
- Timeouts: **0**
- Distribution: **0: 0, 1: 0, 2: 5, 3: 1, 4: 10, 5: 34**

## Scores 0–3, errors, and timeouts

- **Index 155, score 2:** `qs web cppreference execution control library` — The exact DuckDuckGo query URL is returned, but no search results are surfaced in the JSON.
- **Index 157, score 2:** `qs web haskell backpack signatures` — The exact DuckDuckGo query URL is returned, but no Backpack search results are surfaced in the JSON.
- **Index 164, score 2:** `qs web python free threading status` — The exact DuckDuckGo query URL is returned, but no free-threading status results are surfaced in the JSON.
- **Index 168, score 2:** `qs 1912` — The exact DuckDuckGo query URL is returned, but no search results for 1912 are surfaced in the JSON.
- **Index 183, score 3:** `qs ubuntu netplan policy routing` — Official Netplan documentation and a policy-routing site search are useful, but no Ubuntu Wiki target is returned.
- **Index 195, score 2:** `qs web rust crate serde` — The exact DuckDuckGo query URL is returned, but no Serde search results are surfaced in the JSON.

## Results

| Index | Class | Query | Expected | Top results | Score | Assessment |
|---:|---|---|---|---|---:|---|
| 150 | source-routing | `qs rfc 8949` | `RFC 8949` | 1. Concise Binary Object Representation (CBOR) | **5** | RFC 8949 is first, with the canonical RFC Editor page. |
| 151 | source-routing | `qs haskell System.Mem.performGC` | `Hoogle results for System.Mem.performGC` | 1. performGC :: IO () | **5** | The exact performGC API and Hackage anchor are first and immediately useful. |
| 152 | source-routing | `qs gh python-trio/trio` | `GitHub repository python-trio/trio` | 1. python-trio/trio<br>2. python-trio/trustme<br>3. florimondmanca/aiometer<br>4. python-trio/purerpc<br>5. libfuse/pyfuse3 | **5** | The requested python-trio/trio repository is first. |
| 153 | source-routing | `qs man gofmt` | `Local gofmt manual page` | 1. gofmt command documentation | **4** | The local manual is unavailable, but the first fallback is official gofmt command documentation. |
| 154 | source-routing | `qs crate zerocopy` | `crates.io page for zerocopy` | 1. zerocopy<br>2. musli-zerocopy<br>3. ix-schema-zerocopy<br>4. derive_aliases<br>5. derive_aliases_proc_macro | **5** | The exact zerocopy crate is first on docs.rs and is immediately useful, although the expected host was crates.io. |
| 155 | source-routing | `qs web cppreference execution control library` | `DuckDuckGo results for cppreference execution control library` | 1. Search DuckDuckGo for cppreference execution control library | **2** | The exact DuckDuckGo query URL is returned, but no search results are surfaced in the JSON. |
| 156 | source-routing | `qs so c strict aliasing effective type` | `Stack Overflow search for C strict aliasing effective type` | 1. Effective type rules with relation to strict aliasing<br>2. Strict aliasing rule in C<br>3. Does dereferencing void**-casted type** break strict aliasing?<br>4. Confused by effective type rules<br>5. What rules are there for qualifiers of effective type? | **5** | The Stack Overflow route returns several directly relevant strict-aliasing and effective-type questions. |
| 157 | source-routing | `qs web haskell backpack signatures` | `DuckDuckGo results for Haskell Backpack signatures` | 1. Search DuckDuckGo for haskell backpack signatures | **2** | The exact DuckDuckGo query URL is returned, but no Backpack search results are surfaced in the JSON. |
| 158 | source-routing | `qs pip orjson` | `PyPI package page for orjson` | 1. Search PyPI for orjson | **4** | The exact PyPI search is returned rather than the orjson project page; it remains useful with one more selection. |
| 159 | source-routing | `qs gh golang/tools` | `GitHub repository golang/tools` | 1. golang/tools<br>2. fortio/fortio<br>3. cirosantilli/china-dictatorship<br>4. qax-os/goreporter<br>5. dreddsa5dies/goHackTools | **5** | The requested golang/tools repository is first. |
| 160 | source-routing | `qs std std::hint::black_box` | `Official Rust standard-library page for std::hint::black_box` | 1. black_box in std::hint - Rust<br>2. Search Rust standard library for std::hint::black_box | **5** | The exact official std::hint::black_box page is first. |
| 161 | source-routing | `qs man clang-format` | `Local clang-format manual page` | 1. ClangFormat documentation | **4** | The local manual is unavailable, but official ClangFormat documentation is first and useful. |
| 162 | source-routing | `qs wiki C standard library` | `Wikipedia result for C standard library` | 1. C standard library<br>2. C++ Standard Library<br>3. Standard library<br>4. Standard Template Library<br>5. Boost (C++ libraries) | **5** | The exact Wikipedia article is first. |
| 163 | source-routing | `qs so haskell quantified constraints` | `Stack Overflow search for Haskell quantified constraints` | 1. Derive Ord with Quantified Constraints (forall a. Ord a => Ord (f a))<br>2. Quantified constraints vs. (closed) type families<br>3. Change in Behaviour of Quantified Constraints in GHC 9<br>4. Quantified Constraints for Higher-kinded Typeclasses<br>5. Using a quantified type equality constraint from the instance constraints | **5** | The first result directly addresses Haskell quantified constraints, and the remaining results are also tightly scoped. |
| 164 | source-routing | `qs web python free threading status` | `DuckDuckGo results for Python free threading status` | 1. Search DuckDuckGo for python free threading status | **2** | The exact DuckDuckGo query URL is returned, but no free-threading status results are surfaced in the JSON. |
| 165 | source-routing | `qs rfc QUIC transport` | `RFC title search for QUIC transport` | 1. QUIC: A UDP-Based Multiplexed and Secure Transport | **5** | The core QUIC transport specification, RFC 9000, is first and canonical. |
| 166 | source-routing | `qs docs alloc::alloc::Layout` | `Official Rust alloc docs for alloc::alloc::Layout` | 1. Layout in alloc::alloc - Rust<br>2. Search Rust standard library for alloc::alloc::Layout | **5** | The exact official alloc::alloc::Layout page is first. |
| 167 | non-language-routing-control | `qs 9780143127741` | `Product lookup for barcode 9780143127741` | 1. The Body Keeps the Score - by Bessel Van Der Kolk (Paperback) | **5** | The barcode resolves directly to the matching book product. |
| 168 | non-language-routing-control | `qs 1912` | `DuckDuckGo web search for 1912` | 1. Search DuckDuckGo for 1912 | **2** | The exact DuckDuckGo query URL is returned, but no search results for 1912 are surfaced in the JSON. |
| 169 | non-language-routing-control | `qs item 012345678905` | `Product lookup for barcode 012345678905` | 1. Sphère plaisir | **5** | The barcode resolves directly to a specific product. |
| 170 | non-language-routing-control | `qs stock ASML` | `Stock lookup for ASML` | 1. Search products for ASML<br>2. Possible ASML market quote | **4** | A direct ASML market-quote link is second and useful, but an unrelated product search is first. |
| 171 | non-language-routing-control | `qs cve CVE-2024-3094` | `NVD advisory for CVE-2024-3094` | 1. CVE-2024-3094 | **5** | The exact NVD advisory is first. |
| 172 | non-language-routing-control | `qs nvd libwebp heap overflow` | `NVD advisory search for libwebp heap overflow` | 1. CVE-2018-25009<br>2. CVE-2018-25010<br>3. CVE-2018-25011<br>4. CVE-2018-25012<br>5. CVE-2018-25013 | **5** | The first results are NVD advisories for libwebp heap-buffer overflows. |
| 173 | non-language-routing-control | `qs npm fast-glob` | `npm registry page for fast-glob` | 1. fast-glob<br>2. tinyglobby<br>3. @bluelovers/fast-glob<br>4. fdir<br>5. @neodx/glob | **5** | The exact npm package page is first. |
| 174 | non-language-routing-control | `qs gem faraday` | `RubyGems page for faraday` | 1. faraday<br>2. faraday_middleware<br>3. faraday-retry<br>4. faraday-multipart<br>5. faraday-excon | **5** | The exact RubyGems package page is first. |
| 175 | non-language-routing-control | `qs hex bandit` | `Hex package page for bandit` | 1. bandit<br>2. opentelemetry_bandit<br>3. lasso<br>4. francis<br>5. vancouver | **5** | The exact Hex package page is first. |
| 176 | non-language-routing-control | `qs maven org.flywaydb flyway-core` | `Maven registry result for org.flywaydb:flyway-core` | 1. org.flywaydb:flyway-core | **5** | The exact Maven artifact page is first. |
| 177 | non-language-routing-control | `qs nuget Polly` | `NuGet page for Polly` | 1. Polly<br>2. Polly.Core<br>3. AWSSDK.Polly<br>4. Polly.Extensions.Http<br>5. Microsoft.Extensions.Http.Polly | **4** | Polly is first and useful, but the result links to GitHub rather than the expected NuGet page. |
| 178 | non-language-routing-control | `qs wiki Ada Lovelace` | `Wikipedia page for Ada Lovelace` | 1. Ada Lovelace<br>2. Ada Lovelace (microarchitecture)<br>3. Ada Lovelace Day<br>4. Ada Lovelace Award<br>5. Portrait of Ada Lovelace | **5** | The exact Wikipedia article is first. |
| 179 | non-language-routing-control | `qs mdn AbortSignal.any` | `MDN page for AbortSignal.any` | 1. AbortSignal: any() static method<br>2. AbortSignal<br>3. AbortSignal: abort event<br>4. AbortSignal: abort() static method<br>5. TaskSignal: any() static method | **5** | The exact MDN API page is first. |
| 180 | non-language-routing-control | `qs so git rerere` | `Stack Overflow search for git rerere` | 1. Are there any downsides to enabling git rerere?<br>2. What is git-rerere and how does it work?<br>3. Have git rerere automatically mark files as resolved?<br>4. Git rerere and resolution covering non-conflicting files<br>5. Can git rerere be applied retroactively? | **5** | The first Stack Overflow result directly addresses git rerere, with an explanatory result second. |
| 181 | non-language-routing-control | `qs gh sharkdp/bat` | `GitHub repository sharkdp/bat` | 1. sharkdp/bat<br>2. victor-gp/cmd-help-sublime-syntax<br>3. ttys3/syntect-assets<br>4. badloop/bat-rose-pine<br>5. teleivo/bat-dogrun | **5** | The exact GitHub repository is first. |
| 182 | non-language-routing-control | `qs debian unattended upgrades` | `Debian Wiki result for unattended upgrades` | 1. Unattended upgrades | **5** | The exact Debian Wiki UnattendedUpgrades article is first. |
| 183 | non-language-routing-control | `qs ubuntu netplan policy routing` | `Ubuntu Wiki result for netplan policy routing` | 1. Netplan documentation<br>2. Search Netplan documentation for policy routing | **3** | Official Netplan documentation and a policy-routing site search are useful, but no Ubuntu Wiki target is returned. |
| 184 | non-language-routing-control | `qs fedora toolbox` | `Fedora Wiki result for toolbox` | 1. Toolbox | **5** | The canonical Fedora Toolbox documentation is first and immediately useful. |
| 185 | non-language-routing-control | `qs gentoo eselect repository` | `Gentoo Wiki result for eselect repository` | 1. Eselect/Repository/ru<br>2. Eselect-repository<br>3. Eselect/Repository<br>4. Eselect/Repository/ta<br>5. Eselect/Repository/hu | **4** | The canonical Eselect/Repository page is third, after a localized page and a redirect. |
| 186 | non-language-routing-control | `qs arch mkinitcpio unified kernel image` | `ArchWiki result for unified kernel image` | 1. Mkinitcpio<br>2. Mkinitcpio (Magyar)<br>3. Mkinitcpio (Italiano)<br>4. Mkinitcpio-btrfs<br>5. Mkinitcpio (Dansk) | **4** | The canonical Mkinitcpio ArchWiki page is first and relevant, though it does not directly identify a unified-kernel-image section. |
| 187 | non-language-routing-control | `qs man 5 resolv.conf` | `Local resolv.conf manual section 5` | 1. resolv.conf (5) | **5** | The exact local resolv.conf section 5 manual page is first. |
| 188 | non-language-routing-control | `qs cargo metadata` | `cargo-metadata manual page` | 1. cargo-metadata (1) | **5** | The exact local cargo-metadata manual page is first. |
| 189 | non-language-routing-control | `qs podman kube play` | `podman-kube-play manual page` | 1. podman-kube-play (1) | **5** | The exact local podman-kube-play manual page is first. |
| 190 | non-language-routing-control | `qs dnf ripgrep` | `DNF package information for ripgrep` | None | **5** | The response body contains complete local DNF package information for ripgrep. |
| 191 | non-language-routing-control | `qs apt policy` | `apt-policy manual page` | 1. apt-policy | **4** | The exact apt-cache policy section in the remote Debian manual is first; no local manual was available. |
| 192 | non-language-routing-control | `qs brew leaves` | `brew-leaves manual page` | 1. brew-leaves | **4** | The Homebrew manpage is useful, but it is a general page rather than a direct brew-leaves anchor. |
| 193 | non-language-routing-control | `qs flatpak remotes` | `flatpak-remotes manual page` | 1. flatpak-remotes (1) | **5** | The exact local flatpak-remotes manual page is first. |
| 194 | non-language-routing-control | `qs snap connections` | `snap-connections manual page` | 1. snap-connections | **4** | The Snap command reference is useful, but it is not a direct snap-connections manual page. |
| 195 | non-language-routing-control | `qs web rust crate serde` | `DuckDuckGo results for rust crate serde` | 1. Search DuckDuckGo for rust crate serde | **2** | The exact DuckDuckGo query URL is returned, but no Serde search results are surfaced in the JSON. |
| 196 | non-language-routing-control | `qs !aur paru-bin` | `DuckDuckGo bang result for !aur paru-bin` | 1. Search AUR packages for paru-bin | **5** | The bang routes directly to the exact AUR package search and is immediately useful. |
| 197 | non-language-routing-control | `qs cheat tar extract strip components` | `cheat.sh result for tar extraction` | 1. tar manual: tar extract strip components | **5** | The first result directly explains `--strip-components=NUMBER` for tar extraction. |
| 198 | non-language-routing-control | `qs rfc structured field values HTTP` | `RFC title search for Structured Field Values for HTTP` | 1. Structured Field Values for HTTP<br>2. Structured Field Values for HTTP<br>3. Structured Field Values for HTTP<br>4. Structured Field Values for HTTP | **5** | The exact current Structured Field Values RFC is first, with the prior RFC second. |
| 199 | non-language-routing-control | `qs price of palladium per ounce` | `DuckDuckGo web search for palladium price` | 1. Palladium futures quotes — CME Group<br>2. Search the web for price of palladium per ounce | **5** | The official CME palladium futures quote page is first, with the exact web search as a fallback. |
