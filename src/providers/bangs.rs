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

use crate::http::redirect_location;
use crate::model::{Hit, Outcome};
use crate::providers::fallback;
use crate::text::encode_query;

pub fn search(name: &str, query: &str) -> Result<Outcome, String> {
    if name.eq_ignore_ascii_case("aur") {
        let url = format!(
            "https://aur.archlinux.org/packages?O=0&K={}",
            encode_query(query)
        );
        let mut outcome = Outcome::new("aur", query);
        outcome.source_url = Some(url.clone());
        outcome
            .hits
            .push(Hit::new(format!("Search AUR packages for {query}"), url));
        return Ok(outcome);
    }
    let target = format!(
        "https://duckduckgo.com/?q=!{}+{}",
        encode_query(name),
        encode_query(query)
    );
    let location = match redirect_location(&target) {
        Ok(location) => normalize_location(location),
        Err(reason) => {
            return Ok(fallback::search(
                name,
                query,
                format!("Open !{name} search for {query}"),
                target,
                reason,
            ));
        }
    };
    let mut outcome = Outcome::new(name, query);
    outcome.source_url = Some(target);
    outcome.hits.push(Hit::new(query, location));
    Ok(outcome)
}

fn normalize_location(location: String) -> String {
    if location.starts_with("http") {
        location
    } else if location.starts_with("//") {
        format!("https:{location}")
    } else {
        format!("https://duckduckgo.com{location}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aur_bang_has_a_direct_fallback_without_redirect_scraping() {
        let outcome = search("aur", "paru-bin").unwrap();
        assert!(outcome.hits[0].url.contains("aur.archlinux.org/packages"));
        assert!(outcome.hits[0].url.contains("paru-bin"));
    }

    #[test]
    fn normalizes_redirect_locations() {
        assert_eq!(
            normalize_location("https://example.test/x".into()),
            "https://example.test/x"
        );
        assert_eq!(
            normalize_location("//example.test/x".into()),
            "https://example.test/x"
        );
        assert_eq!(
            normalize_location("/l/?x=1".into()),
            "https://duckduckgo.com/l/?x=1"
        );
    }
}
