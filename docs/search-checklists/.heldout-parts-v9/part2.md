# Held-out evaluation v9 — part 2 (indexes 50–99)

- Binary: `/tmp/qs-gate/release/qs`
- Mode: `--json`; 30-second timeout per query
- Commands: 50/50 exited 0; 0 timeouts; 50/50 valid JSON
- Score: **245/250 (4.90/5, 98.0%)**
- Distribution: `5`: 45, `4`: 5, `3`: 0, `2`: 0, `1`: 0, `0`: 0

## Key weaknesses

- Index 57: `std::set_terminate` ranked second, after `std::terminate_handler`.
- Index 61: `sort.SliceStable` ranked second, after `SortStableFunc`.
- Index 64: `fgetwc` ranked second, after the broad file-I/O overview.
- Index 66: `email.utils.parseaddr` ranked second, after a related `email.headerregistry` page.
- Index 77: `Data.Char.generalCategory` ranked second, after its `GeneralCategory` result type.

No query scored below 4. All other expected APIs ranked first.

## Results

| Index | Language | Class | Expected | First result | Score |
|---:|---|---|---|---|---:|
| 50 | rust | negative-contrast | `checked_add, not wrapping_add` | `checked_add` | 5 |
| 51 | cpp | negative-contrast | `std::midpoint, not (a+b)/2` | `std::midpoint` | 5 |
| 52 | c | negative-contrast | `strpbrk, not strstr` | `strpbrk` | 5 |
| 53 | haskell | negative-contrast | `Data.ByteString, not Data.ByteString.Lazy` | `data ByteString` | 5 |
| 54 | python | negative-contrast | `secrets.randbelow(), not random.randrange()` | `secrets.randbelow` | 5 |
| 55 | go | negative-contrast | `html.EscapeString, not url.QueryEscape` | `html.EscapeString` | 5 |
| 56 | rust | negative-contrast | `saturating_mul, not checked_mul` | `saturating_mul` | 5 |
| 57 | cpp | negative-contrast | `std::set_terminate, not exception matching` | `std::terminate_handler` | 4 |
| 58 | c | negative-contrast | `wcsrtombs, not mbsrtowcs` | `wcsrtombs` | 5 |
| 59 | haskell | negative-contrast | `Control.DeepSeq.force, not seq` | `force :: NFData a => a -> a` | 5 |
| 60 | python | negative-contrast | `gzip.open(), not zipfile.ZipFile` | `gzip.open(filename, mode='rb', compresslevel=6, encoding=None, errors=None, newline=None)` | 5 |
| 61 | go | negative-contrast | `sort.SliceStable, not sort.Slice` | `func SortStableFunc 1.21` | 4 |
| 62 | rust | negative-contrast | `std::process::abort, not panic!` | `Function abort` | 5 |
| 63 | cpp | negative-contrast | `std::inclusive_scan, not std::exclusive_scan` | `std::inclusive_scan` | 5 |
| 64 | c | negative-contrast | `fgetwc, not fgetc` | `File input/output` | 4 |
| 65 | haskell | negative-contrast | `Data.Text.Lazy.Builder, not (++)` | `data Builder` | 5 |
| 66 | python | negative-contrast | `email.utils.parseaddr(), not urllib.parse` | `email.headerregistry: Custom Header Objects` | 4 |
| 67 | go | negative-contrast | `subtle.ConstantTimeCompare, not bytes.Equal` | `func ConstantTimeCompare` | 5 |
| 68 | rust | surplus-token-adversarial | `std::mem::take` | `take in std::mem - Rust` | 5 |
| 69 | cpp | surplus-token-adversarial | `std::ranges::iota_view` | `std::ranges::iota_view` | 5 |
| 70 | c | surplus-token-adversarial | `getenv` | `getenv, getenv_s` | 5 |
| 71 | haskell | surplus-token-adversarial | `Data.Set.fromList` | `fromList :: Ord a => [a] -> Set a` | 5 |
| 72 | python | surplus-token-adversarial | `contextlib.ExitStack` | `contextlib.ExitStack` | 5 |
| 73 | go | surplus-token-adversarial | `net.IP.IsPrivate` | `net.IP.IsPrivate()` | 5 |
| 74 | rust | surplus-token-adversarial | `slice::rotate_left` | `slice::rotate_left` | 5 |
| 75 | cpp | surplus-token-adversarial | `std::byteswap` | `std::byteswap` | 5 |
| 76 | c | surplus-token-adversarial | `difftime` | `difftime` | 5 |
| 77 | haskell | surplus-token-adversarial | `Data.Char.generalCategory` | `data GeneralCategory` | 4 |
| 78 | python | surplus-token-adversarial | `PurePath.match()` | `pathlib.PurePath.match()` | 5 |
| 79 | go | surplus-token-adversarial | `cmp.Or` | `cmp.Or()` | 5 |
| 80 | rust | surplus-token-adversarial | `Vec::spare_capacity_mut` | `Vec::spare_capacity_mut` | 5 |
| 81 | cpp | surplus-token-adversarial | `std::ranges::fold_left` | `std::ranges::fold_left` | 5 |
| 82 | c | surplus-token-adversarial | `div` | `div` | 5 |
| 83 | haskell | surplus-token-adversarial | `Data.Sequence.adjust` | `adjust :: (a -> a) -> Int -> Seq a -> Seq a` | 5 |
| 84 | python | surplus-token-adversarial | `graphlib.TopologicalSorter` | `graphlib.TopologicalSorter` | 5 |
| 85 | go | surplus-token-adversarial | `math.Nextafter` | `math.Nextafter()` | 5 |
| 86 | rust | surplus-token-adversarial | `Path::ancestors` | `Path::ancestors` | 5 |
| 87 | cpp | surplus-token-adversarial | `std::launder` | `std::launder` | 5 |
| 88 | c | surplus-token-adversarial | `localeconv` | `localeconv` | 5 |
| 89 | haskell | surplus-token-adversarial | `Control.Arrow.loop` | `loop :: ArrowLoop a => a (b, d) (c, d) -> a b c` | 5 |
| 90 | python | surplus-token-adversarial | `weakref.finalize` | `weakref.finalize` | 5 |
| 91 | go | surplus-token-adversarial | `testing.F.Cleanup` | `testing.F.Cleanup()` | 5 |
| 92 | rust | surplus-token-adversarial | `std::array::from_fn` | `from_fn in std::array - Rust` | 5 |
| 93 | cpp | surplus-token-adversarial | `std::ranges::borrowed_range` | `std::ranges::borrowed_range` | 5 |
| 94 | c | surplus-token-adversarial | `nearbyint` | `nearbyint` | 5 |
| 95 | haskell | surplus-token-adversarial | `Data.Ord.clamp` | `clamp :: Ord a => (a, a) -> a -> a` | 5 |
| 96 | python | surplus-token-adversarial | `socket.socketpair()` | `socket.socketpair()` | 5 |
| 97 | go | surplus-token-adversarial | `log.Logger.Writer` | `log.Logger.Writer()` | 5 |
| 98 | rust | surplus-token-adversarial | `OsStr::as_encoded_bytes` | `OsStr::as_encoded_bytes` | 5 |
| 99 | cpp | surplus-token-adversarial | `std::unreachable_sentinel_t` | `std::unreachable_sentinel_t` | 5 |
