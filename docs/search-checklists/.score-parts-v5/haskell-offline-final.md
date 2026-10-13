# Haskell offline-final score report

## Summary

- Binary: `/tmp/qs-gate/release/qs`.
- Commands run: **100/100**.
- Per-command timeout: **30 seconds**.
- Average: **2.81/5**.
- Median: **3/5**.
- Distribution: **0: 0**, **1: 32**, **2: 16**, **3: 19**, **4: 5**, **5: 28**.
- Practical pass rate (score >=3): **52/100 (52%)**.
- Strong-result rate (score >=4): **33/100 (33%)**.
- Execution errors: **0**. Timeouts: **0**.

## Comparison with previous v5 run

- Previous report: `.score-parts-v5/haskell.md`.
- Previous metrics: 2.76 average, 2 median, distribution 0: 0, 1: 40, 2: 12, 3: 13, 4: 2, 5: 33, 48% >=3, 35% >=4, 0 errors, and 0 timeouts.
- Offline-final change: **+0.05 average points**, **+1 median point**, **+4 percentage points at >=3**, and **-2 percentage points at >=4**. Errors and timeouts remain at zero.
- Broad language concepts now consistently open the Haskell 2010 Report, while many package-qualified and natural-task searches still rank unrelated base APIs. Direct Prelude and `Data.List` APIs remain the strongest area.

## Results

