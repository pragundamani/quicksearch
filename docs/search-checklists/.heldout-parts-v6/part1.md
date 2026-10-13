# Held-out evaluation: indexes 0–49

- Entries: **50**
- Mean score: **3.32 / 5**
- Distribution: **5: 18, 4: 9, 3: 6, 2: 6, 1: 10, 0: 1**

## Failures (scores 0–2)

| Index | Score | Query | Expected | Note |
|---:|---:|---|---|---|
| 2 | 1 | qs rust keep only successful iterator values | Iterator::map_while or filter_map | Results discuss iterators generally but do not show map_while or filter_map. |
| 4 | 1 | qs c duplicate part of a string into bounded buffer | strncpy | Results mention bounds checking and formatted output but not strncpy. |
| 7 | 2 | qs go notify one waiting goroutine | sync.Cond.Signal | Cond.Wait appears only at rank 5; visible output does not identify Signal as the one-waiter operation. |
| 8 | 1 | qs rust turn option into result with lazy error | Option::ok_or_else | Results do not identify ok_or_else; only generic error and Option-adjacent material appears. |
| 12 | 1 | qs python execute cleanup when interpreter exits | atexit.register() | Results do not show atexit.register; most are unrelated cleanup or exit pages. |
| 14 | 1 | qs rust replace matching slice prefix | slice::strip_prefix | Results concern unrelated prefix concepts and do not show strip_prefix. |
| 19 | 1 | qs go write quoted go string literal | strconv.Quote | Results discuss other quoting or Go-syntax facilities but not strconv.Quote. |
| 20 | 2 | qs rust split slice once on a predicate | slice::split_once | Results show related split iterators but not slice::split_once. |
| 26 | 2 | qs rust get two mutable slice elements safely | slice::get_many_mut | The slice documentation is relevant, but visible hits do not show get_many_mut. |
| 27 | 2 | qs cpp count consecutive equal groups in a range | std::ranges::chunk_by_view | unique is related to consecutive equal values, but results do not show chunk_by_view or group counting. |
| 30 | 1 | qs python preserve function metadata in decorator | functools.wraps() | Results concern metadata and decorators generally but do not show functools.wraps. |
| 31 | 1 | qs go mirror bytes read from a reader into a writer | io.TeeReader | Results cover reader methods but do not show TeeReader or byte mirroring. |
| 36 | 2 | qs python shell style splitting not regular expression splitting | shlex.split(), not re.split() | A rank-5 shlex page discusses shell compatibility, but no result exposes shlex.split. |
| 37 | 2 | qs go compare strings case insensitively not byte equality | strings.EqualFold, not bytes.Equal | The first hit gives a case-insensitive workaround; EqualFold is absent. |
| 38 | 0 | qs rust remove suffix not prefix from string slice | str::strip_suffix, not strip_prefix | The sole result is an unrelated Unsafe Rust page. |
| 40 | 1 | qs c write one character not a formatted string | fputc, not fprintf | Results do not show fputc and are mostly unrelated to stream character output. |
| 48 | 1 | qs python compare sequences in constant time not normal equality | hmac.compare_digest(), not == | Results do not show compare_digest or any constant-time comparison API. |
