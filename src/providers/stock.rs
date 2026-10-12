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

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
use crate::providers::fallback;
use crate::providers::langs::cache_dir;
use crate::text::{collapse_ws, encode_query, truncate};

pub fn is_barcode(query: &str) -> bool {
    let query = query.trim();
    let digits = query.chars().all(|ch| ch.is_ascii_digit());
    digits && matches!(query.len(), 8 | 12 | 13 | 14)
}

pub fn search(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    if is_barcode(query) {
        barcode(client, query.trim())
    } else {
        by_name(client, query, limit)
    }
}

fn barcode(client: &Client, code: &str) -> Result<Outcome, String> {
    let path = cache_dir().join("stock").join(format!("{code}.json"));
    let cached = crate::cache::get_or_fetch(
        &path,
        Duration::from_secs(14 * 24 * 60 * 60),
        Duration::from_secs(30 * 24 * 60 * 60),
        1,
        code,
        || {
            open_food_facts_code(client, code)
                .or_else(|| upcitemdb(client, code))
                .ok_or_else(|| {
                    format!("product services returned no usable live result for barcode {code}")
                })
        },
    );
    match cached {
        Ok(cached) => Ok(cached.value.into_outcome(code)),
        Err(error) => Ok(barcode_fallback(code, &error.to_string())),
    }
}

fn by_name(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://world.openfoodfacts.org/cgi/search.pl?search_simple=1&action=process&json=1&page_size={limit}&search_terms={}",
        encode_query(query)
    );
    let response: Value = match client.get_json(&url, APP_UA, &[]) {
        Ok(response) => response,
        Err(reason) => {
            let mut outcome = name_fallback(query, &reason);
            append_market_candidate(&mut outcome, query, limit);
            return Ok(outcome);
        }
    };
    let products = response
        .get("products")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut outcome = Outcome::new("stock", query);
    outcome.source_url = Some(format!(
        "https://world.openfoodfacts.org/cgi/search.pl?search_terms={}",
        encode_query(query)
    ));
    for product in products.into_iter().take(limit) {
        let name = text_field(&product, "product_name");
        let code = text_field(&product, "code");
        if name.is_empty() {
            continue;
        }
        let url = if code.is_empty() {
            outcome.source_url.clone().unwrap_or_default()
        } else {
            format!("https://world.openfoodfacts.org/product/{code}")
        };
        let brand = text_field(&product, "brands");
        let quantity = text_field(&product, "quantity");
        let detail = [brand, quantity]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("  ·  ");
        outcome.hits.push(Hit::new(name, url).snippet(detail));
    }
    if outcome.hits.is_empty() {
        outcome = name_fallback(query, &format!("no products matched {query}"));
    }
    append_market_candidate(&mut outcome, query, limit);
    Ok(outcome)
}

fn append_market_candidate(outcome: &mut Outcome, query: &str, limit: usize) {
    let symbol = query.trim();
    let possible_ticker =
        (1..=5).contains(&symbol.len()) && symbol.chars().all(|ch| ch.is_ascii_uppercase());
    if possible_ticker && outcome.hits.len() < limit {
        outcome.hits.push(
            Hit::new(
                format!("Possible {symbol} market quote"),
                format!("https://finance.yahoo.com/quote/{symbol}"),
            )
            .snippet("The uppercase product query can also be a market ticker."),
        );
    }
}

fn name_fallback(query: &str, reason: &str) -> Outcome {
    let url = format!(
        "https://world.openfoodfacts.org/cgi/search.pl?search_terms={}",
        encode_query(query)
    );
    fallback::search(
        "stock",
        query,
        format!("Search products for {query}"),
        url,
        reason,
    )
}

fn barcode_fallback(code: &str, reason: &str) -> Outcome {
    let url = format!("https://world.openfoodfacts.org/product/{code}");
    fallback::search(
        "stock",
        code,
        format!("Look up barcode {code}"),
        url,
        reason,
    )
}

fn open_food_facts_code(client: &Client, code: &str) -> Option<Product> {
    let url = format!("https://world.openfoodfacts.org/api/v2/product/{code}");
    let response: Value = client.get_json(&url, APP_UA, &[]).ok()?;
    if response.get("status").and_then(Value::as_i64) == Some(0) {
        return None;
    }
    let product = response.get("product")?.clone();
    let name = text_field(&product, "product_name");
    if name.is_empty() {
        return None;
    }
    Some(Product {
        name,
        brand: text_field(&product, "brands"),
        quantity: text_field(&product, "quantity"),
        price: String::new(),
        url: format!("https://world.openfoodfacts.org/product/{code}"),
    })
}

fn upcitemdb(client: &Client, code: &str) -> Option<Product> {
    let url = format!("https://api.upcitemdb.com/prod/trial/lookup?upc={code}");
    let response: Value = client.get_json(&url, APP_UA, &[]).ok()?;
    let item = response.get("items")?.as_array()?.first()?.clone();
    let name = text_field(&item, "title");
    if name.is_empty() {
        return None;
    }
    let offers = item.get("offers").and_then(Value::as_array);
    let price = offers
        .and_then(|offers| offers.first())
        .map(|offer| {
            let price = text_field(offer, "price");
            let currency = text_field(offer, "currency");
            [currency, price]
                .into_iter()
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    Some(Product {
        name,
        brand: text_field(&item, "brand"),
        quantity: String::new(),
        price,
        url: format!("https://www.upcitemdb.com/upc/{code}"),
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Product {
    name: String,
    brand: String,
    quantity: String,
    price: String,
    url: String,
}

impl Product {
    fn into_outcome(self, code: &str) -> Outcome {
        let mut outcome = Outcome::new("stock", code);
        outcome.source_url = Some(self.url.clone());
        let detail = [self.brand, self.quantity, self.price]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("  ·  ");
        outcome
            .hits
            .push(Hit::new(self.name, self.url).snippet(detail));
        outcome
    }
}

fn text_field(value: &Value, key: &str) -> String {
    truncate(
        &collapse_ws(value.get(key).and_then(Value::as_str).unwrap_or("")),
        180,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_fallback_is_clickable_and_encoded() {
        let got = name_fallback("tea & biscuits", "HTTP 503");
        assert!(got.hits[0]
            .url
            .ends_with("search_terms=tea%20%26%20biscuits"));
        assert!(got.has_content());
    }

    #[test]
    fn barcode_fallback_uses_known_product_page() {
        let got = barcode_fallback("012345678905", "upstreams unavailable");
        assert_eq!(
            got.hits[0].url,
            "https://world.openfoodfacts.org/product/012345678905"
        );
    }

    #[test]
    fn uppercase_ambiguous_names_keep_products_first_and_offer_a_quote() {
        let mut outcome = name_fallback("MILK", "offline");
        append_market_candidate(&mut outcome, "MILK", 5);
        assert!(outcome.hits[0].title.contains("products"));
        assert_eq!(outcome.hits[1].url, "https://finance.yahoo.com/quote/MILK");

        let mut ordinary = name_fallback("milk", "offline");
        append_market_candidate(&mut ordinary, "milk", 5);
        assert_eq!(ordinary.hits.len(), 1);
    }
}
