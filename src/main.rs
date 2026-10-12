// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod cache;
mod example;
mod http;
mod model;
mod providers;
mod render;
mod route;
mod semantic;
mod state;
mod text;

use std::io::{IsTerminal, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

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
`qs cache status` reports the global 4 MiB cache; `qs cache clear` removes cache data.
`qs dnf`, `qs cargo build`, `qs brew install`, and `qs winget` open that command's manual.
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
  qs --open=2 QUERY
  qs --example how to read a file in rust
  qs e py multithread
  qs --example-setup
  qs --example-status
  qs cache status
  qs history

`--example` sends the query and up to three bounded documentation results to the
selected Google Gemini provider. Generated code is untrusted and is never run.
Gemini CLI must be at least version 0.63.0 and signed in through its native
keyring; README.md has the one-run sign-in command. Gemini API keys use the OS
credential store or the one-run GEMINI_API_KEY environment value.

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
  hs, haskell      Haskell APIs and GHC documentation
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

    /// Open a result. `--open` uses the first hit. `--open=2` uses the second.
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = "1",
        require_equals = true,
        value_name = "N"
    )]
    open: Option<usize>,

    /// Copy a result URL. `--copy` uses the first hit. `--copy=2` uses the second.
    #[arg(
        long,
        num_args = 0..=1,
        default_missing_value = "1",
        require_equals = true,
        value_name = "N"
    )]
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

    /// Generate one grounded code example using the configured provider.
    #[arg(
        long,
        conflicts_with_all = ["open", "copy", "edit", "example_setup", "example_status", "example_reset"]
    )]
    example: bool,

    /// Configure or change the example provider interactively.
    #[arg(
        long,
        conflicts_with_all = ["example", "example_status", "example_reset", "open", "copy", "edit", "provider"]
    )]
    example_setup: bool,

    /// Show the saved example-provider status without revealing credentials.
    #[arg(
        long,
        conflicts_with_all = ["example", "example_setup", "example_reset", "open", "copy", "edit", "provider"]
    )]
    example_status: bool,

    /// Remove the saved example provider and its stored API credential.
    #[arg(
        long,
        conflicts_with_all = ["example", "example_setup", "example_status", "open", "copy", "edit", "provider"]
    )]
    example_reset: bool,
}

static JSON_ERRORS: AtomicBool = AtomicBool::new(false);
static EXAMPLE_ERRORS: AtomicBool = AtomicBool::new(false);

fn json_option_before_query() -> bool {
    let normalized = normalize_cli_args(std::env::args_os());
    let mut args = normalized.into_iter().skip(1);
    while let Some(arg) = args.next() {
        let text = arg.to_string_lossy();
        if text == "--" {
            break;
        }
        if text == "--json" {
            return true;
        }
        if matches!(text.as_ref(), "-p" | "--provider" | "-n" | "--limit") {
            let _ = args.next();
            continue;
        }
        if text.starts_with('-') {
            continue;
        }
        break;
    }
    false
}

fn example_option_before_query() -> bool {
    let normalized = normalize_cli_args(std::env::args_os());
    let mut args = normalized.into_iter().skip(1);
    while let Some(arg) = args.next() {
        let text = arg.to_string_lossy();
        if text == "--" {
            break;
        }
        if matches!(
            text.as_ref(),
            "--example" | "--example-setup" | "--example-status" | "--example-reset"
        ) {
            return true;
        }
        if matches!(text.as_ref(), "-p" | "--provider" | "-n" | "--limit") {
            let _ = args.next();
            continue;
        }
        if text.starts_with('-') {
            continue;
        }
        break;
    }
    false
}

