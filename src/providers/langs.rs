use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::Duration;

use regex::Regex;
use serde::Deserialize;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::text::{collapse_ws, encode_query, html_unescape, strip_tags, truncate};

const CATALOG_URL: &str = "https://devdocs.io/docs.json";
const CACHE_TTL: Duration = Duration::from_secs(14 * 24 * 60 * 60);

pub fn try_search(client: &Client, query: &str, limit: usize) -> Result<Option<Outcome>, String> {
    let Some((language, topic)) = split_language(query) else {
        return Ok(None);
    };
    if is_haskell(&language) {
        return Ok(Some(search_hoogle(client, &topic, limit)?));
    }
    if let Some(concept) = super::concepts::find(&language) {
        return Ok(Some(super::concepts::search(
            client, concept, &topic, limit,
        )?));
    }

    let docset = match resolve_docset(client, &language) {
        Ok(Some(docset)) => docset,
        Ok(None) => return Ok(None),
        Err(_) => match fallback_docset(&language) {
            Some(docset) => docset,
            None => return Ok(None),
        },
    };
    Ok(Some(search_docset(client, &docset, &topic, limit)?))
}

fn split_language(query: &str) -> Option<(String, String)> {
    let (language, topic) = query.split_once(char::is_whitespace)?;
    let topic = topic.trim();
    if language.is_empty() || topic.is_empty() {
        return None;
    }
    Some((alias_word(language), topic.to_string()))
}

fn alias_word(word: &str) -> String {
    match word.to_ascii_lowercase().as_str() {
        "golang" => "go".to_string(),
        "nodejs" | "node.js" => "node".to_string(),
        "ml" => "ocaml".to_string(),
        "hs" => "haskell".to_string(),
        "cplusplus" => "cpp".to_string(),
        other => other.to_string(),
    }
}

fn is_haskell(language: &str) -> bool {
    language.eq_ignore_ascii_case("haskell") || language.eq_ignore_ascii_case("hs")
}

fn search_hoogle(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://hoogle.haskell.org/?mode=json&start=1&count={limit}&hoogle={}",
        encode_query(query)
    );
    let items: Vec<HoogleItem> = client.get_json(&url, APP_UA, &[])?;
    let mut outcome = Outcome::new("Haskell", query);
    outcome.source_url = Some(format!(
        "https://hoogle.haskell.org/?hoogle={}",
        encode_query(query)
    ));
    outcome.hits = items
        .into_iter()
        .take(limit)
        .map(|item| {
            let title = truncate(&plain_html(&item.item), 220);
            let package = item.package.as_ref().map(|pkg| pkg.name.as_str());
            let module = item.module.as_ref().map(|module| module.name.as_str());
            let meta = match (package, module) {
                (Some(package), Some(module)) => format!("{package}  ·  {module}"),
                (Some(package), None) => package.to_string(),
                (None, Some(module)) => module.to_string(),
                (None, None) => String::new(),
            };
            Hit::new(title, item.url)
                .snippet(truncate(&plain_html(&item.docs), 320))
                .meta_line(meta)
        })
        .filter(|hit| !hit.title.is_empty())
        .collect();
    if outcome.hits.is_empty() {
        return Err(format!("no haskell results for {query}"));
    }
    Ok(outcome)
}

fn search_docset(
    client: &Client,
    docset: &Docset,
    query: &str,
    limit: usize,
) -> Result<Outcome, String> {
    let ranked = ranked_docset_hits(client, docset, query, limit, true, false)?;
    if ranked.is_empty() {
        return Err(format!("no {} docs matched {query}", docset.name));
    }
    let mut outcome = Outcome::new(&docset_label(docset), query);
    outcome.source_url = Some(format!("https://devdocs.io/{}", docset.slug));
    outcome.hits = ranked.into_iter().map(|(_, hit)| hit).collect();
    Ok(outcome)
}

