# Haskell quicksearch score report

## Summary

All 100 checklist commands were run through the installed `qs` command. Each command had a 25-second timeout. No command timed out. However, 57 commands exited with an application error: 35 reported `qs: no haskell results ...`, and 22 reported `qs: invalid JSON ... missing field name`.

- **Average:** 1.26 / 5
- **Median:** 0 / 5
- **Distribution:** 0: 58, 1: 14, 2: 6, 3: 4, 4: 2, 5: 16
- **Pass rate (>=3):** 22% (22/100)
- **Strong pass rate (>=4):** 18% (18/100)

## All queries

| # | Command | Score | Observed top result or error | Practical assessment |
|---:|---|---:|---|---|
| 1 | `qs haskell lists` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected List type, syntax, and `Data.List` result is unusable. |
| 2 | `qs haskell tuples` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Tuple types and tuple syntax result is unusable. |
| 3 | `qs haskell pattern matching` | 1 | `patternMatchingAllowed` (Agda) | Haskell syntax help is absent; the top results are Agda/compiler internals. |
| 4 | `qs haskell guards` | 1 | `guards :: ArrowIf ...` (hxt) | Misleading for language guards; only a source-AST helper appears at #3. |
| 5 | `qs haskell list comprehensions` | 0 | Error: `no haskell results for list comprehensions` | No result is shown for the expected List-comprehension syntax target. |
| 6 | `qs haskell type classes` | 1 | `typeEnvClasses` (GHC internal) | Related internals and extensions do not explain `class` or `instance`. |
| 7 | `qs haskell algebraic data types` | 1 | `matchingAlgebraicDataTypes` | Only a migration helper matches the words; no ADT declaration help. |
| 8 | `qs haskell newtypes` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `newtype` declarations result is unusable. |
| 9 | `qs haskell record syntax` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Record declarations, construction, and updates result is unusable. |
| 10 | `qs haskell deriving` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Derived instances and deriving strategies result is unusable. |
| 11 | `qs haskell modules` | 1 | `modules :: Config -> Maybe GlobPattern` | Results use “modules” as an identifier, not Haskell module syntax. |
| 12 | `qs haskell qualified imports` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Qualified module imports result is unusable. |
| 13 | `qs haskell infix operators` | 0 | `infixOperators` (JavaScript lexer) | Wrong language and no Haskell fixity declaration help. |
| 14 | `qs haskell strict evaluation` | 0 | Error: `no haskell results for strict evaluation` | No result is shown for the expected Strictness, `seq`, and bang patterns target. |
| 15 | `qs haskell thunks` | 1 | `ThunkSelector` (GHC internal) | Related runtime internals, but no useful explanation of lazy thunks. |
| 16 | `qs haskell tail recursion` | 0 | Error: `no haskell results for tail recursion` | No result is shown for the expected Tail-recursive function design target. |
| 17 | `qs haskell currying` | 1 | `class Currying ...` (Agda) | A niche Agda type-level API does not explain ordinary curried functions. |
| 18 | `qs haskell partial application` | 1 | `PartialApplication` (Servant type error) | The saturation message is related, but it does not teach partial application. |
| 19 | `qs haskell higher order functions` | 0 | Error: `no haskell results for higher order functions` | No result is shown for the expected Functions that accept or return functions target. |
| 20 | `qs haskell function composition` | 0 | Error: `no haskell results for function composition` | No result is shown for the expected The `(.)` operator target. |
| 21 | `qs haskell map` | 5 | `map :: (a -> b) -> [a] -> [b]` (Prelude) | Exact API at #1 with clear semantics and examples. |
| 22 | `qs haskell filter` | 5 | `filter :: (a -> Bool) -> [a] -> [a]` (Prelude) | Exact API at #1 with a useful signature and examples. |
| 23 | `qs haskell zipWith` | 5 | `zipWith :: ...` (Prelude) | Exact API at #1 with signature, behavior, and examples. |
| 24 | `qs haskell concat map` | 5 | `concatMap :: ...` (Prelude) | Exact API at #1; `Data.List` is #2 and the example is useful. |
| 25 | `qs haskell reverse` | 5 | `reverse :: [a] -> [a]` (Prelude) | Exact API at #1 with examples and a finite-list warning. |
| 26 | `qs haskell take while` | 5 | `takeWhile :: ...` (Prelude) | Exact API at #1 with behavior and laziness examples. |
| 27 | `qs haskell dropWhile` | 5 | `dropWhile :: ...` (Prelude) | Exact API at #1 with clear examples. |
| 28 | `qs haskell split list at index` | 0 | Error: `no haskell results for split list at index` | No result is shown for the expected `Prelude.splitAt` target. |
| 29 | `qs haskell partition list` | 2 | `partitionByList :: [Bool] -> [a] -> ...` | It partitions a list, but not by a predicate and not `Data.List.partition`. |
| 30 | `qs haskell element membership` | 0 | Error: `no haskell results for element membership` | No result is shown for the expected `elem` and `notElem` target. |
| 31 | `qs haskell association list lookup` | 0 | Error: `no haskell results for association list lookup` | No result is shown for the expected `Prelude.lookup` target. |
| 32 | `qs haskell maximum by` | 5 | `maximumBy :: ...` (Data.List) | Exact result at #1 with comparator example. |
| 33 | `qs haskell sort on field` | 1 | `ftCommonFieldSortable` (Redis) | Matches “field/sort” tokens but does not sort a Haskell list. |
| 34 | `qs haskell group adjacent values` | 0 | Error: `no haskell results for group adjacent values` | No result is shown for the expected `Data.List.group` / `groupBy` target. |
| 35 | `qs haskell remove duplicates` | 1 | `SortFlagsRemoveDuplicates` (GtkSource) | A flag named after the task is not the useful `nub` function. |
| 36 | `qs haskell foldr` | 5 | `foldr :: ...` (Prelude) | Exact API at #1 with a detailed fold explanation. |
| 37 | `qs haskell "foldl'"` | 5 | `foldl' :: ...` (Prelude) | Exact strict fold at #1; explains thunk avoidance. |
| 38 | `qs haskell running totals` | 0 | Error: `no haskell results for running totals` | No result is shown for the expected `Prelude.scanl` target. |
| 39 | `qs haskell infinite iteration` | 0 | Error: `no haskell results for infinite iteration` | No result is shown for the expected `Prelude.iterate` target. |
| 40 | `qs haskell repeat value n times` | 0 | Error: `no haskell results for repeat value n times` | No result is shown for the expected `Prelude.replicate` target. |
| 41 | `qs haskell Maybe` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Maybe` result is unusable. |
| 42 | `qs haskell default maybe value` | 0 | Error: `no haskell results for default maybe value` | No result is shown for the expected `Data.Maybe.fromMaybe` target. |
| 43 | `qs haskell map_maybe` | 0 | Error: `no haskell results for map_maybe` | No result is shown for the expected `Data.Maybe.mapMaybe` target. |
| 44 | `qs haskell discard Nothing values` | 0 | Error: `no haskell results for discard Nothing values` | No result is shown for the expected `Data.Maybe.catMaybes` target. |
| 45 | `qs haskell Either` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Either` result is unusable. |
| 46 | `qs haskell separate lefts and rights` | 0 | Error: `no haskell results for separate lefts and rights` | No result is shown for the expected `Data.Either.partitionEithers` target. |
| 47 | `qs haskell parse string safely` | 0 | Error: `no haskell results for parse string safely` | No result is shown for the expected `Text.Read.readMaybe` target. |
| 48 | `qs haskell convert value to string` | 0 | Error: `no haskell results for convert value to string` | No result is shown for the expected `Show` and `show` target. |
| 49 | `qs haskell ordering comparison` | 0 | Error: `no haskell results for ordering comparison` | No result is shown for the expected `Ord` and `compare` target. |
| 50 | `qs haskell enumerate range` | 0 | Error: `no haskell results for enumerate range` | No result is shown for the expected `Enum` and range syntax target. |
| 51 | `qs haskell Functor` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Functor` and `fmap` result is unusable. |
| 52 | `qs haskell applicatives` | 2 | `ApplicativeS` (selective) | Related class, but standard `Applicative`, `pure`, and `<*>` are missing. |
| 53 | `qs haskell monadic bind` | 0 | Error: `no haskell results for monadic bind` | No result is shown for the expected `Monad` and `(>>=)` target. |
| 54 | `qs haskell Alternative` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Control.Applicative.Alternative` result is unusable. |
| 55 | `qs haskell foldables` | 1 | `mapFoldableS` (conduit) | Name match only; no `Foldable` class documentation. |
| 56 | `qs haskell traverse` | 4 | `traverse :: ...` (Prelude) | Exact function at #1; requested `Data.Traversable` entry is #2 with good context. |
| 57 | `qs haskell Semigroup` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Semigroup` and `(<>)` result is unusable. |
| 58 | `qs haskell monoids` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Monoid`, `mempty`, and `mappend` result is unusable. |
| 59 | `qs haskell bimap` | 5 | `bimap :: ...` (Data.Bifunctor) | Exact module and function at #1 with examples. |
| 60 | `qs haskell monad transformers` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected Transformer types and `lift` result is unusable. |
| 61 | `qs haskell Data.Map.Strict.lookup` | 5 | `lookup :: ...` (Data.Map.Strict) | Exact qualified target at #1 with signature and example. |
| 62 | `qs haskell insert map key` | 2 | `insertKeyMap` (hls-graph) | Related insertion shape, but standard `Data.Map.insert` is absent. |
| 63 | `qs haskell integer keyed maps` | 0 | Error: `no haskell results for integer keyed maps` | No result is shown for the expected `Data.IntMap` target. |
| 64 | `qs haskell set intersection` | 3 | `setIntersection` (GHC internal) | Usable intersection signatures appear, but not `Data.Set.intersection`. |
| 65 | `qs haskell hash maps` | 2 | `hashMapStrict` (aeson-combinators) | Shows `HashMap` in a decoder, but not the container type/module requested. |
| 66 | `qs haskell sequences` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Data.Sequence.Seq` result is unusable. |
| 67 | `qs haskell vectors` | 1 | `vectors` (gnuplot) | The plural identifier matches, but results do not expose `Data.Vector.Vector`. |
| 68 | `qs haskell byte strings` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Data.ByteString.ByteString` result is unusable. |
| 69 | `qs haskell Text` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Data.Text.Text` result is unusable. |
| 70 | `qs haskell split_on` | 0 | Error: `no haskell results for split_on` | No result is shown for the expected `Data.Text.splitOn` target. |
| 71 | `qs haskell replace substring` | 0 | Error: `no haskell results for replace substring` | No result is shown for the expected `Data.Text.replace` target. |
| 72 | `qs haskell encode utf8` | 3 | `utf8EncodeByteArray#`; `encodeUtf8` at #4 | The right function and module exist, but three low-level GHC APIs outrank it. |
| 73 | `qs haskell handle invalid Unicode` | 0 | Error: `no haskell results for handle invalid Unicode` | No result is shown for the expected `Data.Text.Encoding.decodeUtf8With` and `lenientDecode` target. |
| 74 | `qs haskell parse JSON` | 4 | `parseIndexedJSON`; `parseJSON` at #2 (aeson) | Strong Aeson result for `FromJSON`; `eitherDecode` is absent. |
| 75 | `qs haskell generic programming` | 0 | Error: `no haskell results for generic programming` | No result is shown for the expected `GHC.Generics.Generic` target. |
| 76 | `qs haskell lens view set` | 0 | Error: `no haskell results for lens view set` | No result is shown for the expected `Control.Lens.view`, `set`, and lens basics target. |
| 77 | `qs haskell putStrLn` | 5 | `putStrLn :: String -> IO ()` (Prelude) | Exact API at #1 with concise behavior. |
| 78 | `qs haskell readFile` | 5 | `readFile :: FilePath -> IO String` (Prelude) | Exact API at #1 with laziness and examples. |
| 79 | `qs haskell write text file` | 3 | `writeFileLText` / `writeFileText` (relude) | Useful text-file functions rank well, but standard `writeFile` is absent. |
| 80 | `qs haskell safe file handle` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `System.IO.withFile` result is unusable. |
| 81 | `qs haskell command line arguments` | 1 | GI `ApplicationCommandLine...` metadata | Framework metadata is not `System.Environment.getArgs`. |
| 82 | `qs haskell environment variables` | 2 | `expandEnvironmentVariables` (Agda) | Environment-related, but it does not read a variable via `lookupEnv`/`getEnv`. |
| 83 | `qs haskell list directory` | 5 | `listDirectory :: FilePath -> IO [FilePath]` | Exact `System.Directory` API at #1 with useful failure details. |
| 84 | `qs haskell check file exists` | 0 | Error: `no haskell results for check file exists` | No result is shown for the expected `System.Directory.doesFileExist` target. |
| 85 | `qs haskell file extension` | 3 | `fileExtension` (path) | A useful extension API ranks first, but standard `takeExtension` is missing. |
| 86 | `qs haskell temporary files` | 0 | Error: `no haskell results for temporary files` | No result is shown for the expected `System.IO.Temp` target. |
| 87 | `qs haskell exceptions` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Control.Exception` result is unusable. |
| 88 | `qs haskell bracket resource cleanup` | 0 | Error: `no haskell results for bracket resource cleanup` | No result is shown for the expected `Control.Exception.bracket` target. |
| 89 | `qs haskell async concurrency` | 0 | Error: `no haskell results for async concurrency` | No result is shown for the expected `Control.Concurrent.Async` target. |
| 90 | `qs haskell forkIO` | 5 | `forkIO :: IO () -> IO ThreadId` | Exact `Control.Concurrent` API at #1 with clear semantics. |
| 91 | `qs haskell MVar` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Control.Concurrent.MVar.MVar` result is unusable. |
| 92 | `qs haskell software transactional memory` | 0 | Error: `no haskell results for software transactional memory` | No result is shown for the expected `Control.Concurrent.STM` target. |
| 93 | `qs haskell operation timeout` | 1 | `CurlOperationTimeout` | A curl error code is not an operation-timeout function. |
| 94 | `qs haskell run external process` | 0 | Error: `no haskell results for run external process` | No result is shown for the expected `System.Process` / `readProcess` target. |
| 95 | `qs haskell random numbers` | 0 | Error: `no haskell results for random numbers` | No result is shown for the expected `System.Random` and `randomRIO` target. |
| 96 | `qs haskell HTTP client request` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Network.HTTP.Client` result is unusable. |
| 97 | `qs haskell QuickCheck properties` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Test.QuickCheck` and `property` result is unusable. |
| 98 | `qs haskell Hspec` | 0 | Error: `invalid JSON ... missing field name` | The command aborts, so the expected `Test.Hspec` result is unusable. |
| 99 | `qs haskell GHCi commands` | 0 | Error: `no haskell results for GHCi commands` | No result is shown for the expected GHCi interactive commands target. |
| 100 | `qs haskell language extensions` | 2 | `languageExtensions` (GHC internal) | Related compiler data, but no `LANGUAGE` pragma or user-facing extension guide. |

