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

pub mod aliases;
pub mod bangs;
mod cheat;
pub mod commands;
mod commodity;
pub mod concepts;
mod crates;
mod cve;
mod docs;
mod fallback;
mod ghc_guide;
mod github;
mod go_packages;
mod hackage_packages;
pub mod history;
pub mod langs;
mod man;
mod mdn;
mod python_docs;
pub(crate) mod registries;
mod rfc;
mod rust_guides;
mod rustdoc;
pub mod sources;
mod stack;
mod stack_similar;
pub mod stock;
mod web;
mod wiki;
mod wikis;

use crate::http::Client;
use crate::model::Outcome;
use crate::route::Provider;

pub fn search(
    client: &Client,
    provider: Provider,
    query: &str,
    limit: usize,
    color: bool,
) -> Result<Outcome, String> {
    let result = match provider {
        Provider::Web => commodity::search(client, query, limit),
        Provider::Cheat => cheat::search(client, query, color),
        Provider::Crate => crates::search(client, query, limit),
        Provider::Docs => docs::search(client, query, limit),
        Provider::Rust => rustdoc::search_std(client, query, limit),
        Provider::So => stack::search(client, query, limit),
        Provider::Gh => github::search(client, query, limit),
        Provider::Mdn => mdn::search(client, query, limit),
        Provider::Wiki => wiki::search(client, query, limit),
        Provider::Man => man::search(query, limit),
        Provider::Arch => wikis::search(client, &wikis::arch(), query, limit),
        Provider::Gentoo => wikis::search(client, &wikis::gentoo(), query, limit),
        Provider::Fedora => wikis::search(client, &wikis::fedora(), query, limit),
        Provider::Debian => wikis::search(client, &wikis::debian(), query, limit),
        Provider::Ubuntu => wikis::search(client, &wikis::ubuntu(), query, limit),
        Provider::Rfc => rfc::search(client, query, limit),
        Provider::Npm => registries::npm(client, query, limit),
        Provider::Pypi => registries::pypi(client, query, limit),
        Provider::Gem => registries::gems(client, query, limit),
        Provider::Hex => registries::hex(client, query, limit),
        Provider::GoMod => registries::gomod(client, query, limit),
        Provider::Maven => registries::maven(client, query, limit),
        Provider::Nuget => registries::nuget(client, query, limit),
        Provider::Cve => cve::search(client, query, limit),
        Provider::Stock => stock::search(client, query, limit),
    };
    let mut outcome = result.or_else(|reason| provider_fallback(provider, query, reason))?;
    let provider_id = provider_id(provider);
    if outcome.provider != provider_id {
        outcome.source_provider = Some(outcome.provider.clone());
    }
    outcome.provider = provider_id.into();
    Ok(outcome)
}

fn provider_id(provider: Provider) -> &'static str {
    match provider {
        Provider::Web => "web",
        Provider::Cheat => "cheat",
        Provider::Crate => "crate",
        Provider::Docs => "docs",
        Provider::Rust => "rust",
        Provider::So => "so",
        Provider::Gh => "gh",
        Provider::Mdn => "mdn",
        Provider::Wiki => "wiki",
        Provider::Man => "man",
        Provider::Arch => "arch",
        Provider::Gentoo => "gentoo",
        Provider::Fedora => "fedora",
        Provider::Debian => "debian",
        Provider::Ubuntu => "ubuntu",
        Provider::Rfc => "rfc",
        Provider::Npm => "npm",
        Provider::Pypi => "pypi",
        Provider::Gem => "gem",
        Provider::Hex => "hex",
        Provider::GoMod => "go-mod",
        Provider::Maven => "maven",
        Provider::Nuget => "nuget",
        Provider::Cve => "cve",
        Provider::Stock => "stock",
    }
}

fn provider_fallback(provider: Provider, query: &str, reason: String) -> Result<Outcome, String> {
    use crate::text::encode_query;

    let encoded = encode_query(query);
    let (label, url) = match provider {
        Provider::Web | Provider::Stock => return Err(reason),
        Provider::Cheat => ("cheat.sh", format!("https://cheat.sh/~{encoded}")),
        Provider::Crate => ("crates.io", format!("https://crates.io/search?q={encoded}")),
        Provider::Docs | Provider::Rust => (
            "Rust documentation",
            format!("https://docs.rs/releases/search?query={encoded}"),
        ),
        Provider::So => (
            "Stack Overflow",
            format!("https://stackoverflow.com/search?q={encoded}"),
        ),
        Provider::Gh => ("GitHub", format!("https://github.com/search?q={encoded}")),
        Provider::Mdn => (
            "MDN",
            format!("https://developer.mozilla.org/search?q={encoded}"),
        ),
        Provider::Wiki => (
            "Wikipedia",
            format!("https://en.wikipedia.org/w/index.php?search={encoded}"),
        ),
        Provider::Man => (
            "online manuals",
            format!("https://manpages.debian.org/cgi-bin/search.py?q={encoded}"),
        ),
        Provider::Arch => (
            "Arch Wiki",
            format!("https://wiki.archlinux.org/index.php?search={encoded}"),
        ),
        Provider::Gentoo => (
            "Gentoo Wiki",
            format!("https://wiki.gentoo.org/index.php?search={encoded}"),
        ),
        Provider::Fedora => (
            "Fedora Wiki",
            format!("https://fedoraproject.org/wiki/Special:Search?search={encoded}"),
        ),
        Provider::Debian => (
            "Debian Wiki",
            format!("https://wiki.debian.org/?action=fullsearch&value={encoded}"),
        ),
        Provider::Ubuntu => (
            "Ubuntu documentation",
            format!("https://documentation.ubuntu.com/search/?q={encoded}"),
        ),
        Provider::Rfc => (
            "RFC Editor",
            format!("https://www.rfc-editor.org/search/rfc_search_detail.php?title={encoded}"),
        ),
        Provider::Npm => ("npm", format!("https://www.npmjs.com/search?q={encoded}")),
        Provider::Pypi => ("PyPI", format!("https://pypi.org/search/?q={encoded}")),
        Provider::Gem => (
            "RubyGems",
            format!("https://rubygems.org/search?query={encoded}"),
        ),
        Provider::Hex => ("Hex", format!("https://hex.pm/packages?search={encoded}")),
        Provider::GoMod => (
            "Go packages",
            format!("https://pkg.go.dev/search?q={encoded}"),
        ),
        Provider::Maven => (
            "Maven Central",
            format!("https://central.sonatype.com/search?q={encoded}"),
        ),
        Provider::Nuget => (
            "NuGet",
            format!("https://www.nuget.org/packages?q={encoded}"),
        ),
        Provider::Cve => (
            "NVD",
            format!("https://nvd.nist.gov/vuln/search/results?query={encoded}"),
        ),
    };
    Ok(fallback::search(
        label,
        query,
        format!("Search {label} for {query}"),
        url,
        reason,
    ))
}
