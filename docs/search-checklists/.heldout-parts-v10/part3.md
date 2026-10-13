# Held-out final-candidate evaluation: manifest indexes 100–149

**Binary:** `/tmp/qs-gate/release/qs`  
**SHA256:** `a048a86983a8877eea4240c5a24e4e9119e901ceaeff44bc8d13e15911f8cabd`  
**Run:** Fresh process per query with `--json`; 30-second timeout.  
**Rubric:** 5 expected first/immediately useful; 4 expected top 3/useful; 3 relevant but weak; 2 partial; 1 misleading; 0 error/no relevant result.

## Summary

- Total: **219/250**
- Average: **4.38/5**
- Score ≥3: **46/50 (92%)**
- Score ≥4: **42/50 (84%)**
- Errors: **2**
- Timeouts: **2**
- Distribution: **0: 2, 1: 2, 2: 0, 3: 4, 4: 5, 5: 37**

## Results

| Index | Class | Query | Expected | Top results | Score | Assessment |
|---:|---|---|---|---|---:|---|
| 100 | surplus-token-adversarial | `qs c reference standard code example quick answer feclearexcept floating environment` | `feclearexcept` | 1. feclearexcept | **5** | The expected target is the first result (`feclearexcept`). It is immediately useful. |
| 101 | unseen-api | `qs haskell Data.IntMap.lookupGE` | `Data.IntMap.lookupGE` | 1. lookupGE :: Key -> IntMap a -> Maybe (Key, a)<br>2. lookupGE :: Key -> IntMap a -> Maybe (Key, a)<br>3. lookupGE :: Key -> IntMap a -> Maybe (Key, a)<br>4. lookupGE :: Key -> IntMap a -> Maybe (Key, a) | **5** | The expected target is the first result (`lookupGE :: Key -> IntMap a -> Maybe (Key, a)`). It is immediately useful. |
| 102 | unseen-api | `qs python mmap.mmap.madvise` | `mmap.mmap.madvise()` | 1. mmap.mmap.madvise() | **5** | The expected target is the first result (`mmap.mmap.madvise()`). It is immediately useful. |
| 103 | unseen-api | `qs go net/netip.ParsePrefix` | `net/netip.ParsePrefix` | 1. net/netip.ParsePrefix | **5** | The expected target is the first result (`net/netip.ParsePrefix`). It is immediately useful. |
| 104 | unseen-api | `qs rust std::sync::LazyLock::force_mut` | `LazyLock::force_mut` | 1. LazyLock::force_mut<br>2. Search Rust standard library for std::sync::LazyLock::force_mut | **5** | The expected target is the first result (`LazyLock::force_mut`). It is immediately useful. |
| 105 | unseen-api | `qs cpp std::ranges::cartesian_product_view` | `std::ranges::cartesian_product_view` | 1. std::ranges::cartesian_product_view<br>2. std::ranges::cartesian_product_view::end<br>3. std::ranges::cartesian_product_view::size<br>4. std::ranges::cartesian_product_view<First<br>5. std::ranges::cartesian_product_view::begin | **5** | The expected target is the first result (`std::ranges::cartesian_product_view`). It is immediately useful. |
| 106 | unseen-api | `qs c fgetpos` | `fgetpos` | 1. fgetpos | **5** | The expected target is the first result (`fgetpos`). It is immediately useful. |
| 107 | unseen-api | `qs haskell Data.Array.accumArray` | `Data.Array.accumArray` | 1. Haskell language documentation — Data.Array.accumArray | **1** | The only hit misleadingly embeds the requested API in its title but links to the generic Haskell 2010 Report, with no `accumArray` documentation. |
| 108 | unseen-api | `qs python os.posix_spawnp` | `os.posix_spawnp()` | 1. os.posix_spawnp() | **5** | The expected target is the first result (`os.posix_spawnp()`). It is immediately useful. |
| 109 | unseen-api | `qs go slices.Chunk` | `slices.Chunk` | 1. slices.Chunk | **5** | The expected target is the first result (`slices.Chunk`). It is immediately useful. |
| 110 | unseen-api | `qs rust std::io::BorrowedBuf` | `std::io::BorrowedBuf` | 1. BorrowedBuf in std::io - Rust<br>2. Search Rust standard library for std::io::BorrowedBuf | **5** | The expected target is the first result (`BorrowedBuf in std::io - Rust`). It is immediately useful. |
| 111 | unseen-api | `qs cpp std::flat_map` | `std::flat_map` | 1. std::flat_map<br>2. std::flat_map::at<br>3. std::flat_map::end<br>4. std::flat_map::find<br>5. std::flat_map::keys | **5** | The expected target is the first result (`std::flat_map`). It is immediately useful. |
| 112 | unseen-api | `qs c wcscoll` | `wcscoll` | 1. wcscoll | **5** | The expected target is the first result (`wcscoll`). It is immediately useful. |
| 113 | unseen-api | `qs haskell Data.Graph.stronglyConnCompR` | `Data.Graph.stronglyConnCompR` | 1. stronglyConnCompR :: Ord key => [(node, key, [key])] -> [SCC (node, key, [key])] | **5** | The expected target is the first result (`stronglyConnCompR :: Ord key => [(node, key, [key])] -> [SCC (node, key, [key])]`). It is immediately useful. |
| 114 | unseen-api | `qs python ssl.SSLContext.num_tickets` | `SSLContext.num_tickets` | 1. ssl.SSLContext.num_tickets | **5** | The expected target is the first result (`ssl.SSLContext.num_tickets`). It is immediately useful. |
| 115 | unseen-api | `qs go reflect.TypeFor` | `reflect.TypeFor` | 1. reflect.TypeFor | **5** | The expected target is the first result (`reflect.TypeFor`). It is immediately useful. |
| 116 | unseen-api | `qs rust std::task::Waker::data` | `Waker::data` | 1. Waker::data<br>2. Search Rust standard library for std::task::Waker::data | **5** | The expected target is the first result (`Waker::data`). It is immediately useful. |
| 117 | unseen-api | `qs cpp std::ranges::fold_left_with_iter` | `std::ranges::fold_left_with_iter` | 1. std::ranges::fold_left_with_iter<br>2. std::ranges::fold_left_first_with_iter | **5** | The expected target is the first result (`std::ranges::fold_left_with_iter`). It is immediately useful. |
| 118 | unseen-api | `qs c quick_exit` | `quick_exit` | 1. quick_exit<br>2. at_quick_exit | **5** | The expected target is the first result (`quick_exit`). It is immediately useful. |
| 119 | unseen-api | `qs haskell Text.ParserCombinators.ReadP.gather` | `ReadP.gather` | 1. gather :: ReadP a -> ReadP (String, a) | **5** | The expected target is the first result (`gather :: ReadP a -> ReadP (String, a)`). It is immediately useful. |
| 120 | unseen-api | `qs python tarfile.data_filter` | `tarfile.data_filter` | 1. tarfile.data_filter() | **5** | The expected target is the first result (`tarfile.data_filter()`). It is immediately useful. |
| 121 | unseen-api | `qs go archive/zip.Writer.Copy` | `archive/zip.Writer.Copy` | 1. func (*Writer) SetOffset 1.5<br>2. func (*Writer) WriteHeader<br>3. zip.Writer.Copy()<br>4. type ValueKind 1.16<br>5. Constants | **4** | The exact `zip.Writer.Copy()` result is third. The first two results are other writer APIs. |
| 122 | unseen-api | `qs rust std::thread::scope` | `std::thread::scope` | 1. scope in std::thread - Rust<br>2. Search Rust standard library for std::thread::scope | **5** | The expected target is the first result (`scope in std::thread - Rust`). It is immediately useful. |
| 123 | unseen-api | `qs cpp std::expected::transform_error` | `std::expected::transform_error` | 1. std::expected::transform_error | **5** | The expected target is the first result (`std::expected::transform_error`). It is immediately useful. |
| 124 | unseen-api | `qs c thrd_sleep` | `thrd_sleep` | 1. thrd_sleep | **5** | The expected target is the first result (`thrd_sleep`). It is immediately useful. |
| 125 | unseen-api | `qs haskell Data.Typeable.eqT` | `Data.Typeable.eqT` | 1. eqT :: forall {k} (a :: k) (b :: k). (Typeable a, Typeable b) => Maybe (a :~: b) | **5** | The expected target is the first result (`eqT :: forall {k} (a :: k) (b :: k). (Typeable a, Typeable b) => Maybe (a :~: b)`). It is immediately useful. |
| 126 | unseen-api | `qs python sqlite3.Connection.blobopen` | `Connection.blobopen()` | 1. sqlite3.Connection.blobopen() | **5** | The expected target is the first result (`sqlite3.Connection.blobopen()`). It is immediately useful. |
| 127 | unseen-api | `qs go crypto/ecdh.X25519` | `crypto/ecdh.X25519` | 1. crypto/ecdh.X25519 | **5** | The expected target is the first result (`crypto/ecdh.X25519`). It is immediately useful. |
| 128 | unseen-api | `qs rust std::net::Ipv6Addr::to_ipv4_mapped` | `Ipv6Addr::to_ipv4_mapped` | 1. Ipv6Addr::to_ipv4_mapped<br>2. Search Rust standard library for std::net::Ipv6Addr::to_ipv4_mapped | **5** | The expected target is the first result (`Ipv6Addr::to_ipv4_mapped`). It is immediately useful. |
| 129 | unseen-api | `qs cpp std::generator` | `std::generator` | 1. std::generator<br>2. std::generator::end<br>3. std::generator::begin<br>4. std::generator::iterator<br>5. std::generator::generator | **5** | The expected target is the first result (`std::generator`). It is immediately useful. |
| 130 | unseen-api | `qs c ckd_add` | `ckd_add` | 1. ckd_add | **5** | The expected target is the first result (`ckd_add`). It is immediately useful. |
| 131 | unseen-api | `qs haskell Data.Foldable.foldMap1` | `Data.Foldable.foldMap1` | 1. Haskell language documentation — Data.Foldable.foldMap1 | **1** | The only hit misleadingly embeds the requested API in its title but links to the generic Haskell 2010 Report, with no `foldMap1` documentation. |
| 132 | unseen-api | `qs python asyncio.Barrier` | `asyncio.Barrier` | 1. asyncio.Barrier<br>2. asyncio.Barrier.broken<br>3. asyncio.Barrier.wait()<br>4. asyncio.Barrier.abort()<br>5. asyncio.Barrier.parties | **5** | The expected target is the first result (`asyncio.Barrier`). It is immediately useful. |
| 133 | unseen-api | `qs go math/big.Float.Scan` | `math/big.Float.Scan` | 1. func (*Float) Scan 1.8<br>2. func Scan<br>3. big.Float.Scan()<br>4. func ToFloat 1.6<br>5. A note about floats | **4** | The exact `big.Float.Scan()` result is third. The first two results are a related DevDocs entry and an unrelated `fmt.Scan` entry. |
| 134 | source-routing | `qs pypi msgspec` | `PyPI package page for msgspec` | 1. Search PyPI for msgspec | **3** | The result is a relevant PyPI search fallback, but it does not provide the expected `msgspec` package page or package details. |
| 135 | source-routing | `qs go-mod golang.org/x/sync` | `Go package registry result for golang.org/x/sync` | 1. golang.org/x/sync/errgroup<br>2. golang.org/x/sync/singleflight<br>3. golang.org/x/sync/semaphore<br>4. golang.org/x/sync/syncmap<br>5. github.com/bir3/gocompiler/src/xvendor/golang.org/x/sync/semaphore | **4** | The query routes to pkg.go.dev and returns useful `golang.org/x/sync` subpackages, but not the module root. |
| 136 | source-routing | `qs crate camino` | `crates.io page for camino` | 1. camino<br>2. camino-tempfile-ext<br>3. caminos<br>4. camino-tempfile<br>5. camino-anchored | **4** | The `camino` crate is first and useful, but its URL is docs.rs rather than the expected crates.io package page. |
| 137 | source-routing | `qs man cmake` | `Local cmake manual page` | 1. cmake (1)<br>2. ccmake (1)<br>3. cmake-buildsystem (7)<br>4. cmake-commands (7)<br>5. cmake-compile-features (7) | **5** | The expected target is the first result (`cmake (1)`). It is immediately useful. |
| 138 | source-routing | `qs man 3 getaddrinfo` | `Section 3 getaddrinfo manual page` | 1. getaddrinfo (3) | **5** | The expected target is the first result (`getaddrinfo (3)`). It is immediately useful. |
| 139 | source-routing | `qs web Hackage attoparsec` | `DuckDuckGo results for Hackage attoparsec` | None | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |
| 140 | source-routing | `qs pypi platformdirs` | `PyPI package page for platformdirs` | 1. Search PyPI for platformdirs | **3** | The result is a relevant PyPI search fallback, but it does not provide the expected `platformdirs` package page or package details. |
| 141 | source-routing | `qs gh charmbracelet/lipgloss` | `GitHub repository charmbracelet/lipgloss` | 1. charmbracelet/lipgloss<br>2. 76creates/stickers<br>3. whit3rabbit/lipgloss-rs<br>4. oakoliver/lipgloss<br>5. junhinhow/charm-themes | **5** | The expected target is the first result (`charmbracelet/lipgloss`). It is immediately useful. |
| 142 | source-routing | `qs docs smallvec` | `docs.rs documentation for smallvec` | 1. smallvec | **5** | The expected target is the first result (`smallvec`). It is immediately useful. |
| 143 | source-routing | `qs so c++ object slicing` | `Stack Overflow search for C++ object slicing` | 1. What is object slicing?<br>2. Object Slicing in c++<br>3. how object slicing happens in c++?<br>4. Object slicing while casting in C++<br>5. C++: object slicing and exceptions | **5** | The expected target is the first result (`What is object slicing?`). It is immediately useful. |
| 144 | source-routing | `qs cheat awk field separator` | `cheat.sh result for awk field separator` | 1. Search cheat.sh for awk field separator | **3** | The query routes to cheat.sh, but it reports no exact sheet and only returns a related-search link. |
| 145 | source-routing | `qs gh haskell-unordered-containers` | `GitHub repositories for haskell-unordered-containers` | 1. konsumlamm/extended-containers<br>2. solus-packages/haskell-unordered-containers<br>3. deepin-community/haskell-unordered-containers<br>4. comatose/stl-container | **3** | The query routes to GitHub repository search, but the canonical unordered-containers repository is absent and the returned repositories are weak matches. |
| 146 | source-routing | `qs mdn python webassembly api` | `MDN search for python webassembly api` | 1. Python<br>2. Using the WebAssembly JavaScript API<br>3. Django Web Framework (Python)<br>4. WebAssembly<br>5. WebAssembly | **4** | The useful WebAssembly JavaScript API result is second, behind a generic Python glossary result. |
| 147 | source-routing | `qs web go vulnerability database` | `DuckDuckGo results for Go vulnerability database` | None | **0** | The command timed out after 30 seconds and returned no JSON or relevant result. |
| 148 | source-routing | `qs docs core::cell::OnceCell` | `Official Rust core docs for core::cell::OnceCell` | 1. OnceCell in core::cell - Rust<br>2. Search Rust standard library for core::cell::OnceCell | **5** | The expected target is the first result (`OnceCell in core::cell - Rust`). It is immediately useful. |
| 149 | source-routing | `qs wiki C++ name mangling` | `Wikipedia result for C++ name mangling` | 1. Name mangling<br>2. C++ syntax<br>3. Nm (Unix)<br>4. C++<br>5. Namespace | **5** | The expected target is the first result (`Name mangling`). It is immediately useful. |

## Failures

- **107:** Misleading generic Haskell 2010 Report fallback for `Data.Array.accumArray`.
- **131:** Misleading generic Haskell 2010 Report fallback for `Data.Foldable.foldMap1`.
- **139:** Timed out after 30 seconds with no output (`qs web Hackage attoparsec`).
- **147:** Timed out after 30 seconds with no output (`qs web go vulnerability database`).
