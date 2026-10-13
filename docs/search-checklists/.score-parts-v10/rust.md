# Rust v10 exact final-candidate evaluation

- Binary: `/tmp/qs-gate/release/qs`
- SHA256: `a048a86983a8877eea4240c5a24e4e9119e901ceaeff44bc8d13e15911f8cabd`
- Commands run: **100/100**
- Invocation: **`--json`, 30-second timeout per command**
- Total score: **485/500**
- Mean score: **4.85/5.00**
- Provider: **rust (100/100)**
- Errors/timeouts/parse errors: **0/0/0**
- Distribution: **5: 85, 4: 15, 3: 0, 2: 0, 1: 0, 0: 0**
- Maximum command duration: **1.355s**

## Failures (score 3 or lower)

None.

## Useful but incomplete results (score 4)

| Query | Score | Note |
|---|---:|---|
| `lifetimes` | 4 | The first result directly explains lifetime annotations, but lifetime elision is not surfaced. |
| `mutable references` | 4 | First result is the right borrowing page, but output omits the `&mut T` syntax. |
| `struct destructuring` | 4 | First result explains patterns and destructuring, but not struct-specific syntax. |
| `attributes` | 4 | The first result is the attributes topic, but outer versus inner attributes is omitted. |
| `procedural macros` | 4 | Procedural macros are identified, but their three forms are not described. |
| `raw pointers` | 4 | The first result mentions raw-pointer operations, but omits `*const T` and `*mut T`. |
| `foreign function interface` | 4 | The first result mentions FFI, but omits extern blocks and ABI strings. |
| `format!` | 4 | `format!` is present in the top results, but `format_args!` ranks first. |
| `AsRef<Path>` | 4 | The first result is `AsRef`, but the output does not explain the `Path` bound. |
| `mutexes` | 4 | The stable `std::sync::Mutex` result is third; two nightly mutex variants rank above it. |
| `environment variables` | 4 | `std::env::var` is first and useful, but `vars` is not surfaced. |
| `command line arguments` | 4 | `std::env::args` is first and useful, but `args_os` is not surfaced. |
| `parse socket address` | 4 | Parsing and FromStr context is in the top results, but SocketAddr itself is not surfaced in the top three. |
| `parse integer` | 4 | `str::parse` is first, but integer types and `ParseIntError` are not surfaced. |
| `cfg target_os` | 4 | The attributes result mentions conditional compilation, but not `target_os` syntax. |
