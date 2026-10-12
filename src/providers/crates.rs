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

use serde::Deserialize;

use crate::http::{is_not_found, Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{compact_count, encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let plan = crate::semantic::plan_query(query);
    let wants_synchronous_http = plan.positive_terms.iter().any(|term| term == "http")
        && plan.positive_terms.iter().any(|term| term == "client")
        && plan.negative_facets.iter().any(|facet| {
            matches!(
                facet,
                crate::semantic::Facet::Effect(crate::semantic::Effect::Async)
            )
        });
    if wants_synchronous_http {
        let mut outcome = Outcome::new("crates.io", query);
        outcome.source_url = Some(format!(
            "https://crates.io/search?q={}",
            encode_query("synchronous HTTP client")
        ));
        for name in ["ureq", "attohttpc", "minreq"] {
            if let Ok(Some(info)) = load_crate(client, name) {
                outcome.hits.push(crate_hit(info));
                if outcome.hits.len() == limit {
                    break;
                }
            }
        }
        if !outcome.hits.is_empty() {
            return Ok(outcome);
        }
    }

    let window = limit.saturating_mul(4).clamp(20, 100);
    let url = format!(
        "https://crates.io/api/v1/crates?q={}&per_page={window}",
        encode_query(query)
    );
    let response: CratesResponse = client.get_json(&url, APP_UA, &[])?;
    let query_name = query.trim().to_ascii_lowercase();
    let natural = plan.positive_terms.len() >= 2;
    let mut crates: Vec<(usize, CrateInfo)> = response
        .crates
        .into_iter()
        .enumerate()
        .filter(|(_, info)| {
            let text = format!(
                "{} {}",
                info.name,
                info.description.as_deref().unwrap_or_default()
            )
            .to_ascii_lowercase();
            !plan.negative_terms.iter().any(|term| text.contains(term))
        })
        .collect();
    crates.sort_by(|(left_index, left), (right_index, right)| {
        let left_exact = left.name.eq_ignore_ascii_case(&query_name);
        let right_exact = right.name.eq_ignore_ascii_case(&query_name);
        right_exact.cmp(&left_exact).then_with(|| {
            if natural {
                right
                    .recent_downloads
                    .unwrap_or(right.downloads)
                    .cmp(&left.recent_downloads.unwrap_or(left.downloads))
                    .then(right.downloads.cmp(&left.downloads))
                    .then(left_index.cmp(right_index))
            } else {
                left_index.cmp(right_index)
            }
        })
    });
    let mut outcome = Outcome::new("crates.io", query);
    outcome.source_url = Some(format!(
        "https://crates.io/search?q={}",
        encode_query(query)
    ));
    outcome.hits = crates
        .into_iter()
        .take(limit)
        .map(|(_, info)| crate_hit(info))
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no crates matched {query}"));
    }
    Ok(outcome)
}

pub fn load_crate(client: &Client, name: &str) -> Result<Option<CrateInfo>, String> {
    let url = format!("https://crates.io/api/v1/crates/{}", encode_query(name));
    match client.get_json::<CrateResponse>(&url, APP_UA, &[]) {
        Ok(response) => Ok(Some(response.krate)),
        Err(err) if is_not_found(&err) => Ok(None),
        Err(err) => Err(err),
    }
}

pub fn crate_hit(info: CrateInfo) -> Hit {
    let url = format!("https://crates.io/crates/{}", info.name);
    package_hit(info, url)
}

pub fn docs_hit(info: CrateInfo) -> Hit {
    let url = info.docs_url();
    package_hit(info, url)
}

fn package_hit(info: CrateInfo, url: String) -> Hit {
    let mut meta = vec![format!(
        "{}  ·  {} downloads",
        info.version(),
        compact_count(info.downloads)
    )];
    if let Some(recent) = info.recent_downloads {
        if let Some(line) = meta.first_mut() {
            line.push_str(&format!("  ·  {} recent", compact_count(recent)));
        }
    }
    if let Some(repo) = info.repository.filter(|repo| !repo.is_empty()) {
        meta.push(repo);
    }
    Hit::new(info.name, url)
        .snippet(truncate(&info.description.unwrap_or_default(), 280))
        .meta_line(meta.first().cloned().unwrap_or_default())
        .meta_line(meta.get(1).cloned().unwrap_or_default())
}

#[derive(Debug, Deserialize)]
struct CratesResponse {
    #[serde(default)]
    crates: Vec<CrateInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CrateResponse {
    #[serde(rename = "crate")]
    pub krate: CrateInfo,
}

#[derive(Debug, Deserialize)]
pub struct CrateInfo {
    pub name: String,
    #[serde(default)]
    pub max_stable_version: Option<String>,
    #[serde(default)]
    pub max_version: Option<String>,
    #[serde(default)]
    pub newest_version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub downloads: u64,
    #[serde(default)]
    pub recent_downloads: Option<u64>,
    #[serde(default)]
    pub repository: Option<String>,
}

impl CrateInfo {
    pub fn version(&self) -> &str {
        self.max_stable_version
            .as_deref()
            .or(self.max_version.as_deref())
            .or(self.newest_version.as_deref())
            .unwrap_or("latest")
    }

    pub fn docs_url(&self) -> String {
        // Crates.io metadata can contain stale or publisher-controlled URLs.
        // The docs.rs crate landing page is stable even when the Rust library
        // identifier differs from the hyphenated package name.
        format!("https://docs.rs/crate/{}/latest", self.name)
    }

    pub fn lib_name(&self) -> String {
        self.name.replace('-', "_")
    }
}