| # | Command | Score | Observed top result or error | Brief user assessment |
|---:|---|---:|---|---|
| 1 | `qs haskell lists` | 3 | `Haskell language documentation — lists` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 2 | `qs haskell tuples` | 3 | `Haskell language documentation — tuples` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 3 | `qs haskell pattern matching` | 3 | `Haskell language documentation — pattern matching` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 4 | `qs haskell guards` | 3 | `Haskell language documentation — guards` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 5 | `qs haskell list comprehensions` | 3 | `Haskell language documentation — list comprehensions` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 6 | `qs haskell type classes` | 3 | `Haskell language documentation — type classes` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 7 | `qs haskell algebraic data types` | 3 | `Haskell language documentation — algebraic data types` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 8 | `qs haskell newtypes` | 3 | `Haskell language documentation — newtypes` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 9 | `qs haskell record syntax` | 3 | `Haskell language documentation — record syntax` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 10 | `qs haskell deriving` | 3 | `Haskell language documentation — deriving` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 11 | `qs haskell modules` | 3 | `Haskell language documentation — modules` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 12 | `qs haskell qualified imports` | 3 | `Haskell language documentation — qualified imports` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 13 | `qs haskell infix operators` | 3 | `Haskell language documentation — infix operators` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 14 | `qs haskell strict evaluation` | 3 | `Haskell language documentation — strict evaluation` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 15 | `qs haskell thunks` | 2 | `Haskell language documentation — thunks` | The results are related, but they do not directly provide Lazy evaluation and deferred computations. |
| 16 | `qs haskell tail recursion` | 2 | `Haskell language documentation — tail recursion` | The results are related, but they do not directly provide Tail-recursive function design. |
| 17 | `qs haskell currying` | 3 | `Haskell language documentation — currying` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 18 | `qs haskell partial application` | 3 | `Haskell language documentation — partial application` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 19 | `qs haskell higher order functions` | 3 | `Haskell language documentation — higher order functions` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 20 | `qs haskell function composition` | 3 | `Haskell language documentation — function composition` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 21 | `qs haskell map` | 5 | `map :: (a -> b) -> [a] -> [b]` | The expected target is first with a useful signature, description, or direct documentation. |
| 22 | `qs haskell filter` | 5 | `filter :: (a -> Bool) -> [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 23 | `qs haskell zipWith` | 5 | `zipWith :: (a -> b -> c) -> [a] -> [b] -> [c]` | The expected target is first with a useful signature, description, or direct documentation. |
| 24 | `qs haskell concat map` | 5 | `concatMap :: (a -> [b]) -> [a] -> [b]` | The expected target is first with a useful signature, description, or direct documentation. |
| 25 | `qs haskell reverse` | 5 | `reverse :: [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 26 | `qs haskell take while` | 1 | `unfoldr :: (b -> Maybe (a, b)) -> b -> [a]` | The ranking is misleading or barely useful; it does not provide \`Prelude.takeWhile\`. |
| 27 | `qs haskell dropWhile` | 5 | `dropWhile :: (a -> Bool) -> [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 28 | `qs haskell split list at index` | 5 | `splitAt :: Int -> [a] -> ([a], [a])` | The expected target is first with a useful signature, description, or direct documentation. |
| 29 | `qs haskell partition list` | 5 | `partition :: (a -> Bool) -> [a] -> ([a], [a])` | The expected target is first with a useful signature, description, or direct documentation. |
| 30 | `qs haskell element membership` | 5 | `elem :: (Foldable t, Eq a) => a -> t a -> Bool` | The expected target is first with a useful signature, description, or direct documentation. |
| 31 | `qs haskell association list lookup` | 5 | `lookup :: Eq a => a -> [(a, b)] -> Maybe b` | The expected target is first with a useful signature, description, or direct documentation. |
| 32 | `qs haskell maximum by` | 1 | `[max_mem_in_use_bytes] :: RTSStats -> Word64` | The ranking is misleading or barely useful; it does not provide \`Data.List.maximumBy\`. |
| 33 | `qs haskell sort on field` | 5 | `sortOn :: Ord b => (a -> b) -> [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 34 | `qs haskell group adjacent values` | 5 | `group :: Eq a => [a] -> [[a]]` | The expected target is first with a useful signature, description, or direct documentation. |
| 35 | `qs haskell remove duplicates` | 5 | `nub :: Eq a => [a] -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 36 | `qs haskell foldr` | 5 | `foldr :: Foldable t => (a -> b -> b) -> b -> t a -> b` | The expected target is first with a useful signature, description, or direct documentation. |
| 37 | `qs haskell "foldl'"` | 5 | `foldl' :: Foldable t => (b -> a -> b) -> b -> t a -> b` | The expected target is first with a useful signature, description, or direct documentation. |
| 38 | `qs haskell running totals` | 5 | `scanl :: (b -> a -> b) -> b -> [a] -> [b]` | The expected target is first with a useful signature, description, or direct documentation. |
| 39 | `qs haskell infinite iteration` | 5 | `iterate :: (a -> a) -> a -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 40 | `qs haskell repeat value n times` | 5 | `replicate :: Int -> a -> [a]` | The expected target is first with a useful signature, description, or direct documentation. |
| 41 | `qs haskell Maybe` | 2 | `maybe :: b -> (a -> b) -> Maybe a -> b` | The results are related, but they do not directly provide \`Maybe\`. |
| 42 | `qs haskell default maybe value` | 1 | `(<$) :: Functor f => a -> f b -> f a` | The ranking is misleading or barely useful; it does not provide \`Data.Maybe.fromMaybe\`. |
| 43 | `qs haskell map_maybe` | 5 | `mapMaybe :: (a -> Maybe b) -> [a] -> [b]` | The expected target is first with a useful signature, description, or direct documentation. |
| 44 | `qs haskell discard Nothing values` | 4 | `void :: Functor f => f a -> f ()` | The expected target is in the top three with useful context. |
| 45 | `qs haskell Either` | 2 | `either :: (a -> c) -> (b -> c) -> Either a b -> c` | The results are related, but they do not directly provide \`Either\`. |
| 46 | `qs haskell separate lefts and rights` | 5 | `partitionEithers :: [Either a b] -> ([a], [b])` | The expected target is first with a useful signature, description, or direct documentation. |
| 47 | `qs haskell parse string safely` | 2 | `maybe :: b -> (a -> b) -> Maybe a -> b` | The results are related, but they do not directly provide \`Text.Read.readMaybe\`. |
| 48 | `qs haskell convert value to string` | 1 | `fmap :: Functor f => (a -> b) -> f a -> f b` | The ranking is misleading or barely useful; it does not provide \`Show\` and \`show\`. |
| 49 | `qs haskell ordering comparison` | 2 | `maximumBy :: Foldable t => (a -> a -> Ordering) -> t a -> a` | The results are related, but they do not directly provide \`Ord\` and \`compare\`. |
| 50 | `qs haskell enumerate range` | 1 | `isAlphaNum :: Char -> Bool` | The ranking is misleading or barely useful; it does not provide \`Enum\` and range syntax. |
| 51 | `qs haskell Functor` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Functor\` and \`fmap\`. |
| 52 | `qs haskell applicatives` | 1 | `wai` | The ranking is misleading or barely useful; it does not provide \`Applicative\`, \`pure\`, and \`<*>\`. |
| 53 | `qs haskell monadic bind` | 4 | `foldrM :: (Foldable t, Monad m) => (a -> b -> m b) -> b -> t a -> m b` | The expected target is in the top three with useful context. |
| 54 | `qs haskell Alternative` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Control.Applicative.Alternative\`. |
| 55 | `qs haskell foldables` | 1 | `safe` | The ranking is misleading or barely useful; it does not provide \`Data.Foldable.Foldable\`. |
| 56 | `qs haskell traverse` | 5 | `traverse :: (Traversable t, Applicative f) => (a -> f b) -> t a -> f (t b)` | The expected target is first with a useful signature, description, or direct documentation. |
| 57 | `qs haskell Semigroup` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Semigroup\` and \`(<>)\`. |
| 58 | `qs haskell monoids` | 2 | `monoids` | The results are related, but they do not directly provide \`Monoid\`, \`mempty\`, and \`mappend\`. |
| 59 | `qs haskell bimap` | 5 | `bimap :: Bifunctor p => (a -> b) -> (c -> d) -> p a c -> p b d` | The expected target is first with a useful signature, description, or direct documentation. |
| 60 | `qs haskell monad transformers` | 2 | `liftIO :: MonadIO m => IO a -> m a` | The results are related, but they do not directly provide Transformer types and \`lift\`. |
| 61 | `qs haskell Data.Map.Strict.lookup` | 5 | `lookup :: Ord k => k -> Map k a -> Maybe a` | The expected target is first with a useful signature, description, or direct documentation. |
| 62 | `qs haskell insert map key` | 4 | `lookup :: Int -> Seq a -> Maybe a` | The expected target is in the top three with useful context. |
| 63 | `qs haskell integer keyed maps` | 1 | `mapException :: (Exception e1, Exception e2) => (e1 -> e2) -> a -> a` | The ranking is misleading or barely useful; it does not provide \`Data.IntMap\`. |
| 64 | `qs haskell set intersection` | 2 | `mergeA :: (Applicative f, Ord a) => WhenMissing f a -> WhenMissing f a -> WhenMatched f a -> Set a -> Set a -> f (Set a)` | The results are related, but they do not directly provide \`Data.Set.intersection\`. |
| 65 | `qs haskell hash maps` | 1 | `mapException :: (Exception e1, Exception e2) => (e1 -> e2) -> a -> a` | The ranking is misleading or barely useful; it does not provide \`Data.HashMap\` from \`unordered-containers\`. |
| 66 | `qs haskell sequences` | 2 | `compact-sequences` | The results are related, but they do not directly provide \`Data.Sequence.Seq\`. |
| 67 | `qs haskell vectors` | 2 | `vinyl-vectors` | The results are related, but they do not directly provide \`Data.Vector.Vector\`. |
| 68 | `qs haskell byte strings` | 1 | `foldMap' :: (Foldable t, Monoid m) => (a -> m) -> t a -> m` | The ranking is misleading or barely useful; it does not provide \`Data.ByteString.ByteString\`. |
| 69 | `qs haskell Text` | 1 | `Text :: Symbol -> ErrorMessage` | The ranking is misleading or barely useful; it does not provide \`Data.Text.Text\`. |
| 70 | `qs haskell split_on` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Data.Text.splitOn\`. |
| 71 | `qs haskell replace substring` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Data.Text.replace\`. |
| 72 | `qs haskell encode utf8` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Data.Text.Encoding.encodeUtf8\`. |
| 73 | `qs haskell handle invalid Unicode` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Data.Text.Encoding.decodeUtf8With\` and \`lenientDecode\`. |
| 74 | `qs haskell parse JSON` | 1 | `isLeft :: Either a b -> Bool` | The ranking is misleading or barely useful; it does not provide \`Data.Aeson.eitherDecode\` / \`FromJSON\`. |
| 75 | `qs haskell generic programming` | 2 | `gmapT :: Data a => (forall b. Data b => b -> b) -> a -> a` | The results are related, but they do not directly provide \`GHC.Generics.Generic\`. |
| 76 | `qs haskell lens view set` | 2 | `alterF :: (Ord a, Functor f) => (Bool -> f Bool) -> a -> Set a -> f (Set a)` | The results are related, but they do not directly provide \`Control.Lens.view\`, \`set\`, and lens basics. |
| 77 | `qs haskell putStrLn` | 5 | `putStrLn :: String -> IO ()` | The expected target is first with a useful signature, description, or direct documentation. |
| 78 | `qs haskell readFile` | 5 | `readFile :: FilePath -> IO String` | The expected target is first with a useful signature, description, or direct documentation. |
| 79 | `qs haskell write text file` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`writeFile\`. |
| 80 | `qs haskell safe file handle` | 2 | `hWaitForInput :: Handle -> Int -> IO Bool` | The results are related, but they do not directly provide \`System.IO.withFile\`. |
| 81 | `qs haskell command line arguments` | 3 | `getFullArgs :: IO [String]` | A relevant result is present, but the page or ranking gives weak query-specific detail. |
| 82 | `qs haskell environment variables` | 1 | `unsafePerformIO :: IO a -> a` | The ranking is misleading or barely useful; it does not provide \`System.Environment.lookupEnv\` / \`getEnv\`. |
| 83 | `qs haskell list directory` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`System.Directory.listDirectory\`. |
| 84 | `qs haskell check file exists` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`System.Directory.doesFileExist\`. |
| 85 | `qs haskell file extension` | 1 | `Biographical profile sample event` | The ranking is misleading or barely useful; it does not provide \`System.FilePath.takeExtension\` / \`replaceExtension\`. |
| 86 | `qs haskell temporary files` | 1 | `withCString :: String -> (CString -> IO a) -> IO a` | The ranking is misleading or barely useful; it does not provide \`System.IO.Temp\`. |
| 87 | `qs haskell exceptions` | 2 | `exceptions` | The results are related, but they do not directly provide \`Control.Exception\`. |
| 88 | `qs haskell bracket resource cleanup` | 4 | `mask :: ((forall a. () => IO a -> IO a) -> IO b) -> IO b` | The expected target is in the top three with useful context. |
| 89 | `qs haskell async concurrency` | 2 | `forkIO :: IO () -> IO ThreadId` | The results are related, but they do not directly provide \`Control.Concurrent.Async\`. |
| 90 | `qs haskell forkIO` | 5 | `forkIO :: IO () -> IO ThreadId` | The expected target is first with a useful signature, description, or direct documentation. |
| 91 | `qs haskell MVar` | 5 | `MVar :: MVar# RealWorld a -> MVar a` | The expected target is first with a useful signature, description, or direct documentation. |
| 92 | `qs haskell software transactional memory` | 1 | `HeapOverflow :: AsyncException` | The ranking is misleading or barely useful; it does not provide \`Control.Concurrent.STM\`. |
| 93 | `qs haskell operation timeout` | 1 | `(<+>) :: ArrowPlus a => a b c -> a b c -> a b c` | The ranking is misleading or barely useful; it does not provide \`System.Timeout.timeout\`. |
| 94 | `qs haskell run external process` | 1 | `openFileBlocking :: FilePath -> IOMode -> IO Handle` | The ranking is misleading or barely useful; it does not provide \`System.Process\` / \`readProcess\`. |
| 95 | `qs haskell random numbers` | 1 | `isAlphaNum :: Char -> Bool` | The ranking is misleading or barely useful; it does not provide \`System.Random\` and \`randomRIO\`. |
| 96 | `qs haskell HTTP client request` | 1 | `forever :: Applicative f => f a -> f b` | The ranking is misleading or barely useful; it does not provide \`Network.HTTP.Client\`. |
| 97 | `qs haskell QuickCheck properties` | 1 | `takeMVar :: MVar a -> IO a` | The ranking is misleading or barely useful; it does not provide \`Test.QuickCheck\` and \`property\`. |
| 98 | `qs haskell Hspec` | 1 | `arr :: Arrow a => (b -> c) -> a b c` | The ranking is misleading or barely useful; it does not provide \`Test.Hspec\`. |
| 99 | `qs haskell GHCi commands` | 4 | `Documenting command-line options and GHCi commands` | The expected target is in the top three with useful context. |
| 100 | `qs haskell language extensions` | 5 | `GHC language extensions — language extensions` | The expected target is first with a useful signature, description, or direct documentation. |
