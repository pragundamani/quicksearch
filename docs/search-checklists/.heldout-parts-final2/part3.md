# Final held-out evaluation, part 3 (manifest entries 100–149)

## Method

- Candidate: `/tmp/qs-gate/release/qs`
- Manifest: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/heldout_manifest.json`
- Range: zero-based entries **100–149** (50 commands)
- Each command was run from a fresh, isolated `XDG_CACHE_HOME`; only the leading `qs` was replaced with the candidate path.
- Timeout: **30 seconds per command**.
- Scoring: 5 = expected first/useful; 4 = expected in top 3/useful; 3 = relevant with weak rank/detail; 2 = partial; 1 = barely useful or misleading; 0 = no relevant result or error.
- All 50 rows are positive labels. There are no rejected targets to check in this slice.

## Summary

- Total score: **230/250**
- Average: **4.60/5**
- Score ≥3: **47/50 (94%)**
- Score ≥4: **45/50 (90%)**
- Distribution: **0: 0, 1: 1, 2: 2, 3: 2, 4: 6, 5: 39**
- Nonzero exits: **0**
- Timeouts: **0**

## Per-entry results

| Entry | Language | Query | Expected | Class / label | Exit / time | Score | Observed top result and assessment |
|---:|---|---|---|---|---:|---:|---|
| 100 | c | `qs c reference standard code example quick answer feclearexcept floating environment` | `feclearexcept` | `surplus-token-adversarial / positive` | 0 / 2.901s | **2** | **Top:** feholdexcept. Exact `feclearexcept` is absent. The five results concern the floating-point environment, so this is only partial help. |
| 101 | haskell | `qs haskell Data.IntMap.lookupGE` | `Data.IntMap.lookupGE` | `unseen-api / positive` | 0 / 0.282s | **5** | **Top:** lookupGE :: Key -> IntMap a -> Maybe (Key, a). Exact API is result 1 with its signature, behavior, and examples. |
| 102 | python | `qs python mmap.mmap.madvise` | `mmap.mmap.madvise()` | `unseen-api / positive` | 0 / 2.394s | **5** | **Top:** mmap.mmap.madvise(). Exact method is result 1 with useful argument and platform details. |
| 103 | go | `qs go net/netip.ParsePrefix` | `net/netip.ParsePrefix` | `unseen-api / positive` | 0 / 3.656s | **4** | **Top:** func MustParsePrefix 1.18. Exact `netip.ParsePrefix()` is result 3 with its signature and behavior; related parsing functions rank above it. |
| 104 | rust | `qs rust std::sync::LazyLock::force_mut` | `LazyLock::force_mut` | `unseen-api / positive` | 0 / 0.599s | **5** | **Top:** LazyLock::force_mut. Exact method is result 1 and links to the official Rust docs. |
| 105 | cpp | `qs cpp std::ranges::cartesian_product_view` | `std::ranges::cartesian_product_view` | `unseen-api / positive` | 0 / 0.957s | **5** | **Top:** std::ranges::cartesian_product_view. Exact type is result 1 with a useful code example. |
| 106 | c | `qs c fgetpos` | `fgetpos` | `unseen-api / positive` | 0 / 0.590s | **5** | **Top:** fgetpos. Exact function is result 1 with a useful example. |
| 107 | haskell | `qs haskell Data.Array.accumArray` | `Data.Array.accumArray` | `unseen-api / positive` | 0 / 0.321s | **5** | **Top:** accumArray :: Ix i => (e -> a -> e) -> e -> (i, i) -> [(i, a)] -> Array i e. Exact API is result 1 with its signature and explanation. |
| 108 | python | `qs python os.posix_spawnp` | `os.posix_spawnp()` | `unseen-api / positive` | 0 / 2.405s | **5** | **Top:** os.posix_spawnp(). Exact function is result 1 with a concise description. |
| 109 | go | `qs go slices.Chunk` | `slices.Chunk` | `unseen-api / positive` | 0 / 0.798s | **5** | **Top:** slices.Chunk(). Exact function is result 1 with its signature, behavior, and example. |
| 110 | rust | `qs rust std::io::BorrowedBuf` | `std::io::BorrowedBuf` | `unseen-api / positive` | 0 / 0.277s | **5** | **Top:** BorrowedBuf in std::io - Rust. Exact type is result 1 with a concise description. |
| 111 | cpp | `qs cpp std::flat_map` | `std::flat_map` | `unseen-api / positive` | 0 / 1.793s | **5** | **Top:** std::flat_map. Exact type is result 1 with its declaration and example. |
| 112 | c | `qs c wcscoll` | `wcscoll` | `unseen-api / positive` | 0 / 0.588s | **5** | **Top:** wcscoll. Exact function is result 1 with a useful locale example. |
| 113 | haskell | `qs haskell Data.Graph.stronglyConnCompR` | `Data.Graph.stronglyConnCompR` | `unseen-api / positive` | 0 / 0.300s | **5** | **Top:** stronglyConnCompR :: Ord key => [(node, key, [key])] -> [SCC (node, key, [key])]. Exact API is result 1 with its signature and explanation. |
| 114 | python | `qs python ssl.SSLContext.num_tickets` | `SSLContext.num_tickets` | `unseen-api / positive` | 0 / 1.178s | **5** | **Top:** ssl.SSLContext.num_tickets. Exact attribute is result 1 with useful TLS behavior details. |
| 115 | go | `qs go reflect.TypeFor` | `reflect.TypeFor` | `unseen-api / positive` | 0 / 0.754s | **5** | **Top:** reflect.TypeFor(). Exact function is result 1 with its generic signature and description. |
| 116 | rust | `qs rust std::task::Waker::data` | `Waker::data` | `unseen-api / positive` | 0 / 0.474s | **5** | **Top:** Waker::data. Exact method is result 1 and links to the official Rust docs. |
| 117 | cpp | `qs cpp std::ranges::fold_left_with_iter` | `std::ranges::fold_left_with_iter` | `unseen-api / positive` | 0 / 0.810s | **5** | **Top:** std::ranges::fold_left_with_iter. Exact algorithm is result 1 with an explanation and code example. |
| 118 | c | `qs c quick_exit` | `quick_exit` | `unseen-api / positive` | 0 / 0.576s | **5** | **Top:** quick_exit. Exact function is result 1 with an example; related `at_quick_exit` is second. |
| 119 | haskell | `qs haskell Text.ParserCombinators.ReadP.gather` | `ReadP.gather` | `unseen-api / positive` | 0 / 0.247s | **5** | **Top:** gather :: ReadP a -> ReadP (String, a). Exact API is result 1 with its signature, behavior, and an important caveat. |
| 120 | python | `qs python tarfile.data_filter` | `tarfile.data_filter` | `unseen-api / positive` | 0 / 1.000s | **5** | **Top:** tarfile.data_filter(). Exact function is result 1, though its snippet is brief. |
| 121 | go | `qs go archive/zip.Writer.Copy` | `archive/zip.Writer.Copy` | `unseen-api / positive` | 0 / 3.603s | **4** | **Top:** func (*Writer) SetOffset 1.5. Exact `zip.Writer.Copy()` is result 3 with its signature and behavior; weaker entries rank above it. |
| 122 | rust | `qs rust std::thread::scope` | `std::thread::scope` | `unseen-api / positive` | 0 / 0.235s | **5** | **Top:** scope in std::thread - Rust. Exact function is result 1 with a concise description. |
| 123 | cpp | `qs cpp std::expected::transform_error` | `std::expected::transform_error` | `unseen-api / positive` | 0 / 0.766s | **5** | **Top:** std::expected::transform_error. Exact method is result 1 with its declarations. |
| 124 | c | `qs c thrd_sleep` | `thrd_sleep` | `unseen-api / positive` | 0 / 0.561s | **5** | **Top:** thrd_sleep. Exact function is result 1 with a useful example. |
| 125 | haskell | `qs haskell Data.Typeable.eqT` | `Data.Typeable.eqT` | `unseen-api / positive` | 0 / 0.244s | **1** | **Top:** Haskell language documentation — Data.Typeable.eqT. Only a generic Haskell 2010 Report link is returned. It does not document `Data.Typeable.eqT`. |
| 126 | python | `qs python sqlite3.Connection.blobopen` | `Connection.blobopen()` | `unseen-api / positive` | 0 / 0.961s | **5** | **Top:** sqlite3.Connection.blobopen(). Exact method is result 1, though its snippet has no detail. |
| 127 | go | `qs go crypto/ecdh.X25519` | `crypto/ecdh.X25519` | `unseen-api / positive` | 0 / 3.285s | **3** | **Top:** Package ecdh. The correct `crypto/ecdh` package is result 1, but no exact `X25519` result or signature appears. |
| 128 | rust | `qs rust std::net::Ipv6Addr::to_ipv4_mapped` | `Ipv6Addr::to_ipv4_mapped` | `unseen-api / positive` | 0 / 0.516s | **5** | **Top:** Ipv6Addr::to_ipv4_mapped. Exact method is result 1 and links to the official Rust docs. |
| 129 | cpp | `qs cpp std::generator` | `std::generator` | `unseen-api / positive` | 0 / 0.680s | **5** | **Top:** std::generator. Exact type is result 1 with a substantial example. |
| 130 | c | `qs c ckd_add` | `ckd_add` | `unseen-api / positive` | 0 / 0.522s | **5** | **Top:** ckd_add. Exact function is result 1 with an overflow-checking example. |
| 131 | haskell | `qs haskell Data.Foldable.foldMap1` | `Data.Foldable.foldMap1` | `unseen-api / positive` | 0 / 0.216s | **5** | **Top:** foldMap1 :: (Foldable1 t, Semigroup m) => (a -> m) -> t a -> m. Exact function is result 1 with its signature, explanation, and example. |
| 132 | python | `qs python asyncio.Barrier` | `asyncio.Barrier` | `unseen-api / positive` | 0 / 0.879s | **5** | **Top:** asyncio.Barrier. Exact type is result 1 with an explanation and example. |
| 133 | go | `qs go math/big.Float.Scan` | `math/big.Float.Scan` | `unseen-api / positive` | 0 / 3.748s | **5** | **Top:** func (*Float) Scan 1.8. The exact receiver method is result 1 with its signature; a fully qualified entry and example are result 3. |
| 134 | python | `qs pypi msgspec` | `PyPI package page for msgspec` | `source-routing / positive` | 0 / 0.295s | **5** | **Top:** msgspec. Exact PyPI project is result 1 with its summary and version. |
| 135 | go | `qs go-mod golang.org/x/sync` | `Go package registry result for golang.org/x/sync` | `source-routing / positive` | 0 / 0.545s | **5** | **Top:** golang.org/x/sync. Exact Go module is result 1, followed by its version and subpackages. |
| 136 | rust | `qs crate camino` | `crates.io page for camino` | `source-routing / positive` | 0 / 0.186s | **4** | **Top:** camino. Exact crate is result 1 and useful, but it links to docs.rs rather than the expected crates.io page. |
| 137 | cpp | `qs man cmake` | `Local cmake manual page` | `source-routing / positive` | 0 / 0.218s | **5** | **Top:** Direct local manual page. The local `cmake` manual opens directly with synopsis and description. |
| 138 | c | `qs man 3 getaddrinfo` | `Section 3 getaddrinfo manual page` | `source-routing / positive` | 0 / 0.114s | **5** | **Top:** Direct local manual page. The correct section 3 manual opens directly with synopsis and description. |
| 139 | haskell | `qs web Hackage attoparsec` | `DuckDuckGo results for Hackage attoparsec` | `source-routing / positive` | 0 / 0.211s | **4** | **Top:** Search DuckDuckGo for Hackage attoparsec. The correct DuckDuckGo search URL is result 1, but no fetched result listing is shown. |
| 140 | python | `qs pypi platformdirs` | `PyPI package page for platformdirs` | `source-routing / positive` | 0 / 0.182s | **5** | **Top:** platformdirs. Exact PyPI project is result 1 with its summary and version. |
| 141 | go | `qs gh charmbracelet/lipgloss` | `GitHub repository charmbracelet/lipgloss` | `source-routing / positive` | 0 / 0.393s | **5** | **Top:** charmbracelet/lipgloss. Exact GitHub repository is result 1 with description and star metadata. |
| 142 | rust | `qs docs smallvec` | `docs.rs documentation for smallvec` | `source-routing / positive` | 0 / 0.229s | **5** | **Top:** smallvec. Exact docs.rs crate documentation is result 1. |
| 143 | cpp | `qs so c++ object slicing` | `Stack Overflow search for C++ object slicing` | `source-routing / positive` | 0 / 0.264s | **5** | **Top:** What is object slicing?. The canonical Stack Overflow question is result 1 with strong answer metadata. |
| 144 | c | `qs cheat awk field separator` | `cheat.sh result for awk field separator` | `source-routing / positive` | 0 / 0.376s | **2** | **Top:** Search cheat.sh for awk field separator. No exact cheat.sh sheet is returned. The output only offers the same related-search URL. |
| 145 | haskell | `qs gh haskell-unordered-containers` | `GitHub repositories for haskell-unordered-containers` | `source-routing / positive` | 0 / 0.373s | **3** | **Top:** konsumlamm/extended-containers. GitHub repository search works, but the likely canonical project is absent and weak alternatives lead. |
| 146 | python | `qs mdn python webassembly api` | `MDN search for python webassembly api` | `source-routing / positive` | 0 / 0.128s | **4** | **Top:** Python. The useful WebAssembly JavaScript API page is result 2; a generic Python glossary page ranks first. |
| 147 | go | `qs web go vulnerability database` | `DuckDuckGo results for Go vulnerability database` | `source-routing / positive` | 0 / 0.176s | **4** | **Top:** Search DuckDuckGo for go vulnerability database. The correct DuckDuckGo search URL is result 1, but no fetched result listing is shown. |
| 148 | rust | `qs docs core::cell::OnceCell` | `Official Rust core docs for core::cell::OnceCell` | `source-routing / positive` | 0 / 0.156s | **5** | **Top:** OnceCell in core::cell - Rust. Exact official `core::cell::OnceCell` documentation is result 1. |
| 149 | cpp | `qs wiki C++ name mangling` | `Wikipedia result for C++ name mangling` | `source-routing / positive` | 0 / 0.380s | **5** | **Top:** Name mangling (featured result and result 1). The exact Wikipedia article is shown immediately and again as result 1. |
