# Haskell v8 regression score

- Commands evaluated: 100/100
- Successful executions: 100
- Errors or timeouts: 0
- Total score: 448/500
- Mean score: 4.48/5
- Score distribution: 5 = 67, 4 = 16, 3 = 15, 2 = 2, 1 = 0, 0 = 0

## Key failures

- `default maybe value` — **2/5**. `fromMaybe` is rank 5. Three `<$` results lead.
- `separate lefts and rights` — **2/5**. `partitionEithers` is rank 5. Four unrelated `ST` results lead.
- `monad transformers` — **3/5**. Results show `MonadIO` and `liftIO`, not transformer types and general `lift`.
- `set intersection` — **3/5**. `intersection` is rank 4.
- `command line arguments` — **3/5**. `getArgs` is rank 4.

## Other ranking gaps

- `discard Nothing values`: `catMaybes` is rank 3, after two `void` results.
- `ordering comparison`: `Ord` is rank 3; `compare` is not directly returned.
- `insert map key`: `Data.Map.insert` is rank 2, after unrelated `Seq.lookup`.
- `Text`: `Data.Text.Text` is rank 3, after type-level `Text` results.
- `bracket resource cleanup`: `bracket` is rank 3.
- `operation timeout`: `timeout` is rank 2, after the `Timeout` exception type.

Several language-concept searches scored 3 because they return a broad synthesized Haskell Report chapter result. The result is relevant, but the snippet does not explain the requested concept.
