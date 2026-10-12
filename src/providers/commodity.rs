// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::http::Client;
use crate::model::{Hit, Outcome};
use crate::providers::web;
use crate::text::encode_query;

pub fn search(_client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let Some(reference) = price_reference(query) else {
        return web::search_explicit(_client, query, limit);
    };
    let mut outcome = Outcome::new("commodity", query);
    outcome.source_url = Some(reference.url.clone());
    outcome.hits.push(reference);
    if limit > 1 {
        outcome.hits.push(Hit::new(
            format!("Search the web for {query}"),
            format!("https://duckduckgo.com/?q={}", encode_query(query)),
        ));
    }
    Ok(outcome)
}

fn price_reference(query: &str) -> Option<Hit> {
    let words: Vec<String> = query
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    if !words
        .iter()
        .any(|word| matches!(word.as_str(), "price" | "prices" | "quote" | "quotes"))
    {
        return None;
    }
    let (name, url) = [
        (
            "gold",
            "https://www.cmegroup.com/markets/metals/precious/gold.html",
        ),
        (
            "silver",
            "https://www.cmegroup.com/markets/metals/precious/silver.html",
        ),
        (
            "platinum",
            "https://www.cmegroup.com/markets/metals/precious/platinum.html",
        ),
        (
            "palladium",
            "https://www.cmegroup.com/markets/metals/precious/palladium.html",
        ),
    ]
    .into_iter()
    .find(|(name, _)| words.iter().any(|word| word == name))?;
    let mut chars = name.chars();
    let display_name = chars
        .next()
        .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
        .unwrap_or_default();
    Some(
        Hit::new(format!("{display_name} futures quotes — CME Group"), url)
            .snippet("Open the CME Group market page for current and delayed exchange quotes."),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precious_metal_prices_get_a_direct_exchange_reference() {
        let hit = price_reference("price of palladium per ounce").unwrap();
        assert_eq!(hit.title, "Palladium futures quotes — CME Group");
        assert_eq!(
            hit.url,
            "https://www.cmegroup.com/markets/metals/precious/palladium.html"
        );
        assert!(hit.snippet.contains("Open the CME Group"));
        assert!(price_reference("palladium properties").is_none());
    }
}
