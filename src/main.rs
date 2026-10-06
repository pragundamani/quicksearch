mod http;
mod model;
mod providers;
mod render;
mod route;
mod text;

use std::io::IsTerminal;
use std::process::{Command, Stdio};

use clap::Parser;

use crate::render::View;
use crate::route::{route, Provider};

macro_rules! help_text {
    ($sources:literal) => {
        const SOURCES: &str = $sources;
        const AFTER_HELP: &str = concat!(
            $sources,
            "
A leading word selects the source. `!bang` does the same thing, and an unknown
bang follows DuckDuckGo. A path like std::fs::read picks rust or docs on its own.
`qs lang <pattern>` fuzzy-finds language shorthands.
`qs topic <pattern>` fuzzy-finds concepts such as linux, networking, and security.
`qs dnf`, `qs cargo build`, and `qs brew install` open that command's manual.
`qs dnf rust` shows package info and does not install anything.
`web` forces a normal web search. Add your own leading word in
~/.config/quicksearch/sources as `word https://example.test/?q={query}`.

Examples
  qs how to read a file in rust
  qs std::fs::read
  qs dnf rust
  qs dnf install
  qs cargo build
  qs stock 8901491101833
  qs item jio tag
  qs arch pacman
  qs rfc 9110
  qs npm lodash
  qs cve openssl
  qs !aur paru
  qs --open 2 QUERY
  qs history

With no query, qs opens $VISUAL or $EDITOR. Options go before the query.
"
        );
    };
}

help_text!(
    "\
Sources
  web              DuckDuckGo results and instant answers (the default)
  cheat, cht       cheat.sh sheets
  crate            crates.io
  docs             docs.rs, with std/core/alloc sent to the official Rust docs
  rust, std, rs    Rust standard library
  so, stack        Stack Overflow
  gh, github       GitHub repositories
  mdn              MDN Web Docs
  wiki             Wikipedia
  man              local manual pages
  arch             Arch Wiki
  gentoo           Gentoo Wiki
  fedora           Fedora Wiki
  debian           Debian Wiki
  ubuntu           Ubuntu Wiki
  rfc              RFC text, or a title search
  npm              the npm registry
  pypi, pip        PyPI
  gem              RubyGems
  hex              Hex
  go-mod           Go module search
  maven            Maven Central
  nuget            NuGet
  cve, nvd         NVD advisories
  stock, item      a product name or barcode

Languages
  c                C
  cpp, c++         C++
  py, python       Python
  hs, haskell      Haskell, via Hoogle
  ocaml, ml        OCaml
  go               Go
  java             Java
  ruby, rb         Ruby
  js, javascript   JavaScript
  ts, typescript   TypeScript
  php              PHP
  zig              Zig
  kt, kotlin       Kotlin
  lua              Lua
  ex, elixir       Elixir
  perl             Perl
  scala            Scala
  clj, clojure     Clojure
  erl, erlang      Erlang
  jl, julia        Julia
  nim              Nim
  bash, sh         Bash
  zsh              Zsh
  css              CSS
  html             HTML
  node             Node.js
  dart             Dart
  r                R
"
);

#[derive(Parser)]
#[command(
    name = "qs",
    bin_name = "qs",
    version,
    about = "Look things up from the terminal",
    after_long_help = AFTER_HELP
)]
struct Cli {
    /// Search query. With no query, compose one in $VISUAL or $EDITOR.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    query: Vec<String>,

    /// Source to search. A leading word or !bang in the query selects one too.
    #[arg(short, long, value_enum)]
    provider: Option<Provider>,

    /// How many results to show.
    #[arg(short = 'n', long, default_value_t = 5)]
    limit: usize,

    /// Open a result. `--open` uses the first hit. `--open 2` uses the second.
    #[arg(short, long, num_args = 0..=1, default_missing_value = "1", value_name = "N")]
    open: Option<usize>,

    /// Copy a result URL. `--copy` uses the first hit. `--copy 2` uses the second.
    #[arg(long, num_args = 0..=1, default_missing_value = "1", value_name = "N")]
    copy: Option<usize>,

    /// Print JSON instead of formatted text.
    #[arg(long)]
    json: bool,

    /// Disable color.
    #[arg(long)]
    no_color: bool,

    /// Compose or revise the query in $VISUAL or $EDITOR.
    #[arg(short, long)]
    edit: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("qs: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    if !(1..=20).contains(&cli.limit) {
        return Err("--limit must be from 1 to 20".into());
    }

