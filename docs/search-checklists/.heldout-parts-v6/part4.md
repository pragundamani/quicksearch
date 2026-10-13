# Held-out evaluation: part 4 (indexes 150–199)

- Candidate: `/tmp/qs-gate/release/qs`
- Entries: 50
- Mean score: **3.80 / 5**
- Command errors or timeouts: **0**

## Score distribution

| Score | Count |
|---:|---:|
| 5 | 29 |
| 4 | 5 |
| 3 | 3 |
| 2 | 3 |
| 1 | 10 |
| 0 | 0 |

## Failures and weak results

- **153 — score 1:** `qs man gofmt` No local gofmt manual was found. The only hit is an online manual-search placeholder with no manual content.
- **161 — score 1:** `qs man clang-format` No local clang-format manual was found. The only hit is an online manual-search placeholder with no manual content.
- **163 — score 2:** `qs so haskell quantified constraints` The results are Haskell type-system questions, but none visibly targets quantified constraints.
- **170 — score 1:** `qs stock ASML` The lookup reports no products matched ASML and offers only a generic product-search link.
- **182 — score 2:** `qs debian unattended upgrades` The only hit is a Debian Wiki full-search link with no page content or context.
- **183 — score 1:** `qs ubuntu netplan policy routing` The Ubuntu Wiki lookup failed with HTTP 404 and returned only a DuckDuckGo site-search placeholder.
- **184 — score 2:** `qs fedora toolbox` The only result is a RISC-V-specific toolbox guide, not a general Fedora Toolbox result.
- **186 — score 1:** `qs arch mkinitcpio unified kernel image` No ArchWiki page matched. The only hit is a generic ArchWiki search link.
- **195 — score 1:** `qs web rust crate serde` No DuckDuckGo results were returned. The only hit is a generic search link with no result context.
- **196 — score 1:** `qs !aur paru-bin` The bang redirect failed. The only hit is a generic DuckDuckGo search link with no AUR result.
- **197 — score 1:** `qs cheat tar extract strip components` The exact cheat.sh topic was unavailable, and the only hit is a related-search placeholder with no extraction guidance.
- **198 — score 1:** `qs rfc structured field values HTTP` No matching RFC was returned. The only hit is a generic IETF document-search link.
- **199 — score 1:** `qs price of palladium per ounce` No palladium price result was returned. The only hit is a generic DuckDuckGo search link with no price context.

## Other ranking or context issues

- **154 — score 4:** `qs crate zerocopy` The exact zerocopy crate is first with useful package context, but the hit links to docs.rs rather than its crates.io package page.
- **165 — score 4:** `qs rfc QUIC transport` The first result is a directly relevant QUIC transport RFC, but not the core QUIC transport specification.
- **177 — score 4:** `qs nuget Polly` The exact Polly package is first with useful context, but the hit links to GitHub rather than NuGet.
- **180 — score 4:** `qs so git rerere` The results are directly relevant; the clearest general explanation of git rerere is second.
- **185 — score 4:** `qs gentoo eselect repository` The English Eselect/Repository page is third; localized variants rank above it.
- **191 — score 3:** `qs apt policy` No local apt-policy manual was found, but the fallback links to the relevant remote apt manual.
- **192 — score 3:** `qs brew leaves` No local brew-leaves manual was found, but the fallback links to the Homebrew manpage.
- **194 — score 3:** `qs snap connections` No local snap-connections manual was found, but the fallback links to Snap command-reference documentation.
