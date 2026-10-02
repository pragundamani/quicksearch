use serde::Deserialize;

use crate::http::{is_not_found, Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{compact_count, encode_query, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://crates.io/api/v1/crates?q={}&per_page={limit}",
        encode_query(query)
    );
    let response: CratesResponse = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("crates.io", query);
    outcome.source_url = Some(format!(
        "https://crates.io/search?q={}",
        encode_query(query)
    ));
    outcome.hits = response
        .crates
        .into_iter()
        .take(limit)
        .map(crate_hit)
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
    let docs = info.docs_url();
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
    Hit::new(info.name, docs)
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
    pub documentation: Option<String>,
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
        self.documentation
            .clone()
            .filter(|url| url.starts_with("http"))
            .unwrap_or_else(|| format!("https://docs.rs/{}", self.name))
    }

    pub fn lib_name(&self) -> String {
        self.name.replace('-', "_")
    }
}
