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
use crate::text::encode_query;

fn npm_exact_name(query: &str) -> Option<&str> {
    let name = query.trim();
    let valid = !name.is_empty()
        && !name.contains(char::is_whitespace)
        && name.len() <= 214
        && name.chars().all(|ch| {
            ch.is_ascii_lowercase()
                || ch.is_ascii_digit()
                || matches!(ch, '-' | '_' | '.' | '@' | '/')
        })
        && (!name.starts_with('@') || name[1..].contains('/'));
    valid.then_some(name)
}

pub fn npm(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let direct = npm_exact_name(query).or_else(|| {
        query
            .split_whitespace()
            .next()
            .and_then(npm_exact_name)
            .filter(|name| {
                query.split_whitespace().count() <= 5
                    && !matches!(
                        *name,
                        "add"
                            | "build"
                            | "command"
                            | "create"
                            | "find"
                            | "how"
                            | "parse"
                            | "read"
                            | "remove"
                            | "search"
                            | "use"
                            | "write"
                    )
            })
    });
    if let Some(name) = direct {
        let metadata_url = format!("https://registry.npmjs.org/{}", encode_query(name));
        if let Ok(metadata) = client.get_json::<Value>(&metadata_url, APP_UA, &[]) {
            if metadata.get("name").and_then(Value::as_str) == Some(name) {
                let mut outcome = Outcome::new("npm", query);
                let package_url = format!("https://www.npmjs.com/package/{name}");
                outcome.source_url = Some(package_url.clone());
                if limit > 0 {
                    let description = metadata
                        .get("description")
                        .and_then(Value::as_str)
                        .unwrap_or_default();
                    let version = metadata
                        .get("dist-tags")
                        .and_then(|tags| tags.get("latest"))
                        .and_then(Value::as_str)
                        .unwrap_or_default();
                    outcome.hits.push(
                        Hit::new(name, package_url)
                            .snippet(description)
                            .meta_line(version),
                    );
                }
                return Ok(outcome);
            }
        }
    }
    let url = format!(
        "https://registry.npmjs.org/-/v1/search?size={limit}&text={}",
        encode_query(query)
    );
    let response: NpmSearch = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("npm", query);
    outcome.source_url = Some(format!(
        "https://www.npmjs.com/search?q={}",
        encode_query(query)
    ));
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
    let name = query.trim();
    if name.is_empty() {
        return Err("empty PyPI project name".into());
    }
    let exact_name = name.len() <= 100
        && !name
            .chars()
            .next()
            .is_some_and(|ch| matches!(ch, '-' | '_' | '.'))
        && !name.contains(char::is_whitespace)
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if exact_name {
        let project_url = format!("https://pypi.org/project/{}/", encode_query(name));
        if let Ok(html) = client.get_text(&project_url, APP_UA, &[]) {
            if let Some(title) = html
                .split_once("<title>")
                .and_then(|(_, rest)| rest.split_once("</title>").map(|(title, _)| title.trim()))
                .and_then(|title| title.strip_suffix(" · PyPI"))
                .filter(|title| normalized_project_name(title) == normalized_project_name(name))
            {
                let mut outcome = Outcome::new("pypi", query);
                outcome.source_url = Some(project_url.clone());
                if limit > 0 {
                    outcome.hits.push(Hit::new(title, project_url));
                }
                return Ok(outcome);
            }
        }
    }
    let search_url = format!("https://pypi.org/search/?q={}", encode_query(name));
    let mut outcome = Outcome::new("pypi", query);
    outcome.source_url = Some(search_url.clone());
    if limit > 0 {
        outcome.hits.push(
            Hit::new(format!("Search PyPI for {name}"), search_url).snippet(
                "Open this search in a browser; qs does not automate robots-disallowed PyPI search.",
            ),
        );
    }
    Ok(outcome)
}

