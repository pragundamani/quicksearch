# Held-out evaluation v8 — part 1 (indexes 0–49)

- Queries: 50
- Mean score: 4.32/5
- Score distribution: 5: 34, 4: 7, 3: 3, 2: 3, 1: 3, 0: 0
- Command errors/timeouts: 0

## Key failures (score 0–2)

- **0 — 1/5** — `qs python pair adjacent iterator items`: Only a generic iterator glossary hit; pairwise is absent.
- **14 — 1/5** — `qs rust replace matching slice prefix`: No slice::strip_prefix result; hits are unrelated prefix concepts.
- **24 — 2/5** — `qs python iterate over calendar month rows`: Only the broad calendar module is returned; the requested method is absent.
- **27 — 1/5** — `qs cpp count consecutive equal groups in a range`: No chunk_by_view; unique is related to adjacent equality but does not group/count runs.
- **33 — 2/5** — `qs cpp construct object in optional storage in place`: No emplace hit; optional-related pages are only partial navigation aids.
- **47 — 2/5** — `qs haskell catch only io errors not every exception`: tryIOError is related to IO-only errors, but catchIOError is absent and the operation differs.

## Weak results (score 3)

- **17 — 3/5** — `qs haskell combine maps and resolve duplicate keys`: union is third and mentions unionWith, but unionWith itself is not returned.
- **25 — 3/5** — `qs go return a reader limited to n bytes`: LimitedReader is first and explains the limit, but LimitReader is absent.
- **37 — 3/5** — `qs go compare strings case insensitively not byte equality`: EqualFold is absent; the first example gives a weaker lowercase-comparison approach.
