// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::model::{Hit, Outcome};

/// Turn a failed or empty source lookup into a useful, clickable result.
pub fn search(
    provider: &str,
    query: &str,
    title: impl Into<String>,
    url: impl Into<String>,
    detail: impl AsRef<str>,
) -> Outcome {
    let url = url.into();
    let mut outcome = Outcome::new(provider, query);
    outcome.source_url = Some(url.clone());
    let detail = detail.as_ref().trim();
    if !detail.is_empty() {
        outcome.summary = Some(format!("Live lookup was unavailable: {detail}"));
    }
    outcome.hits.push(Hit::new(title, url));
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_is_contentful_and_clickable() {
        let got = search(
            "docs",
            "thing",
            "Search docs",
            "https://example.test/?q=thing",
            "HTTP 503",
        );
        assert!(got.has_content());
        assert_eq!(got.first_url(), Some("https://example.test/?q=thing"));
        assert!(got.summary.as_deref().unwrap().contains("HTTP 503"));
    }
}
