# Haskell held-out offline delta evaluation

- Binary: `/tmp/qs-gate/release/qs`
- Manifest: `/home/paragon/settings-configs/scripts/quicksearch/docs/search-checklists/heldout_manifest.json`
- Scope: all 27 entries whose manifest `language` is `haskell`; indices remain global.
- Execution: `--json`, 30-second timeout per query.
- Rubric: 5 expected first/useful; 4 expected top 3/useful; 3 relevant but weak rank/detail; 2 partial; 1 barely useful/misleading; 0 no relevant result/error.

## Summary

- Queries: **27/27**
- Total: **82/135**
- Average: **3.04/5**
- Score >=3: **19/27 (70.4%)**
- Score >=4: **13/27 (48.1%)**
- Distribution: **0: 8, 1: 0, 2: 0, 3: 6, 4: 1, 5: 12**
- Execution errors: **0**
- Timeouts: **0**

## Evaluated rows

| Global index | Class | Query | Expected | Top result | Assessment | Contrast check | Exit/error | Score |
|---:|---|---|---|---|---|---|---|---:|
| 5 | paraphrase | `qs haskell apply function to first tuple component` | `Data.Bifunctor.first` | atomicModifyIORef2Lazy :: IORef a -> (a -> (a, b)) -> IO (a, (a, b)) | No `Data.Bifunctor.first`; results are unrelated or generic functions. | — | 0 | **0** |
| 11 | paraphrase | `qs haskell attach an index to every traversable value` | `Data.Traversable.mapAccumL` | fmap :: Functor f => (a -> b) -> f a -> f b | Exact `mapAccumL` is result 4 with a useful signature and description. | — | 0 | **3** |
| 17 | paraphrase | `qs haskell combine maps and resolve duplicate keys` | `Data.Map.unionWith` | merge :: Ord k => SimpleWhenMissing k a c -> SimpleWhenMissing k b c -> SimpleWhenMatched k a b c -> Map k a -> Map k b -> Map k c | Result 3 is `union` and its snippet names `unionWith`; useful but the expected API is not a result. | — | 0 | **3** |
| 23 | paraphrase | `qs haskell run action only when condition is true` | `Control.Monad.when` | unsafeDupablePerformIO :: IO a -> a | No `Control.Monad.when`; results are unrelated. | — | 0 | **0** |
| 29 | paraphrase | `qs haskell unfold a container from a seed` | `Data.List.unfoldr` | and :: Foldable t => t Bool -> Bool | Exact `unfoldr` is result 5 with the seed-driven behavior. | — | 0 | **3** |
| 35 | negative-contrast | `qs haskell sequence effects and discard results not collect them` | `mapM_, not mapM` | unzipWith :: (a -> (b, c)) -> Seq a -> (Seq b, Seq c) | Result 2 is `sequence` and points to `sequence_` for discarding results; useful alternative, but no `mapM_` result. | PASS: `mapM` is absent. | 0 | **3** |
| 41 | negative-contrast | `qs haskell left fold with monadic effects not pure fold` | `Control.Monad.foldM, not foldl` | foldM :: (Foldable t, Monad m) => (b -> a -> m b) -> b -> t a -> m b | Exact `foldM` is result 1 and contrasts it with `foldl`. | PASS: pure `foldl` does not lead. | 0 | **5** |
| 47 | negative-contrast | `qs haskell catch only io errors not every exception` | `System.IO.Error.catchIOError, not Control.Exception.catch` | tryIOError :: IO a -> IO (Either IOError a) | `tryIOError` is result 1 and has the correct I/O-only exception scope, but `catchIOError` is absent. | PASS: general `catch` does not lead. | 0 | **3** |
| 53 | negative-contrast | `qs haskell strict bytestring not lazy bytestring` | `Data.ByteString, not Data.ByteString.Lazy` | foldMap :: (Foldable t, Monoid m) => (a -> m) -> t a -> m | No useful strict `Data.ByteString` result; results are unrelated. | PASS: lazy ByteString does not lead. | 0 | **0** |
| 59 | negative-contrast | `qs haskell deep evaluation not weak head normal form` | `Control.DeepSeq.force, not seq` | evaluate :: a -> IO a | Only WHNF APIs (`evaluate` and `seq`) appear; no deep-evaluation or `force` result. | FAIL: rejected WHNF APIs lead. | 0 | **0** |
| 65 | negative-contrast | `qs haskell text builder not string concatenation` | `Data.Text.Lazy.Builder, not (++)` | unescapeArgs :: String -> [String] | No `Data.Text.Lazy.Builder`; results are unrelated. | PASS: `(++)` does not lead. | 0 | **0** |
| 71 | surplus-token-adversarial | `qs haskell hoogle package docs simple example now Data.Set.fromList` | `Data.Set.fromList` | fromList :: Ord a => [a] -> Set a | Exact `fromList` is result 1 with signature and description. | — | 0 | **5** |
| 77 | surplus-token-adversarial | `qs haskell latest base library documentation beginner example Data.Char.generalCategory` | `Data.Char.generalCategory` | generalCategory :: Char -> GeneralCategory | Exact `generalCategory` is result 1 with signature and description. | — | 0 | **5** |
| 83 | surplus-token-adversarial | `qs haskell package docs latest tutorial quick answer Data.Sequence.adjust examples` | `Data.Sequence.adjust` | adjust :: (a -> a) -> Int -> Seq a -> Seq a | Exact `adjust` is result 1 with signature and description. | — | 0 | **5** |
| 89 | surplus-token-adversarial | `qs haskell current base hoogle quick tutorial Control.Arrow.loop examples` | `Control.Arrow.loop` | loop :: ArrowLoop a => a (b, d) (c, d) -> a b c | Exact `loop` is result 1 with signature. | — | 0 | **5** |
| 95 | surplus-token-adversarial | `qs haskell hoogle official base docs concise example Data.Ord.clamp` | `Data.Ord.clamp` | clamp :: Ord a => (a, a) -> a -> a | Exact `clamp` is result 1 with signature and behavior. | — | 0 | **5** |
| 101 | unseen-api | `qs haskell Data.IntMap.lookupGE` | `Data.IntMap.lookupGE` | arr :: Arrow a => (b -> c) -> a b c | No `Data.IntMap.lookupGE`; generic `Control.Arrow` results are unrelated. | — | 0 | **0** |
| 107 | unseen-api | `qs haskell Data.Array.accumArray` | `Data.Array.accumArray` | arr :: Arrow a => (b -> c) -> a b c | No `Data.Array.accumArray`; generic `Control.Arrow` results are unrelated. | — | 0 | **0** |
| 113 | unseen-api | `qs haskell Data.Graph.stronglyConnCompR` | `Data.Graph.stronglyConnCompR` | stronglyConnCompR :: Ord key => [(node, key, [key])] -> [SCC (node, key, [key])] | Exact `stronglyConnCompR` is result 1 with signature and description. | — | 0 | **5** |
| 119 | unseen-api | `qs haskell Text.ParserCombinators.ReadP.gather` | `ReadP.gather` | gather :: ReadP a -> ReadP (String, a) | Exact `gather` is result 1 with signature, behavior, and caveat. | — | 0 | **5** |
| 125 | unseen-api | `qs haskell Data.Typeable.eqT` | `Data.Typeable.eqT` | eqT :: forall {k} (a :: k) (b :: k). (Typeable a, Typeable b) => Maybe (a :~: b) | Exact `eqT` is result 1 with signature and description. | — | 0 | **5** |
| 131 | unseen-api | `qs haskell Data.Foldable.foldMap1` | `Data.Foldable.foldMap1` | arr :: Arrow a => (b -> c) -> a b c | No `Data.Foldable.foldMap1`; generic `Control.Arrow` results are unrelated. | — | 0 | **0** |
| 139 | source-routing | `qs web Hackage attoparsec` | `DuckDuckGo results for Hackage attoparsec` | attoparsec: Fast combinator parsing for bytestrings and text | Correct web provider; the relevant Hackage attoparsec page is result 1. | — | 0 | **5** |
| 145 | source-routing | `qs gh haskell-unordered-containers` | `GitHub repositories for haskell-unordered-containers` | konsumlamm/extended-containers | Correct GitHub provider, but the canonical repository is absent and weak alternatives lead. | — | 0 | **3** |
| 151 | source-routing | `qs haskell System.Mem.performGC` | `Hoogle results for System.Mem.performGC` | performGC :: IO () | Exact `performGC` is result 1 with its behavior. | — | 0 | **5** |
| 157 | source-routing | `qs web haskell backpack signatures` | `DuckDuckGo results for Haskell Backpack signatures` | Backpack support - Stack | Correct web provider with useful Haskell Backpack results led by a relevant guide. | — | 0 | **5** |
| 163 | source-routing | `qs so haskell quantified constraints` | `Stack Overflow search for Haskell quantified constraints` | Bifunctors in Haskell vs in category theory | Correct Stack Overflow provider, but returned questions are only weakly related to quantified constraints. | — | 0 | **4** |
