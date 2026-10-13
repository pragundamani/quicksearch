# Held-out evaluation v9 — part 1 (indexes 0–49)

- Queries: 50
- Mean score: 4.78/5
- Score distribution: 5: 41, 4: 7, 3: 2, 2: 0, 1: 0, 0: 0
- Command errors/timeouts: 0

## Key failures (score 0–2)

- None.

## Weak results (score 3)

- **17 — 3/5** — `qs haskell combine maps and resolve duplicate keys`: `union` is third and mentions `unionWith`, but `unionWith` itself is not returned.
- **25 — 3/5** — `qs go return a reader limited to n bytes`: `LimitedReader` is first and explains the limit, but `LimitReader` is absent.

## Lower-ranked useful results (score 4)

- **1 — 4/5** — `qs go find mime type from a filename extension`: `TypeByExtension` is third and directly answers the query.
- **6 — 4/5** — `qs python find common path prefix by components`: `commonprefix` is third, but its snippet explicitly recommends `commonpath`.
- **9 — 4/5** — `qs cpp test whether range begins with another range`: `starts_with` is second and exact.
- **13 — 4/5** — `qs go read exactly n bytes or fail`: `ReadFull` is second and exact.
- **18 — 4/5** — `qs python wait for whichever future finishes first`: `concurrent.futures.FIRST_COMPLETED` is second and `wait` is third.
- **28 — 4/5** — `qs c transform string for locale collation`: `strxfrm` is fourth; the first snippet points to it.
- **49 — 4/5** — `qs go lexical path cleanup not filesystem symlink resolution`: `Clean` is fourth and exact, after three noisy hits.