pub(crate) fn family_hits(
    client: &Client,
    family: &str,
    query: &str,
    limit: usize,
    fetch: bool,
) -> Vec<(i32, Hit)> {
    let docset = if fetch {
        match resolve_docset(client, family) {
            Ok(Some(docset)) => docset,
            _ => match fallback_docset(family) {
                Some(docset) => docset,
                None => return Vec::new(),
            },
        }
    } else {
        let Some(catalog) = load_catalog_local() else {
            return Vec::new();
        };
        let Some(docset) = pick_docset(&catalog, family).cloned().or_else(|| fallback_docset(family)) else {
            return Vec::new();
        };
        let index = cache_dir().join(format!("{}.index.json", docset.slug));
        if read_fresh(&index).is_none() {
            return Vec::new();
        }
        docset
    };
    ranked_docset_hits(client, &docset, query, limit, false, true).unwrap_or_default()
}

fn ranked_docset_hits(
    client: &Client,
    docset: &Docset,
    query: &str,
    limit: usize,
    snippet_first: bool,
    strong_only: bool,
) -> Result<Vec<(i32, Hit)>, String> {
    let index = load_index(client, docset)?;
    let mut ranked: Vec<(i32, &Entry)> = index
        .entries
        .iter()
        .filter_map(|entry| score(query, &entry.name).map(|score| (score, entry)))
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.len().cmp(&b.1.name.len())));
    if strong_only || ranked.first().is_some_and(|(score, _)| *score >= 500) {
        ranked.retain(|(score, _)| *score >= 500);
    }
    let mut seen = std::collections::HashSet::new();
    ranked.retain(|(_, entry)| seen.insert(entry.path.clone()));
    ranked.truncate(limit);
    let mut hits = Vec::new();
    for (index, (rank, entry)) in ranked.iter().enumerate() {
        let url = format!("https://devdocs.io/{}/{}", docset.slug, entry.path);
        let mut hit = Hit::new(&entry.name, url).meta_line(format!(
            "{}  ·  {}",
            docset.name, entry.kind
        ));
        if snippet_first && index < 2 {
            if let Some(notes) = page_notes(client, docset, &entry.path) {
                if let Some(snippet) = notes.snippet {
                    hit = hit.snippet(snippet);
                }
                if let Some(usage) = notes.usage {
                    hit = hit.meta_line(usage);
                }
            }
        }
        hits.push((*rank, hit));
    }
    Ok(hits)
}

fn resolve_docset(client: &Client, language: &str) -> Result<Option<Docset>, String> {
    let catalog = load_catalog(client)?;
    Ok(pick_docset(&catalog, language).cloned())
}

fn pick_docset<'a>(catalog: &'a [Docset], language: &str) -> Option<&'a Docset> {
    let language = language.to_ascii_lowercase();
    if let Some(docset) = catalog
        .iter()
        .find(|docset| docset.slug.eq_ignore_ascii_case(&language) && !docset.slug.contains('~'))
    {
        return Some(docset);
    }
    if let Some(docset) = catalog
        .iter()
        .find(|docset| docset.slug.eq_ignore_ascii_case(&language))
    {
        return Some(docset);
    }

    let aliased: Vec<_> = catalog
        .iter()
        .filter(|docset| {
            docset
                .alias
                .as_ref()
                .is_some_and(|alias| alias.eq_ignore_ascii_case(&language))
        })
        .collect();
    if !aliased.is_empty() {
        return Some(latest(aliased));
    }

    let prefixed: Vec<_> = catalog
        .iter()
        .filter(|docset| {
            docset
                .slug
                .split('~')
                .next()
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(&language))
        })
        .collect();
    if !prefixed.is_empty() {
        if let Some(docset) = prefixed.iter().find(|docset| !docset.slug.contains('~')) {
            return Some(docset);
        }
        return Some(latest(prefixed));
    }

    let named: Vec<_> = catalog
        .iter()
        .filter(|docset| docset.name.eq_ignore_ascii_case(&language))
        .collect();
    if !named.is_empty() {
        if let Some(docset) = named.iter().find(|docset| !docset.slug.contains('~')) {
            return Some(docset);
        }
        return Some(latest(named));
    }
    None
}

