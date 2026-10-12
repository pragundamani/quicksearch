// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use std::fmt;
use std::time::Duration;

const MAX_ERROR_MESSAGE_BYTES: usize = 512;

/// The example-generation backend used for a result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ProviderId {
    GeminiCli,
    GeminiApi,
}

impl ProviderId {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::GeminiCli => "gemini-cli",
            Self::GeminiApi => "gemini-api",
        }
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderMetadata {
    pub(crate) provider: ProviderId,
    pub(crate) model: String,
    pub(crate) elapsed: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderResult {
    pub(crate) text: String,
    pub(crate) metadata: ProviderMetadata,
}

impl ProviderResult {
    pub(crate) fn new(
        provider: ProviderId,
        model: impl Into<String>,
        text: String,
        elapsed: Duration,
    ) -> Self {
        Self {
            text,
            metadata: ProviderMetadata {
                provider,
                model: model.into(),
                elapsed,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderErrorKind {
    NotInstalled,
    Auth,
    RateLimit,
    Model,
    Timeout,
    BadOutput,
    Service,
}

/// A provider failure with a terminal-safe, size-bounded message.
///
/// Credentials must never be put in `message`. Provider entry points accept
/// credentials as borrowed strings and never retain them in this type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderError {
    kind: ProviderErrorKind,
    message: String,
}

impl ProviderError {
    pub(crate) fn new(kind: ProviderErrorKind, message: impl AsRef<str>) -> Self {
        Self {
            kind,
            message: safe_message(message.as_ref()),
        }
    }

    pub(crate) const fn kind(&self) -> ProviderErrorKind {
        self.kind
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ProviderError {}

fn safe_message(input: &str) -> String {
    let mut output = String::new();
    let mut pending_space = false;

    for character in input.chars() {
        if character.is_whitespace() || character.is_control() || is_format_control(character) {
            pending_space = !output.is_empty();
            continue;
        }
        if pending_space {
            if output.len() + 1 > MAX_ERROR_MESSAGE_BYTES {
                break;
            }
            output.push(' ');
            pending_space = false;
        }
        if output.len() + character.len_utf8() > MAX_ERROR_MESSAGE_BYTES {
            break;
        }
        output.push(character);
    }

    if output.is_empty() {
        "provider request failed".to_string()
    } else {
        output
    }
}

fn is_format_control(character: char) -> bool {
    matches!(
        character,
        '\u{061c}'
            | '\u{200b}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{206f}'
            | '\u{feff}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_ids_are_stable() {
        assert_eq!(ProviderId::GeminiCli.as_str(), "gemini-cli");
        assert_eq!(ProviderId::GeminiApi.as_str(), "gemini-api");
    }

    #[test]
    fn error_messages_are_safe_and_bounded() {
        let hostile = format!("auth\n\x1b[31mfailed\u{202e} {}", "x".repeat(700));
        let error = ProviderError::new(ProviderErrorKind::Auth, hostile);
        assert!(error.message().len() <= MAX_ERROR_MESSAGE_BYTES);
        assert!(!error.message().contains('\n'));
        assert!(!error.message().contains('\x1b'));
        assert!(!error.message().contains('\u{202e}'));
    }

    #[test]
    fn empty_messages_get_a_safe_fallback() {
        let error = ProviderError::new(ProviderErrorKind::Service, "\n\x1b");
        assert_eq!(error.message(), "provider request failed");
    }
}
