use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use crate::http::{Client, APP_UA, BROWSER_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape, strip_tags, truncate};

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let encoded = encode_query(query);
    let lite_url = format!("https://lite.duckduckgo.com/lite/?q={encoded}");
    let instant_url =
        format!("https://api.duckduckgo.com/?q={encoded}&format=json&no_html=1&skip_disambig=1");

    let instant_agent = client.agent.clone();
    let lite_agent = client.agent.clone();
    let lite_fetch_url = lite_url.clone();
    let (lite, instant) = std::thread::scope(|scope| {
        let instant_handle = scope.spawn(move || fetch_instant(&instant_agent, &instant_url));
        let lite_handle = scope.spawn(move || fetch_lite(&lite_agent, &lite_fetch_url, limit));
        (
            lite_handle.join().expect("web search thread"),
            instant_handle.join().expect("instant answer thread"),
        )
    });

    let instant = instant.ok().flatten();
    let mut outcome = match lite {
        Ok(outcome) => outcome,
        Err(err) => {
            if let Some(answer) = instant.filter(Instant::has_text) {
                return Ok(answer_outcome(query, &lite_url, answer, Vec::new()));
            }
            return Err(err);
        }
    };

    if let Some(answer) = instant.filter(Instant::has_text) {
        apply_answer(&mut outcome, answer);
    }
    outcome.query = query.to_string();
    if outcome.hits.is_empty() && outcome.summary.is_none() {
        return Err(format!("no web results for {query}: {lite_url}"));
    }
    Ok(outcome)
}

fn fetch_lite(agent: &ureq::Agent, url: &str, limit: usize) -> Result<Outcome, String> {
    let html = agent
        .get(url)
        .set("User-Agent", BROWSER_UA)
        .call()
        .map_err(|err| err.to_string())?
        .into_string()
        .map_err(|err| err.to_string())?;
    let mut outcome = Outcome::new("web", "");
    outcome.source_url = Some(url.to_string());
    outcome.hits = parse_lite_html(&html, limit);
    Ok(outcome)
}

fn fetch_instant(agent: &ureq::Agent, url: &str) -> Result<Option<Instant>, String> {
    let text = agent
        .get(url)
        .set("User-Agent", APP_UA)
        .call()
        .map_err(|err| err.to_string())?
        .into_string()
        .map_err(|err| err.to_string())?;
    serde_json::from_str(&text).map_err(|err| err.to_string())
}

fn answer_outcome(query: &str, source: &str, answer: Instant, hits: Vec<Hit>) -> Outcome {
    let mut outcome = Outcome::new("web", query);
    outcome.source_url = Some(source.to_string());
    outcome.hits = hits;
    apply_answer(&mut outcome, answer);
    outcome
}

fn apply_answer(outcome: &mut Outcome, answer: Instant) {
    let (text, url) = answer.best();
    outcome.summary_title = Some(if answer.heading.trim().is_empty() {
        "Instant answer".to_string()
    } else {
        answer.heading.trim().to_string()
    });
    outcome.summary = Some(truncate(&collapse_ws(text), 700));
    if !url.trim().is_empty() {
        outcome.summary_url = Some(url.trim().to_string());
    }
}

#[derive(Debug, Deserialize)]
struct Instant {
    #[serde(default, rename = "Heading")]
    heading: String,
    #[serde(default, rename = "AbstractText")]
    abstract_text: String,
    #[serde(default, rename = "AbstractURL")]
    abstract_url: String,
    #[serde(default, rename = "Answer")]
    answer: String,
    #[serde(default, rename = "Definition")]
    definition: String,
    #[serde(default, rename = "DefinitionURL")]
    definition_url: String,
}

impl Instant {
    fn has_text(&self) -> bool {
        self.best().0.chars().any(|ch| !ch.is_whitespace())
    }

    fn best(&self) -> (&str, &str) {
        if !self.answer.trim().is_empty() {
            return (self.answer.trim(), self.abstract_url.trim());
        }
        if !self.abstract_text.trim().is_empty() {
            return (self.abstract_text.trim(), self.abstract_url.trim());
        }
        (self.definition.trim(), self.definition_url.trim())
    }
}

pub fn parse_lite_html(html: &str, limit: usize) -> Vec<Hit> {
    static LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"(?s)<a[^>]*href=['"]([^'"]+)['"][^>]*class=['"]result-link['"][^>]*>(.*?)</a>|<a[^>]*class=['"]result-link['"][^>]*href=['"]([^'"]+)['"][^>]*>(.*?)</a>"#,
        )
        .unwrap()
    });
    static SNIPPET_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(?s)<td class=['"]result-snippet['"]>(.*?)</td>"#).unwrap());

    let mut hits = Vec::new();
    for caps in LINK_RE.captures_iter(html) {
        let href = caps
            .get(1)
            .or_else(|| caps.get(3))
            .map(|m| m.as_str())
            .unwrap_or("");
        let title_html = caps
            .get(2)
            .or_else(|| caps.get(4))
            .map(|m| m.as_str())
            .unwrap_or("");
        let title = collapse_ws(&html_unescape(&strip_tags(title_html)));
        let url = unwrap_ddg_url(href);
        if title.is_empty() || url.is_empty() {
            continue;
        }
        let end = caps.get(0).map(|m| m.end()).unwrap_or(0);
        let window = &html[end..html.len().min(end + 1500)];
        let snippet = SNIPPET_RE
            .captures(window)
            .and_then(|cap| cap.get(1))
            .map(|m| truncate(&collapse_ws(&html_unescape(&strip_tags(m.as_str()))), 320))
            .unwrap_or_default();
        hits.push(Hit::new(title, url).snippet(snippet));
        if hits.len() >= limit {
            break;
        }
    }
    hits
}

pub fn unwrap_ddg_url(raw: &str) -> String {
    let url = html_unescape(raw);
    let url = if let Some(rest) = url.strip_prefix("//") {
        format!("https://{rest}")
    } else {
        url
    };
    if let Some(query) = url.split("duckduckgo.com/l/?").nth(1) {
        for pair in query.split('&') {
            if let Some(value) = pair.strip_prefix("uddg=") {
                return urlencoding::decode(value)
                    .map(|cow| cow.into_owned())
                    .unwrap_or_else(|_| value.to_string());
            }
        }
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lite_results_and_unwraps_redirects() {
        let html = r#"
            <a rel="nofollow" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fdoc.rust%2Dlang.org%2Fstd%2Fvec%2Fstruct.Vec.html&amp;rut=abc" class='result-link'>Vec in std::vec - Rust</a>
            <td class='result-snippet'>A growable <b>array</b>.</td>
        "#;
        let hits = parse_lite_html(html, 3);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].title, "Vec in std::vec - Rust");
        assert_eq!(
            hits[0].url,
            "https://doc.rust-lang.org/std/vec/struct.Vec.html"
        );
        assert_eq!(hits[0].snippet, "A growable array.");
    }
}