fn main() {
    JSON_ERRORS.store(json_option_before_query(), Ordering::Relaxed);
    EXAMPLE_ERRORS.store(example_option_before_query(), Ordering::Relaxed);
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !panic_payload_is_broken_pipe(info.payload()) {
            default_hook(info);
        }
    }));
    let result = std::panic::catch_unwind(|| {
        if let Err(err) = run() {
            report_error_and_exit(err);
        }
    });
    match result {
        Ok(()) => {}
        Err(payload) if panic_payload_is_broken_pipe(payload.as_ref()) => {}
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

fn panic_payload_is_broken_pipe(payload: &(dyn std::any::Any + Send)) -> bool {
    payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .is_some_and(|message| message.contains("Broken pipe"))
}

fn report_error_and_exit(err: String) -> ! {
    let err = sanitize_error(&err);
    if JSON_ERRORS.load(Ordering::Relaxed) && EXAMPLE_ERRORS.load(Ordering::Relaxed) {
        let error = example::ExampleError {
            version: 1,
            code: example::ExampleErrorCode::InvalidPrompt,
            message: err,
        };
        println!(
            "{}",
            example::to_json(&error).unwrap_or_else(|_| {
                "{\"version\":1,\"code\":\"example_output_error\",\"message\":\"example error could not be serialized\"}".into()
            })
        );
    } else if JSON_ERRORS.load(Ordering::Relaxed) {
        let mut outcome = crate::model::Outcome::new("error", "");
        outcome.summary_title = Some("Search failed".into());
        outcome.summary = Some(err);
        // Serialization of this fixed model should not fail. Keep stdout
        // valid JSON even when a provider or argument lookup failed.
        println!(
            "{}",
            serde_json::to_string_pretty(&outcome)
                .unwrap_or_else(|_| "{\"provider\":\"error\",\"query\":\"\",\"hits\":[]}".into())
        );
    } else {
        eprintln!("qs: {err}");
    }
    std::process::exit(1);
}

fn sanitize_error(error: &str) -> String {
    crate::text::sanitize_terminal_line(error)
}

fn normalize_cli_args<I, T>(arguments: I) -> Vec<std::ffi::OsString>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString>,
{
    let mut arguments = arguments.into_iter().map(Into::into).collect::<Vec<_>>();
    let mut expects_value = false;
    let mut explicit_example_mode = false;
    for argument in arguments.iter_mut().skip(1) {
        let text = argument.to_string_lossy();
        if expects_value {
            expects_value = false;
            continue;
        }
        if text == "--" {
            break;
        }
        if matches!(text.as_ref(), "-p" | "--provider" | "-n" | "--limit") {
            expects_value = true;
            continue;
        }
        if matches!(
            text.as_ref(),
            "--example" | "--example-setup" | "--example-status" | "--example-reset"
        ) {
            explicit_example_mode = true;
            continue;
        }
        if text.starts_with('-') {
            continue;
        }
        if !explicit_example_mode && text.eq_ignore_ascii_case("e") {
            *argument = std::ffi::OsString::from("--example");
        }
        break;
    }
    arguments
}

fn run() -> Result<(), String> {
    let cli = match Cli::try_parse_from(normalize_cli_args(std::env::args_os())) {
        Ok(cli) => cli,
        Err(error)
            if matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) =>
        {
            print!("{error}");
            return Ok(());
        }
        Err(error) => return Err(error.to_string()),
    };
    JSON_ERRORS.store(cli.json, Ordering::Relaxed);
    EXAMPLE_ERRORS.store(
        cli.example || cli.example_setup || cli.example_status || cli.example_reset,
        Ordering::Relaxed,
    );
    if cli.example && (cli.open.is_some() || cli.copy.is_some() || cli.edit) {
        return Err("--example does not apply to --open, --copy, or --edit".into());
    }
    if !(1..=20).contains(&cli.limit) {
        return Err("--limit must be from 1 to 20".into());
    }
    if cli.open == Some(0) || cli.copy == Some(0) {
        return Err("result indexes start at 1".into());
    }

    let supplied_query = !cli.query.is_empty();
    let mut query = cli.query.join(" ");
    if cli.example_setup || cli.example_status || cli.example_reset {
        if supplied_query {
            return Err("example management commands do not accept a query".into());
        }
        return handle_example_management(&cli);
    }
    if cli.example && !supplied_query {
        return Err("--example requires a query".into());
    }
    if cli.edit || !supplied_query {
        query = compose_query(&query, cli.json)?;
    } else if query.trim().is_empty() {
        return Err("empty query".into());
    }
    if !cli.example {
        providers::history::migrate_legacy();
    }
    if cli.provider.is_none()
        && query
            .split_whitespace()
            .next()
            .is_some_and(|word| word.eq_ignore_ascii_case("cache"))
    {
        if cli.example {
            return Err("--example does not apply to cache commands".into());
        }
        return handle_cache_command(&query, cli.json, cli.open.is_some() || cli.copy.is_some());
    }
    // Repair legacy cache trees before providers read from them. If the cache
    // is read-only, searches still work and every later write is skipped.
    let _ = crate::cache::enforce_global_budget(&crate::cache::root_dir());

    let mut routed = route(cli.provider, &query)?;
    if routed.list_sources {
        if cli.example {
            return Err("--example does not apply to the source list".into());
        }
        if cli.open.is_some() || cli.copy.is_some() {
            return Err("--open and --copy cannot be used with the source list".into());
        }
        if cli.json {
            let mut outcome = crate::model::Outcome::new("sources", &query);
            outcome.body = Some(SOURCES.to_string());
            let json = serde_json::to_string_pretty(&outcome).map_err(|err| err.to_string())?;
            println!("{json}");
        } else {
            render::print_sources(SOURCES, color_enabled(cli.no_color));
        }
        return Ok(());
    }
    if routed.list_history && !routed.query.trim().is_empty() {
        query = providers::history::rerun_query(&routed.query)
            .ok_or_else(|| "that history entry does not exist".to_string())?;
        routed = route(None, &query)?;
    }
    if cli.example && (routed.list_history || routed.list_langs || routed.list_topics) {
        return Err("--example requires a documentation search, not a list command".into());
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
        if let Some(mut docs) = providers::langs::try_search(&client, &routed.query, cli.limit)? {
            if let Some(provider) = providers::langs::provider_id_for_query(&routed.query) {
                if docs.provider != provider {
                    docs.source_provider = Some(docs.provider.clone());
                    docs.provider = provider.into();
                }
            }
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

    if cli.example {
        let generated = example::generate(
            &query,
            &outcome,
            example::GenerateOptions {
                json: cli.json,
                noninteractive: !std::io::stdin().is_terminal() || !std::io::stderr().is_terminal(),
            },
        );
        return emit_example_result(generated, cli.json);
    }

    // In JSON mode, perform requested actions before serializing. A failed
    // index or opener must produce one error object, never two JSON values.
    if cli.json {
        if let Some(index) = cli.copy {
            copy_url(&nth_url(&outcome, index)?)?;
        }
        if let Some(index) = cli.open {
            open_url(&nth_url(&outcome, index)?, true)?;
        }
        let json = serde_json::to_string_pretty(&outcome).map_err(|err| err.to_string())?;
        println!("{json}");
    } else {
        let view = View {
            color,
            links: std::io::stdout().is_terminal(),
        };
        render::print_outcome(&outcome, &view);
        if let Some(index) = cli.copy {
            copy_url(&nth_url(&outcome, index)?)?;
        }
        if let Some(index) = cli.open {
            open_url(&nth_url(&outcome, index)?, false)?;
        }
    }

    if !routed.list_history && !routed.list_langs && !routed.list_topics {
        providers::history::record(&query);
    }
    Ok(())
}

fn handle_example_management(cli: &Cli) -> Result<(), String> {
    let interactive =
        !cli.json && std::io::stdin().is_terminal() && std::io::stderr().is_terminal();
    if cli.example_setup {
        if !interactive {
            return emit_example_error(
                example::ExampleError {
                    version: 1,
                    code: example::ExampleErrorCode::SetupRequired,
                    message: "example setup requires an interactive terminal in human mode".into(),
                },
                cli.json,
            );
        }
        return match example::setup() {
            Ok(report) if cli.json => emit_example_json(&report),
            Ok(report) => {
                match report.state {
                    example::SetupState::Configured => println!(
                        "Example provider configured: {}",
                        report.provider.as_deref().unwrap_or("unknown")
                    ),
                    example::SetupState::Cancelled => println!("Example setup cancelled."),
                }
                Ok(())
            }
            Err(error) => emit_example_error(error, cli.json),
        };
    }
    if cli.example_status {
        return match example::status(!interactive) {
            Ok(report) if cli.json => emit_example_json(&report),
            Ok(report) => {
                if report.configured {
                    println!(
                        "Example provider: {}",
                        report.provider.as_deref().unwrap_or("unknown")
                    );
                    println!("Credential: {:?}", report.credential);
                } else {
                    println!("Example provider: not configured");
                }
                Ok(())
            }
            Err(error) => emit_example_error(error, cli.json),
        };
    }
    if cli.example_reset {
        if !interactive {
            return emit_example_error(
                example::ExampleError {
                    version: 1,
                    code: example::ExampleErrorCode::SetupRequired,
                    message: "example reset requires an interactive terminal in human mode".into(),
                },
                cli.json,
            );
        }
        return match example::reset() {
            Ok(report) if cli.json => emit_example_json(&report),
            Ok(report) => {
                match report.state {
                    example::ResetState::Removed => println!("Example provider removed."),
                    example::ResetState::NotConfigured => {
                        println!("Example provider was not configured.")
                    }
                }
                if report.credential_cleanup == example::ResetCredentialCleanup::Unavailable {
                    eprintln!(
                        "qs: the native credential store was unavailable; run --example-reset again when it is available"
                    );
                }
                Ok(())
            }
            Err(error) => emit_example_error(error, cli.json),
        };
    }
    Err("no example management action was selected".into())
}

fn emit_example_result(
    result: Result<example::ExampleSuccess, example::ExampleError>,
    json: bool,
) -> Result<(), String> {
    match result {
        Ok(output) if json => emit_example_json(&output),
        Ok(output) => example::render_human(&output, &mut std::io::stdout())
            .map_err(|error| format!("example: {}", error.message)),
        Err(mut error) => {
            const FALLBACK: &str = "; run the same query without --example to view source results";
            if error.message.len() + FALLBACK.len() <= 512 {
                error.message.push_str(FALLBACK);
            }
            emit_example_error(error, json)
        }
    }
}

fn emit_example_json(value: &impl serde::Serialize) -> Result<(), String> {
    let encoded = example::to_json(value).map_err(|error| error.message)?;
    println!("{encoded}");
    Ok(())
}

fn emit_example_error(error: example::ExampleError, json: bool) -> Result<(), String> {
    if json {
        let encoded = example::to_json(&error).map_err(|serialization| serialization.message)?;
        println!("{encoded}");
        std::process::exit(1);
    }
    Err(format!("example: {}", error.message))
}

fn handle_cache_command(query: &str, json: bool, has_action: bool) -> Result<(), String> {
    if has_action {
        return Err("--open and --copy do not apply to cache commands".into());
    }
    let words = query.split_whitespace().collect::<Vec<_>>();
    if words.len() != 2 {
        return Err("usage: qs cache <status|clear>".into());
    }
    let root = crate::cache::root_dir();
    let action = words[1].to_ascii_lowercase();
    let (before, stats) = match action.as_str() {
        "status" => {
            let stats = crate::cache::enforce_global_budget(&root)?;
            (stats.clone(), stats)
        }
        "clear" => crate::cache::clear_cache_report(&root)?,
        _ => return Err("usage: qs cache <status|clear>".into()),
    };
    if json {
        let value = serde_json::json!({
            "action": action,
            "root": root,
            "bytes": stats.bytes,
            "files": stats.files,
            "limit_bytes": stats.limit_bytes,
            "within_limit": stats.bytes <= stats.limit_bytes,
            "removed_bytes": if words[1].eq_ignore_ascii_case("clear") {
                before.bytes.saturating_sub(stats.bytes)
            } else { 0 },
            "removed_files": if words[1].eq_ignore_ascii_case("clear") {
                before.files.saturating_sub(stats.files)
            } else { 0 },
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&value).map_err(|error| error.to_string())?
        );
    } else if action == "clear" {
        println!(
            "Cleared {} files ({} bytes). Cache now uses {} of {} bytes.",
            before.files.saturating_sub(stats.files),
            before.bytes.saturating_sub(stats.bytes),
            stats.bytes,
            stats.limit_bytes
        );
    } else {
        println!(
            "Cache: {}\nUsage: {} / {} bytes ({} files)",
            root.display(),
            stats.bytes,
            stats.limit_bytes,
            stats.files
        );
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
    let attempts: &[(&str, &[&str])] = if cfg!(target_os = "windows") {
        &[("clip", &[])]
    } else if cfg!(target_os = "macos") {
        &[("pbcopy", &[])]
    } else {
        &[("wl-copy", &[]), ("xclip", &["-selection", "clipboard"])]
    };
    for &(bin, args) in attempts {
        let mut child = match Command::new(bin)
            .args(args)
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
    Err(format!(
        "{url}\nno clipboard tool (wl-copy, xclip, pbcopy, or clip)"
    ))
}

fn color_enabled(no_color: bool) -> bool {
    if no_color || std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    std::io::stdout().is_terminal()
}

fn editor_command(editor: &str) -> Result<(String, Vec<String>), String> {
    let mut parts = shlex::split(editor)
        .ok_or("EDITOR has invalid quoting")?
        .into_iter();
    let bin = parts.next().ok_or("EDITOR is empty")?;
    Ok((bin, parts.collect()))
}

static EDIT_TEMP_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

struct TempQueryFile {
    dir: std::path::PathBuf,
    path: std::path::PathBuf,
}

impl TempQueryFile {
    fn create(seed: &str) -> Result<Self, String> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        for _ in 0..128 {
            let sequence = EDIT_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!(
                ".qs-edit-{}-{nanos}-{sequence}",
                std::process::id()
            ));
            #[cfg(unix)]
            let created = {
                use std::os::unix::fs::DirBuilderExt;
                let mut builder = std::fs::DirBuilder::new();
                builder.mode(0o700);
                builder.create(&dir)
            };
            #[cfg(not(unix))]
            let created = std::fs::create_dir(&dir);
            match created {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.to_string()),
            }
            let path = dir.join("query.txt");
            let opened = {
                let mut options = std::fs::OpenOptions::new();
                options.write(true).create_new(true);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.mode(0o600);
                }
                options.open(&path)
            };
            let mut file = match opened {
                Ok(file) => file,
                Err(error) => {
                    let _ = std::fs::remove_dir(&dir);
                    return Err(error.to_string());
                }
            };
            if let Err(error) = file.write_all(seed.as_bytes()).and_then(|()| file.flush()) {
                let _ = std::fs::remove_file(&path);
                let _ = std::fs::remove_dir(&dir);
                return Err(error.to_string());
            }
            return Ok(Self { dir, path });
        }
        Err("could not create a private editor temporary directory".into())
    }
}

