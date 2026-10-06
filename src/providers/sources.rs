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
