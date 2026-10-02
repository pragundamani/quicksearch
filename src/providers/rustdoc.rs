use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;

use regex::Regex;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape, strip_tags, truncate};

const ITEM_KINDS: &[&str] = &[
    "struct",
    "enum",
    "trait",
    "union",
    "fn",
    "macro",
    "type",
    "constant",
    "primitive",
    "keyword",
];

const FRAGMENT_KINDS: &[&str] = &[
    "method",
    "tymethod",
    "variant",
    "associatedconstant",
    "associatedtype",
    "structfield",
];

pub fn search_std(client: &Client, query: &str) -> Result<Outcome, String> {
    let mut outcome = Outcome::new("rust", query);
    if !query.contains("::") {
        let url = std_search_url(query);
        outcome.source_url = Some(url.clone());
        outcome.hits.push(
            Hit::new(format!("std search: {query}"), url)
                .snippet("Opens the Rust standard library search."),
        );
        return Ok(outcome);
    }

    let path = normalize_std_path(query);
    let search = std_search_url(&path.replace("::", " "));
    outcome.source_url = Some(search.clone());
    if let Some(page) = resolve(client, "https://doc.rust-lang.org", &path) {
        outcome.hits.push(page_hit(&page));
    }
    outcome.hits.push(
        Hit::new("Standard library search", search)
            .snippet("Every matching item in std, core, and alloc."),
    );
    Ok(outcome)
}

pub fn resolve(client: &Client, root: &str, path: &str) -> Option<Page> {
    if let Some(page) = probe_item(client, root, path) {
        return Some(page);
    }
    let parts: Vec<&str> = path.split("::").filter(|part| !part.is_empty()).collect();
    if parts.len() < 2 {
        return None;
    }
    let child = parts[parts.len() - 1];
    let parent = parts[..parts.len() - 1].join("::");
    let mut page = probe_item(client, root, &parent)?;
    if let Some(fragment) = find_fragment(&page.html, child) {
        page.url = format!("{}#{fragment}", page.url);
        page.title = format!("{}::{child}", parts[parts.len() - 2]);
    }
    page.html.clear();
    Some(page)
}

pub struct Page {
    pub url: String,
    pub title: String,
    pub snippet: String,
    html: String,
}

fn page_hit(page: &Page) -> Hit {
    Hit::new(&page.title, &page.url).snippet(&page.snippet)
}

fn probe_item(client: &Client, root: &str, path: &str) -> Option<Page> {
    let ranked = candidate_urls(root, path);
    let url = first_live(client, ranked)?;
    let html = client.get_text(&url, APP_UA, &[]).ok()?;
    let (title, snippet) = page_text(&html);
    let title = if title.is_empty() {
        path.to_string()
    } else {
        title
    };
    Some(Page {
        url,
        title,
        snippet,
        html,
    })
}

pub fn candidate_urls(root: &str, path: &str) -> Vec<(u8, String)> {
    let parts: Vec<&str> = path.split("::").filter(|part| !part.is_empty()).collect();
    if parts.is_empty() {
        return Vec::new();
    }
    let item = parts[parts.len() - 1];
    let parent = parts[..parts.len() - 1].join("/");
    let prefix = if parent.is_empty() {
        root.trim_end_matches('/').to_string()
    } else {
        format!("{}/{}", root.trim_end_matches('/'), parent)
    };
    let type_name = item.chars().next().is_some_and(|ch| ch.is_uppercase());
    let mut urls = Vec::new();
    for kind in ITEM_KINDS {
        urls.push((
            kind_rank(type_name, kind),
            format!("{prefix}/{kind}.{item}.html"),
        ));
    }
    // Prefer a real item, then the module page, then primitive pages that share a name.
    urls.push((3, format!("{prefix}/{item}/index.html")));
    urls
}

