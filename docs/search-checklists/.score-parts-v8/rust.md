# Rust v8 regression evaluation

- Commands run: **100/100**
- Total score: **452/500**
- Mean score: **4.52/5.00**
- Provider: **rust (100/100)**
- Errors/timeouts: **0**
- Distribution: **5: 69, 4: 14, 3: 17, 2: 0, 1: 0, 0: 0**

## Key failures (score 3)

| Query | Note |
|---|---|
| `rust lifetimes` | First result covers borrowing but does not explain lifetime annotations or elision. |
| `rust let else` | The patterns page is relevant, but the output does not identify `let...else`. |
| `rust dyn Trait` | The traits page is relevant, but the output does not mention trait objects or `dyn Trait`. |
| `rust generic functions` | The generic traits page is relevant, but the output does not explain generic functions. |
| `rust where clauses` | The result mentions bounds, but does not identify `where` clauses. |
| `rust const generics` | The generic traits page is relevant, but const generic parameters are not shown. |
| `rust pub(crate)` | The visibility page is relevant, but crate-scoped `pub(crate)` is not shown. |
| `rust derive macros` | The macros page is relevant, but the derive attribute and derivable traits are not shown. |
| `rust collect iterator into vector` | Iterator pages are relevant, but no top result clearly shows `collect::<Vec<_>>()` or Vec inference. |
| `rust reference counted pointer` | The ownership result mentions Rc, but does not identify it as the reference-counted pointer API. |
| `rust atomic reference counting` | The ownership result mentions Arc, but does not identify it as atomic reference counting. |
| `rust unix timestamp` | SystemTime ranks third, but the output does not surface `UNIX_EPOCH` or the conversion pattern. |
| `rust inclusive ranges` | The top three results are `RangeToInclusive`; the expected `RangeInclusive` appears only at rank four. |
| `rust tuple structs` | The structs topic is relevant, but tuple struct declaration and construction are not shown. |
| `rust named field structs` | The structs topic is relevant, but named-field syntax is not shown. |
| `rust enums with data` | The enums topic is relevant, but data-carrying variant syntax is not shown. |
| `rust derive PartialEq and Ord` | Results cover PartialOrd and related derives, but not the requested combined PartialEq and Ord behavior. |

## Method

Each checklist command was run against `/tmp/qs-gate/release/qs` with `--json` and a 30-second timeout. Scores were assigned from the actual ranked titles and snippets using the requested rubric.