fn latest(docsets: Vec<&Docset>) -> &Docset {
    docsets
        .into_iter()
        .max_by(|left, right| {
            version_key(&left.slug)
                .cmp(&version_key(&right.slug))
                .then(right.slug.len().cmp(&left.slug.len()))
        })
        .expect("docset list is non-empty")
}

fn version_key(slug: &str) -> Vec<u32> {
    let Some(version) = slug.split_once('~').map(|(_, version)| version) else {
        return vec![u32::MAX];
    };
    let numeric: String = version
        .chars()
        .take_while(|ch| ch.is_ascii_digit() || *ch == '.')
        .collect();
    if numeric.is_empty() {
        return vec![0];
    }
    numeric
        .split('.')
        .map(|part| part.parse().unwrap_or(0))
        .collect()
}

fn docset_label(docset: &Docset) -> String {
    match docset
        .release
        .as_deref()
        .filter(|release| !release.is_empty())
    {
        Some(release) => format!("{} {release}", docset.name),
        None => docset.name.clone(),
    }
}

fn fallback_docset(language: &str) -> Option<Docset> {
    let (slug, name) = match language.to_ascii_lowercase().as_str() {
        "c" => ("c", "C"),
        "cpp" | "c++" => ("cpp", "C++"),
        "python" | "py" => ("python~3.14", "Python"),
        "ocaml" | "ml" => ("ocaml", "OCaml"),
        "go" | "golang" => ("go", "Go"),
        "java" => ("openjdk~25", "OpenJDK"),
        "ruby" | "rb" => ("ruby~4.0", "Ruby"),
        "js" | "javascript" => ("javascript", "JavaScript"),
        "ts" | "typescript" => ("typescript", "TypeScript"),
        "php" => ("php", "PHP"),
        "zig" => ("zig", "Zig"),
        "kotlin" | "kt" => ("kotlin~2", "Kotlin"),
        "lua" => ("lua~5.5", "Lua"),
        "elixir" | "ex" => ("elixir~1.20", "Elixir"),
        "perl" => ("perl~5.44", "Perl"),
        "scala" => ("scala~3", "Scala"),
        "clojure" | "clj" => ("clojure~1.11", "Clojure"),
        "erlang" | "erl" => ("erlang~26", "Erlang"),
        "julia" | "jl" => ("julia~1.13", "Julia"),
        "nim" => ("nim", "Nim"),
        "bash" | "sh" => ("bash", "Bash"),
        "zsh" => ("zsh", "Zsh"),
        "css" => ("css", "CSS"),
        "html" => ("html", "HTML"),
        "node" | "nodejs" => ("node", "Node.js"),
        "dart" => ("dart~3", "Dart"),
        "r" => ("r", "R"),
        _ => return None,
    };
    Some(Docset {
        name: name.to_string(),
        slug: slug.to_string(),
        alias: None,
        release: None,
    })
}

fn load_catalog_local() -> Option<Vec<Docset>> {
    let text = read_fresh(&cache_dir().join("docs.json"))?;
    serde_json::from_str(&text).ok()
}

fn load_catalog(client: &Client) -> Result<Vec<Docset>, String> {
    let text = cached_text(
        client,
        CATALOG_URL,
        &cache_dir().join("docs.json"),
        "documentation catalog",
    )?;
    serde_json::from_str(&text).map_err(|err| format!("devdocs catalog: {err}"))
}

fn load_index(client: &Client, docset: &Docset) -> Result<Index, String> {
    let url = format!("https://documents.devdocs.io/{}/index.json", docset.slug);
    let path = cache_dir().join(format!("{}.index.json", docset.slug));
    let text = cached_text(client, &url, &path, &format!("{} docs", docset.name))?;
    serde_json::from_str(&text).map_err(|err| format!("{} docs index: {err}", docset.name))
}

