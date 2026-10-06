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

use crate::http::{Client, CURL_UA};
use crate::model::Outcome;
use crate::text::strip_ansi;

pub fn search(client: &Client, query: &str, color: bool) -> Result<Outcome, String> {
    let path = cheat_path(query);
    let mut url = format!("https://cheat.sh/{path}");
    if !color {
        url.push_str("?T");
    }
    let body = client.get_text(&url, CURL_UA, &[])?;
    if body.trim_start().starts_with("<!DOCTYPE") || body.trim_start().starts_with("<html") {
        return Err(format!("cheat.sh returned HTML instead of a sheet: {url}"));
    }
    let mut outcome = Outcome::new("cheat.sh", query);
    outcome.source_url = Some(url);
    let text = if color { body } else { strip_ansi(&body) };
    if text.trim().is_empty() {
        return Err(format!("cheat.sh had no sheet for {query}"));
    }
    outcome.body = Some(text.trim_end().to_string());
    Ok(outcome)
}

pub fn cheat_path(query: &str) -> String {
    query
        .trim()
        .trim_matches('/')
        .split('/')
        .map(|segment| {
            segment
                .split_whitespace()
                .map(encode_segment)
                .collect::<Vec<_>>()
                .join("+")
        })
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/")
}

fn encode_segment(segment: &str) -> String {
    let mut out = String::new();
    for ch in segment.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '~') {
            out.push(ch);
        } else {
            for byte in ch.encode_utf8(&mut [0; 4]).as_bytes() {
                out.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spaces_become_plus_and_slashes_stay() {
        assert_eq!(cheat_path("tar"), "tar");
        assert_eq!(cheat_path("rust read file"), "rust+read+file");
        assert_eq!(cheat_path("rust/vec push"), "rust/vec+push");
        assert_eq!(cheat_path("c++"), "c%2B%2B");
    }
}
