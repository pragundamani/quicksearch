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

use crate::http::{Client, APP_UA, BROWSER_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query};

pub fn npm(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://registry.npmjs.org/-/v1/search?size={limit}&text={}",
        encode_query(query)
    );
    let response: NpmSearch = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("npm", query);
    outcome.source_url = Some(format!("https://www.npmjs.com/search?q={}", encode_query(query)));
    outcome.hits = response
        .objects
        .into_iter()
        .filter_map(|item| {
            let package = item.package?;
            let url = package
                .links
                .and_then(|links| links.npm)
                .unwrap_or_else(|| format!("https://www.npmjs.com/package/{}", package.name));
            Some(
                Hit::new(package.name, url)
                    .snippet(package.description.unwrap_or_default())
                    .meta_line(package.version.unwrap_or_default()),
            )
        })
        .take(limit)
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no npm packages matched {query}"));
    }
    Ok(outcome)
}

pub fn pypi(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!("https://pypi.org/search/?q={}", encode_query(query));
    let html = client.get_text(&url, BROWSER_UA, &[])?;
    static BLOCK: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"(?s)package-snippet__name">([^<]+)</span>.*?package-snippet__version">([^<]*)</span>.*?package-snippet__description">([^<]*)</p>"#,
        )
        .unwrap()
    });
    let mut outcome = Outcome::new("pypi", query);
    outcome.source_url = Some(url);
    for caps in BLOCK.captures_iter(&html).take(limit) {
        let name = collapse_ws(&caps[1]);
        let version = collapse_ws(&caps[2]);
        let description = collapse_ws(&caps[3]);
        if name.is_empty() {
            continue;
        }
        outcome.hits.push(
            Hit::new(
                name.clone(),
                format!("https://pypi.org/project/{}/", encode_query(&name)),
            )
            .snippet(description)
            .meta_line(version),
        );
    }
    if outcome.hits.is_empty() {
        return Err(format!("no PyPI packages matched {query}"));
    }
    Ok(outcome)
}

pub fn gems(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://rubygems.org/api/v1/search.json?query={}",
        encode_query(query)
    );
    let gems: Vec<Gem> = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("rubygems", query);
    outcome.source_url = Some(format!(
        "https://rubygems.org/search?query={}",
        encode_query(query)
    ));
    outcome.hits = gems
        .into_iter()
        .take(limit)
        .map(|gem| {
            let url = gem
                .project_uri
                .unwrap_or_else(|| format!("https://rubygems.org/gems/{}", gem.name));
            Hit::new(gem.name, url)
                .snippet(gem.info.unwrap_or_default())
                .meta_line(gem.version.unwrap_or_default())
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no RubyGems matched {query}"));
    }
    Ok(outcome)
}

pub fn hex(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://hex.pm/api/packages?sort=downloads&search={}",
        encode_query(query)
    );
    let packages: Vec<HexPackage> = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("hex", query);
    outcome.source_url = Some(format!("https://hex.pm/packages?search={}", encode_query(query)));
    outcome.hits = packages
        .into_iter()
        .take(limit)
        .map(|package| {
            let description = package
                .meta
                .and_then(|meta| meta.description)
                .unwrap_or_default();
            Hit::new(
                package.name.clone(),
                format!("https://hex.pm/packages/{}", package.name),
            )
            .snippet(description)
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no Hex packages matched {query}"));
    }
    Ok(outcome)
}

pub fn maven(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://search.maven.org/solrsearch/select?rows={limit}&wt=json&q={}",
        encode_query(query)
    );
    let response: MavenSearch = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("maven", query);
    outcome.source_url = Some(format!(
        "https://search.maven.org/search?q={}",
        encode_query(query)
    ));
    outcome.hits = response
        .response
        .docs
        .into_iter()
        .take(limit)
        .map(|doc| {
            let title = format!("{}:{}", doc.group, doc.artifact);
            Hit::new(
                title,
                format!(
                    "https://central.sonatype.com/artifact/{}/{}",
                    doc.group, doc.artifact
                ),
            )
            .meta_line(doc.latest_version.unwrap_or_default())
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no Maven artifacts matched {query}"));
    }
    Ok(outcome)
}

pub fn nuget(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://azuresearch-usnc.nuget.org/query?take={limit}&q={}",
        encode_query(query)
    );
    let response: NugetSearch = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("nuget", query);
    outcome.source_url = Some(format!(
        "https://www.nuget.org/packages?q={}",
        encode_query(query)
    ));
    outcome.hits = response
        .data
        .into_iter()
        .take(limit)
        .map(|package| {
            let url = package.project_url.filter(|url| url.starts_with("http")).unwrap_or_else(|| {
                format!("https://www.nuget.org/packages/{}", package.id)
            });
            Hit::new(package.id, url)
                .snippet(package.description.unwrap_or_default())
                .meta_line(package.version.unwrap_or_default())
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no NuGet packages matched {query}"));
    }
    Ok(outcome)
}

pub fn gomod(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!("https://pkg.go.dev/search?q={}", encode_query(query));
    let html = client.get_text(&url, BROWSER_UA, &[])?;
    static LINK: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"href="/(?P<path>[a-z0-9][^"]*\.[^"]+)""#).unwrap()
    });
    let mut outcome = Outcome::new("go modules", query);
    outcome.source_url = Some(url);
    let mut seen = std::collections::HashSet::new();
    for caps in LINK.captures_iter(&html) {
        let path = caps.name("path").map(|item| item.as_str()).unwrap_or("");
        let path = path.split('?').next().unwrap_or(path).trim_end_matches('/');
        if path.is_empty() || path.starts_with("search") || !seen.insert(path.to_string()) {
            continue;
        }
        outcome
            .hits
            .push(Hit::new(path, format!("https://pkg.go.dev/{path}")));
        if outcome.hits.len() == limit {
            break;
        }
    }
    if outcome.hits.is_empty() {
        return Err(format!("no Go modules matched {query}"));
    }
    Ok(outcome)
}

#[derive(Deserialize)]
struct NpmSearch {
    #[serde(default)]
    objects: Vec<NpmObject>,
}

#[derive(Deserialize)]
struct NpmObject {
    package: Option<NpmPackage>,
}

#[derive(Deserialize)]
struct NpmPackage {
    name: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    links: Option<NpmLinks>,
}

#[derive(Deserialize)]
struct NpmLinks {
    npm: Option<String>,
}

#[derive(Deserialize)]
struct Gem {
    name: String,
    #[serde(default)]
    info: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(rename = "project_uri", default)]
    project_uri: Option<String>,
}

#[derive(Deserialize)]
struct HexPackage {
    name: String,
    #[serde(default)]
    meta: Option<HexMeta>,
}

#[derive(Deserialize)]
struct HexMeta {
    #[serde(default)]
    description: Option<String>,
}

#[derive(Deserialize)]
struct MavenSearch {
    response: MavenResponse,
}

#[derive(Deserialize)]
struct MavenResponse {
    #[serde(default)]
    docs: Vec<MavenDoc>,
}

#[derive(Deserialize)]
struct MavenDoc {
    #[serde(rename = "g")]
    group: String,
    #[serde(rename = "a")]
    artifact: String,
    #[serde(rename = "latestVersion", default)]
    latest_version: Option<String>,
}

#[derive(Deserialize)]
struct NugetSearch {
    #[serde(default)]
    data: Vec<NugetPackage>,
}

#[derive(Deserialize)]
struct NugetPackage {
    id: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "projectUrl", default)]
    project_url: Option<String>,
}