fn normalized_project_name(name: &str) -> String {
    let mut normalized = String::new();
    let mut separator = false;
    for ch in name.chars().flat_map(char::to_lowercase) {
        if matches!(ch, '-' | '_' | '.') {
            if !separator {
                normalized.push('-');
                separator = true;
            }
        } else {
            normalized.push(ch);
            separator = false;
        }
    }
    normalized
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
    outcome.source_url = Some(format!(
        "https://hex.pm/packages?search={}",
        encode_query(query)
    ));
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
    // Accept both Maven's `group:artifact` spelling and the common two-word
    // spelling, but validate coordinates against Central before presenting a
    // direct artifact URL. The repository metadata endpoint is smaller and
    // much more reliable than Central's Solr search endpoint.
    let coordinates = maven_coordinates(query);
    if let Some((group, artifact)) = coordinates {
        let metadata = format!(
            "https://repo.maven.apache.org/maven2/{}/{artifact}/maven-metadata.xml",
            group.replace('.', "/")
        );
        let body = client.get_text(&metadata, APP_UA, &[])?;
        if !body.contains("<metadata") {
            return Err(format!("no Maven artifact matched {group}:{artifact}"));
        }
        let url = format!("https://central.sonatype.com/artifact/{group}/{artifact}");
        let mut outcome = Outcome::new("maven", query);
        outcome.source_url = Some(url.clone());
        outcome
            .hits
            .push(Hit::new(format!("{group}:{artifact}"), url));
        return Ok(outcome);
    }
    let lookup = query.to_string();
    let url = format!(
        "https://search.maven.org/solrsearch/select?rows={limit}&wt=json&q={}",
        encode_query(&lookup)
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

pub(crate) fn maven_coordinates(query: &str) -> Option<(&str, &str)> {
    let query = query.trim();
    let safe = |part: &str| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
    };
    if let Some(first) = query
        .split_whitespace()
        .next()
        .filter(|part| part.contains(':'))
    {
        let first = first.trim_end_matches([',', ';']);
        let (group, artifact) = first.split_once(':')?;
        if artifact.contains(':') {
            return None;
        }
        return (safe(group) && safe(artifact)).then_some((group, artifact));
    }
    let parts = query.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 || !parts[0].contains('.') {
        return None;
    }
    (safe(parts[0]) && safe(parts[1])).then_some((parts[0], parts[1]))
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
            let url = format!("https://www.nuget.org/packages/{}", package.id);
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
    let mut outcome = super::go_packages::search_any(client, query, limit)?;
    outcome.provider = "go modules".into();
    promote_verified_module(&mut outcome, query.trim(), limit);
    Ok(outcome)
}

fn promote_verified_module(outcome: &mut Outcome, module: &str, limit: usize) {
    let verified = module.contains('/')
        && outcome.hits.iter().any(|hit| {
            hit.title == module
                || hit.meta.iter().any(|line| {
                    line.split("  ·  ")
                        .any(|part| part.strip_prefix("module ") == Some(module))
                })
        });
    if !verified || limit == 0 {
        return;
    }
    let url = format!("https://pkg.go.dev/{module}");
    outcome.hits.retain(|hit| hit.url != url);
    outcome.hits.insert(
        0,
        Hit::new(module, url).snippet("Verified Go module on pkg.go.dev."),
    );
    outcome.hits.truncate(limit);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_verified_go_module_is_promoted_before_subpackages() {
        let mut outcome = Outcome::new("go modules", "golang.org/x/sync");
        outcome.hits.push(
            Hit::new(
                "golang.org/x/sync/errgroup",
                "https://pkg.go.dev/golang.org/x/sync/errgroup",
            )
            .meta_line("v0.24.0  ·  module golang.org/x/sync"),
        );
        promote_verified_module(&mut outcome, "golang.org/x/sync", 5);
        assert_eq!(outcome.hits[0].title, "golang.org/x/sync");
        assert_eq!(outcome.hits[0].url, "https://pkg.go.dev/golang.org/x/sync");
    }

    #[test]
    fn maven_coordinate_spellings_are_recognized() {
        assert_eq!(
            maven_coordinates("org.flywaydb flyway-core"),
            Some(("org.flywaydb", "flyway-core"))
        );
        assert_eq!(
            maven_coordinates("org.flywaydb:flyway-core"),
            Some(("org.flywaydb", "flyway-core"))
        );
        assert_eq!(maven_coordinates("junit:junit"), Some(("junit", "junit")));
        assert_eq!(maven_coordinates("logging library"), None);
    }

    #[test]
    fn pypi_uses_browser_search_for_non_exact_queries() {
        let outcome = pypi(&Client::new(), "orjson package", 3).unwrap();
        assert_eq!(outcome.hits[0].title, "Search PyPI for orjson package");
        assert_eq!(
            outcome.hits[0].url,
            "https://pypi.org/search/?q=orjson%20package"
        );
        assert!(outcome.source_url.unwrap().contains("/search/?q="));
        assert_eq!(
            normalized_project_name("Pydantic_Settings"),
            "pydantic-settings"
        );
    }
}