fn cached_text(
    client: &Client,
    url: &str,
    path: &std::path::Path,
    label: &str,
) -> Result<String, String> {
    if let Some(text) = read_fresh(path) {
        return Ok(text);
    }
    eprintln!("qs: fetching {label}...");
    let text = client.get_text(url, APP_UA, &[])?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let tmp = path.with_extension("tmp");
    if std::fs::write(&tmp, &text).is_ok() {
        let _ = std::fs::rename(&tmp, path);
    }
    Ok(text)
}

fn read_fresh(path: &std::path::Path) -> Option<String> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    if modified.elapsed().ok()? > CACHE_TTL {
        return None;
    }
    let text = std::fs::read_to_string(path).ok()?;
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

pub(crate) fn cache_dir() -> PathBuf {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .unwrap_or_else(std::env::temp_dir);
    base.join("quicksearch")
}

struct PageNotes {
    snippet: Option<String>,
    usage: Option<String>,
}

fn page_notes(client: &Client, docset: &Docset, path: &str) -> Option<PageNotes> {
    let file = path.split('#').next().unwrap_or(path);
    let url = format!("https://documents.devdocs.io/{}/{file}.html", docset.slug);
    let html = client.get_text(&url, APP_UA, &[]).ok()?;
    let snippet = explain_sentence(&html)
        .or_else(|| first_paragraph(&html))
        .map(|text| truncate(&text, 320))
        .filter(|text| !text.is_empty());
    let usage = usage_block(&html);
    if snippet.is_none() && usage.is_none() {
        None
    } else {
        Some(PageNotes { snippet, usage })
    }
}

fn first_paragraph(html: &str) -> Option<String> {
    static PARAGRAPH: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<p[^>]*>(.*?)</p>").unwrap());
    PARAGRAPH.captures_iter(html).find_map(|caps| {
        let text = plain_html(caps.get(1)?.as_str());
        if text.chars().count() < 40 || looks_like_code(&text) {
            return None;
        }
        Some(text)
    })
}

fn explain_sentence(html: &str) -> Option<String> {
    let text = collapse_ws(&break_tags(html));
    let marker = text.find(" is a ")?;
    let begin = text[..marker]
        .rfind(['.', ')', ':', ';'])
        .map(|index| index + 1)
        .unwrap_or(0);
    let end = text[marker..].find('.')? + marker + 1;
    let mut sentence = text[begin..end].trim().to_string();
    if let Some((lead, rest)) = sentence.split_once(") ") {
        if lead.chars().all(|ch| ch.is_ascii_digit() || ch == '(') {
            sentence = rest.to_string();
        }
    }
    if sentence.chars().count() < 40 {
        None
    } else {
        Some(sentence)
    }
}

fn usage_block(html: &str) -> Option<String> {
    let pres = pre_blocks(html);
    if let Some(example) = pres.iter().find(|pre| pre.contains("#include")) {
        return Some(example.clone());
    }
    pres.into_iter()
        .filter(|pre| {
            let text = pre.trim_start();
            text.contains("class ") || text.contains("namespace ") || text.starts_with("template")
        })
        .max_by_key(|pre| pre.len())
}

fn pre_blocks(html: &str) -> Vec<String> {
    static PRE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<pre\b[^>]*>(.*?)</pre>").unwrap());
    PRE.captures_iter(html)
        .map(|caps| pre_text(&caps[1]))
        .filter(|text| text.lines().filter(|line| !line.trim().is_empty()).count() >= 2)
        .collect()
}

fn pre_text(inner: &str) -> String {
    static BR: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<br\s*/?>").unwrap());
    static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").unwrap());
    let with_breaks = BR.replace_all(inner, "\n");
    let plain = html_unescape(&TAG.replace_all(&with_breaks, ""));
    let lines: Vec<&str> = plain.lines().collect();
    let Some(start) = lines.iter().position(|line| !line.trim().is_empty()) else {
        return String::new();
    };
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .unwrap_or(start);
    lines[start..=end].join("\n")
}

