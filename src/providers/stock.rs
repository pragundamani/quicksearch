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

use serde_json::Value;

use crate::http::{Client, APP_UA};
use crate::model::{Hit, Outcome};
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
    let path = cache_dir().join("stock").join(code);
    if let Some(text) = read_cached(&path) {
        if let Some(outcome) = outcome_from_cached(code, &text) {
            return Ok(outcome);
        }
    }
    let product = open_food_facts_code(client, code).or_else(|| upcitemdb(client, code));
    let Some(product) = product else {
        return Err(format!("no product for barcode {code}"));
    };
    write_cached(&path, &product.cache_line());
    Ok(product.into_outcome(code))
}

fn by_name(client: &Client, query: &str, limit: usize) -> Result<Outcome, String> {
    let url = format!(
        "https://world.openfoodfacts.org/cgi/search.pl?search_simple=1&action=process&json=1&page_size={limit}&search_terms={}",
        encode_query(query)
    );
    let response: Value = client.get_json(&url, APP_UA, &[])?;
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
        return Err(format!("no products matched {query}"));
    }
    Ok(outcome)
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

struct Product {
    name: String,
    brand: String,
    quantity: String,
    price: String,
    url: String,
}

impl Product {
    fn cache_line(&self) -> String {
        [
            self.name.as_str(),
            self.brand.as_str(),
            self.quantity.as_str(),
            self.price.as_str(),
            self.url.as_str(),
        ]
        .join("\t")
    }

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

fn outcome_from_cached(code: &str, text: &str) -> Option<Outcome> {
    let mut parts = text.split('\t');
    let name = parts.next()?.trim().to_string();
    if name.is_empty() {
        return None;
    }
    let product = Product {
        name,
        brand: parts.next().unwrap_or("").to_string(),
        quantity: parts.next().unwrap_or("").to_string(),
        price: parts.next().unwrap_or("").to_string(),
        url: parts.next().unwrap_or("").to_string(),
    };
    Some(product.into_outcome(code))
}

fn text_field(value: &Value, key: &str) -> String {
    truncate(
        &collapse_ws(value.get(key).and_then(Value::as_str).unwrap_or("")),
        180,
    )
}

fn read_cached(path: &std::path::Path) -> Option<String> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    if modified.elapsed().ok()? > std::time::Duration::from_secs(14 * 24 * 60 * 60) {
        return None;
    }
    std::fs::read_to_string(path).ok()
}

fn write_cached(path: &std::path::Path, text: &str) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, text);
}
