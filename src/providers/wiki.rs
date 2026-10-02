use serde::Deserialize;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape, strip_tags, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&utf8=&format=json&srlimit={limit}&srsearch={}",
        encode_query(query)
    );
    let response: WikiSearch = client.get_json(&url, APP_UA, &[])?;
    let hits = response.query.search;
    if hits.is_empty() {
        return Err(format!("no wikipedia pages matched {query}"));
    }

    let mut outcome = Outcome::new("wikipedia", query);
    outcome.source_url = Some(format!(
        "https://en.wikipedia.org/w/index.php?search={}",
        encode_query(query)
    ));

    if let Some(first) = hits.first() {
        if let Some(summary) = fetch_summary(client, &first.title) {
            outcome.summary_title = Some(summary.title);
            outcome.summary = Some(truncate(&collapse_ws(&summary.extract), 700));
            outcome.summary_url = Some(wiki_url(&first.title));
        }
    }

    outcome.hits = hits
        .into_iter()
        .map(|hit| {
            let snippet = truncate(&collapse_ws(&html_unescape(&strip_tags(&hit.snippet))), 280);
            Hit::new(hit.title.clone(), wiki_url(&hit.title)).snippet(snippet)
        })
        .collect();
    Ok(outcome)
}

fn fetch_summary(client: &Client, title: &str) -> Option<Summary> {
    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        encode_query(&title.replace(' ', "_"))
    );
    client.get_json(&url, APP_UA, &[]).ok()
}

fn wiki_url(title: &str) -> String {
    format!(
        "https://en.wikipedia.org/wiki/{}",
        encode_query(&title.replace(' ', "_"))
    )
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

#[derive(Debug, Deserialize)]
struct Summary {
    title: String,
    #[serde(default)]
    extract: String,
}
