# qs

Fast lookups from the terminal. The default is a web search. A leading word picks a manual, a doc set, a package registry, or a wiki.

The command is `qs`.

qs is free software under the GNU General Public License, version 3 or any later version. See `LICENSE`.

Current version: **0.2.1**.

![qs](docs/qs.gif)

## v0.2.1 platform status

The tag-triggered release workflow is configured to package Linux x86_64/aarch64, macOS x86_64/Apple Silicon, and Windows x86_64/ARM (MSVC). An asset is available only after its workflow job succeeds.

The v0.2.1 pre-release checks ran natively on Linux x86_64. They also cross-built Linux x86_64/aarch64 (GNU and musl) and Windows x86_64/i686 (GNU). They did not run binaries natively on Windows, use MSVC, or build or test on macOS.

## Install

### Cargo

```sh
cargo install quicksearch-cli
```

### Homebrew

```sh
brew tap pragundamani/qs
brew trust pragundamani/qs
brew install qs
```

Download a binary, or build it yourself.

### Download

The installers fetch the latest release binary for this machine. Public tag-workflow Linux archives currently target GNU/glibc. Pre-release validation also cross-built GNU and musl Linux targets.

```sh
curl -fsSL https://raw.githubusercontent.com/pragundamani/quicksearch/main/install.sh | bash -s -- --release
```

```sh
curl -fsSL https://raw.githubusercontent.com/pragundamani/quicksearch/main/install.py | python3 - --release
```

```powershell
$script = Join-Path $env:TEMP "qs-install.ps1"
Invoke-WebRequest https://raw.githubusercontent.com/pragundamani/quicksearch/main/install.ps1 -OutFile $script
powershell -File $script -Release
```

From a clone: `./install.sh --release`, `python3 install.py --release`, or `powershell -File install.ps1 -Release`.

