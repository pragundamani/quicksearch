# Haskell candidate-v5 score report

## Summary

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **30 seconds**.
- Average: **2.76/5**.
- Median: **2/5**.
- Distribution: **0: 0**, **1: 40**, **2: 12**, **3: 13**, **4: 2**, **5: 33**.
- Practical pass rate (score >=3): **48/100 (48%)**.
- Strong-result rate (score >=4): **35/100 (35%)**.
- Execution errors: **0**. Timeouts: **0**.

## Comparison

- No dedicated `.score-parts-v3/haskell.md` report exists. The v3 summary in `language_scores.md` uses the same source-specific Haskell candidate reported in `.score-parts-after/haskell.md`.
- **Prior validated candidate:** 2.79 average, 2 median, distribution 0: 0, 1: 28, 2: 25, 3: 15, 4: 4, 5: 28, 47% >=3, 32% >=4, 0 errors, and 0 timeouts.
- **Candidate-v5 change:** **-0.03 average points**, **no median change**, **+1 percentage point at >=3**, and **+3 percentage points at >=4**. Errors and timeouts remain at zero.
- The run is more polarized. Several natural task aliases now resolve directly to the expected API, but common bare or plural terms often route to Hackage package search or the GHC guide instead of Hoogle API results.

## Results

| # | Command | Score | Observed top result or error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs haskell lists` | 2 | `lists` | The result is related, but it does not directly provide List type, syntax, and Data.List. |
| 2 | `qs haskell tuples` | 2 | `tuples` | The result is related, but it does not directly provide Tuple types and tuple syntax. |
| 3 | `qs haskell pattern matching` | 3 | `Haskell language documentation — pattern matching` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 4 | `qs haskell guards` | 1 | `cml` | The ranking is misleading or barely useful; it does not provide Guard syntax. |
| 5 | `qs haskell list comprehensions` | 3 | `Haskell language documentation — list comprehensions` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 6 | `qs haskell type classes` | 3 | `Haskell language documentation — type classes` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 7 | `qs haskell algebraic data types` | 3 | `Haskell language documentation — algebraic data types` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 8 | `qs haskell newtypes` | 2 | `ANum` | The result is related, but it does not directly provide newtype declarations. |
| 9 | `qs haskell record syntax` | 3 | `Haskell language documentation — record syntax` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 10 | `qs haskell deriving` | 2 | `deriving-aeson` | The result is related, but it does not directly provide Derived instances and deriving strategies. |
| 11 | `qs haskell modules` | 1 | `file-modules` | The ranking is misleading or barely useful; it does not provide Module declarations and exports. |
| 12 | `qs haskell qualified imports` | 3 | `Haskell language documentation — qualified imports` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 13 | `qs haskell infix operators` | 3 | `Haskell language documentation — infix operators` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 14 | `qs haskell strict evaluation` | 3 | `Haskell language documentation — strict evaluation` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 15 | `qs haskell thunks` | 2 | `nothunks` | The result is related, but it does not directly provide Lazy evaluation and deferred computations. |
| 16 | `qs haskell tail recursion` | 2 | `Haskell language documentation — tail recursion` | The result is related, but it does not directly provide Tail-recursive function design. |
| 17 | `qs haskell currying` | 1 | `nat` | The ranking is misleading or barely useful; it does not provide Curried functions. |
| 18 | `qs haskell partial application` | 3 | `Haskell language documentation — partial application` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 19 | `qs haskell higher order functions` | 3 | `Haskell language documentation — higher order functions` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 20 | `qs haskell function composition` | 3 | `Haskell language documentation — function composition` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 21 | `qs haskell map` | 1 | `Map` | The ranking is misleading or barely useful; it does not provide Prelude.map. |
| 22 | `qs haskell filter` | 1 | `filter-logger` | The ranking is misleading or barely useful; it does not provide Prelude.filter. |
| 23 | `qs haskell zipWith` | 5 | `zipWith :: (a -> b -> c) -> [a] -> [b] -> [c]` | The expected target is first with a useful signature, description, or direct documentation. |
| 24 | `qs haskell concat map` | 1 | `Direct Mapping` | The ranking is misleading or barely useful; it does not provide Data.List.concatMap. |
| 25 | `qs haskell reverse` | 2 | `reverse-list` | The result is related, but it does not directly provide Prelude.reverse. |
| 26 | `qs haskell take while` | 1 | `The data keyword in import/export lists` | The ranking is misleading or barely useful; it does not provide Prelude.takeWhile. |
| 27 | `qs haskell dropWhile` | 5 | `dropWhile :: (a -> Bool) -> [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 28 | `qs haskell split list at index` | 5 | `splitAt :: Int -> [a] -> ([a], [a])` | The expected target is first with a useful signature, description, or direct documentation. |
| 29 | `qs haskell partition list` | 5 | `partition :: (a -> Bool) -> [a] -> ([a], [a])` | The expected target is first with a useful signature, description, or direct documentation. |
| 30 | `qs haskell element membership` | 5 | `elem :: (Foldable t, Eq a) => a -> t a -> Bool` | The expected target is first with a useful signature, description, or direct documentation. |
| 31 | `qs haskell association list lookup` | 5 | `lookup :: Eq a => a -> [(a, b)] -> Maybe b` | The expected target is first with a useful signature, description, or direct documentation. |
| 32 | `qs haskell maximum by` | 1 | `Inserting cost centres by hand` | The ranking is misleading or barely useful; it does not provide Data.List.maximumBy. |
| 33 | `qs haskell sort on field` | 5 | `sortOn :: Ord b => (a -> b) -> [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 34 | `qs haskell group adjacent values` | 5 | `group :: Eq a => [a] -> [[a]]` | The expected target is first with a useful signature, description, or direct documentation. |
| 35 | `qs haskell remove duplicates` | 5 | `nub :: Eq a => [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 36 | `qs haskell foldr` | 1 | `fugue` | The ranking is misleading or barely useful; it does not provide Prelude.foldr. |
| 37 | `qs haskell "foldl'"` | 5 | `foldl' :: Foldable t => (b -> a -> b) -> b -> t a -> b` | The expected target is first with a useful signature, description, or direct documentation. |
| 38 | `qs haskell running totals` | 5 | `scanl :: (b -> a -> b) -> b -> [a] -> [b]` | The expected target is first with a useful signature, description, or direct documentation. |
| 39 | `qs haskell infinite iteration` | 5 | `iterate :: (a -> a) -> a -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 40 | `qs haskell repeat value n times` | 5 | `replicate :: Int -> a -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 41 | `qs haskell Maybe` | 5 | `data Maybe a` | The expected target is first with a useful signature, description, or direct documentation. |
| 42 | `qs haskell default maybe value` | 1 | `Annotating values` | The ranking is misleading or barely useful; it does not provide Data.Maybe.fromMaybe. |
| 43 | `qs haskell map_maybe` | 5 | `mapMaybe :: (a -> Maybe b) -> [a] -> [b]` | The expected target is first with a useful signature, description, or direct documentation. |
| 44 | `qs haskell discard Nothing values` | 1 | `Annotating values` | The ranking is misleading or barely useful; it does not provide Data.Maybe.catMaybes. |
| 45 | `qs haskell Either` | 5 | `data Either a b` | The expected target is first with a useful signature, description, or direct documentation. |
| 46 | `qs haskell separate lefts and rights` | 1 | `Filenames and separate compilation` | The ranking is misleading or barely useful; it does not provide Data.Either.partitionEithers. |
| 47 | `qs haskell parse string safely` | 1 | `CPP and string gaps` | The ranking is misleading or barely useful; it does not provide Text.Read.readMaybe. |
| 48 | `qs haskell convert value to string` | 1 | `Annotating values` | The ranking is misleading or barely useful; it does not provide Show and show. |
| 49 | `qs haskell ordering comparison` | 1 | `Ordering of specified variables` | The ranking is misleading or barely useful; it does not provide Ord and compare. |
| 50 | `qs haskell enumerate range` | 2 | `Haskell language documentation — enumerate range` | The result is related, but it does not directly provide Enum and range syntax. |
| 51 | `qs haskell Functor` | 5 | `class Functor (f :: Type -> Type)` | The expected target is first with a useful signature, description, or direct documentation. |
| 52 | `qs haskell applicatives` | 1 | `wai` | The ranking is misleading or barely useful; it does not provide Applicative, pure, and <*>. |
| 53 | `qs haskell monadic bind` | 1 | `Bindings` | The ranking is misleading or barely useful; it does not provide Monad and (>>=). |
| 54 | `qs haskell Alternative` | 5 | `class Applicative f => Alternative (f :: Type -> Type)` | The expected target is first with a useful signature, description, or direct documentation. |
| 55 | `qs haskell foldables` | 1 | `safe` | The ranking is misleading or barely useful; it does not provide Data.Foldable.Foldable. |
| 56 | `qs haskell traverse` | 1 | `traverse-code` | The ranking is misleading or barely useful; it does not provide Data.Traversable.traverse. |
| 57 | `qs haskell Semigroup` | 5 | `class Semigroup a` | The expected target is first with a useful signature, description, or direct documentation. |
| 58 | `qs haskell monoids` | 2 | `monoids` | The result is related, but it does not directly provide Monoid, mempty, and mappend. |
| 59 | `qs haskell bimap` | 1 | `bimap` | The ranking is misleading or barely useful; it does not provide Data.Bifunctor.bimap. |
| 60 | `qs haskell monad transformers` | 1 | `Building secure systems (restricted IO Monads)` | The ranking is misleading or barely useful; it does not provide Transformer types and lift. |
| 61 | `qs haskell Data.Map.Strict.lookup` | 5 | `lookup :: Ord k => k -> Map k a -> Maybe a` | The expected target is first with a useful signature, description, or direct documentation. |
| 62 | `qs haskell insert map key` | 1 | `Direct Mapping` | The ranking is misleading or barely useful; it does not provide Data.Map.insert. |
| 63 | `qs haskell integer keyed maps` | 1 | `Binary integer literals` | The ranking is misleading or barely useful; it does not provide Data.IntMap. |
| 64 | `qs haskell set intersection` | 1 | `Capability sets` | The ranking is misleading or barely useful; it does not provide Data.Set.intersection. |
| 65 | `qs haskell hash maps` | 1 | `The magic hash` | The ranking is misleading or barely useful; it does not provide Data.HashMap from unordered-containers. |
| 66 | `qs haskell sequences` | 2 | `compact-sequences` | The result is related, but it does not directly provide Data.Sequence.Seq. |
| 67 | `qs haskell vectors` | 2 | `vinyl-vectors` | The result is related, but it does not directly provide Data.Vector.Vector. |
| 68 | `qs haskell byte strings` | 1 | `Overloaded strings` | The ranking is misleading or barely useful; it does not provide Data.ByteString.ByteString. |
| 69 | `qs haskell Text` | 4 | `module GHC.IO.Handle.Text` | The expected target is in the top three with useful context, but it does not rank first. |
| 70 | `qs haskell split_on` | 5 | `splitOn :: HasCallStack => Text -> Text -> [Text]` | The expected target is first with a useful signature, description, or direct documentation. |
| 71 | `qs haskell replace substring` | 5 | `replace :: HasCallStack => Text -> Text -> Text -> Text` | The expected target is first with a useful signature, description, or direct documentation. |
| 72 | `qs haskell encode utf8` | 5 | `encodeUtf8 :: Text -> ByteString` | The expected target is first with a useful signature, description, or direct documentation. |
| 73 | `qs haskell handle invalid Unicode` | 5 | `decodeUtf8With :: OnDecodeError -> ByteString -> Text` | The expected target is first with a useful signature, description, or direct documentation. |
| 74 | `qs haskell parse JSON` | 1 | `JSON profile format` | The ranking is misleading or barely useful; it does not provide Data.Aeson.eitherDecode / FromJSON. |
| 75 | `qs haskell generic programming` | 3 | `Generic programming` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 76 | `qs haskell lens view set` | 1 | `Capability sets` | The ranking is misleading or barely useful; it does not provide Control.Lens.view, set, and lens basics. |
| 77 | `qs haskell putStrLn` | 5 | `putStrLn :: String -> IO ()` | The expected target is first with a useful signature, description, or direct documentation. |
| 78 | `qs haskell readFile` | 5 | `readFile :: FilePath -> IO String` | The expected target is first with a useful signature, description, or direct documentation. |
| 79 | `qs haskell write text file` | 5 | `writeFile :: FilePath -> Text -> IO ()` | The expected target is first with a useful signature, description, or direct documentation. |
| 80 | `qs haskell safe file handle` | 1 | `Biographical profile sample event` | The ranking is misleading or barely useful; it does not provide System.IO.withFile. |
| 81 | `qs haskell command line arguments` | 1 | `Command-line arguments` | The ranking is misleading or barely useful; it does not provide System.Environment.getArgs. |
| 82 | `qs haskell environment variables` | 1 | `Other environment variables` | The ranking is misleading or barely useful; it does not provide System.Environment.lookupEnv / getEnv. |
| 83 | `qs haskell list directory` | 5 | `listDirectory :: FilePath -> IO [FilePath]` | The expected target is first with a useful signature, description, or direct documentation. |
| 84 | `qs haskell check file exists` | 5 | `doesFileExist :: FilePath -> IO Bool` | The expected target is first with a useful signature, description, or direct documentation. |
| 85 | `qs haskell file extension` | 1 | `Biographical profile sample event` | The ranking is misleading or barely useful; it does not provide System.FilePath.takeExtension / replaceExtension. |
| 86 | `qs haskell temporary files` | 1 | `Redirecting temporary files` | The ranking is misleading or barely useful; it does not provide System.IO.Temp. |
| 87 | `qs haskell exceptions` | 2 | `exceptions` | The result is related, but it does not directly provide Control.Exception. |
| 88 | `qs haskell bracket resource cleanup` | 1 | `To GHC resources` | The ranking is misleading or barely useful; it does not provide Control.Exception.bracket. |
| 89 | `qs haskell async concurrency` | 1 | `Interaction with async exception` | The ranking is misleading or barely useful; it does not provide Control.Concurrent.Async. |
| 90 | `qs haskell forkIO` | 5 | `forkIO :: IO () -> IO ThreadId` | The expected target is first with a useful signature, description, or direct documentation. |
| 91 | `qs haskell MVar` | 5 | `module Control.Concurrent.MVar` | The expected target is first with a useful signature, description, or direct documentation. |
| 92 | `qs haskell software transactional memory` | 3 | `Software Transactional Memory` | The first result is relevant, but it is a broad documentation page with weak query-specific detail. |
| 93 | `qs haskell operation timeout` | 1 | `Alternative modes of operation` | The ranking is misleading or barely useful; it does not provide System.Timeout.timeout. |
| 94 | `qs haskell run external process` | 1 | `Running the interpreter in a separate process` | The ranking is misleading or barely useful; it does not provide System.Process / readProcess. |
| 95 | `qs haskell random numbers` | 1 | `Numbers, basic types, and built-in classes` | The ranking is misleading or barely useful; it does not provide System.Random and randomRIO. |
| 96 | `qs haskell HTTP client request` | 1 | `Client code that discriminates among back ends` | The ranking is misleading or barely useful; it does not provide Network.HTTP.Client. |
| 97 | `qs haskell QuickCheck properties` | 1 | `Ambiguous types and the ambiguity check` | The ranking is misleading or barely useful; it does not provide Test.QuickCheck and property. |
| 98 | `qs haskell Hspec` | 5 | `module Test.Hspec` | The expected target is first with a useful signature, description, or direct documentation. |
| 99 | `qs haskell GHCi commands` | 4 | `Documenting command-line options and GHCi commands` | The expected target is in the top three with useful context, but it does not rank first. |
| 100 | `qs haskell language extensions` | 5 | `GHC language extensions — language extensions` | The expected target is first with a useful signature, description, or direct documentation. |
