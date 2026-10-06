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

use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape, strip_tags, truncate};

pub struct WikiSite {
    pub label: &'static str,
    pub api: Option<&'static str>,
    pub search: &'static str,
    pub page: &'static str,
}

pub fn arch() -> WikiSite {
    WikiSite {
        label: "Arch Wiki",
        api: Some("https://wiki.archlinux.org/api.php"),
        search: "https://wiki.archlinux.org/index.php?search=",
        page: "https://wiki.archlinux.org/title/",
    }
}

pub fn gentoo() -> WikiSite {
    WikiSite {
        label: "Gentoo Wiki",
        api: Some("https://wiki.gentoo.org/api.php"),
        search: "https://wiki.gentoo.org/index.php?search=",
        page: "https://wiki.gentoo.org/wiki/",
    }
}

pub fn fedora() -> WikiSite {
    WikiSite {
        label: "Fedora Wiki",
        api: Some("https://fedoraproject.org/w/api.php"),
        search: "https://fedoraproject.org/wiki/Special:Search?search=",
        page: "https://fedoraproject.org/wiki/",
    }
}

pub fn debian() -> WikiSite {
    WikiSite {
        label: "Debian Wiki",
        api: None,
        search: "https://wiki.debian.org/?action=fullsearch&context=180&value=",
        page: "https://wiki.debian.org/",
    }
}

pub fn ubuntu() -> WikiSite {
    WikiSite {
        label: "Ubuntu Wiki",
        api: None,
        search: "https://wiki.ubuntu.com/?action=fullsearch&context=180&value=",
        page: "https://wiki.ubuntu.com/",
    }
}

pub fn search(client: &Client, site: &WikiSite, query: &str, limit: usize) -> Result<Outcome, String> {
    if let Some(api) = site.api {
        mediawiki(client, site, api, query, limit)
    } else {
        moin(client, site, query, limit)
    }
}

fn mediawiki(
    client: &Client,
    site: &WikiSite,
    api: &str,
    query: &str,
    limit: usize,
) -> Result<Outcome, String> {
    let url = format!(
        "{api}?action=query&list=search&utf8=&format=json&srlimit={limit}&srsearch={}",
        encode_query(query)
    );
    let response: WikiSearch = client.get_json(&url, APP_UA, &[])?;
    let hits = response.query.search;
    if hits.is_empty() {
        return Err(format!("no {} pages matched {query}", site.label));
    }
    let mut outcome = Outcome::new(site.label, query);
    outcome.source_url = Some(format!("{}{}", site.search, encode_query(query)));
    outcome.hits = hits
        .into_iter()
        .take(limit)
        .map(|hit| {
            let snippet = truncate(&collapse_ws(&html_unescape(&strip_tags(&hit.snippet))), 280);
            let title = hit.title.replace(' ', "_");
            Hit::new(hit.title, format!("{}{title}", site.page)).snippet(snippet)
        })
        .collect();
    Ok(outcome)
}

fn moin(client: &Client, site: &WikiSite, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!("{}{}", site.search, encode_query(query));
    let html = client.get_text(&url, APP_UA, &[])?;
    static LINK: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new("href=\"(?:https?://[^\"]+)?/([A-Za-z][^\"#?]*)\"[^>]*>([^<]+)</a>").unwrap()
    });
    let mut hits = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for caps in LINK.captures_iter(&html) {
        let slug = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let title = collapse_ws(&html_unescape(caps.get(2).map(|m| m.as_str()).unwrap_or("")));
        if slug.is_empty()
            || title.is_empty()
            || slug.contains("action=")
            || slug.eq_ignore_ascii_case("FindPage")
            || slug.eq_ignore_ascii_case("FrontPage")
            || !seen.insert(slug.to_string())
        {
            continue;
        }
        hits.push(Hit::new(title, format!("{}{slug}", site.page)));
        if hits.len() == limit {
            break;
        }
    }
    if hits.is_empty() {
        hits.push(Hit::new(query, url.clone()));
    }
    let mut outcome = Outcome::new(site.label, query);
    outcome.source_url = Some(url);
    outcome.hits = hits;
    Ok(outcome)
}

#[derive(Debug, Deserialize)]
struct WikiSearch {
    query: WikiQuery,
}

#[derive(Debug, Deserialize)]
struct WikiQuery {
    #[serde(default)]
    search: Vec<WikiHit>,
}

#[derive(Debug, Deserialize)]
struct WikiHit {
    title: String,
    #[serde(default)]
    snippet: String,
}
