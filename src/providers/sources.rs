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

use std::path::PathBuf;

use crate::text::encode_query;

pub fn lookup(word: &str, query: &str) -> Option<String> {
    let text = std::fs::read_to_string(sources_path()).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((words, template)) = line.split_once(char::is_whitespace) else {
            continue;
        };
        let matched = words
            .split(',')
            .map(str::trim)
            .any(|alias| alias.eq_ignore_ascii_case(word));
        if matched {
            return Some(template.replace("{query}", &encode_query(query)));
        }
    }
    None
}

pub fn sources_path() -> PathBuf {
    crate::providers::langs::config_dir().join("sources")
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_line_maps_a_word_to_its_url() {
        let line = "aur,aurweb https://aur.archlinux.org/packages?K={query}";
        let (words, template) = line.split_once(char::is_whitespace).unwrap();
        assert!(words.split(',').any(|word| word == "aur"));
        assert!(template.contains("{query}"));
    }
}
