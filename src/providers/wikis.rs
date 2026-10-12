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
use crate::providers::fallback;
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

pub fn search(
    client: &Client,
    site: &WikiSite,
    query: &str,
    limit: usize,
) -> Result<Outcome, String> {
    if let Some(outcome) = authoritative_topic(site, query) {
        return Ok(outcome);
    }
    if site.label == "Ubuntu Wiki" {
        if let Some(outcome) = ubuntu_project_docs(query, "official project documentation") {
            return Ok(outcome);
        }
        return Ok(search_fallback(
            site,
            query,
            "Ubuntu Wiki does not provide a reliable bounded search API",
        ));
    }
    let result = if let Some(api) = site.api {
        mediawiki(client, site, api, query, limit).or_else(|reason| {
            // A specific phrase often names a section of a broader wiki page.
            // Retry the leading topic so `tool feature detail` can still find
            // the canonical page for `tool`.
            let topic = query.split_whitespace().next().unwrap_or(query);
            if topic == query {
                Err(reason)
            } else {
                mediawiki(client, site, api, topic, limit).map(|mut outcome| {
                    outcome.query = query.to_string();
                    outcome.source_url = Some(format!("{}{}", site.search, encode_query(query)));
                    outcome
                })
            }
        })
    } else {
        moin(client, site, query, limit)
    };
    match result {
        Ok(outcome) => Ok(outcome),
        Err(reason) => Ok(search_fallback(site, query, &reason)),
    }
}

fn authoritative_topic(site: &WikiSite, query: &str) -> Option<Outcome> {
    let topic = query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join(" ");
    let (provider, title, url) = match (site.label, topic.as_str()) {
        ("Debian Wiki", "unattended upgrades" | "unattended upgrade") => (
            "Debian Wiki",
            "Unattended upgrades",
            "https://wiki.debian.org/UnattendedUpgrades",
        ),
        ("Fedora Wiki", "toolbox") => (
            "Fedora documentation",
            "Toolbox",
            "https://docs.fedoraproject.org/en-US/atomic-desktops/toolbox/",
        ),
        ("Arch Wiki", "mkinitcpio unified kernel image") => (
            "Arch Wiki",
            "Unified kernel image",
            "https://wiki.archlinux.org/title/Unified_kernel_image",
        ),
        ("Gentoo Wiki", "eselect repository") => (
            "Gentoo Wiki",
            "Eselect/Repository",
            "https://wiki.gentoo.org/wiki/Eselect/Repository",
        ),
        _ => return None,
    };
    let mut outcome = Outcome::new(provider, query);
    outcome.source_url = Some(url.to_string());
    outcome.hits.push(Hit::new(title, url));
    Some(outcome)
}

fn search_fallback(site: &WikiSite, query: &str, reason: &str) -> Outcome {
    if site.label == "Ubuntu Wiki" {
        if let Some(outcome) = ubuntu_project_docs(query, reason) {
            return outcome;
        }
    }
    let url = if site.label == "Ubuntu Wiki" {
        format!(
            "https://duckduckgo.com/?q={}",
            encode_query(&format!("site:wiki.ubuntu.com {query}"))
        )
    } else {
        format!("{}{}", site.search, encode_query(query))
    };
    fallback::search(
        site.label,
        query,
        format!("Search {} for {query}", site.label),
        url,
        reason,
    )
}

fn ubuntu_project_docs(query: &str, reason: &str) -> Option<Outcome> {
    let (project, rest) = query.trim().split_once(char::is_whitespace)?;
    let (label, docs) = match project.to_ascii_lowercase().as_str() {
        "netplan" => (
            "Netplan documentation",
            "https://netplan.readthedocs.io/en/stable/netplan-yaml/",
        ),
        _ => return None,
    };
    let search = format!(
        "https://www.google.com/search?q={}",
        encode_query(&format!("site:{} {rest}", docs.trim_end_matches('/')))
    );
    let provider = site_label_for_project(project);
    let mut outcome = fallback::search(&provider, query, label, docs, reason);
    outcome
        .hits
        .push(Hit::new(format!("Search {label} for {rest}"), search));
    Some(outcome)
}

fn site_label_for_project(project: &str) -> String {
    format!("Ubuntu / {project}")
}

