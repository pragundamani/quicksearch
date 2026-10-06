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
use serde_json::Value;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    if let Some(number) = rfc_number(query) {
        return one_rfc(client, number);
    }
    let url = format!(
        "https://datatracker.ietf.org/api/v1/doc/document/?format=json&limit={limit}&title__icontains={}",
        encode_query(query)
    );
    let response: Value = client.get_json(&url, APP_UA, &[])?;
    let objects = response
        .get("objects")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut outcome = Outcome::new("rfc", query);
    outcome.source_url = Some(format!(
        "https://datatracker.ietf.org/doc/search/?name={}",
        encode_query(query)
    ));
    for object in objects.into_iter().take(limit) {
        let title = object
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let name = object
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if title.is_empty() && name.is_empty() {
            continue;
        }
        let number = object
            .get("rfc")
            .and_then(|value| value.as_str().map(str::to_string).or_else(|| value.as_i64().map(|n| n.to_string())))
            .or_else(|| name.strip_prefix("rfc").map(str::to_string));
        let url = number
            .as_ref()
            .map(|number| format!("https://www.rfc-editor.org/rfc/rfc{number}"))
            .unwrap_or_else(|| format!("https://datatracker.ietf.org/doc/{name}"));
        let label = number
            .map(|number| format!("RFC {number}"))
            .unwrap_or(name);
        outcome.hits.push(Hit::new(if title.is_empty() { label.clone() } else { title }, url).meta_line(label));
    }
    if outcome.hits.is_empty() {
        return Err(format!("no RFCs matched {query}"));
    }
    Ok(outcome)
}

fn one_rfc(client: &Client, number: &str) -> Result<Outcome, String> {
    let url = format!("https://www.rfc-editor.org/rfc/rfc{number}.json");
    let doc: RfcDoc = client.get_json(&url, APP_UA, &[])?;
    let title = doc.title.unwrap_or_else(|| format!("RFC {number}"));
    let page = format!("https://www.rfc-editor.org/rfc/rfc{number}");
    let mut outcome = Outcome::new("rfc", &format!("RFC {number}"));
    outcome.source_url = Some(page.clone());
    if let Some(abstract_text) = doc.abstract_text.filter(|text| !text.trim().is_empty()) {
        outcome.summary = Some(truncate(&collapse_ws(&abstract_text), 700));
        outcome.summary_title = Some(title.clone());
        outcome.summary_url = Some(page.clone());
    }
    outcome.hits.push(Hit::new(title, page).meta_line(format!("RFC {number}")));
    Ok(outcome)
}

fn rfc_number(query: &str) -> Option<&str> {
    let query = query.trim();
    let digits = query
        .strip_prefix("rfc")
        .or_else(|| query.strip_prefix("RFC"))
        .unwrap_or(query)
        .trim();
    if !digits.is_empty() && digits.chars().all(|ch| ch.is_ascii_digit()) {
        Some(digits)
    } else {
        None
    }
}

#[derive(Deserialize)]
struct RfcDoc {
    #[serde(default)]
    title: Option<String>,
    #[serde(default, rename = "abstract")]
    abstract_text: Option<String>,
}
