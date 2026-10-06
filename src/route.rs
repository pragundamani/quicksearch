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

use clap::ValueEnum;

use crate::providers::commands::{self, CommandRoute};
use crate::providers::stock;
use crate::providers::{concepts, sources};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Provider {
    Web,
    #[value(alias = "cht", alias = "tldr")]
    Cheat,
    #[value(alias = "crates")]
    Crate,
    #[value(alias = "doc")]
    Docs,
    #[value(alias = "std")]
    Rust,
    #[value(alias = "stack", alias = "stackoverflow")]
    So,
    #[value(alias = "github")]
    Gh,
    Mdn,
    #[value(alias = "wikipedia")]
    Wiki,
    Man,
    #[value(alias = "archwiki")]
    Arch,
    #[value(alias = "rfcs")]
    Rfc,
    Npm,
    #[value(alias = "pip")]
    Pypi,
    Gentoo,
    Debian,
    Fedora,
    Ubuntu,
    #[value(alias = "rubygems")]
    Gem,
    Hex,
    #[value(name = "go-mod", alias = "pkggo")]
    GoMod,
    Maven,
    Nuget,
    #[value(alias = "nvd")]
    Cve,
    #[value(alias = "item", alias = "sku", alias = "upc", alias = "barcode")]
    Stock,
}

pub fn provider_names() -> &'static str {
    "web, cheat, crate, docs, rust, so, gh, mdn, wiki, man, arch, rfc, npm, pypi, gentoo, debian, fedora, ubuntu, gem, hex, go-mod, maven, nuget, cve, stock"
}

pub fn parse_provider_name(name: &str) -> Option<Provider> {
    match name.to_ascii_lowercase().as_str() {
        "web" | "ddg" => Some(Provider::Web),
        "cheat" | "cht" | "tldr" => Some(Provider::Cheat),
        "crate" | "crates" => Some(Provider::Crate),
        "docs" | "doc" => Some(Provider::Docs),
        "rust" | "std" | "rs" => Some(Provider::Rust),
        "so" | "stack" | "stackoverflow" => Some(Provider::So),
        "gh" | "github" => Some(Provider::Gh),
        "mdn" => Some(Provider::Mdn),
        "wiki" | "wikipedia" => Some(Provider::Wiki),
        "man" => Some(Provider::Man),
        "arch" | "archwiki" => Some(Provider::Arch),
        "rfc" | "rfcs" => Some(Provider::Rfc),
        "npm" => Some(Provider::Npm),
        "pypi" | "pip" => Some(Provider::Pypi),
        "gentoo" => Some(Provider::Gentoo),
        "debian" => Some(Provider::Debian),
        "fedora" => Some(Provider::Fedora),
        "ubuntu" => Some(Provider::Ubuntu),
        "gem" | "rubygems" => Some(Provider::Gem),
        "hex" => Some(Provider::Hex),
        "go-mod" | "pkggo" => Some(Provider::GoMod),
        "maven" => Some(Provider::Maven),
        "nuget" => Some(Provider::Nuget),
        "cve" | "nvd" => Some(Provider::Cve),
        "item" | "stock" | "sku" | "upc" | "barcode" => Some(Provider::Stock),
        _ => None,
    }
}

pub struct Routed {
    pub provider: Provider,
    pub query: String,
    /// When true, a leading language or docset name can replace a web search.
    pub allow_docset: bool,
    /// `qs lang <pattern>` lists matching language shorthands.
    pub list_langs: bool,
    /// `qs topic <pattern>` lists matching concept shorthands.
    pub list_topics: bool,
    /// `qs history` lists recent searches.
    pub list_history: bool,
    /// `qs qs` prints the sources table.
    pub list_sources: bool,
    /// Exact manual page, such as `dnf-install`.
    pub manual: Option<String>,
    /// Package manager for `qs dnf rust`.
    pub package_tool: Option<String>,
    /// DuckDuckGo bang name without the leading `!`.
    pub bang: Option<String>,
    /// Expanded URL from the user's sources file.
    pub custom_url: Option<String>,
}

