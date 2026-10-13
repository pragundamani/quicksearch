# Held-out manifest evaluation, part 3

Entries are global zero-based indices 100–149 (`[100:150]`). Each manifest command was run separately. Only the leading `qs` was replaced with `/tmp/qs-gate/release/qs`. The timeout was 30 seconds per command.

## Summary

- Score distribution: **5: 39**, **4: 4**, **3: 3**, **2: 2**, **1: 0**, **0: 2**.
- Average: **4.48/5**.
- Score ≥3: **46/50 (92%)**.
- Score ≥4: **43/50 (86%)**.
- Errors/timeouts: **0 total** — no nonzero exits and no timeouts.

## Results

| Global index | Command | Class | Label | Score | Observed top result/error | Candid assessment |
|---:|---|---|---|---:|---|---|
| 100 | `qs c reference standard code example quick answer feclearexcept floating environment` | surplus-token-adversarial | positive | 2 | `exit` is #1; related floating-point-environment results follow, but `feclearexcept` is absent. | The output is partly relevant to the floating-point environment, but it does not return the requested function. |
| 101 | `qs haskell Data.IntMap.lookupGE` | unseen-api | positive | 5 | `Data.IntMap.lookupGE` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 102 | `qs python mmap.mmap.madvise` | unseen-api | positive | 5 | `mmap.mmap.madvise()` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 103 | `qs go net/netip.ParsePrefix` | unseen-api | positive | 4 | `netip.ParsePrefix()` is #3 after two package pages. | The exact API is useful and in the top three, but broad package pages outrank it. |
| 104 | `qs rust std::sync::LazyLock::force_mut` | unseen-api | positive | 5 | `LazyLock::force_mut` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 105 | `qs cpp std::ranges::cartesian_product_view` | unseen-api | positive | 5 | `std::ranges::cartesian_product_view` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 106 | `qs c fgetpos` | unseen-api | positive | 5 | `fgetpos` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 107 | `qs haskell Data.Array.accumArray` | unseen-api | positive | 5 | `Data.Array.accumArray` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 108 | `qs python os.posix_spawnp` | unseen-api | positive | 5 | `os.posix_spawnp()` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 109 | `qs go slices.Chunk` | unseen-api | positive | 5 | `slices.Chunk` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 110 | `qs rust std::io::BorrowedBuf` | unseen-api | positive | 5 | `std::io::BorrowedBuf` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 111 | `qs cpp std::flat_map` | unseen-api | positive | 5 | `std::flat_map` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 112 | `qs c wcscoll` | unseen-api | positive | 5 | `wcscoll` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 113 | `qs haskell Data.Graph.stronglyConnCompR` | unseen-api | positive | 5 | `Data.Graph.stronglyConnCompR` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 114 | `qs python ssl.SSLContext.num_tickets` | unseen-api | positive | 5 | `SSLContext.num_tickets` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 115 | `qs go reflect.TypeFor` | unseen-api | positive | 0 | An unrelated `github.com/kbirk/di` package is the only result. | The requested standard-library API is absent, so the result does not answer the query. |
| 116 | `qs rust std::task::Waker::data` | unseen-api | positive | 5 | `Waker::data` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 117 | `qs cpp std::ranges::fold_left_with_iter` | unseen-api | positive | 5 | `std::ranges::fold_left_with_iter` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 118 | `qs c quick_exit` | unseen-api | positive | 5 | `quick_exit` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 119 | `qs haskell Text.ParserCombinators.ReadP.gather` | unseen-api | positive | 5 | `ReadP.gather` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 120 | `qs python tarfile.data_filter` | unseen-api | positive | 5 | `tarfile.data_filter` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 121 | `qs go archive/zip.Writer.Copy` | unseen-api | positive | 4 | `zip.Writer.Copy()` is #3 after two package pages. | The exact API is useful and in the top three, but broad package pages outrank it. |
| 122 | `qs rust std::thread::scope` | unseen-api | positive | 5 | `std::thread::scope` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 123 | `qs cpp std::expected::transform_error` | unseen-api | positive | 5 | `std::expected::transform_error` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 124 | `qs c thrd_sleep` | unseen-api | positive | 5 | `thrd_sleep` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 125 | `qs haskell Data.Typeable.eqT` | unseen-api | positive | 0 | A generic Haskell 2010 report link is the only result. | The fallback is related only to the language and does not answer the API query. |
| 126 | `qs python sqlite3.Connection.blobopen` | unseen-api | positive | 5 | `Connection.blobopen()` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 127 | `qs go crypto/ecdh.X25519` | unseen-api | positive | 3 | The `crypto/ecdh` package page is #1 and lists `X25519()`, but no exact function result appears. | The package page is relevant, but the function detail and direct anchor are missing. |
| 128 | `qs rust std::net::Ipv6Addr::to_ipv4_mapped` | unseen-api | positive | 5 | `Ipv6Addr::to_ipv4_mapped` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 129 | `qs cpp std::generator` | unseen-api | positive | 5 | `std::generator` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 130 | `qs c ckd_add` | unseen-api | positive | 5 | `ckd_add` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 131 | `qs haskell Data.Foldable.foldMap1` | unseen-api | positive | 5 | `Data.Foldable.foldMap1` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 132 | `qs python asyncio.Barrier` | unseen-api | positive | 5 | `asyncio.Barrier` is #1 with documentation and a direct link. | The exact API target is ranked first with useful context. |
| 133 | `qs go math/big.Float.Scan` | unseen-api | positive | 4 | `big.Float.Scan()` is #3 after `math/big` and `fmt` package pages. | The exact API is useful and in the top three, but broad package pages outrank it. |
| 134 | `qs pypi msgspec` | source-routing | positive | 5 | `msgspec` is #1 with its PyPI project URL, summary, and version. | The intended package page is first and immediately useful. |
| 135 | `qs go-mod golang.org/x/sync` | source-routing | positive | 5 | `golang.org/x/sync` is #1 with its pkg.go.dev URL. | The intended registry result is first and immediately useful. |
| 136 | `qs crate camino` | source-routing | positive | 4 | `camino` is #1 with metadata, but its main result link goes to docs.rs. | The requested crate is first and useful, but the result does not link to its crates.io package page. |
| 137 | `qs man cmake` | source-routing | positive | 5 | The local `cmake(1)` manual page is returned. | The requested manual page is returned directly and is immediately useful. |
| 138 | `qs man 3 getaddrinfo` | source-routing | positive | 5 | The local section 3 `getaddrinfo` manual page is returned. | The requested section-specific manual page is returned directly. |
| 139 | `qs web Hackage attoparsec` | source-routing | positive | 3 | A DuckDuckGo query link is #1, but no fetched search results are shown. | The source route and query are correct, but the output lacks result detail. |
| 140 | `qs pypi platformdirs` | source-routing | positive | 5 | `platformdirs` is #1 with its PyPI project URL, summary, and version. | The intended package page is first and immediately useful. |
| 141 | `qs gh charmbracelet/lipgloss` | source-routing | positive | 5 | `charmbracelet/lipgloss` is #1 on GitHub. | The requested repository is first and immediately useful. |
| 142 | `qs docs smallvec` | source-routing | positive | 5 | `smallvec` is #1 with its docs.rs URL and package metadata. | The intended documentation is first and immediately useful. |
| 143 | `qs so c++ object slicing` | source-routing | positive | 5 | `What is object slicing?` is #1 on Stack Overflow. | The source route and first result directly answer the topic. |
| 144 | `qs cheat awk field separator` | source-routing | positive | 2 | No exact cheat.sh sheet is available; only a related-search link is returned. | The route is correct, but it does not provide the requested cheat-sheet content. |
| 145 | `qs gh haskell-unordered-containers` | source-routing | positive | 5 | GitHub repository search returns relevant unordered-containers repositories. | The requested source is selected and relevant repository results are immediately available. |
| 146 | `qs mdn python webassembly api` | source-routing | positive | 5 | MDN results show `Python` #1 and `Using the WebAssembly JavaScript API` #2. | The requested MDN search is returned with relevant top results. |
| 147 | `qs web go vulnerability database` | source-routing | positive | 3 | A DuckDuckGo query link is #1, but no fetched search results are shown. | The source route and query are correct, but the output lacks result detail. |
| 148 | `qs docs core::cell::OnceCell` | source-routing | positive | 5 | `OnceCell in core::cell` is #1 with the official Rust core-docs link. | The intended official documentation is first and immediately useful. |
| 149 | `qs wiki C++ name mangling` | source-routing | positive | 5 | Wikipedia `Name mangling` is #1. | The intended article is first and immediately useful. |
