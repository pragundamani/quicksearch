use serde::Deserialize;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{encode_query, html_unescape};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://api.stackexchange.com/2.3/search/advanced?order=desc&sort=relevance&site=stackoverflow&pagesize={limit}&q={}",
        encode_query(query)
    );
    let response: SoResponse = client.get_json_gzip(
        &url,
        APP_UA,
        &[("Accept-Encoding", "gzip"), ("Accept", "application/json")],
    )?;
    if let Some(message) = response.error_message.filter(|message| !message.is_empty()) {
        if response.items.is_empty() {
            return Err(format!("stackoverflow: {message}"));
        }
    }
    let mut outcome = Outcome::new("stackoverflow", query);
    outcome.source_url = Some(format!(
        "https://stackoverflow.com/search?q={}",
        encode_query(query)
    ));
    outcome.hits = response
        .items
        .into_iter()
        .take(limit)
        .map(|item| {
            let answered = if item.accepted_answer_id.is_some() {
                "accepted answer"
            } else if item.is_answered {
                "answered"
            } else {
                "unanswered"
            };
            let tags = item
                .tags
                .iter()
                .take(5)
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ");
            Hit::new(html_unescape(&item.title), item.link).meta_line(format!(
                "score {}  ·  {} answers  ·  {answered}  ·  {tags}",
                item.score, item.answer_count
            ))
        })
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no stack overflow questions matched {query}"));
    }
    Ok(outcome)
}

#[derive(Debug, Deserialize)]
struct SoResponse {
    #[serde(default)]
    items: Vec<SoItem>,
    #[serde(default)]
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SoItem {
    title: String,
    link: String,
    #[serde(default)]
    score: i64,
    #[serde(default)]
    answer_count: i64,
    #[serde(default)]
    is_answered: bool,
    #[serde(default)]
    accepted_answer_id: Option<u64>,
    #[serde(default)]
    tags: Vec<String>,
}
