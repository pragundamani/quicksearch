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

use crate::http::Client;
use crate::model::{Hit, Outcome};
use crate::providers::crates::{crate_hit, load_crate, search as search_crates, CrateInfo};
use crate::providers::rustdoc::{self, normalize_std_path};
use crate::text::encode_query;

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let (name, rest) = split_crate_query(query);
    if is_std_crate(&name) {
        let path = match &rest {
            Some(rest) => format!("{name}::{rest}"),
            None => name.clone(),
        };
        return rustdoc::search_std(client, &normalize_std_path(&path));
    }

    if let Some(info) = load_crate(client, &name)? {
        return Ok(exact_docs(client, query, info, rest.as_deref()));
    }

    let mut outcome = search_crates(client, query, limit)?;
    outcome.provider = "docs".to_string();
    outcome.summary_title = Some(format!("No crate named {name}"));
    outcome.summary = Some("Closest crates on crates.io.".to_string());
    Ok(outcome)
}

fn exact_docs(client: &Client, query: &str, info: CrateInfo, rest: Option<&str>) -> Outcome {
    let mut outcome = Outcome::new("docs", query);
    let lib = info.lib_name();
    let root = format!("https://docs.rs/{}/latest/{}", info.name, lib);
    outcome.source_url = Some(root.clone());

    if let Some(rest) = rest.filter(|rest| !rest.is_empty()) {
        let rel = rest.replace(' ', "::");
        if let Some(page) = rustdoc::resolve(client, &root, &rel) {
            outcome.hits.push(
                Hit::new(page.title, page.url).snippet(if page.snippet.is_empty() {
                    info.description.clone().unwrap_or_default()
                } else {
                    page.snippet
                }),
            );
        } else {
            let search = format!("{root}/?search={}", encode_query(rest));
            outcome.hits.push(
                Hit::new(format!("{} search: {rest}", info.name), search)
                    .snippet("No exact page matched. This opens the crate search."),
            );
        }
    }

    outcome.hits.push(crate_hit(info));
    outcome
}

pub fn split_crate_query(query: &str) -> (String, Option<String>) {
    let query = query.trim();
    if let Some((name, rest)) = query.split_once("::") {
        let rest = rest.trim();
        return (
            name.trim().to_string(),
            if rest.is_empty() {
                None
            } else {
                Some(rest.to_string())
            },
        );
    }
    if let Some((name, rest)) = query.split_once(char::is_whitespace) {
        return (name.to_string(), Some(rest.trim().to_string()));
    }
    (query.to_string(), None)
}

fn is_std_crate(name: &str) -> bool {
    matches!(name, "std" | "core" | "alloc" | "proc_macro" | "test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_paths_and_words() {
        assert_eq!(
            split_crate_query("tokio::task::spawn"),
            ("tokio".into(), Some("task::spawn".into()))
        );
        assert_eq!(
            split_crate_query("serde Serialize"),
            ("serde".into(), Some("Serialize".into()))
        );
        assert_eq!(split_crate_query("serde"), ("serde".into(), None));
    }
}