/// Pick a source from an explicit flag, a `!bang`, a leading source name, or a Rust path.
pub fn route(flag: Option<Provider>, query: &str) -> Result<Routed, String> {
    let query = query.trim();
    if query.is_empty() {
        return Err("empty query".into());
    }

    if let Some(rest) = query.strip_prefix('!') {
        let (name, inner) = split_first(rest);
        if inner.is_empty() {
            return Err(format!(
                "missing query after !{name}. sources: {}",
                provider_names()
            ));
        }
        if let Some(provider) = parse_provider_name(name) {
            return Ok(routed(provider, inner.to_string()));
        }
        let mut routed = routed(Provider::Web, inner.to_string());
        routed.bang = Some(name.to_string());
        return Ok(routed);
    }

    if let Some(provider) = flag {
        return Ok(routed(provider, query.to_string()));
    }

    if query.eq_ignore_ascii_case("qs") {
        let mut routed = routed(Provider::Web, String::new());
        routed.list_sources = true;
        return Ok(routed);
    }

    let (name, inner) = split_first(query);
    if is_lang_command(name) {
        let mut routed = routed(Provider::Web, inner.to_string());
        routed.list_langs = true;
        return Ok(routed);
    }
    if is_topic_command(name) {
        let mut routed = routed(Provider::Web, inner.to_string());
        routed.list_topics = true;
        return Ok(routed);
    }
    if is_history_command(name) {
        let mut routed = routed(Provider::Web, inner.to_string());
        routed.list_history = true;
        return Ok(routed);
    }
    if !inner.is_empty() {
        if let Some(provider) = parse_provider_name(name) {
            return Ok(routed(provider, inner.to_string()));
        }
    }
    if let Some(command) = commands::route_command(name, inner) {
        let mut routed = routed(Provider::Man, String::new());
        match command {
            CommandRoute::Manual(page) => {
                routed.manual = Some(page);
                routed.query = query.to_string();
            }
            CommandRoute::Package { tool, name } => {
                routed.package_tool = Some(tool);
                routed.query = name;
            }
        }
        return Ok(routed);
    }
    if stock::is_barcode(query) {
        return Ok(routed(Provider::Stock, query.to_string()));
    }
    if concepts::find(name).is_none() {
        if let Some(url) = sources::lookup(name, inner) {
            if !inner.is_empty() {
                let mut routed = routed(Provider::Web, inner.to_string());
                routed.custom_url = Some(url);
                return Ok(routed);
            }
        }
    }

    if let Some(provider) = infer_code_path(query) {
        return Ok(routed(provider, query.to_string()));
    }

    let mut routed = routed(Provider::Web, query.to_string());
    routed.allow_docset = true;
    Ok(routed)
}

fn routed(provider: Provider, query: String) -> Routed {
    Routed {
        provider,
        query,
        allow_docset: false,
        list_langs: false,
        list_topics: false,
        list_history: false,
        list_sources: false,
        manual: None,
        package_tool: None,
        bang: None,
        custom_url: None,
    }
}

fn is_lang_command(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "lang" | "langs" | "languages"
    )
}

fn is_topic_command(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "topic" | "topics" | "concept" | "concepts"
    )
}

fn is_history_command(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(), "history" | "hist")
}

fn infer_code_path(query: &str) -> Option<Provider> {
    let first = query.split_whitespace().next().unwrap_or("");
    if !first.contains("::") {
        return None;
    }
    match first.split("::").next().unwrap_or("") {
        "std" | "core" | "alloc" | "proc_macro" | "test" => Some(Provider::Rust),
        "" => None,
        _ => Some(Provider::Docs),
    }
}

