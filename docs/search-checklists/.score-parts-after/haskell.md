# Haskell candidate quicksearch scores

## Summary

All 100 checklist commands were run separately against `/tmp/qs-candidate/release/qs`. Each command had a 20-second timeout.

- **Average:** 2.79 / 5
- **Median:** 2 / 5
- **Distribution:** 0: 0, 1: 28, 2: 25, 3: 15, 4: 4, 5: 28
- **Pass rate (score >=3):** 47% (47/100)
- **Strong-result rate (score >=4):** 32% (32/100)
- **Errors/timeouts:** 0 / 0

### Baseline comparison

The `language_scores.md` Haskell baseline was 1.26 average, 0 median, 22% score >=3, 18% score >=4, distribution `0: 58, 1: 14, 2: 6, 3: 4, 4: 2, 5: 16`, with 57 application errors and no timeouts. The candidate improves the average by **1.53**, median by **2**, pass rate by **25 percentage points**, and strong-result rate by **14 percentage points**. Hard errors fall from **57 to 0**. Generic Haskell Report fallbacks remove failures, but many task queries still do not name the expected API.

## All queries

| # | Command | Score | Observed top result or error | Practical assessment |
|---:|---|---:|---|---|
| 1 | `qs haskell lists` | 1 | `list :: (a -> Encoding) -> [a] -> Encoding` | The result is barely useful or misleading for the expected target. |
| 2 | `qs haskell tuples` | 1 | `tuple :: Encoding' InArray -> Encoding` | The result is barely useful or misleading for the expected target. |
| 3 | `qs haskell pattern matching` | 3 | `Haskell language documentation — pattern matching` | A relevant result exists, but its detail or ranking is weak. |
| 4 | `qs haskell guards` | 1 | `guard :: Alternative f => Bool -> f ()` | The result is barely useful or misleading for the expected target. |
| 5 | `qs haskell list comprehensions` | 3 | `Haskell language documentation — list comprehensions` | A relevant result exists, but its detail or ranking is weak. |
| 6 | `qs haskell type classes` | 3 | `Haskell language documentation — type classes` | A relevant result exists, but its detail or ranking is weak. |
| 7 | `qs haskell algebraic data types` | 3 | `Haskell language documentation — algebraic data types` | A relevant result exists, but its detail or ranking is weak. |
| 8 | `qs haskell newtypes` | 2 | `package newtype` | Only a generic documentation link or partial match appears. |
| 9 | `qs haskell record syntax` | 3 | `Haskell language documentation — record syntax` | A relevant result exists, but its detail or ranking is weak. |
| 10 | `qs haskell deriving` | 2 | `data Deriving l` | Only a generic documentation link or partial match appears. |
| 11 | `qs haskell modules` | 1 | `data Module` | The result is barely useful or misleading for the expected target. |
| 12 | `qs haskell qualified imports` | 3 | `Haskell language documentation — qualified imports` | A relevant result exists, but its detail or ranking is weak. |
| 13 | `qs haskell infix operators` | 3 | `Haskell language documentation — infix operators` | A relevant result exists, but its detail or ranking is weak. |
| 14 | `qs haskell strict evaluation` | 3 | `Haskell language documentation — strict evaluation` | A relevant result exists, but its detail or ranking is weak. |
| 15 | `qs haskell thunks` | 1 | `ThunkSelector :: SelectorOffset -> ClosureTypeInfo` | The result is barely useful or misleading for the expected target. |
| 16 | `qs haskell tail recursion` | 2 | `Haskell language documentation — tail recursion` | Only a generic documentation link or partial match appears. |
| 17 | `qs haskell currying` | 1 | `class Currying (as :: [Type]) b` | The result is barely useful or misleading for the expected target. |
| 18 | `qs haskell partial application` | 3 | `Haskell language documentation — partial application` | A relevant result exists, but its detail or ranking is weak. |
| 19 | `qs haskell higher order functions` | 3 | `Haskell language documentation — higher order functions` | A relevant result exists, but its detail or ranking is weak. |
| 20 | `qs haskell function composition` | 3 | `Haskell language documentation — function composition` | A relevant result exists, but its detail or ranking is weak. |
| 21 | `qs haskell map` | 5 | `map :: (a -> b) -> [a] -> [b]` | The expected API or type is first and immediately useful. |
| 22 | `qs haskell filter` | 5 | `filter :: (a -> Bool) -> [a] -> [a]` | The expected API or type is first and immediately useful. |
| 23 | `qs haskell zipWith` | 5 | `zipWith :: (a -> b -> c) -> [a] -> [b] -> [c]` | The expected API or type is first and immediately useful. |
| 24 | `qs haskell concat map` | 5 | `concatMap :: Foldable t => (a -> [b]) -> t a -> [b]` | The expected API or type is first and immediately useful. |
| 25 | `qs haskell reverse` | 5 | `reverse :: [a] -> [a]` | The expected API or type is first and immediately useful. |
| 26 | `qs haskell take while` | 5 | `takeWhile :: (a -> Bool) -> [a] -> [a]` | The expected API or type is first and immediately useful. |
| 27 | `qs haskell dropWhile` | 5 | `dropWhile :: (a -> Bool) -> [a] -> [a]` | The expected API or type is first and immediately useful. |
| 28 | `qs haskell split list at index` | 2 | `Haskell language documentation — split list at index` | Only a generic documentation link or partial match appears. |
| 29 | `qs haskell partition list` | 2 | `Haskell language documentation — partition list` | Only a generic documentation link or partial match appears. |
| 30 | `qs haskell element membership` | 2 | `Haskell language documentation — element membership` | Only a generic documentation link or partial match appears. |
| 31 | `qs haskell association list lookup` | 2 | `Haskell language documentation — association list lookup` | Only a generic documentation link or partial match appears. |
| 32 | `qs haskell maximum by` | 5 | `maximumBy :: Foldable t => (a -> a -> Ordering) -> t a -> a` | The expected API or type is first and immediately useful. |
| 33 | `qs haskell sort on field` | 1 | `Haskell language documentation — sort on field` | The result is barely useful or misleading for the expected target. |
| 34 | `qs haskell group adjacent values` | 2 | `Haskell language documentation — group adjacent values` | Only a generic documentation link or partial match appears. |
| 35 | `qs haskell remove duplicates` | 2 | `Haskell language documentation — remove duplicates` | Only a generic documentation link or partial match appears. |
| 36 | `qs haskell foldr` | 5 | `foldr :: Foldable t => (a -> b -> b) -> b -> t a -> b` | The expected API or type is first and immediately useful. |
| 37 | `qs haskell "foldl'"` | 5 | `foldl' :: Foldable t => (b -> a -> b) -> b -> t a -> b` | The expected API or type is first and immediately useful. |
| 38 | `qs haskell running totals` | 2 | `Haskell language documentation — running totals` | Only a generic documentation link or partial match appears. |
| 39 | `qs haskell infinite iteration` | 2 | `Haskell language documentation — infinite iteration` | Only a generic documentation link or partial match appears. |
| 40 | `qs haskell repeat value n times` | 2 | `Haskell language documentation — repeat value n times` | Only a generic documentation link or partial match appears. |
| 41 | `qs haskell Maybe` | 5 | `data Maybe a` | The expected API or type is first and immediately useful. |
| 42 | `qs haskell default maybe value` | 2 | `Haskell language documentation — default maybe value` | Only a generic documentation link or partial match appears. |
| 43 | `qs haskell map_maybe` | 5 | `mapMaybe :: (a -> Maybe b) -> [a] -> [b]` | The expected API or type is first and immediately useful. |
| 44 | `qs haskell discard Nothing values` | 2 | `Haskell language documentation — discard Nothing values` | Only a generic documentation link or partial match appears. |
| 45 | `qs haskell Either` | 5 | `data Either a b` | The expected API or type is first and immediately useful. |
| 46 | `qs haskell separate lefts and rights` | 2 | `Haskell language documentation — separate lefts and rights` | Only a generic documentation link or partial match appears. |
| 47 | `qs haskell parse string safely` | 2 | `Haskell language documentation — parse string safely` | Only a generic documentation link or partial match appears. |
| 48 | `qs haskell convert value to string` | 2 | `Haskell language documentation — convert value to string` | Only a generic documentation link or partial match appears. |
| 49 | `qs haskell ordering comparison` | 2 | `Haskell language documentation — ordering comparison` | Only a generic documentation link or partial match appears. |
| 50 | `qs haskell enumerate range` | 2 | `Haskell language documentation — enumerate range` | Only a generic documentation link or partial match appears. |
| 51 | `qs haskell Functor` | 5 | `class Functor (f :: Type -> Type)` | The expected API or type is first and immediately useful. |
| 52 | `qs haskell applicatives` | 5 | `class Functor f => Applicative (f :: Type -> Type)` | The expected API or type is first and immediately useful. |
| 53 | `qs haskell monadic bind` | 2 | `Haskell language documentation — monadic bind` | Only a generic documentation link or partial match appears. |
| 54 | `qs haskell Alternative` | 5 | `class Applicative f => Alternative (f :: Type -> Type)` | The expected API or type is first and immediately useful. |
| 55 | `qs haskell foldables` | 1 | `foldable :: (Foldable t, ToJSON a) => t a -> Encoding` | The result is barely useful or misleading for the expected target. |
| 56 | `qs haskell traverse` | 5 | `traverse :: (Traversable t, Applicative f) => (a -> f b) -> t a -> f (t b)` | The expected API or type is first and immediately useful. |
| 57 | `qs haskell Semigroup` | 5 | `class Semigroup a` | The expected API or type is first and immediately useful. |
| 58 | `qs haskell monoids` | 5 | `class Semigroup a => Monoid a` | The expected API or type is first and immediately useful. |
| 59 | `qs haskell bimap` | 5 | `bimap :: Bifunctor p => (a -> b) -> (c -> d) -> p a c -> p b d` | The expected API or type is first and immediately useful. |
| 60 | `qs haskell monad transformers` | 1 | `Haskell language documentation — monad transformers` | The result is barely useful or misleading for the expected target. |
| 61 | `qs haskell Data.Map.Strict.lookup` | 5 | `lookup :: Ord k => k -> Map k a -> Maybe a` | The expected API or type is first and immediately useful. |
| 62 | `qs haskell insert map key` | 2 | `Haskell language documentation — insert map key` | Only a generic documentation link or partial match appears. |
| 63 | `qs haskell integer keyed maps` | 1 | `Haskell language documentation — integer keyed maps` | The result is barely useful or misleading for the expected target. |
| 64 | `qs haskell set intersection` | 3 | `Haskell language documentation — set intersection` | A relevant result exists, but its detail or ranking is weak. |
| 65 | `qs haskell hash maps` | 2 | `Haskell language documentation — hash maps` | Only a generic documentation link or partial match appears. |
| 66 | `qs haskell sequences` | 1 | `sequence :: (Traversable t, Monad m) => t (m a) -> m (t a)` | The result is barely useful or misleading for the expected target. |
| 67 | `qs haskell vectors` | 3 | `vector :: Arbitrary a => Int -> Gen [a]` | A relevant result exists, but its detail or ranking is weak. |
| 68 | `qs haskell byte strings` | 1 | `bytestringsNotEqualButShouldHaveBeenEqual :: ByteString -> ByteString -> IO Assertion` | The result is barely useful or misleading for the expected target. |
| 69 | `qs haskell Text` | 4 | `module GHC.IO.Handle.Text` | The expected target is in the top three with useful context. |
| 70 | `qs haskell split_on` | 5 | `splitOn :: HasCallStack => Text -> Text -> [Text]` | The expected API or type is first and immediately useful. |
| 71 | `qs haskell replace substring` | 1 | `Haskell language documentation — replace substring` | The result is barely useful or misleading for the expected target. |
| 72 | `qs haskell encode utf8` | 4 | `utf8EncodeByteArray# :: String -> ByteArray#` | The expected target is in the top three with useful context. |
| 73 | `qs haskell handle invalid Unicode` | 1 | `Haskell language documentation — handle invalid Unicode` | The result is barely useful or misleading for the expected target. |
| 74 | `qs haskell parse JSON` | 4 | `parseIndexedJSON :: (Value -> Parser a) -> Int -> Value -> Parser a` | The expected target is in the top three with useful context. |
| 75 | `qs haskell generic programming` | 2 | `Haskell language documentation — generic programming` | Only a generic documentation link or partial match appears. |
| 76 | `qs haskell lens view set` | 1 | `Haskell language documentation — lens view set` | The result is barely useful or misleading for the expected target. |
| 77 | `qs haskell putStrLn` | 5 | `putStrLn :: String -> IO ()` | The expected API or type is first and immediately useful. |
| 78 | `qs haskell readFile` | 5 | `readFile :: FilePath -> IO String` | The expected API or type is first and immediately useful. |
| 79 | `qs haskell write text file` | 4 | `Haskell language documentation — write text file` | The expected target is in the top three with useful context. |
| 80 | `qs haskell safe file handle` | 2 | `Haskell language documentation — safe file handle` | Only a generic documentation link or partial match appears. |
| 81 | `qs haskell command line arguments` | 1 | `Haskell language documentation — command line arguments` | The result is barely useful or misleading for the expected target. |
| 82 | `qs haskell environment variables` | 1 | `Haskell language documentation — environment variables` | The result is barely useful or misleading for the expected target. |
| 83 | `qs haskell list directory` | 5 | `listDirectory :: FilePath -> IO [FilePath]` | The expected API or type is first and immediately useful. |
| 84 | `qs haskell check file exists` | 1 | `Haskell language documentation — check file exists` | The result is barely useful or misleading for the expected target. |
| 85 | `qs haskell file extension` | 3 | `Haskell language documentation — file extension` | A relevant result exists, but its detail or ranking is weak. |
| 86 | `qs haskell temporary files` | 1 | `Haskell language documentation — temporary files` | The result is barely useful or misleading for the expected target. |
| 87 | `qs haskell exceptions` | 2 | `exception :: Exception a => Prism' SomeException a` | Only a generic documentation link or partial match appears. |
| 88 | `qs haskell bracket resource cleanup` | 1 | `Haskell language documentation — bracket resource cleanup` | The result is barely useful or misleading for the expected target. |
| 89 | `qs haskell async concurrency` | 1 | `Haskell language documentation — async concurrency` | The result is barely useful or misleading for the expected target. |
| 90 | `qs haskell forkIO` | 5 | `forkIO :: IO () -> IO ThreadId` | The expected API or type is first and immediately useful. |
| 91 | `qs haskell MVar` | 5 | `module Control.Concurrent.MVar` | The expected API or type is first and immediately useful. |
| 92 | `qs haskell software transactional memory` | 1 | `Haskell language documentation — software transactional memory` | The result is barely useful or misleading for the expected target. |
| 93 | `qs haskell operation timeout` | 1 | `Haskell language documentation — operation timeout` | The result is barely useful or misleading for the expected target. |
| 94 | `qs haskell run external process` | 1 | `Haskell language documentation — run external process` | The result is barely useful or misleading for the expected target. |
| 95 | `qs haskell random numbers` | 1 | `Haskell language documentation — random numbers` | The result is barely useful or misleading for the expected target. |
| 96 | `qs haskell HTTP client request` | 1 | `Haskell language documentation — HTTP client request` | The result is barely useful or misleading for the expected target. |
| 97 | `qs haskell QuickCheck properties` | 1 | `Haskell language documentation — QuickCheck properties` | The result is barely useful or misleading for the expected target. |
| 98 | `qs haskell Hspec` | 5 | `module Test.Hspec` | The expected API or type is first and immediately useful. |
| 99 | `qs haskell GHCi commands` | 1 | `Haskell language documentation — GHCi commands` | The result is barely useful or misleading for the expected target. |
| 100 | `qs haskell language extensions` | 3 | `Haskell language documentation — language extensions` | A relevant result exists, but its detail or ranking is weak. |
