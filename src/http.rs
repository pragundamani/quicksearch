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

use std::io::Read;
use std::time::Duration;

use serde::de::DeserializeOwned;

pub const APP_UA: &str = "quicksearch/0.1 (personal lookup cli)";
pub const BROWSER_UA: &str = "Mozilla/5.0 (compatible; quicksearch/0.1)";
pub const CURL_UA: &str = "curl/8.0";
const MAX_RESPONSE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_DECODED_BYTES: u64 = 32 * 1024 * 1024;

pub struct Client {
    pub agent: ureq::Agent,
}

impl Client {
    pub fn new() -> Self {
        let agent = ureq::AgentBuilder::new()
            // Ureq's connect timeout otherwise defaults to 30 seconds and can
            // outlive the overall request budget during DNS/connect failures.
            .timeout_connect(Duration::from_secs(4))
            .timeout_read(Duration::from_secs(8))
            .timeout_write(Duration::from_secs(8))
            .timeout(Duration::from_secs(10))
            .build();
        Self { agent }
    }

    pub fn get_text(
        &self,
        url: &str,
        ua: &str,
        headers: &[(&str, &str)],
    ) -> Result<String, String> {
        let response = self.request("GET", url, ua, headers)?;
        let bytes = read_bounded(response.into_reader(), MAX_RESPONSE_BYTES)?;
        String::from_utf8(bytes).map_err(|error| error.to_string())
    }

    pub fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        ua: &str,
        headers: &[(&str, &str)],
    ) -> Result<T, String> {
        let text = self.get_text(url, ua, headers)?;
        parse_json(&text, url)
    }

    pub fn get_json_gzip<T: DeserializeOwned>(
        &self,
        url: &str,
        ua: &str,
        headers: &[(&str, &str)],
    ) -> Result<T, String> {
        let resp = self.request("GET", url, ua, headers)?;
        let encoding = resp
            .header("Content-Encoding")
            .unwrap_or("")
            .to_ascii_lowercase();
        let raw = read_bounded(resp.into_reader(), MAX_RESPONSE_BYTES)?;
        let text = if encoding.contains("gzip") || raw.starts_with(&[0x1f, 0x8b]) {
            let decoded = decode_gzip_bounded(&raw)?;
            String::from_utf8(decoded).map_err(|error| error.to_string())?
        } else {
            String::from_utf8(raw).map_err(|err| err.to_string())?
        };
        parse_json(&text, url)
    }

    fn request(
        &self,
        method: &str,
        url: &str,
        ua: &str,
        headers: &[(&str, &str)],
    ) -> Result<ureq::Response, String> {
        let mut req = self.agent.request(method, url).set("User-Agent", ua);
        for (name, value) in headers {
            req = req.set(name, value);
        }
        req.call().map_err(format_ureq)
    }
}

fn decode_gzip_bounded(raw: &[u8]) -> Result<Vec<u8>, String> {
    read_bounded(flate2::read::MultiGzDecoder::new(raw), MAX_DECODED_BYTES)
        .map_err(|error| format!("gzip: {error}"))
}

fn read_bounded(reader: impl Read, limit: u64) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    reader
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > limit {
        return Err(format!("response exceeds {} MiB", limit / (1024 * 1024)));
    }
    Ok(bytes)
}

fn parse_json<T: DeserializeOwned>(text: &str, url: &str) -> Result<T, String> {
    serde_json::from_str(text).map_err(|err| {
        let snippet: String = text.chars().take(160).collect();
        format!("invalid JSON from {url}: {err} ({snippet})")
    })
}

pub fn redirect_location(url: &str) -> Result<String, String> {
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout(Duration::from_secs(12))
        .build();
    match agent.get(url).set("User-Agent", BROWSER_UA).call() {
        Ok(response) => response
            .header("Location")
            .map(str::to_string)
            .ok_or_else(|| format!("no redirect from {url}")),
        Err(ureq::Error::Status(code, response)) if (300..400).contains(&code) => response
            .header("Location")
            .map(str::to_string)
            .ok_or_else(|| format!("HTTP {code} from {url}")),
        Err(err) => Err(err.to_string()),
    }
}

pub fn is_not_found(err: &str) -> bool {
    err.starts_with("HTTP 404")
}

fn format_ureq(err: ureq::Error) -> String {
    match err {
        ureq::Error::Status(code, resp) => {
            let body = resp.into_string().unwrap_or_default();
            let body: String = body.chars().take(160).collect();
            let body = body.replace('\n', " ");
            if body.trim().is_empty() {
                format!("HTTP {code}")
            } else {
                format!("HTTP {code}: {body}")
            }
        }
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    fn gzip(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn concatenated_gzip_members_share_one_decoded_limit() {
        let mut encoded = gzip(br#"{"ok":true}"#);
        encoded.extend(gzip(&vec![b'x'; MAX_DECODED_BYTES as usize + 1]));
        assert!(decode_gzip_bounded(&encoded)
            .unwrap_err()
            .contains("response exceeds"));
    }

    #[test]
    fn corrupt_trailing_gzip_member_is_rejected() {
        let mut encoded = gzip(br#"{"ok":true}"#);
        encoded.extend([0x1f, 0x8b, 0x08]);
        assert!(decode_gzip_bounded(&encoded).is_err());
    }
}