fn split_first(text: &str) -> (&str, &str) {
    let text = text.trim();
    match text.split_once(char::is_whitespace) {
        Some((head, tail)) => (head, tail.trim()),
        None => (text, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn routed(flag: Option<Provider>, query: &str) -> Routed {
        route(flag, query).unwrap()
    }

    #[test]
    fn leading_word_selects_a_source() {
        let got = routed(None, "cheat tar");
        assert_eq!(got.provider, Provider::Cheat);
        assert_eq!(got.query, "tar");
        let got = routed(None, "man git rebase");
        assert_eq!(got.provider, Provider::Man);
        assert_eq!(got.query, "git rebase");
    }

    #[test]
    fn bangs_override_and_unknown_bangs_redirect() {
        let got = routed(Some(Provider::Web), "!so lifetimes");
        assert_eq!(got.provider, Provider::So);
        assert_eq!(got.query, "lifetimes");
        assert!(got.bang.is_none());
        let wiki = routed(None, "!wiki vim");
        assert_eq!(wiki.provider, Provider::Wiki);
        assert_eq!(wiki.query, "vim");
        let bang = routed(None, "!aur paru");
        assert_eq!(bang.bang.as_deref(), Some("aur"));
        assert_eq!(bang.query, "paru");
        assert!(route(None, "!man").is_err());
    }

    #[test]
    fn explicit_provider_keeps_the_query_intact() {
        let got = routed(Some(Provider::Web), "man pages");
        assert_eq!(got.provider, Provider::Web);
        assert_eq!(got.query, "man pages");
        assert!(!got.allow_docset);
    }

    #[test]
    fn rust_paths_pick_docs_without_a_prefix() {
        assert_eq!(routed(None, "std::fs::read").provider, Provider::Rust);
        assert_eq!(routed(None, "serde::Deserialize").provider, Provider::Docs);
        let prose = routed(None, "how to read a file in rust");
        assert_eq!(prose.provider, Provider::Web);
        assert!(prose.allow_docset);
        let cargo = routed(None, "how to upgrade cargo binaries");
        assert_eq!(cargo.provider, Provider::Web);
        assert!(cargo.manual.is_none());
        assert!(cargo.allow_docset);
    }

    #[test]
    fn language_prefixes_stay_available_for_docsets() {
        let got = routed(None, "cpp std::vector");
        assert_eq!(got.provider, Provider::Web);
        assert_eq!(got.query, "cpp std::vector");
        assert!(got.allow_docset);
        let forced = routed(None, "web cpp std::vector");
        assert_eq!(forced.provider, Provider::Web);
        assert_eq!(forced.query, "cpp std::vector");
        assert!(!forced.allow_docset);
    }

    #[test]
    fn lang_lists_shorthands() {
        let got = routed(None, "lang py");
        assert!(got.list_langs);
        assert_eq!(got.query, "py");
        let bare = routed(None, "lang");
        assert!(bare.list_langs);
        assert!(bare.query.is_empty());
    }

    #[test]
    fn qs_lists_the_sources_table() {
        let got = routed(None, "qs");
        assert!(got.list_sources);
        assert!(got.query.is_empty());
        let search = routed(Some(Provider::Web), "qs");
        assert!(!search.list_sources);
        assert_eq!(search.query, "qs");
    }

    #[test]
    fn topic_lists_concepts() {
        let got = routed(None, "topic net");
        assert!(got.list_topics);
        assert_eq!(got.query, "net");
        assert!(!got.list_langs);
        let bare = routed(None, "topics");
        assert!(bare.list_topics);
        assert!(bare.query.is_empty());
    }

    #[test]
    fn command_manuals_and_package_info() {
        let page = routed(None, "dnf");
        assert_eq!(page.manual.as_deref(), Some("dnf"));
        let install = routed(None, "dnf install");
        assert_eq!(install.manual.as_deref(), Some("dnf-install"));
        let package = routed(None, "dnf rust");
        assert_eq!(package.package_tool.as_deref(), Some("dnf"));
        assert_eq!(package.query, "rust");
        let both = routed(None, "dnf install rust");
        assert_eq!(both.package_tool.as_deref(), Some("dnf"));
        assert_eq!(both.query, "rust");
        let cargo = routed(None, "cargo build");
        assert_eq!(cargo.manual.as_deref(), Some("cargo-build"));
        let brew = routed(None, "brew install");
        assert_eq!(brew.manual.as_deref(), Some("brew-install"));
        let winget = routed(None, "winget rust");
        assert_eq!(winget.package_tool.as_deref(), Some("winget"));
        assert_eq!(winget.query, "rust");
    }

    #[test]
    fn registries_and_reference_sources() {
        assert_eq!(routed(None, "arch pacman").provider, Provider::Arch);
        assert_eq!(routed(None, "rfc 9110").provider, Provider::Rfc);
        assert_eq!(routed(None, "rfc 9110").query, "9110");
        assert_eq!(routed(None, "npm lodash").provider, Provider::Npm);
        assert_eq!(routed(None, "pypi requests").provider, Provider::Pypi);
        assert_eq!(routed(None, "gem rails").provider, Provider::Gem);
        assert_eq!(routed(None, "crate serde").provider, Provider::Crate);
        assert_eq!(routed(None, "go-mod chi").provider, Provider::GoMod);
        assert_eq!(routed(None, "cve openssl").provider, Provider::Cve);
        assert_eq!(routed(None, "man grep").provider, Provider::Man);
        assert_eq!(routed(None, "linux iptables").provider, Provider::Web);
        assert!(routed(None, "linux iptables").allow_docset);
    }

    #[test]
    fn barcodes_are_stock_and_years_stay_web() {
        let code = routed(None, "8901491101833");
        assert_eq!(code.provider, Provider::Stock);
        let year = routed(None, "1912");
        assert_eq!(year.provider, Provider::Web);
        assert!(year.allow_docset);
        let named = routed(None, "item jio tag");
        assert_eq!(named.provider, Provider::Stock);
        assert_eq!(named.query, "jio tag");
    }
}