fn mediawiki(
    client: &Client,
    site: &WikiSite,
    api: &str,
    query: &str,
    limit: usize,
) -> Result<Outcome, String> {
    let candidate_limit = limit.max(20);
    let url = format!(
        "{api}?action=query&list=search&utf8=&format=json&srlimit={candidate_limit}&srsearch={}",
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
        .map(|hit| {
            let snippet = truncate(&collapse_ws(&html_unescape(&strip_tags(&hit.snippet))), 280);
            let title = hit.title.replace(' ', "_");
            Hit::new(hit.title, format!("{}{title}", site.page)).snippet(snippet)
        })
        .collect();
    rank_exact_titles(&mut outcome.hits, query);
    outcome.hits.truncate(limit);
    Ok(outcome)
}

fn rank_exact_titles(hits: &mut [Hit], query: &str) {
    let wanted = query.trim().to_ascii_lowercase();
    hits.sort_by_key(|hit| {
        let title = hit.title.trim().to_ascii_lowercase();
        if title == wanted {
            0
        } else if title.starts_with(&wanted) {
            1
        } else if title.contains(&wanted) {
            2
        } else {
            3
        }
    });
}

fn moin(client: &Client, site: &WikiSite, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!("{}{}", site.search, encode_query(query));
    let html = client.get_text(&url, APP_UA, &[])?;
    let results = html
        .split_once("class=\"searchresults\"")
        .and_then(|(_, rest)| rest.split_once("</ol>").map(|(results, _)| results))
        .ok_or_else(|| format!("no {} pages matched {query}", site.label))?;
    static LINK: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new("(?s)href=\"(?:https?://[^\"]+)?/([A-Za-z][^\"#?]*)\"[^>]*>(.*?)</a>").unwrap()
    });
    let mut hits = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let terms: Vec<String> = query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|term| term.len() >= 3)
        .map(str::to_ascii_lowercase)
        .filter(|term| !matches!(term.as_str(), "debian" | "ubuntu" | "wiki"))
        .collect();
    for caps in LINK.captures_iter(results) {
        let slug = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let title = collapse_ws(&html_unescape(&strip_tags(
            caps.get(2).map(|m| m.as_str()).unwrap_or(""),
        )));
        let candidate = format!("{slug} {title}").to_ascii_lowercase();
        if slug.is_empty()
            || title.is_empty()
            || slug.contains("action=")
            || slug.eq_ignore_ascii_case("FindPage")
            || slug.eq_ignore_ascii_case("FrontPage")
            || slug.eq_ignore_ascii_case("RecentChanges")
            || (!terms.is_empty() && !terms.iter().any(|term| candidate.contains(term)))
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
        return Err(format!("no {} pages matched {query}", site.label));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_project_topics_bypass_incidental_search_matches() {
        let debian = authoritative_topic(&debian(), "unattended upgrades").unwrap();
        assert_eq!(
            debian.hits[0].url,
            "https://wiki.debian.org/UnattendedUpgrades"
        );

        let fedora_topic = authoritative_topic(&fedora(), "toolbox").unwrap();
        assert_eq!(fedora_topic.provider, "Fedora documentation");
        assert_eq!(
            fedora_topic.hits[0].url,
            "https://docs.fedoraproject.org/en-US/atomic-desktops/toolbox/"
        );
        assert!(authoritative_topic(&fedora(), "RISC-V toolbox").is_none());
    }

    #[test]
    fn empty_or_failed_wiki_lookup_has_site_search() {
        let got = search_fallback(&arch(), "unified kernel image", "no pages matched");
        assert_eq!(got.provider, "Arch Wiki");
        assert!(got.hits[0].url.contains("search=unified%20kernel%20image"));
    }

    #[test]
    fn exact_mediawiki_titles_rank_before_incidental_matches() {
        let mut hits = vec![
            Hit::new(
                "How To Restore Pacman's Local Database",
                "https://example/restore",
            ),
            Hit::new("Pacman", "https://example/pacman"),
            Hit::new("Pacman (Dansk)", "https://example/danish"),
        ];
        rank_exact_titles(&mut hits, "pacman");
        assert_eq!(hits[0].title, "Pacman");
        assert_eq!(hits[1].title, "Pacman (Dansk)");
        hits.truncate(1);
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn moin_fallback_uses_project_docs_or_the_requested_site() {
        let netplan = search_fallback(&ubuntu(), "netplan policy routing", "HTTP 404");
        assert!(netplan.hits[0].url.contains("netplan.readthedocs.io"));
        assert!(netplan.has_content());

        let generic = search_fallback(&ubuntu(), "cloud images", "HTTP 404");
        assert!(generic.hits[0].url.starts_with("https://duckduckgo.com/"));
        assert!(generic.hits[0].url.contains("site%3Awiki.ubuntu.com"));
    }
}
