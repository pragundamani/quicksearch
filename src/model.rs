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

#[derive(Debug, Clone, serde::Serialize)]
pub struct Hit {
    pub title: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub snippet: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meta: Vec<String>,
}

impl Hit {
    pub fn new(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            url: url.into(),
            snippet: String::new(),
            meta: Vec::new(),
        }
    }

    pub fn snippet(mut self, text: impl Into<String>) -> Self {
        self.snippet = crate::text::collapse_ws(&text.into());
        self
    }

    pub fn meta_line(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        if !text.is_empty() {
            self.meta.push(text);
        }
        self
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Outcome {
    pub provider: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub hits: Vec<Hit>,
}

impl Outcome {
    pub fn new(provider: &str, query: &str) -> Self {
        Self {
            provider: provider.to_string(),
            query: query.to_string(),
            source_url: None,
            summary_title: None,
            summary: None,
            summary_url: None,
            body: None,
            hits: Vec::new(),
        }
    }

    pub fn has_content(&self) -> bool {
        self.body.as_ref().is_some_and(|s| !s.trim().is_empty())
            || self.summary.as_ref().is_some_and(|s| !s.trim().is_empty())
            || !self.hits.is_empty()
    }

    pub fn first_url(&self) -> Option<&str> {
        self.hits
            .iter()
            .map(|hit| hit.url.as_str())
            .find(|url| url.starts_with("http") || url.starts_with("man://"))
            .or(self
                .summary_url
                .as_deref()
                .filter(|url| url.starts_with("http")))
            .or(self
                .source_url
                .as_deref()
                .filter(|url| url.starts_with("http")))
    }
}
