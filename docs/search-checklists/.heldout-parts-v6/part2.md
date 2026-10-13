# Held-out evaluation: indexes 50–99

- Entries: 50
- Mean score: **4.18 / 5**
- Execution errors: **0**

## Score distribution

| Score | Count |
|---:|---:|
| 5 | 33 |
| 4 | 7 |
| 3 | 2 |
| 2 | 3 |
| 1 | 4 |
| 0 | 1 |

## Failures (score 0–2)

| Index | Score | Query | Expected | Reason |
|---:|---:|---|---|---|
| 50 | 2 | `qs rust checked addition not wrapping addition` | `checked_add, not wrapping_add` | Expected checked_add is absent; rank 4 add_with_overflow is only a partial match. |
| 52 | 2 | `qs c find first matching span not substring` | `strpbrk, not strstr` | strpbrk is absent; results include only partial analogues such as memchr and wcspbrk. |
| 53 | 1 | `qs haskell strict bytestring not lazy bytestring` | `Data.ByteString, not Data.ByteString.Lazy` | Data.ByteString is absent; generic strict/lazy results do not answer the query. |
| 54 | 1 | `qs python cryptographic random integer not random.randrange` | `secrets.randbelow(), not random.randrange()` | Only random.randrange is returned, which is the explicitly rejected API. |
| 55 | 2 | `qs go escape html text not url query data` | `html.EscapeString, not url.QueryEscape` | html.EscapeString is absent; XML EscapeText is first and QueryEscape is third. |
| 59 | 1 | `qs haskell deep evaluation not weak head normal form` | `Control.DeepSeq.force, not seq` | force/deepseq is absent; the results focus on the explicitly rejected WHNF operations. |
| 65 | 0 | `qs haskell text builder not string concatenation` | `Data.Text.Lazy.Builder, not (++)` | Data.Text.Lazy.Builder is absent and the returned items are not useful for text building. |
| 74 | 1 | `qs rust beginner tutorial standard docs show me slice rotate_left method now` | `slice::rotate_left` | The first result is the unrelated integer rotate_left intrinsic; the slice method is absent. |

## Weak results (score 3)

| Index | Query | Expected | Reason |
|---:|---|---|---|
| 56 | `qs rust saturating multiplication not checked multiplication` | `saturating_mul, not checked_mul` | Saturating arithmetic is relevant, but saturating_mul is absent. |
| 58 | `qs c convert wide string to multibyte not reverse direction` | `wcsrtombs, not mbsrtowcs` | wcsrtombs is correct but only rank 5; the reverse conversion is rank 4. |