fn kind_rank(type_name: bool, kind: &str) -> u8 {
    let order: &[&str] = if type_name {
        &[
            "struct",
            "enum",
            "trait",
            "union",
            "type",
            "macro",
            "primitive",
            "fn",
            "constant",
            "keyword",
        ]
    } else {
        &[
            "fn",
            "macro",
            "constant",
            "type",
            "primitive",
            "keyword",
            "struct",
            "enum",
            "trait",
            "union",
        ]
    };
    order
        .iter()
        .position(|name| *name == kind)
        .map(|index| index as u8)
        .unwrap_or(20)
}

fn first_live(client: &Client, ranked: Vec<(u8, String)>) -> Option<String> {
    if ranked.is_empty() {
        return None;
    }
    let flags: Vec<AtomicBool> = (0..ranked.len()).map(|_| AtomicBool::new(false)).collect();
    std::thread::scope(|scope| {
        for (index, (_, url)) in ranked.iter().enumerate() {
            let agent = client.agent.clone();
            let url = url.clone();
            let flag = &flags[index];
            scope.spawn(move || {
                let ok = match agent.request("HEAD", &url).set("User-Agent", APP_UA).call() {
                    Ok(resp) => (200..300).contains(&resp.status()),
                    Err(ureq::Error::Status(code, _)) => (200..300).contains(&code),
                    Err(_) => false,
                };
                if ok {
                    flag.store(true, Ordering::Relaxed);
                }
            });
        }
    });
    ranked
        .into_iter()
        .zip(flags)
        .filter(|(_, flag)| flag.load(Ordering::Relaxed))
        .min_by_key(|((rank, _), _)| *rank)
        .map(|((_, url), _)| url)
}

fn find_fragment(html: &str, name: &str) -> Option<String> {
    for kind in FRAGMENT_KINDS {
        let id = format!("id=\"{kind}.{name}\"");
        if html.contains(&id) {
            return Some(format!("{kind}.{name}"));
        }
    }
    None
}

fn page_text(html: &str) -> (String, String) {
    static TITLE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?s)<title>(.*?)</title>").unwrap());
    static DESC_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#"(?is)<meta[^>]*name=["']description["'][^>]*content=["']([^"']*)["']|(?is)<meta[^>]*content=["']([^"']*)["'][^>]*name=["']description["']"#,
        )
        .unwrap()
    });
    let title = TITLE_RE
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| collapse_ws(&html_unescape(&strip_tags(m.as_str()))))
        .unwrap_or_default();
    let snippet = DESC_RE
        .captures(html)
        .and_then(|cap| cap.get(1).or_else(|| cap.get(2)))
        .map(|m| truncate(&collapse_ws(&html_unescape(m.as_str())), 320))
        .unwrap_or_default();
    (title, snippet)
}

pub fn normalize_std_path(query: &str) -> String {
    let path = query.trim().trim_matches(':');
    match path.split("::").next().unwrap_or("") {
        "std" | "core" | "alloc" | "proc_macro" | "test" => path.to_string(),
        _ => format!("std::{path}"),
    }
}

fn std_search_url(query: &str) -> String {
    format!(
        "https://doc.rust-lang.org/std/?search={}",
        encode_query(query)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_struct_and_module_urls() {
        let urls = candidate_urls("https://doc.rust-lang.org", "std::vec::Vec");
        assert!(urls
            .iter()
            .any(|(_, url)| url == "https://doc.rust-lang.org/std/vec/struct.Vec.html"));
        assert!(urls
            .iter()
            .any(|(_, url)| url == "https://doc.rust-lang.org/std/vec/Vec/index.html"));
    }

    #[test]
    fn module_page_uses_index() {
        let urls = candidate_urls("https://doc.rust-lang.org", "std::vec");
        assert!(urls
            .iter()
            .any(|(_, url)| url == "https://doc.rust-lang.org/std/vec/index.html"));
    }

    #[test]
    fn prefixes_std_when_the_crate_is_missing() {
        assert_eq!(normalize_std_path("vec::Vec"), "std::vec::Vec");
        assert_eq!(normalize_std_path("core::mem::drop"), "core::mem::drop");
    }

    #[test]
    fn finds_method_fragments() {
        let html = r#"<h4 id="method.push">push</h4>"#;
        assert_eq!(find_fragment(html, "push").as_deref(), Some("method.push"));
    }
}