Other archives are on the [latest release](https://github.com/pragundamani/quicksearch/releases/latest).

Linux x86_64:

```sh
curl -LO https://github.com/pragundamani/quicksearch/releases/latest/download/qs-x86_64-unknown-linux-gnu.tar.gz
tar -xzf qs-x86_64-unknown-linux-gnu.tar.gz
install -m 755 qs "$HOME/.local/bin/qs"
```

Linux ARM:

```sh
curl -LO https://github.com/pragundamani/quicksearch/releases/latest/download/qs-aarch64-unknown-linux-gnu.tar.gz
tar -xzf qs-aarch64-unknown-linux-gnu.tar.gz
install -m 755 qs "$HOME/.local/bin/qs"
```

macOS Apple Silicon:

```sh
curl -LO https://github.com/pragundamani/quicksearch/releases/latest/download/qs-aarch64-apple-darwin.tar.gz
tar -xzf qs-aarch64-apple-darwin.tar.gz
install -m 755 qs "$HOME/.local/bin/qs"
```

macOS Intel:

```sh
curl -LO https://github.com/pragundamani/quicksearch/releases/latest/download/qs-x86_64-apple-darwin.tar.gz
tar -xzf qs-x86_64-apple-darwin.tar.gz
install -m 755 qs "$HOME/.local/bin/qs"
```

Windows x86_64:

```powershell
curl.exe -LO https://github.com/pragundamani/quicksearch/releases/latest/download/qs-x86_64-pc-windows-msvc.zip
tar -xf qs-x86_64-pc-windows-msvc.zip
```

Windows ARM:

```powershell
curl.exe -LO https://github.com/pragundamani/quicksearch/releases/latest/download/qs-aarch64-pc-windows-msvc.zip
tar -xf qs-aarch64-pc-windows-msvc.zip
```

### Build it yourself

Linux and macOS:

```sh
git clone https://github.com/pragundamani/quicksearch.git
cd quicksearch
./install.sh
```

Windows:

```powershell
git clone https://github.com/pragundamani/quicksearch.git
cd quicksearch
powershell -File install.ps1
```

Or use Python on any of them:

```sh
git clone https://github.com/pragundamani/quicksearch.git
cd quicksearch
python3 install.py
```

Or build without installing:

```sh
cargo build --release --target-dir target
./target/release/qs price of gold in inr
```

With no query, `qs` opens `$VISUAL` or `$EDITOR`. Options go before the query.

## Examples

```sh
qs price of gold in inr
qs how to read a file in rust
qs std::fs::read
qs python pathlib.Path
qs c printf
qs haskell foldl
qs dnf rust
qs dnf install
qs cargo build
qs arch pacman
qs rfc 9110
qs npm lodash
qs cve openssl
qs 3017620422003
qs --open=2 QUERY
qs history
qs cache status
qs cache clear
```

`qs dnf rust` shows package info. It does not install or remove anything. `qs cargo build` opens the `cargo-build` manual. `qs how to upgrade cargo binaries` stays a web search.


## Generated examples

```sh
qs --example how to read a file in rust
qs e py multithread          # short form
qs --example-setup
qs --example-status
qs --example-reset
```

`e` is a reserved first-word alias for `--example`. `--example` sends the query and up to three bounded HTTPS documentation results to the selected Google Gemini provider. qs checks citation indexes, the requested language, and basic lexical relevance. These checks do not prove that every generated claim is supported by a source. Treat all generated code as untrusted. qs never runs generated code. It performs a bounded syntax-only check when a fixed, trusted parser is available, but this does not verify behavior or safety.

First use asks you to select one provider in an interactive terminal:

- **Gemini CLI (Unix):** Install Gemini CLI 0.63.0 or later. Sign in once with `GEMINI_FORCE_ENCRYPTED_FILE_STORAGE=true gemini` so Gemini CLI puts OAuth credentials in its native keyring. qs does not start login or install flows. It runs the CLI in a private temporary home with extensions, MCP servers, user instructions, unrelated credentials, and all tools disabled. If Gemini CLI cannot use a native keyring, select Gemini API and use `GEMINI_API_KEY` instead.
- **Gemini API:** Enter a key through the hidden prompt. qs stores it only in the native OS credential store. `GEMINI_API_KEY` is an ephemeral override and is never saved. Systems without a usable native credential store must use that environment variable or Gemini CLI.

JSON and non-interactive generation never prompts, starts login, or changes provider settings. For Gemini API generation in these modes, provide `GEMINI_API_KEY`; qs does not open the OS credential store. `--example-setup` and `--example-reset` require an interactive terminal in human mode.

Example prompts, model responses, generated code, and documentation text are not written to the persistent cache or search history. Provider settings are stored separately in the platform state directory. Use `--example-status` to inspect the saved provider and `--example-reset` to remove it and attempt cleanup of the dedicated stored-key account, including an orphan left by an interrupted setup. If the native credential store is unavailable, reset still removes provider state and asks you to retry credential cleanup later.

## Cache

`qs` enforces one global 4 MiB limit for persistent cache files. Writes use a cross-process lock, atomic replacement, and least-recently-used eviction. Oversized entries are used for the current query but are not saved. A read-only, full, or unavailable cache does not discard a valid live result. Full-text documentation corpora, generated full-text indexes, downloaded documentation pages, and page-derived notes are not persisted.

On Unix, cache and state roots are owner-only directories. Cache reads, writes, removal, and traversal are confined to an opened root and do not follow hostile cache symlinks. Runtime locks stay outside the persistent-cache quota.

Use `qs cache status` to show cache usage. Use `qs cache clear` to remove cached data. Search history is stored separately under the platform state directory and is not cleared.

## Sources

| Leading word | Lookup |
|---|---|
| `web` | DuckDuckGo. Also forces a web search when another word would route the query |
| `cheat`, `cht` | cheat.sh |
| `crate` | crates.io |
| `docs` | docs.rs. `std`, `core`, and `alloc` go to the official Rust docs |
| `rust`, `std`, `rs` | Rust standard library |
| `so` | Stack Overflow |
| `gh` | GitHub repositories |
| `mdn` | MDN |
| `wiki` | Wikipedia |
| `man` | local manual pages |
| `arch`, `gentoo`, `fedora`, `debian`, `ubuntu` | that distro's wiki |
| `rfc` | an RFC, or a title search |
| `npm`, `pypi`, `pip`, `gem`, `hex`, `go-mod`, `maven`, `nuget` | that package registry |
| `cve`, `nvd` | NVD advisories |
| `stock`, `item` | a product name or barcode |

## Languages

A leading word searches that language. `qs lang py` fuzzy-finds a shorthand.

| Leading word | Docs |
|---|---|
| `c` | C |
| `cpp`, `c++` | C++ |
| `py`, `python` | Python |
| `hs`, `haskell` | Haskell APIs and GHC documentation |
| `ocaml`, `ml` | OCaml |
| `go` | Go |
| `java` | Java |
| `ruby`, `rb` | Ruby |
| `js`, `javascript` | JavaScript |
| `ts`, `typescript` | TypeScript |
| `php` | PHP |
| `zig` | Zig |
| `kt`, `kotlin` | Kotlin |
| `lua` | Lua |
| `ex`, `elixir` | Elixir |
| `perl` | Perl |
| `scala` | Scala |
| `clj`, `clojure` | Clojure |
| `erl`, `erlang` | Erlang |
| `jl`, `julia` | Julia |
| `nim` | Nim |
| `bash`, `sh` | Bash |
| `zsh` | Zsh |
| `css` | CSS |
| `html` | HTML |
| `node` | Node.js |
| `dart` | Dart |
| `r` | R |

`rust`, `std`, and `rs` stay on the Rust standard library, in the sources table above.

`!wiki` and the other known bangs use those sources. Any other bang follows DuckDuckGo, for example `qs !aur paru`.

`qs topic net` fuzzy-finds broader concepts such as linux, networking, security, cloud, mail, and editors. Then search with the shorthand: `qs linux iptables`, `qs sec ssh`, `qs db jsonb`.

A command name opens its manual. A subcommand opens that page. A package name shows package info.

```sh
qs dnf
qs apt install
qs brew install
qs flatpak install
qs podman build
```

`dnf`, `rpm`, `apt`, `pacman`, `brew`, `flatpak`, `zypper`, and `winget` can show package info. `cargo`, `podman`, and `snap` open manuals.

An 8, 12, 13, or 14 digit barcode is a product lookup. `qs 1912` stays a web search.

## Screenshots

`qs cpp semaphore`

![qs cpp semaphore](docs/qs-semaphore.png)

`qs -n 1 py json.dumps`

![qs py json.dumps](docs/qs-python.png)

`qs -n 1 std::fs::read`

![qs std::fs::read](docs/qs-rust.png)

`qs qs`

![qs qs](docs/qs-sources.png)

## Options

```sh
qs -n 10 QUERY       # 1 to 20 results, default 5
qs --open QUERY      # open the first hit
qs --open=2 QUERY    # open hit 2
qs --copy QUERY      # copy the first URL
qs --json QUERY
qs --no-color QUERY  # or set NO_COLOR
qs history           # recent searches
qs history 3         # run a saved search again
```

## Your own source

Custom sources live in `~/.config/quicksearch/sources` on Linux, `~/Library/Application Support/quicksearch/sources` on macOS, and `%APPDATA%\quicksearch\sources` on Windows. One source per line. `#` starts a comment. `{query}` is the encoded rest of the command. Built-in words win over a custom line.

```
aur https://aur.archlinux.org/packages?K={query}
```

Doc indexes and product lookups are cached under `~/.cache/quicksearch` on Linux, `~/Library/Caches/quicksearch` on macOS, and `%LOCALAPPDATA%\quicksearch` on Windows. `$XDG_CACHE_HOME` and `$XDG_CONFIG_HOME` still win when they are set.
