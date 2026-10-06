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

pub mod bangs;
mod cheat;
pub mod commands;
pub mod concepts;
mod crates;
mod cve;
mod docs;
mod github;
pub mod history;
pub mod langs;
mod man;
mod mdn;
mod registries;
mod rfc;
mod rustdoc;
pub mod sources;
mod stack;
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
    match provider {
        Provider::Web => web::search(client, query, limit),
        Provider::Cheat => cheat::search(client, query, color),
        Provider::Crate => crates::search(client, query, limit),
        Provider::Docs => docs::search(client, query, limit),
        Provider::Rust => rustdoc::search_std(client, query),
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
    }
}
