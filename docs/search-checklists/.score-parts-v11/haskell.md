# Haskell v11 candidate evaluation

- Binary: `/tmp/qs-gate/release/qs`
- SHA256: `8f05a6dcd581ef25fe5d2904b169cb3794a94056b19d687cf4646a6372cc096d`
- Checklist: `haskell.md`
- Commands: 100, each run with `--json` and a 20 s timeout
- Result: **458/500 (91.6%), mean 4.58/5**
- Execution: 0 timeouts, 0 nonzero exits, 5.492 s cumulative command time

## Score distribution

| Score | Meaning | Count |
|---:|---|---:|
| 5 | canonical/immediately useful first | 72 |
| 4 | expected target top 3/useful | 14 |
| 3 | relevant weak | 14 |
| 2 | partial | 0 |
| 1 | misleading | 0 |
| 0 | error/no relevant | 0 |

## Errors and timeouts

- None. All 100 commands returned valid JSON with exit code 0 within 20 seconds.

## Expected target in top 3 / useful (4/5)

- **01. `qs haskell lists`.** The first hit is the official Haskell Report chapter for predefined types and discusses list syntax, but it does not surface Data.List.
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

- **03. `qs haskell pattern matching`.** The only hit is a broad expressions chapter with a generic snippet; it is relevant but does not take the user to a focused pattern-matching section.
- **04. `qs haskell guards`.** The only hit is a broad expressions chapter with a generic snippet; it is relevant but weak for guard syntax.
- **05. `qs haskell list comprehensions`.** The only hit is a broad expressions chapter with a generic snippet; it is relevant but not a focused list-comprehension result.
- **06. `qs haskell type classes`.** The only hit is the broad declarations chapter. It mentions class and instance declarations but has no focused anchor or specific syntax snippet.
- **07. `qs haskell algebraic data types`.** The only hit is the broad declarations chapter. It mentions data declarations but does not directly explain sum/product types.
- **08. `qs haskell newtypes`.** The only hit is the broad declarations chapter. It mentions newtype but has no focused newtype syntax snippet.
- **09. `qs haskell record syntax`.** The only hit is the broad declarations chapter. It mentions records but does not surface construction and update syntax.
- **11. `qs haskell modules`.** The only hit is the broad modules/imports chapter. It is relevant but lacks a focused module/export result.
- **12. `qs haskell qualified imports`.** The only hit is the broad modules/imports chapter. It mentions qualified imports but has no focused syntax result.
- **13. `qs haskell infix operators`.** The only hit is the broad expressions chapter. It is relevant but does not surface fixity-declaration details.
- **16. `qs haskell tail recursion`.** foldl' is a useful strict-fold technique that avoids thunk buildup, but it is not a direct guide to tail-recursive design.
- **17. `qs haskell currying`.** The broad expressions chapter is relevant to curried functions but the hit and snippet are not focused.
- **18. `qs haskell partial application`.** The broad expressions chapter is relevant to partial application but the hit and snippet are not focused.
- **19. `qs haskell higher order functions`.** The broad expressions chapter is relevant to higher-order functions but the hit and snippet are not focused.

## Five-point results

72 commands scored 5/5. Their first result was canonical or immediately useful.
