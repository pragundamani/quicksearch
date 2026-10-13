# C v8 regression score

- Commands: 100
- Provider: C for 100/100
- Errors or timeouts: 0
- Mean score: 4.76/5
- Total score: 476/500
- Distribution: 5: 86, 4: 6, 3: 6, 2: 2, 1: 0, 0: 0
- Scores 4–5: 92/100
- Scores 0–2: 2/100

## Key failures

- **3/5** `structures` — The exact Struct declaration page ranks fourth; the first result is Enumerations.
- **3/5** `storage duration` — Only static and thread storage-duration pages are returned; automatic and allocated duration are missing.
- **3/5** `flexible array member` — The first result is the general Array declaration page, but its snippet discusses unknown-size arrays rather than flexible array members.
- **3/5** `command line arguments` — The first result is argc, but the snippet is generic and argv is not shown.
- **2/5** `read a file line by line` — fread ranks first and does not answer line-by-line input; fgets is third with only a return-value fragment.
- **3/5** `sort array with comparator` — The qsort detail page ranks fourth. The second result only lists qsort, while the first is bsearch.
- **3/5** `get current calendar time` — timespec_get is useful and first, but the requested time/calendar conversion path is not directly surfaced; time ranks fifth.
- **2/5** `format date and time` — Only a broad Date and time utilities page is returned, with a license-only snippet; strftime is not surfaced.