fn break_tags(html: &str) -> String {
    static TAG: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?is)</?(?:p|pre|code|div|span|a|h[1-6]|br|hr|table|tr|td|th|ul|ol|li|em|strong|b|i|tt|dl|dt|dd|section|script|style|sup|sub|blockquote|html|body|head|meta|link|img)\b[^>]*>",
        )
        .unwrap()
    });
    TAG.replace_all(&html_unescape(html), "\n").into_owned()
}

fn looks_like_code(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.starts_with("added in version")
        || text.matches(");").count() >= 1 && text.matches('(').count() >= 2
}

fn plain_html(text: &str) -> String {
    collapse_ws(&html_unescape(&strip_tags(text)))
}

pub fn score(query: &str, name: &str) -> Option<i32> {
    let query = canon(query);
    let name = canon(name);
    if query.is_empty() || name.is_empty() {
        return None;
    }
    if name == query {
        return Some(10_000);
    }
    let query_tokens: Vec<&str> = query.split_whitespace().collect();
    let name_tokens: Vec<&str> = name.split_whitespace().collect();
    let last = query_tokens.last().copied().unwrap_or("");
    let suffix_type = last.len() > 4
        && name_tokens.len() <= 2
        && name_tokens
            .last()
            .is_some_and(|token| *token != last && token.ends_with(last));
    let mut score = if name_tokens.iter().any(|token| *token == last) {
        500
    } else if suffix_type {
        800
    } else if last.len() > 2 && name_tokens.iter().any(|token| token.contains(last)) {
        120
    } else {
        return None;
    };
    let mut matched = 0;
    for token in &query_tokens {
        if name_tokens.iter().any(|name_token| name_token == token) {
            matched += 1;
            score += 200;
        }
    }
    if matched == query_tokens.len() {
        score += 400;
    }
    score -= name.len() as i32 / 8;
    Some(score)
}

fn canon(text: &str) -> String {
    let mut out = String::new();
    let mut break_word = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            if break_word && !out.is_empty() {
                out.push(' ');
            }
            break_word = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            break_word = true;
        }
    }
    out
}

