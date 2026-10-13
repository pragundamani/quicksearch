# Rust v9 final regression evaluation

- Commands run: **100/100**
- Total score: **452/500**
- Mean score: **4.52/5.00**
- Provider: **rust (100/100)**
- Errors/timeouts: **0/0**
- Distribution: **5: 69, 4: 14, 3: 17, 2: 0, 1: 0, 0: 0**
- Maximum command duration: **1.266s**

## Key failures (score 3 or lower)

| Query | Score | Note |
|---|---:|---|
| `rust lifetimes` | 3 | First result covers borrowing but does not explain lifetime annotations or elision. |
| `rust let else` | 3 | The patterns page is relevant, but the output does not identify `let...else`. |
| `rust dyn Trait` | 3 | The traits page is relevant, but the output does not mention trait objects or `dyn Trait`. |
| `rust generic functions` | 3 | The generic traits page is relevant, but the output does not explain generic functions. |
| `rust where clauses` | 3 | The result mentions bounds, but does not identify `where` clauses. |
| `rust const generics` | 3 | The generic traits page is relevant, but const generic parameters are not shown. |
| `rust pub(crate)` | 3 | The visibility page is relevant, but crate-scoped `pub(crate)` is not shown. |
| `rust derive macros` | 3 | The macros page is relevant, but the derive attribute and derivable traits are not shown. |
| `rust collect iterator into vector` | 3 | Iterator pages are relevant, but no top result clearly shows `collect::<Vec<_>>()` or Vec inference. |
| `rust reference counted pointer` | 3 | The ownership result mentions Rc, but does not identify it as the reference-counted pointer API. |
| `rust atomic reference counting` | 3 | The ownership result mentions Arc, but does not identify it as atomic reference counting. |
| `rust unix timestamp` | 3 | SystemTime ranks third, but the output does not surface `UNIX_EPOCH` or the conversion pattern. |
| `rust inclusive ranges` | 3 | The top three results are `RangeToInclusive`; the expected `RangeInclusive` is not surfaced. |
| `rust tuple structs` | 3 | The structs topic is relevant, but tuple struct declaration and construction are not shown. |
| `rust named field structs` | 3 | The structs topic is relevant, but named-field syntax is not shown. |
| `rust enums with data` | 3 | The enums topic is relevant, but data-carrying variant syntax is not shown. |
| `rust derive PartialEq and Ord` | 3 | Results cover PartialOrd and related derives, but not the requested combined PartialEq and Ord behavior. |

## Method

Each checklist command was run against `/tmp/qs-gate/release/qs` with `--json` before the query and a 30-second timeout. Scores were assigned from the actual ranked titles and snippets using the requested rubric. The JSON report includes the full parsed output for every command.
