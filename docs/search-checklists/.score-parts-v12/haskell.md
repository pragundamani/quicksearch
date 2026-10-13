# Haskell v12 candidate evaluation

- Binary: `/tmp/qs-gate/release/qs`
- SHA256: `f1988ac17d46ea8bf5ab53e3e0dd173aee946ebc7aad4f44e33eecbc8ab987c7`
- Checklist: `haskell.md`
- Commands: 100, each run with `--json` and a 20 s timeout
- Result: **484/500 (96.8%), mean 4.84/5**
- Execution: 0 timeouts, 0 nonzero exits, 4.781 s cumulative command time

## Score distribution

| Score | Meaning | Count |
|---:|---|---:|
| 5 | canonical/immediately useful first | 84 |
| 4 | expected target top 3/useful | 16 |
| 3 | relevant weak | 0 |
| 2 | partial | 0 |
| 1 | misleading | 0 |
| 0 | error/no relevant | 0 |

## Errors and timeouts

- None. All 100 commands returned valid JSON with exit code 0 within 20 seconds.

## Expected target in top 3 / useful (4/5)

- **01. `qs haskell lists`.** The first hit is the official Haskell Report chapter for predefined types and discusses list syntax, but it does not surface Data.List.
- **13. `qs haskell infix operators`.** The first hit is a focused official section for infix operator application, precedence, and associativity, but it does not explicitly surface fixity declarations.
- **16. `qs haskell tail recursion`.** The first hit is a focused official GHC section on tail recursion and loopification, but it emphasizes compiler optimization rather than general tail-recursive function design.
- **44. `qs haskell discard Nothing values`.** catMaybes is present and useful at rank 3, behind two misleading void results.
- **49. `qs haskell ordering comparison`.** Ord is present at rank 3, behind maximumBy and minimumBy; compare is not surfaced in the visible top results.
- **54. `qs haskell Alternative`.** The Alternative class ranks first through internal GHC.Base; the expected public Control.Applicative entry is rank 2.
- **62. `qs haskell insert map key`.** Data.Map.Strict.insert is the useful expected result at rank 2; an unrelated Data.Sequence.lookup result ranks first.
- **67. `qs haskell vectors`.** The boxed Data.Vector.Vector type is at rank 2; the rank-1 generic Vector class is relevant but not the expected type.
- **69. `qs haskell Text`.** Data.Text.Text is at rank 3, behind two unrelated GHC type-level Text constructors.
- **73. `qs haskell handle invalid Unicode`.** decodeUtf8With ranks first and is useful, but the needed lenientDecode handler is not surfaced.
- **76. `qs haskell lens view set`.** Control.Lens.view ranks first and set ranks second, so both requested operations are useful in the top results.
- **88. `qs haskell bracket resource cleanup`.** Control.Exception.bracket is useful at rank 3, behind withCreateProcess and mask.
- **90. `qs haskell forkIO`.** forkIO itself ranks first, but the expected public Control.Concurrent module is only at rank 3 behind two internal GHC modules.
- **93. `qs haskell operation timeout`.** System.Timeout.timeout is useful at rank 2, behind the Timeout exception type.
- **97. `qs haskell QuickCheck properties`.** The Test.QuickCheck Property type ranks first and the expected property function ranks second.
- **99. `qs haskell GHCi commands`.** The actual GHCi command reference appears at rank 3; the first two results point to documentation-authoring material.

## Relevant but weak (3/5)

- None.

## Partial, misleading, or missing (0–2)

- None.
