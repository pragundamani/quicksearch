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

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{compact_count, encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://api.github.com/search/repositories?sort=stars&per_page={limit}&q={}",
        encode_query(query)
    );
    let response: GhResponse =
        client.get_json(&url, APP_UA, &[("Accept", "application/vnd.github+json")])?;
    let mut outcome = Outcome::new("github", query);
    outcome.source_url = Some(format!(
        "https://github.com/search?type=repositories&q={}",
        encode_query(query)
    ));
    outcome.hits = response
        .items
        .into_iter()
        .take(limit)
        .map(|repo| {
            let language = repo.language.unwrap_or_else(|| "unknown".into());
            Hit::new(repo.full_name, repo.html_url)
                .snippet(truncate(&repo.description.unwrap_or_default(), 280))
                .meta_line(format!(
                    "{} stars  ·  {language}",
                    compact_count(repo.stargazers_count)
                ))
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no github repositories matched {query}"));
    }
    Ok(outcome)
}

#[derive(Debug, Deserialize)]
struct GhResponse {
    #[serde(default)]
    items: Vec<GhRepo>,
}

#[derive(Debug, Deserialize)]
struct GhRepo {
    full_name: String,
    html_url: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    stargazers_count: u64,
    #[serde(default)]
    language: Option<String>,
}
