# qs

Fast lookups from the terminal. The default is a web search. A leading word picks a manual, a doc set, a package registry, or a wiki.

The binary is `quicksearch`. The command name is `qs`.

## Build

```sh
cargo build --release --target-dir target
./target/release/quicksearch price of gold in inr
```

Put it on your `PATH` as `qs`:

```sh
install -m 755 target/release/quicksearch ~/.local/bin/qs
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
qs --open 2 QUERY
qs history
```

`qs dnf rust` shows package info. It does not install or remove anything. `qs cargo build` opens the `cargo-build` manual. `qs how to upgrade cargo binaries` stays a web search.

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

`!wiki` and the other known bangs use those sources. Any other bang follows DuckDuckGo, for example `qs !aur paru`.

`qs lang py` fuzzy-finds language shorthands. `qs topic net` fuzzy-finds broader concepts such as linux, networking, security, cloud, mail, and editors. Then search with the shorthand: `qs linux iptables`, `qs sec ssh`, `qs db jsonb`.

A command name opens its manual. A subcommand opens that page. A package name shows package info.

```sh
qs dnf
qs apt install
qs brew install
qs flatpak install
qs podman build
```

`dnf`, `rpm`, `apt`, `pacman`, `brew`, `flatpak`, and `zypper` can show package info. `cargo`, `podman`, and `snap` open manuals.

An 8, 12, 13, or 14 digit barcode is a product lookup. `qs 1912` stays a web search.

## Options

```sh
qs -n 10 QUERY       # 1 to 20 results, default 5
qs --open QUERY      # open the first hit
qs --open 2 QUERY    # open hit 2
qs --copy QUERY      # copy the first URL (wl-copy, then xclip)
qs --json QUERY
qs --no-color QUERY  # or set NO_COLOR
qs history           # recent searches
qs history 3         # run a saved search again
```

## Your own source

`~/.config/quicksearch/sources`, one source per line. `#` starts a comment. `{query}` is the encoded rest of the command. Built-in words win over a custom line.

```
aur https://aur.archlinux.org/packages?K={query}
```

Doc indexes and product lookups are cached under `~/.cache/quicksearch`.