## Strongest patterns

- Exact, case-sensitive function names are excellent. `map`, `filter`, `zipWith`, `foldr`, `foldl'`, `bimap`, `putStrLn`, `readFile`, and `forkIO` all put the useful API at #1 with a signature and often examples.
- Spaced forms of some camel-case names work well. `concat map` and `take while` resolve to `concatMap` and `takeWhile` at #1.
- Fully qualified lookup is the most reliable route. `Data.Map.Strict.lookup` returns the exact module, signature, explanation, and example at #1.
- A few task phrases succeed when they closely match an exported identifier. `list directory` finds `System.Directory.listDirectory` at #1.

## Weakest patterns

- Error handling is the largest failure. 22 commands abort on valid Hoogle payloads whose entries lack a `name` field. This breaks basic targets such as `Maybe`, `Either`, `Functor`, `Semigroup`, `MVar`, `Hspec`, and `exceptions`.
- Natural task queries often return nothing. Examples include `split list at index`, `remove duplicates` (which instead finds an unrelated flag), `default maybe value`, `parse string safely`, `check file exists`, and `run external process`.
- Language concepts rank poorly. `pattern matching` starts with Agda APIs, `infix operators` returns a JavaScript lexer symbol, and `modules` returns unrelated identifiers.
- Plurals and naming variants are fragile. `applicatives`, `foldables`, `vectors`, and `sequences` do not normalize to their standard Haskell types. `map_maybe` and `split_on` return no results.
- Common-library relevance loses to literal token overlap. Redis field metadata outranks `sortOn`; GHC internal set helpers outrank `Data.Set.intersection`; low-level UTF-8 functions outrank `Data.Text.Encoding.encodeUtf8`.

## Prioritized improvements

1. **Make the Hoogle JSON decoder tolerant.** Treat `name` as optional and derive a display label from `item` when needed. This directly fixes 22 hard failures.
2. **Add query normalization and intent aliases.** Normalize plural/singular forms, snake_case/camelCase, and common task phrases such as “remove duplicates” → `nub`, “repeat value n times” → `replicate`, and “check file exists” → `doesFileExist`.
3. **Boost canonical packages and modules.** Prefer `base`, `containers`, `text`, `bytestring`, `directory`, `filepath`, and other common packages over token matches in internal or niche modules.
4. **Use a concept-documentation fallback.** When Hoogle lacks an API match, return a Haskell Report, GHC User Guide, or trusted tutorial section for syntax and concepts such as guards, pattern matching, type classes, ADTs, modules, and language extensions.
5. **Add intent-aware reranking and regression tests.** Reward exact semantic targets and penalize cross-language/internal metadata. Cover every observed miss, including `sort on field`, `command line arguments`, `operation timeout`, `parse JSON`, and UTF-8 encoding.
