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
use crate::text::{encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://developer.mozilla.org/api/v1/search?locale=en-US&q={}",
        encode_query(query)
    );
    let response: MdnResponse = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("mdn", query);
    outcome.source_url = Some(format!(
        "https://developer.mozilla.org/en-US/search?q={}",
        encode_query(query)
    ));
    outcome.hits = response
        .documents
        .into_iter()
        .take(limit)
        .map(|doc| {
            let url = if doc.mdn_url.starts_with("http") {
                doc.mdn_url
            } else {
                format!("https://developer.mozilla.org{}", doc.mdn_url)
            };
            Hit::new(doc.title, url).snippet(truncate(&doc.summary, 320))
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no MDN documents matched {query}"));
    }
    Ok(outcome)
}

#[derive(Debug, Deserialize)]
struct MdnResponse {
    #[serde(default)]
    documents: Vec<MdnDoc>,
}

#[derive(Debug, Deserialize)]
struct MdnDoc {
    mdn_url: String,
    title: String,
    #[serde(default)]
    summary: String,
}