impl Drop for TempQueryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_dir(&self.dir);
    }
}

fn compose_query(seed: &str, quiet_stdout: bool) -> Result<String, String> {
    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            std::env::var("EDITOR")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .unwrap_or_else(|| {
            if cfg!(target_os = "windows") {
                "notepad".into()
            } else {
                "vi".into()
            }
        });
    let temporary = TempQueryFile::create(seed)?;
    let path = &temporary.path;

    let (bin, args) = editor_command(&editor)?;
    let mut command = Command::new(&bin);
    command.args(&args).arg(path);
    if quiet_stdout {
        command.stdout(Stdio::null());
    }
    let status = command
        .status()
        .map_err(|err| format!("launch {bin}: {err}"))?;
    let text = std::fs::read_to_string(path).unwrap_or_default();
    if !status.success() {
        return Err(format!("{bin} exited with {status}"));
    }
    let query = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if query.is_empty() {
        return Err("empty query".into());
    }
    Ok(query)
}

fn open_url(url: &str, quiet: bool) -> Result<(), String> {
    if url.starts_with("man://") {
        let mut parts = url.trim_start_matches("man://").split('/');
        let section = parts.next().unwrap_or("");
        let page = parts.next().unwrap_or("");
        let safe_component = |value: &str| {
            !value.is_empty()
                && !value.starts_with('-')
                && value.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'+')
                })
        };
        if !safe_component(section) || !safe_component(page) || parts.next().is_some() {
            return Err("refusing to open an unsafe man URL".into());
        }
        let mut command = Command::new("man");
        command.args(["-s", section, "--", page]);
        if quiet {
            command.stdout(Stdio::null()).stderr(Stdio::null());
        }
        let status = command.status().map_err(|err| format!("man: {err}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("man exited with {status}"))
        }
    } else {
        let url = crate::text::normalize_http_url(url)
            .ok_or_else(|| "refusing to open an unsafe URL".to_string())?;
        let name = if cfg!(target_os = "macos") {
            "open"
        } else if cfg!(target_os = "windows") {
            "rundll32"
        } else {
            "xdg-open"
        };
        let mut command = Command::new(name);
        if cfg!(target_os = "windows") {
            command.args(["url.dll,FileProtocolHandler", &url]);
        } else {
            command.arg(&url);
        }
        let status = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|err| format!("{name}: {err}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{name} exited with {status}"))
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn bare_actions_do_not_consume_the_query() {
        let open = Cli::try_parse_from(["qs", "--open", "rust", "Vec"]).unwrap();
        assert_eq!(open.open, Some(1));
        assert_eq!(open.query, ["rust", "Vec"]);

        let copy = Cli::try_parse_from(["qs", "--copy", "python", "list"]).unwrap();
        assert_eq!(copy.copy, Some(1));
        assert_eq!(copy.query, ["python", "list"]);

        let indexed = Cli::try_parse_from(["qs", "--open=2", "rust", "Vec"]).unwrap();
        assert_eq!(indexed.open, Some(2));
    }

    #[test]
    fn example_flags_preserve_queries_and_reject_actions() {
        let parsed = Cli::try_parse_from(["qs", "--example", "rust", "Vec"]).unwrap();
        assert!(parsed.example);
        assert_eq!(parsed.query, ["rust", "Vec"]);
        assert!(Cli::try_parse_from(["qs", "--example", "--open", "rust"]).is_err());
        assert!(Cli::try_parse_from(["qs", "--example-setup", "--example-status"]).is_err());
    }

    #[test]
    fn leading_e_is_normalized_before_option_parsing() {
        let alias = Cli::try_parse_from(normalize_cli_args([
            "qs",
            "e",
            "--json",
            "py",
            "multithread",
        ]))
        .unwrap();
        assert!(alias.example);
        assert!(alias.json);
        assert_eq!(alias.query, ["py", "multithread"]);

        let uppercase =
            Cli::try_parse_from(normalize_cli_args(["qs", "E", "rust", "Vec"])).unwrap();
        assert!(uppercase.example);
        assert_eq!(uppercase.query, ["rust", "Vec"]);

        let explicit =
            Cli::try_parse_from(normalize_cli_args(["qs", "--example", "e", "value"])).unwrap();
        assert_eq!(explicit.query, ["e", "value"]);

        let provider = Cli::try_parse_from(normalize_cli_args([
            "qs",
            "-p",
            "web",
            "e",
            "py",
            "multithread",
        ]))
        .unwrap();
        assert!(provider.example);
        assert_eq!(provider.provider, Some(Provider::Web));
        assert_eq!(provider.query, ["py", "multithread"]);

        assert!(Cli::try_parse_from(normalize_cli_args([
            "qs",
            "e",
            "--open",
            "py",
            "multithread",
        ]))
        .is_err());
    }

    #[test]
    fn editor_command_preserves_quoted_argv_boundaries() {
        let (bin, args) = editor_command(r#""/tmp/editor tool" --wait "two words""#).unwrap();
        assert_eq!(bin, "/tmp/editor tool");
        assert_eq!(args, ["--wait", "two words"]);
        assert!(editor_command("\"").is_err());
    }
}
