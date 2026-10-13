# Haskell search quality score (v7)

- **Queries:** 100
- **Mean:** 3.76 / 5
- **Score distribution:** 5: 61, 4: 5, 3: 8, 2: 2, 1: 23, 0: 1
- **Score >=3:** 74/100
- **Score >=4:** 66/100
- **Command errors/timeouts:** 0

## Remaining failures (score 0-2)

- **1. `qs haskell lists` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested List type, syntax, and `Data.List` topic.
- **2. `qs haskell tuples` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Tuple types and tuple syntax topic.
- **3. `qs haskell pattern matching` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Pattern matching in equations and `case` topic.
- **4. `qs haskell guards` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Guard syntax topic.
- **5. `qs haskell list comprehensions` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested List-comprehension syntax topic.
- **6. `qs haskell type classes` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested `class` and `instance` declarations topic.
- **7. `qs haskell algebraic data types` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested `data` declarations and sum/product types topic.
- **8. `qs haskell newtypes` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested `newtype` declarations topic.
- **9. `qs haskell record syntax` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Record declarations, construction, and updates topic.
- **10. `qs haskell deriving` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Derived instances and deriving strategies topic.
- **11. `qs haskell modules` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Module declarations and exports topic.
- **12. `qs haskell qualified imports` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Qualified module imports topic.
- **13. `qs haskell infix operators` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Infix functions and fixity declarations topic.
- **14. `qs haskell strict evaluation` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Strictness, `seq`, and bang patterns topic.
- **15. `qs haskell thunks` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Lazy evaluation and deferred computations topic.
- **16. `qs haskell tail recursion` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Tail-recursive function design topic.
- **17. `qs haskell currying` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Curried functions topic.
- **18. `qs haskell partial application` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Partial function application topic.
- **19. `qs haskell higher order functions` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested Functions that accept or return functions topic.
- **20. `qs haskell function composition` — 1:** The only hit is a generic Haskell 2010 Report fallback; it does not surface the requested The `(.)` operator topic.
- **37. `qs haskell "foldl'"` — 1:** The quoted identifier falls back to the generic Haskell 2010 Report; foldl' is not returned.
- **47. `qs haskell parse string safely` — 2:** readMaybe is absent; Read and readsPrec provide lower-level parsing context but not the requested safe API.
- **48. `qs haskell convert value to string` — 0:** The results are fmap entries and ThreadId; Show and show are absent.
- **50. `qs haskell enumerate range` — 1:** Only a generic Haskell report fallback is returned, without focused Enum or range syntax.
- **53. `qs haskell monadic bind` — 1:** Monad bind and >>= are absent; the returned monadic folds and lens traversal APIs do not answer the query.
- **82. `qs haskell environment variables` — 2:** lookupEnv/getEnv are absent; setEnv and unsetEnv at ranks 4-5 only partially cover environment variables.