#[derive(Clone, Debug, Deserialize)]
struct Docset {
    name: String,
    slug: String,
    #[serde(default)]
    alias: Option<String>,
    #[serde(default)]
    release: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Index {
    #[serde(default)]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    path: String,
    #[serde(default, rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
struct HoogleItem {
    #[serde(default)]
    docs: String,
    #[serde(default)]
    item: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    module: Option<HoogleName>,
    #[serde(default)]
    package: Option<HoogleName>,
}

#[derive(Debug, Deserialize)]
struct HoogleName {
    name: String,
}

#[derive(Clone, Debug)]
struct Shorthand {
    words: Vec<String>,
    label: String,
    url: String,
}

struct Listed {
    groups: Vec<Shorthand>,
    hidden: usize,
}

const STARTERS: &[&str] = &[
    "c", "cpp", "python", "haskell", "ocaml", "rust", "go", "java", "ruby", "js", "ts", "php",
    "zig", "kotlin", "lua", "elixir", "perl", "scala", "bash",
];

pub fn list(client: &Client, pattern: &str, limit: usize) -> Result<Outcome, String> {
    let catalog = load_catalog(client).unwrap_or_default();
    let groups = shorthand_groups(&catalog);
    let listed = rank_shorthands(&groups, pattern, limit);
    if listed.groups.is_empty() {
        return Err(format!("no language shorthand matches {pattern}"));
    }
    let shown = if pattern.trim().is_empty() {
        "languages".to_string()
    } else {
        pattern.to_string()
    };
    let mut outcome = Outcome::new("lang", &shown);
    if listed.hidden > 0 {
        outcome.summary = Some(format!(
            "{} more. pass -n to show them.",
            listed.hidden
        ));
    } else if pattern.trim().is_empty() {
        outcome.summary = Some("Fuzzy-find a shorthand with qs lang <pattern>.".to_string());
    }
    for group in listed.groups {
        let primary = group.words.first().cloned().unwrap_or_default();
        outcome.hits.push(
            Hit::new(group.words.join(", "), group.url)
                .snippet(group.label)
                .meta_line(format!("qs {primary} <topic>")),
        );
    }
    Ok(outcome)
}

fn shorthand_groups(catalog: &[Docset]) -> Vec<Shorthand> {
    let mut groups = vec![
        Shorthand {
            words: vec!["rs".into(), "rust".into(), "std".into()],
            label: "Rust standard library".into(),
            url: "https://doc.rust-lang.org/std/".into(),
        },
        Shorthand {
            words: vec!["hs".into(), "haskell".into()],
            label: "Haskell".into(),
            url: "https://hoogle.haskell.org/".into(),
        },
    ];
    let mut used: std::collections::HashSet<String> = groups
        .iter()
        .flat_map(|group| group.words.iter().cloned())
        .collect();

    let mut words = Vec::new();
    for docset in catalog {
        if let Some(prefix) = docset.slug.split('~').next() {
            words.push(prefix.to_ascii_lowercase());
        }
        if let Some(alias) = &docset.alias {
            words.push(alias.to_ascii_lowercase());
        }
        let name = docset.name.to_ascii_lowercase();
        if is_typeable(&name) {
            words.push(name);
        }
    }
    for extra in [
        "ml", "golang", "nodejs", "node.js", "cplusplus", "py", "c++", "rb", "js", "ts", "kt",
        "ex", "jl", "clj", "erl", "sh", "java", "go", "php", "zig", "lua", "ruby", "scala",
        "perl", "bash", "html", "css", "r",
    ] {
        words.push(extra.to_string());
    }
    words.sort();
    words.dedup();

    let mut by_slug: std::collections::HashMap<String, Shorthand> = std::collections::HashMap::new();
    for word in words {
        if !is_typeable(&word) || used.contains(&word) || is_command_word(&word) {
            continue;
        }
        let lookup = alias_word(&word);
        let docset = pick_docset(catalog, &lookup)
            .cloned()
            .or_else(|| fallback_docset(&lookup));
        let Some(docset) = docset else {
            continue;
        };
        let family = docset.slug.split('~').next().unwrap_or(docset.slug.as_str());
        if matches!(family, "rust" | "haskell") {
            continue;
        }
        let entry = by_slug.entry(docset.slug.clone()).or_insert_with(|| Shorthand {
            words: Vec::new(),
            label: docset_label(&docset),
            url: format!("https://devdocs.io/{}", docset.slug),
        });
        if !entry.words.iter().any(|existing| existing == &word) {
            entry.words.push(word.clone());
        }
        used.insert(word);
    }
    groups.extend(by_slug.into_values());
    for group in &mut groups {
        group.words.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        group.words.dedup();
    }
    groups
}

fn rank_shorthands(groups: &[Shorthand], pattern: &str, limit: usize) -> Listed {
    let pattern: String = pattern
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(|ch| ch.to_lowercase())
        .collect();
    let mut scored: Vec<(i32, &Shorthand)> = Vec::new();
    for group in groups {
        if pattern.is_empty() {
            if group.words.iter().any(|word| STARTERS.contains(&word.as_str())) {
                scored.push((0, group));
            }
            continue;
        }
        let word_score = group.words.iter().filter_map(|word| fuzzy_score(&pattern, word)).max();
        let label_score = fuzzy_score(&pattern, &group.label).map(|score| score.saturating_sub(400));
        let Some(score) = word_score.into_iter().chain(label_score).max() else {
            continue;
        };
        scored.push((score, group));
    }
    if pattern.is_empty() {
        scored.sort_by_key(|(_, group)| starter_index(&group.words));
    } else {
        scored.sort_by(|left, right| {
            right.0.cmp(&left.0).then_with(|| {
                left.1
                    .label
                    .to_ascii_lowercase()
                    .cmp(&right.1.label.to_ascii_lowercase())
            })
        });
    }
    let total = scored.len();
    let shown = limit.clamp(1, 20).min(total);
    let groups = scored
        .into_iter()
        .take(shown)
        .map(|(score, group)| {
            let _ = score;
            let mut group = group.clone();
            if !pattern.is_empty() {
                group.words.sort_by(|left, right| {
                    fuzzy_score(&pattern, right)
                        .unwrap_or(0)
                        .cmp(&fuzzy_score(&pattern, left).unwrap_or(0))
                        .then(left.len().cmp(&right.len()))
                        .then(left.cmp(right))
                });
            }
            group
        })
        .collect();
    Listed {
        groups,
        hidden: total.saturating_sub(shown),
    }
}

fn starter_index(words: &[String]) -> usize {
    STARTERS
        .iter()
        .position(|starter| words.iter().any(|word| word == starter))
        .unwrap_or(STARTERS.len())
}

pub(crate) fn fuzzy_score(pattern: &str, text: &str) -> Option<i32> {
    if pattern.is_empty() {
        return Some(1);
    }
    let text = text.to_ascii_lowercase();
    if text == pattern {
        return Some(10_000 - text.len() as i32);
    }
    if text.starts_with(pattern) {
        return Some(8_000 - text.len() as i32);
    }
    if let Some(at) = text.find(pattern) {
        return Some(6_000 - at as i32 - text.len() as i32);
    }
    let chars: Vec<char> = text.chars().collect();
    let mut from = 0;
    let mut score = 2_000;
    let mut previous = None;
    for needle in pattern.chars() {
        let relative = chars[from..].iter().position(|ch| *ch == needle)?;
        let at = from + relative;
        if previous.is_some_and(|previous| previous + 1 == at) {
            score += 40;
        } else {
            score -= 15;
        }
        if at == 0 || chars.get(at - 1).is_some_and(|ch| !ch.is_ascii_alphanumeric()) {
            score += 25;
        }
        previous = Some(at);
        from = at + 1;
    }
    Some(score - text.len() as i32)
}

fn is_typeable(word: &str) -> bool {
    !word.is_empty()
        && word
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '+' | '#' | '.' | '_' | '-'))
}

