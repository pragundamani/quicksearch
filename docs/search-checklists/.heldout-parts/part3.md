# Held-out manifest evaluation, part 3

Entries are global zero-based indices 100–149 (`[100:150]`). Each manifest command was run separately. Only the leading `qs` was replaced with `/tmp/qs-gate/release/qs`. The timeout was 20 seconds per command.

## Summary

- Score distribution: **5: 38**, **4: 3**, **3: 0**, **2: 1**, **1: 0**, **0: 8**.
- Average: **4.08/5**.
- Score ≥3: **41/50 (82%)**.
- Score ≥4: **41/50 (82%)**.
- Errors/timeouts: **7 total** — 6 errors (five nonzero command errors plus one exit-0 cheat.sh `404 NOT FOUND`) and 1 timeout.

## Results

| Global index | Command | Class | Label | Score | Observed top result/error | Candid assessment |
|---:|---|---|---|---:|---|---|
| 100 | `qs c reference standard code example quick answer feclearexcept floating environment` | surplus-token-adversarial | positive | 4 | `Floating-point environment` is #1; `feclearexcept` is #3. | The target survives surplus tokens and is in the top three, but a broad topic outranks it. |
| 101 | `qs haskell Data.IntMap.lookupGE` | unseen-api | positive | 5 | `lookupGE` is #1 with the `Data.IntMap` signature and Hackage link. | The exact API target is ranked first with useful context. |
| 102 | `qs python mmap.mmap.madvise` | unseen-api | positive | 5 | `mmap.mmap.madvise()` is #1. | The exact API target is ranked first with useful context. |
| 103 | `qs go net/netip.ParsePrefix` | unseen-api | positive | 5 | `netip.ParsePrefix()` is #1. | The exact API target is ranked first with useful context. |
| 104 | `qs rust std::sync::LazyLock::force_mut` | unseen-api | positive | 5 | `LazyLock::force_mut` is #1. | The exact API target is ranked first with useful context. |
| 105 | `qs cpp std::ranges::cartesian_product_view` | unseen-api | positive | 5 | `std::ranges::cartesian_product_view` is #1. | The exact API target is ranked first with useful context. |
| 106 | `qs c fgetpos` | unseen-api | positive | 5 | `fgetpos` is #1. | The exact API target is ranked first with useful context. |
| 107 | `qs haskell Data.Array.accumArray` | unseen-api | positive | 5 | `accumArray` from `Data.Array` is #1. | The exact API target is ranked first with useful context. |
| 108 | `qs python os.posix_spawnp` | unseen-api | positive | 5 | `os.posix_spawnp()` is #1. | The exact API target is ranked first with useful context. |
| 109 | `qs go slices.Chunk` | unseen-api | positive | 5 | `slices.Chunk()` is #1. | The exact API target is ranked first with useful context. |
| 110 | `qs rust std::io::BorrowedBuf` | unseen-api | positive | 5 | `BorrowedBuf in std::io` is #1. | The exact API target is ranked first with useful context. |
| 111 | `qs cpp std::flat_map` | unseen-api | positive | 5 | `std::flat_map` is #1. | The exact API target is ranked first with useful context. |
| 112 | `qs c wcscoll` | unseen-api | positive | 5 | `wcscoll` is #1. | The exact API target is ranked first with useful context. |
| 113 | `qs haskell Data.Graph.stronglyConnCompR` | unseen-api | positive | 5 | `stronglyConnCompR` from `Data.Graph` is #1. | The exact API target is ranked first with useful context. |
| 114 | `qs python ssl.SSLContext.num_tickets` | unseen-api | positive | 5 | `ssl.SSLContext.num_tickets` is #1. | The exact API target is ranked first with useful context. |
| 115 | `qs go reflect.TypeFor` | unseen-api | positive | 5 | `reflect.TypeFor()` is #1. | The exact API target is ranked first with useful context. |
| 116 | `qs rust std::task::Waker::data` | unseen-api | positive | 5 | `Waker::data` is #1. | The exact API target is ranked first with useful context. |
| 117 | `qs cpp std::ranges::fold_left_with_iter` | unseen-api | positive | 5 | `std::ranges::fold_left_with_iter` is #1. | The exact API target is ranked first with useful context. |
| 118 | `qs c quick_exit` | unseen-api | positive | 5 | `quick_exit` is #1. | The exact API target is ranked first with useful context. |
| 119 | `qs haskell Text.ParserCombinators.ReadP.gather` | unseen-api | positive | 5 | `gather` from `Text.ParserCombinators.ReadP` is #1. | The exact API target is ranked first with useful context. |
| 120 | `qs python tarfile.data_filter` | unseen-api | positive | 5 | `tarfile.data_filter()` is #1. | The exact API target is ranked first with useful context. |
| 121 | `qs go archive/zip.Writer.Copy` | unseen-api | positive | 5 | `zip.Writer.Copy()` is #1. | The exact API target is ranked first with useful context. |
| 122 | `qs rust std::thread::scope` | unseen-api | positive | 5 | `scope in std::thread` is #1. | The exact API target is ranked first with useful context. |
| 123 | `qs cpp std::expected::transform_error` | unseen-api | positive | 5 | `std::expected::transform_error` is #1. | The exact API target is ranked first with useful context. |
| 124 | `qs c thrd_sleep` | unseen-api | positive | 5 | `thrd_sleep` is #1. | The exact API target is ranked first with useful context. |
| 125 | `qs haskell Data.Typeable.eqT` | unseen-api | positive | 0 | A generic Haskell 2010 report link is the only result; no `eqT` API result. | The fallback is related only to the language and does not answer the API query. |
| 126 | `qs python sqlite3.Connection.blobopen` | unseen-api | positive | 5 | `sqlite3.Connection.blobopen()` is #1. | The exact API target is ranked first with useful context. |
| 127 | `qs go crypto/ecdh.X25519` | unseen-api | positive | 4 | `crypto/ecdh` package page is #1; `ecdh.X25519()` is #2. | The exact function is useful and in the top three, but the package page outranks it. |
| 128 | `qs rust std::net::Ipv6Addr::to_ipv4_mapped` | unseen-api | positive | 5 | `Ipv6Addr::to_ipv4_mapped` is #1. | The exact API target is ranked first with useful context. |
| 129 | `qs cpp std::generator` | unseen-api | positive | 5 | `std::generator` is #1. | The exact API target is ranked first with useful context. |
| 130 | `qs c ckd_add` | unseen-api | positive | 5 | `ckd_add` is #1. | The exact API target is ranked first with useful context. |
| 131 | `qs haskell Data.Foldable.foldMap1` | unseen-api | positive | 5 | `foldMap1` from `Data.Foldable1` is #1. | The exact API target is ranked first with useful context. |
| 132 | `qs python asyncio.Barrier` | unseen-api | positive | 0 | Timed out after 20 seconds with no output. | No result was available before the required timeout. |
| 133 | `qs go math/big.Float.Scan` | unseen-api | positive | 5 | `big.Float.Scan()` is #1. | The exact API target is ranked first with useful context. |
| 134 | `qs pypi msgspec` | source-routing | positive | 0 | Error: `qs: no PyPI packages matched msgspec`. | The intended PyPI package is not returned. |
| 135 | `qs go-mod golang.org/x/sync` | source-routing | positive | 2 | Correct pkg.go.dev search URL, but results are static assets; favicon is #1. | Routing is correct, but the scraped results are not useful package results. |
| 136 | `qs crate camino` | source-routing | positive | 4 | Crates.io search is selected; `camino` is #1, but its result link goes to docs.rs. | The requested crate is first with useful metadata, but the link is documentation rather than its crates.io page. |
| 137 | `qs man cmake` | source-routing | positive | 5 | Local `cmake(1)` manual page is returned. | The requested local manual page is returned directly and is immediately useful. |
| 138 | `qs man 3 getaddrinfo` | source-routing | positive | 0 | Error: no manual entries for `3 getaddrinfo`. | The requested section 3 page is not returned. |
| 139 | `qs web Hackage attoparsec` | source-routing | positive | 0 | Error: no web results for `Hackage attoparsec`. | The intended web search fails. |
| 140 | `qs pypi platformdirs` | source-routing | positive | 0 | Error: no PyPI packages matched `platformdirs`. | The intended PyPI package is not returned. |
| 141 | `qs gh charmbracelet/lipgloss` | source-routing | positive | 5 | `charmbracelet/lipgloss` is #1 on GitHub. | The requested source is selected and the intended result is immediately useful. |
| 142 | `qs docs smallvec` | source-routing | positive | 5 | `smallvec` is #1 with a docs.rs link. | The requested source is selected and the intended result is immediately useful. |
| 143 | `qs so c++ object slicing` | source-routing | positive | 5 | `What is object slicing?` is #1 on Stack Overflow. | The requested source is selected and the intended result is immediately useful. |
| 144 | `qs cheat awk field separator` | source-routing | positive | 0 | The cheat.sh route returns `404 NOT FOUND` despite exit status 0. | The source route is correct, but the response is an upstream error and has no answer. |
| 145 | `qs gh haskell-unordered-containers` | source-routing | positive | 5 | GitHub repository search runs; relevant unordered-containers repositories are listed. | The requested GitHub repository search is returned with relevant results. |
| 146 | `qs mdn python webassembly api` | source-routing | positive | 5 | MDN search runs; Python is #1 and the WebAssembly JavaScript API is #2. | The requested MDN search is returned with relevant results in the top two. |
| 147 | `qs web go vulnerability database` | source-routing | positive | 0 | Error: no web results for `go vulnerability database`. | The intended web search fails. |
| 148 | `qs docs core::cell::OnceCell` | source-routing | positive | 5 | `OnceCell in core::cell` is #1 with the official core docs link. | The requested source is selected and the intended result is immediately useful. |
| 149 | `qs wiki C++ name mangling` | source-routing | positive | 5 | Wikipedia `Name mangling` is #1. | The intended Wikipedia article is first and immediately useful. |
