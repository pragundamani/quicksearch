# Held-out evaluation: part 3 (manifest entries 100–149)

- Candidate: `/tmp/qs-gate/release/qs`
- Manifest: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/heldout_manifest.json`
- Scope: zero-based entries 100 through 149 (50 commands)
- Execution: replaced only the leading `qs` with the candidate path; per-command timeout was 30 seconds.
- Labels: 50 positive, 0 negative contrast. Therefore, no rejected-target/top-result checks apply in this slice.
- Scoring: 5 = expected first/immediately useful; 4 = expected top 3/useful; 3 = relevant but weak ranking/detail; 2 = partial; 1 = misleading/barely useful; 0 = error/no relevant result.

## Summary

| Metric | Result |
|---|---:|
| Average | 4.50 / 5 |
| Score >= 3 | 46/50 (92%) |
| Score >= 4 | 44/50 (88%) |
| Errors (nonzero exit) | 0 |
| Timeouts | 0 |

### Score distribution

| Score | Count |
|---:|---:|
| 5 | 38 |
| 4 | 6 |
| 3 | 2 |
| 2 | 2 |
| 1 | 1 |
| 0 | 1 |

## Per-entry results

| Entry | Query | Expected | Class | Exit / time | Score | Evaluation |
|---:|---|---|---|---:|---:|---|
| 100 | `qs c reference standard code example quick answer feclearexcept floating environment` | feclearexcept | `surplus-token-adversarial` | 0 / 0.426s | **2** | Exact `feclearexcept` is absent; results are only related floating-point-environment pages. |
| 101 | `qs haskell Data.IntMap.lookupGE` | Data.IntMap.lookupGE | `unseen-api` | 0 / 0.264s | **5** | Exact API is result 1 with signature and examples. |
| 102 | `qs python mmap.mmap.madvise` | mmap.mmap.madvise() | `unseen-api` | 0 / 0.326s | **5** | Exact API is result 1 with useful behavior details. |
| 103 | `qs go net/netip.ParsePrefix` | net/netip.ParsePrefix | `unseen-api` | 0 / 0.791s | **4** | Exact API is result 3; two related functions rank above it. |
| 104 | `qs rust std::sync::LazyLock::force_mut` | LazyLock::force_mut | `unseen-api` | 0 / 0.506s | **5** | Exact method is result 1. |
| 105 | `qs cpp std::ranges::cartesian_product_view` | std::ranges::cartesian_product_view | `unseen-api` | 0 / 0.235s | **5** | Exact type is result 1 with example. |
| 106 | `qs c fgetpos` | fgetpos | `unseen-api` | 0 / 0.072s | **5** | Exact function is result 1 with example. |
| 107 | `qs haskell Data.Array.accumArray` | Data.Array.accumArray | `unseen-api` | 0 / 0.202s | **5** | Exact API is result 1 with signature and explanation. |
| 108 | `qs python os.posix_spawnp` | os.posix_spawnp() | `unseen-api` | 0 / 0.348s | **5** | Exact API is result 1. |
| 109 | `qs go slices.Chunk` | slices.Chunk | `unseen-api` | 0 / 0.331s | **5** | Exact API is result 1 with signature and example. |
| 110 | `qs rust std::io::BorrowedBuf` | std::io::BorrowedBuf | `unseen-api` | 0 / 0.137s | **5** | Exact type is result 1. |
| 111 | `qs cpp std::flat_map` | std::flat_map | `unseen-api` | 0 / 0.072s | **5** | Exact type is result 1 with declaration and example. |
| 112 | `qs c wcscoll` | wcscoll | `unseen-api` | 0 / 0.018s | **5** | Exact function is result 1 with example. |
| 113 | `qs haskell Data.Graph.stronglyConnCompR` | Data.Graph.stronglyConnCompR | `unseen-api` | 0 / 0.173s | **5** | Exact API is result 1 with signature and explanation. |
| 114 | `qs python ssl.SSLContext.num_tickets` | SSLContext.num_tickets | `unseen-api` | 0 / 0.245s | **5** | Exact attribute is result 1 with useful description. |
| 115 | `qs go reflect.TypeFor` | reflect.TypeFor | `unseen-api` | 0 / 0.015s | **0** | Only result is unrelated `github.com/kbirk/di`; `reflect.TypeFor` is not returned. |
| 116 | `qs rust std::task::Waker::data` | Waker::data | `unseen-api` | 0 / 0.419s | **5** | Exact method is result 1. |
| 117 | `qs cpp std::ranges::fold_left_with_iter` | std::ranges::fold_left_with_iter | `unseen-api` | 0 / 0.134s | **5** | Exact algorithm is result 1 with explanation and example. |
| 118 | `qs c quick_exit` | quick_exit | `unseen-api` | 0 / 0.020s | **5** | Exact function is result 1; related `at_quick_exit` follows. |
| 119 | `qs haskell Text.ParserCombinators.ReadP.gather` | ReadP.gather | `unseen-api` | 0 / 0.265s | **5** | Exact API is result 1 with signature and caveat. |
| 120 | `qs python tarfile.data_filter` | tarfile.data_filter | `unseen-api` | 0 / 0.185s | **5** | Exact API is result 1. |
| 121 | `qs go archive/zip.Writer.Copy` | archive/zip.Writer.Copy | `unseen-api` | 0 / 0.522s | **4** | Exact method is result 3; unrelated/adjacent entries rank above it. |
| 122 | `qs rust std::thread::scope` | std::thread::scope | `unseen-api` | 0 / 0.128s | **5** | Exact function is result 1. |
| 123 | `qs cpp std::expected::transform_error` | std::expected::transform_error | `unseen-api` | 0 / 0.123s | **5** | Exact method is result 1 with declarations. |
| 124 | `qs c thrd_sleep` | thrd_sleep | `unseen-api` | 0 / 0.018s | **5** | Exact function is result 1 with example. |
| 125 | `qs haskell Data.Typeable.eqT` | Data.Typeable.eqT | `unseen-api` | 0 / 0.208s | **1** | Returns only a generic Haskell 2010 Report link, not `Data.Typeable.eqT` documentation. |
| 126 | `qs python sqlite3.Connection.blobopen` | Connection.blobopen() | `unseen-api` | 0 / 0.212s | **5** | Exact method is result 1, though the snippet has no detail. |
| 127 | `qs go crypto/ecdh.X25519` | crypto/ecdh.X25519 | `unseen-api` | 0 / 0.381s | **3** | The `crypto/ecdh` package is result 1, but no exact `X25519` result or signature appears. |
| 128 | `qs rust std::net::Ipv6Addr::to_ipv4_mapped` | Ipv6Addr::to_ipv4_mapped | `unseen-api` | 0 / 0.351s | **5** | Exact method is result 1. |
| 129 | `qs cpp std::generator` | std::generator | `unseen-api` | 0 / 0.064s | **5** | Exact type is result 1 with a useful example. |
| 130 | `qs c ckd_add` | ckd_add | `unseen-api` | 0 / 0.021s | **5** | Exact function is result 1 with example. |
| 131 | `qs haskell Data.Foldable.foldMap1` | Data.Foldable.foldMap1 | `unseen-api` | 0 / 0.162s | **5** | Exact function is result 1 with signature and example. |
| 132 | `qs python asyncio.Barrier` | asyncio.Barrier | `unseen-api` | 0 / 0.116s | **5** | Exact type is result 1 with explanation and example. |
| 133 | `qs go math/big.Float.Scan` | math/big.Float.Scan | `unseen-api` | 0 / 0.372s | **5** | Exact receiver method is result 1; fully qualified duplicate appears at result 3. |
| 134 | `qs pypi msgspec` | PyPI package page for msgspec | `source-routing` | 0 / 0.168s | **5** | Exact PyPI project is result 1 with summary and version. |
| 135 | `qs go-mod golang.org/x/sync` | Go package registry result for golang.org/x/sync | `source-routing` | 0 / 0.392s | **5** | Exact Go module is result 1. |
| 136 | `qs crate camino` | crates.io page for camino | `source-routing` | 0 / 0.151s | **4** | Exact crate is result 1 and useful, but link is docs.rs rather than the expected crates.io page. |
| 137 | `qs man cmake` | Local cmake manual page | `source-routing` | 0 / 0.201s | **5** | Local `cmake(1)` manual opens directly with synopsis and description. |
| 138 | `qs man 3 getaddrinfo` | Section 3 getaddrinfo manual page | `source-routing` | 0 / 0.084s | **5** | Correct section 3 manual opens directly with synopsis and description. |
| 139 | `qs web Hackage attoparsec` | DuckDuckGo results for Hackage attoparsec | `source-routing` | 0 / 0.152s | **4** | Correct DuckDuckGo search URL is result 1, but no fetched search results are shown. |
| 140 | `qs pypi platformdirs` | PyPI package page for platformdirs | `source-routing` | 0 / 0.142s | **5** | Exact PyPI project is result 1 with summary and version. |
| 141 | `qs gh charmbracelet/lipgloss` | GitHub repository charmbracelet/lipgloss | `source-routing` | 0 / 0.313s | **5** | Exact GitHub repository is result 1. |
| 142 | `qs docs smallvec` | docs.rs documentation for smallvec | `source-routing` | 0 / 0.136s | **5** | Exact docs.rs crate documentation is result 1. |
| 143 | `qs so c++ object slicing` | Stack Overflow search for C++ object slicing | `source-routing` | 0 / 0.216s | **5** | Canonical Stack Overflow question is result 1 with strong answer metadata. |
| 144 | `qs cheat awk field separator` | cheat.sh result for awk field separator | `source-routing` | 0 / 0.360s | **2** | No exact cheat.sh sheet; output only offers the same related-search URL. |
| 145 | `qs gh haskell-unordered-containers` | GitHub repositories for haskell-unordered-containers | `source-routing` | 0 / 0.319s | **3** | Repository search works, but the likely canonical project is absent and weakly related forks/packages lead. |
| 146 | `qs mdn python webassembly api` | MDN search for python webassembly api | `source-routing` | 0 / 0.090s | **4** | The useful WebAssembly JavaScript API page is result 2; generic Python ranks first. |
| 147 | `qs web go vulnerability database` | DuckDuckGo results for Go vulnerability database | `source-routing` | 0 / 0.161s | **4** | Correct DuckDuckGo search URL is result 1, but no fetched search results are shown. |
| 148 | `qs docs core::cell::OnceCell` | Official Rust core docs for core::cell::OnceCell | `source-routing` | 0 / 0.125s | **5** | Exact official core documentation is result 1. |
| 149 | `qs wiki C++ name mangling` | Wikipedia result for C++ name mangling | `source-routing` | 0 / 0.364s | **5** | Exact Wikipedia article is the immediate featured result and result 1. |

## Observations

- Most exact API and source-routed queries put the expected target first.
- Entry 115 has no relevant result and is the only score 0.
- Entry 125 returns a generic language report instead of the requested API.
- Entry 100 loses the exact API under surplus tokens.
- Entries 139 and 147 route to the correct DuckDuckGo searches but do not show fetched result listings.
- All commands exited successfully. No command reached the 30-second timeout.