fn is_command_word(word: &str) -> bool {
    const WORDS: &[&str] = &[
        "web", "ddg", "cheat", "cht", "tldr", "crate", "crates", "docs", "doc", "so", "stack",
        "stackoverflow", "gh", "github", "mdn", "wiki", "wikipedia", "man", "lang", "langs",
        "languages", "topic", "topics", "concept", "concepts", "history", "hist", "arch",
        "archwiki", "rfc", "rfcs", "npm", "pypi", "pip", "gentoo", "debian", "fedora", "ubuntu",
        "gem", "rubygems", "hex", "go-mod", "pkggo", "maven", "nuget", "cve", "nvd", "item",
        "stock", "sku", "upc", "barcode", "dnf", "apt", "apt-get", "pacman", "brew", "cargo",
        "rpm", "flatpak", "zypper", "snap", "podman",
    ];
    WORDS.contains(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docset(name: &str, slug: &str, alias: Option<&str>, release: Option<&str>) -> Docset {
        Docset {
            name: name.into(),
            slug: slug.into(),
            alias: alias.map(str::to_string),
            release: release.map(str::to_string),
        }
    }

    #[test]
    fn picks_latest_alias_and_plain_slug() {
        let catalog = vec![
            docset("Python", "python~3.9", Some("py"), Some("3.9")),
            docset("Python", "python~3.14", Some("py"), Some("3.14")),
            docset("C++", "cpp", Some("c++"), None),
            docset("OCaml", "ocaml", None, Some("5.5")),
            docset("OCaml", "ocaml~4.14", None, Some("4.14")),
        ];
        assert_eq!(pick_docset(&catalog, "py").unwrap().slug, "python~3.14");
        assert_eq!(pick_docset(&catalog, "c++").unwrap().slug, "cpp");
        assert_eq!(pick_docset(&catalog, "ocaml").unwrap().slug, "ocaml");
        assert_eq!(pick_docset(&catalog, "python").unwrap().slug, "python~3.14");
    }

    #[test]
    fn ranks_exact_symbols_above_lookalikes() {
        let vector = score("vector", "vector").unwrap();
        let std_vector = score("vector", "std::vector").unwrap();
        assert!(vector > std_vector);
        let exact = score("std::vector::push_back", "std::vector::push_back").unwrap();
        let partial = score("std::vector::push_back", "std::vector::push_back_trivially").unwrap();
        assert!(exact > partial);
        assert!(score("std::vector::push_back", "std::vector::push_front").is_none());
        let list_map = score("List.map", "val map [Module List]").unwrap();
        let concat = score("List.map", "val concat_map [Module List]").unwrap();
        let labels = score("List.map", "val map [Module ListLabels]").unwrap();
        assert!(list_map > concat);
        assert!(list_map > labels);
        let counting = score("semaphore", "std::counting_semaphore").unwrap();
        let header = score("semaphore", "semaphore").unwrap();
        assert!(header > counting);
        assert!(counting >= 500);
        assert!(score("semaphore", "std::counting_semaphore::acquire").unwrap_or(0) < 500);
    }

    #[test]
    fn usage_block_keeps_the_example_and_drops_program_output() {
        let html = r#"<p>This header is part of the thread support library.</p>
            <pre>namespace std {
  class counting_semaphore {
  public:
    void acquire();
  };
}</pre>
            <pre data-language="c">#include &lt;semaphore&gt;
std::binary_semaphore
    smphSignalMainToThread{0};

void ThreadProc()
{
    smph.release();
    smph.acquire();
}
</pre>
            <pre>[main] Got the signal
[thread] Send the signal</pre>
            <p>Licensed under the CC license</p>"#;
        let usage = usage_block(html).unwrap();
        assert!(usage.starts_with("#include <semaphore>"));
        assert!(usage.contains("    smphSignalMainToThread{0};"));
        assert!(usage.contains("    smph.release();"));
        assert!(!usage.contains("[main]"));
        assert!(!usage.contains("Licensed"));
        let synopsis = usage_block(
            "<pre>namespace std {\n  class counting_semaphore {\n  public:\n    void acquire();\n  };\n}</pre>",
        )
        .unwrap();
        assert!(synopsis.contains("  class counting_semaphore {"));
        assert!(synopsis.contains("    void acquire();"));
        let sentence = explain_sentence(
            "<p>(since C++20) 1) A counting_semaphore is a lightweight synchronization primitive that can control access to a shared resource. 2) Other text.</p>",
        )
        .unwrap();
        assert_eq!(
            sentence,
            "A counting_semaphore is a lightweight synchronization primitive that can control access to a shared resource."
        );
    }

    #[test]
    fn fuzzy_prefers_a_prefix_over_scattered_letters() {
        let python = fuzzy_score("pyth", "python").unwrap();
        let pytorch = fuzzy_score("pyth", "pytorch").unwrap();
        assert!(python > pytorch);
        assert!(fuzzy_score("hs", "hs").unwrap() > fuzzy_score("hs", "haskell").unwrap());
        assert!(fuzzy_score("ocam", "ocaml").is_some());
    }

    #[test]
    fn lang_search_orders_shorthands() {
        let groups = shorthand_groups(&[
            docset("Python", "python~3.14", Some("py"), Some("3.14.7")),
            docset("OCaml", "ocaml", None, Some("5.5")),
            docset("Pygame", "pygame", None, None),
        ]);
        let listed = rank_shorthands(&groups, "pyth", 5);
        assert!(listed.groups[0].label.starts_with("Python"));
        assert!(listed.groups[0].words.iter().any(|word| word == "py"));
        let ocaml = rank_shorthands(&groups, "ocam", 5);
        assert!(ocaml.groups[0].words.iter().any(|word| word == "ocaml"));
        assert!(ocaml.groups[0].words.iter().any(|word| word == "ml"));
        let haskell = rank_shorthands(&groups, "hs", 5);
        assert_eq!(haskell.groups[0].label, "Haskell");
        assert_eq!(haskell.groups[0].words[0], "hs");
    }
}