    let mut query = cli.query.join(" ");
    if cli.edit || query.trim().is_empty() {
        query = compose_query(&query)?;
    }
    let mut routed = route(cli.provider, &query)?;
    if routed.list_sources {
        render::print_sources(SOURCES, color_enabled(cli.no_color));
        return Ok(());
    }
    if routed.list_history && !routed.query.trim().is_empty() {
        query = providers::history::rerun_query(&routed.query)
            .ok_or_else(|| "that history entry does not exist".to_string())?;
        routed = route(None, &query)?;
    }
    let color = color_enabled(cli.no_color);
    let client = http::Client::new();
    let outcome = if routed.list_history {
        providers::history::list(&routed.query, cli.limit)?
    } else if let Some(url) = &routed.custom_url {
        custom_outcome(&routed.query, url)
    } else if let Some(name) = &routed.bang {
        providers::bangs::search(name, &routed.query)?
    } else if let Some(page) = &routed.manual {
        providers::commands::manual(page)?
    } else if let Some(tool) = &routed.package_tool {
        providers::commands::package(tool, &routed.query)?
    } else if routed.list_langs {
        providers::langs::list(&client, &routed.query, cli.limit)?
    } else if routed.list_topics {
        providers::concepts::list(&routed.query, cli.limit)?
    } else if routed.allow_docset {
        if let Some(docs) = providers::langs::try_search(&client, &routed.query, cli.limit)? {
            docs
        } else {
            providers::search(
                &client,
                routed.provider,
                &routed.query,
                cli.limit,
                color && !cli.json,
            )?
        }
    } else {
        providers::search(
            &client,
            routed.provider,
            &routed.query,
            cli.limit,
            color && !cli.json,
        )?
    };
    if !outcome.has_content() {
        return Err(format!("no results for {query}"));
    }

    if cli.json {
        let json = serde_json::to_string_pretty(&outcome).map_err(|err| err.to_string())?;
        println!("{json}");
    } else {
        let view = View {
            color,
            links: std::io::stdout().is_terminal(),
        };
        render::print_outcome(&outcome, &view);
    }

    if !routed.list_history && !routed.list_langs && !routed.list_topics {
        providers::history::record(&query);
    }

    if let Some(index) = cli.copy {
        let url = nth_url(&outcome, index)?;
        copy_url(&url)?;
    }
    if let Some(index) = cli.open {
        let url = nth_url(&outcome, index)?;
        open_url(&url)?;
    }
    Ok(())
}

fn custom_outcome(query: &str, url: &str) -> crate::model::Outcome {
    let mut outcome = crate::model::Outcome::new("custom", query);
    outcome.hits.push(crate::model::Hit::new(query, url));
    outcome
}

fn nth_url(outcome: &crate::model::Outcome, index: usize) -> Result<String, String> {
    if index == 0 {
        return Err("result numbers start at 1".into());
    }
    outcome
        .hits
        .get(index - 1)
        .map(|hit| hit.url.clone())
        .filter(|url| !url.is_empty())
        .or_else(|| {
            if index == 1 {
                outcome.first_url().map(str::to_string)
            } else {
                None
            }
        })
        .ok_or_else(|| format!("no result {index}"))
}

fn copy_url(url: &str) -> Result<(), String> {
    let attempts = [
        ("wl-copy", vec![]),
        ("xclip", vec!["-selection", "clipboard"]),
    ];
    for (bin, args) in attempts {
        let mut child = match Command::new(bin)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => child,
            Err(_) => continue,
        };
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(url.as_bytes());
        }
        if child.wait().map(|status| status.success()).unwrap_or(false) {
            return Ok(());
        }
    }
    Err(format!("{url}\nno clipboard tool (wl-copy or xclip)"))
}

fn color_enabled(no_color: bool) -> bool {
    if no_color || std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    std::io::stdout().is_terminal()
}

fn compose_query(seed: &str) -> Result<String, String> {
    let editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".into());
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    path.push(format!("qs-{}-{nanos}.txt", std::process::id()));
    std::fs::write(&path, seed).map_err(|err| err.to_string())?;

    let mut parts = editor.split_whitespace();
    let bin = parts.next().ok_or("EDITOR is empty")?;
    let args: Vec<&str> = parts.collect();
    let status = Command::new(bin)
        .args(args)
        .arg(&path)
        .status()
        .map_err(|err| format!("launch {bin}: {err}"))?;
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let _ = std::fs::remove_file(&path);
    if !status.success() {
        return Err(format!("{bin} exited with {status}"));
    }
    let query = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if query.is_empty() {
        return Err("empty query".into());
    }
    Ok(query)
}

fn open_url(url: &str) -> Result<(), String> {
    if url.starts_with("man://") {
        let mut parts = url.trim_start_matches("man://").split('/');
        let section = parts.next().unwrap_or("");
        let page = parts.next().unwrap_or("");
        let status = Command::new("man")
            .args(["-s", section, page])
            .status()
            .map_err(|err| format!("man: {err}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("man exited with {status}"))
        }
    } else {
        let status = Command::new("xdg-open")
            .arg(url)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|err| format!("xdg-open: {err}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("xdg-open exited with {status}"))
        }
    }
}
