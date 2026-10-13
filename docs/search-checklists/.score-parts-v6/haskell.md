# Haskell search quality score

- **Queries:** 100
- **Mean:** 2.56 / 5
- **Score distribution:** 5: 32, 4: 10, 3: 9, 2: 8, 1: 13, 0: 28
- **Score >=3:** 51/100
- **Score >=4:** 42/100
- **Command errors/timeouts:** 0

## Failures (score 0-1)

- **1. `qs haskell lists` — 1:** Basic list docs are absent; top hits are export/import warning flags and OverloadedLists.
- **11. `qs haskell modules` — 1:** Results are module-related compiler flags, not module declarations or export syntax.
- **13. `qs haskell infix operators` — 0:** Broad bug, warning, and flag pages do not answer infix syntax or fixity declarations.
- **14. `qs haskell strict evaluation` — 0:** Broad guide pages do not expose seq, bang patterns, or strictness guidance.
- **15. `qs haskell thunks` — 1:** The nothunks package is related to thunks, but does not explain lazy evaluation or deferred computation.
- **16. `qs haskell tail recursion` — 0:** Returned guide pages are incidental matches and do not explain tail recursion.
- **17. `qs haskell currying` — 1:** Only a generic Haskell 2010 Report fallback is returned; it has no targeted currying context.
- **18. `qs haskell partial application` — 0:** Incidental broad-guide matches do not explain partial application.
- **19. `qs haskell higher order functions` — 0:** Incidental guide matches do not define or demonstrate higher-order functions.
- **20. `qs haskell function composition` — 0:** Incidental broad-guide matches do not identify the (.) operator.
- **26. `qs haskell take while` — 1:** takeWhile is absent; unfoldr and takeWhileAntitone are not useful substitutes for the expected list API.
- **32. `qs haskell maximum by` — 0:** maximumBy is absent; the results are unrelated APIs such as max_mem_in_use_bytes and printf.
- **50. `qs haskell enumerate range` — 1:** Only a generic Haskell report fallback is returned, without focused Enum or range syntax.
- **51. `qs haskell Functor` — 1:** Only a generic Haskell report fallback is returned, without a focused Functor/fmap result.
- **52. `qs haskell applicatives` — 0:** The generic Haskell 2010 Report fallback does not provide the expected Applicative APIs.
- **53. `qs haskell monadic bind` — 1:** Only monadic fold functions appear; Monad bind and >>= are absent.
- **54. `qs haskell Alternative` — 0:** The generic language-report fallback does not identify Control.Applicative.Alternative.
- **55. `qs haskell foldables` — 1:** A generic report fallback is broadly related but does not locate Data.Foldable.Foldable.
- **57. `qs haskell Semigroup` — 0:** The generic Haskell 2010 Report fallback does not provide Semigroup or <>.
- **58. `qs haskell monoids` — 1:** Results are similarly named Hackage packages, not the base Monoid class or its methods.
- **63. `qs haskell integer keyed maps` — 0:** The generic language-report fallback does not identify Data.IntMap.
- **65. `qs haskell hash maps` — 0:** The generic language-report fallback does not cover unordered-containers or Data.HashMap.
- **66. `qs haskell sequences` — 1:** Results are unrelated Hackage packages containing 'sequences', not Data.Sequence.Seq.
- **67. `qs haskell vectors` — 0:** The generic language-report fallback does not cover the vector package or Data.Vector.Vector.
- **68. `qs haskell byte strings` — 0:** The sole streamCString# hit is unrelated to Data.ByteString.ByteString.
- **74. `qs haskell parse JSON` — 0:** The generic language-report fallback does not identify Aeson, eitherDecode, or FromJSON.
- **75. `qs haskell generic programming` — 0:** The generic language-report fallback does not identify GHC.Generics.Generic.
- **76. `qs haskell lens view set` — 1:** Only containers alterF functions appear; a note mentions Control.Lens.At, but view/set basics are absent.
- **80. `qs haskell safe file handle` — 1:** withFile is absent; returned handle input/output functions do not address safe resource cleanup.
- **83. `qs haskell list directory` — 0:** The generic language-report fallback does not identify System.Directory.listDirectory.
- **84. `qs haskell check file exists` — 0:** The generic language-report fallback does not identify doesFileExist.
- **85. `qs haskell file extension` — 0:** Broad GHC guide pages are incidental matches and do not expose FilePath extension APIs.
- **86. `qs haskell temporary files` — 0:** The generic language-report fallback does not identify System.IO.Temp.
- **89. `qs haskell async concurrency` — 0:** The generic language-report fallback does not identify the async package or Control.Concurrent.Async.
- **92. `qs haskell software transactional memory` — 0:** The generic language-report fallback does not identify STM or Control.Concurrent.STM.
- **93. `qs haskell operation timeout` — 0:** The generic language-report fallback does not identify System.Timeout.timeout.
- **94. `qs haskell run external process` — 0:** The sole openFileBlocking hit is unrelated to running external processes.
- **95. `qs haskell random numbers` — 0:** The generic language-report fallback does not identify System.Random or randomRIO.
- **96. `qs haskell HTTP client request` — 0:** The generic language-report fallback does not identify Network.HTTP.Client.
- **97. `qs haskell QuickCheck properties` — 0:** The sole decodeASCIIPrefix hit is unrelated to QuickCheck properties.
- **98. `qs haskell Hspec` — 0:** The generic language-report fallback does not identify Test.Hspec.

## Most important regressions

- Natural language and package queries often fall back to a generic Haskell 2010 Report result whose title merely repeats the query. This causes many false-positive-looking but unusable results, including Aeson, IntMap, HashMap, Vector, async, STM, timeout, random, HTTP, and Hspec.
- Basic language concepts are routed to GHC User's Guide search results dominated by flags, extensions, or broad incidental text matches. Lists, modules, infix operators, strict evaluation, tail recursion, partial application, higher-order functions, and composition miss their basic documentation.
- Token normalization is inconsistent. Exact identifiers work very well, but `take while` misses `takeWhile`, `maximum by` misses `maximumBy`, and several intent queries rank unrelated APIs above the target.
- External/common package coverage is weak. Aeson, lens, async, random, http-client, QuickCheck, Hspec, temporary, and directory APIs are frequently absent.
- Exact Hoogle-style API searches remain the strong area: core list functions, Text operations, map lookup, traverse, bimap, file I/O, forkIO, MVar, GHCi commands, and language extensions commonly score 5.
