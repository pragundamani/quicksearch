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

use serde_json::Value;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let trimmed = query.trim();
    let url = if is_cve_id(trimmed) {
        format!(
            "https://services.nvd.nist.gov/rest/json/cves/2.0?cveId={}",
            encode_query(&trimmed.to_ascii_uppercase())
        )
    } else {
        format!(
            "https://services.nvd.nist.gov/rest/json/cves/2.0?resultsPerPage={limit}&keywordSearch={}",
            encode_query(trimmed)
        )
    };
    let response: Value = client.get_json(&url, APP_UA, &[])?;
    let vulns = response
        .get("vulnerabilities")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut outcome = Outcome::new("nvd", query);
    outcome.source_url = Some(format!(
        "https://nvd.nist.gov/vuln/search/results?query={}",
        encode_query(query)
    ));
    for item in vulns.into_iter().take(limit) {
        let cve = item.get("cve").cloned().unwrap_or(Value::Null);
        let id = cve
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if id.is_empty() {
            continue;
        }
        let description = cve
            .get("descriptions")
            .and_then(Value::as_array)
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("lang").and_then(Value::as_str) == Some("en")
                })
            })
            .and_then(|item| item.get("value").and_then(Value::as_str))
            .unwrap_or("");
        let severity = cve
            .get("metrics")
            .and_then(severity_of);
        let page = format!("https://nvd.nist.gov/vuln/detail/{id}");
        let mut hit = Hit::new(id, page).snippet(truncate(&collapse_ws(description), 320));
        if let Some(severity) = severity {
            hit = hit.meta_line(severity);
        }
        outcome.hits.push(hit);
    }
    if outcome.hits.is_empty() {
        return Err(format!("no CVEs matched {query}"));
    }
    Ok(outcome)
}

fn is_cve_id(query: &str) -> bool {
    let query = query.trim();
    let Some(rest) = query.get(4..) else {
        return false;
    };
    query.len() > 8
        && query[..3].eq_ignore_ascii_case("cve")
        && query.as_bytes().get(3) == Some(&b'-')
        && rest.contains('-')
        && rest.chars().all(|ch| ch.is_ascii_digit() || ch == '-')
}

fn severity_of(metrics: &Value) -> Option<String> {
    for key in ["cvssMetricV31", "cvssMetricV30", "cvssMetricV2"] {
        let Some(items) = metrics.get(key).and_then(Value::as_array) else {
            continue;
        };
        let Some(data) = items.first().and_then(|item| item.get("cvssData")) else {
            continue;
        };
        let severity = data
            .get("baseSeverity")
            .and_then(Value::as_str)
            .map(str::to_string);
        let score = data.get("baseScore").and_then(Value::as_f64);
        return match (severity, score) {
            (Some(severity), Some(score)) => Some(format!("{severity} {score}")),
            (Some(severity), None) => Some(severity),
            (None, Some(score)) => Some(score.to_string()),
            (None, None) => None,
        };
    }
    None
}
