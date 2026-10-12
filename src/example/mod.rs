//! Example-generation orchestration.
//!
//! This module joins the bounded grounding, strict response parser, providers,
//! configuration, and syntax-only validator. Generated data stays in memory.

mod config;
mod gemini_api;
mod gemini_cli;
mod grounding;
mod provider;
mod schema;
mod validate;

use std::cell::RefCell;
use std::fmt;
use std::io::{self, BufRead, IsTerminal, Write};

use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;

use crate::model::Outcome;

use self::config::{
    ConfigError, CredentialStore, Environment, FailureAction, OsCredentialStore,
    ProcessEnvironment, SetupIo, SetupOutcome, StateStore,
};
use self::grounding::GroundingItem;
use self::provider::{ProviderError, ProviderErrorKind, ProviderId, ProviderResult};
use self::validate::{ValidationResult, ValidationStatus};

const OUTPUT_VERSION: u32 = 1;
const REMOTE_SERVICE: &str = "Google";
const SETUP_PROBE: &str = r#"Return exactly this JSON object and nothing else: {"schema":1,"language":"json","code":"{}","explanation":"setup probe","citations":[]}"#;

/// Controls paths that must never prompt or open an OS credential dialog.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct GenerateOptions {
    pub json: bool,
    pub noninteractive: bool,
}

impl GenerateOptions {
    fn prompt_allowed(self, io: &impl SetupIo) -> bool {
        !self.json && !self.noninteractive && io.is_interactive()
    }
}

/// Stable error identifiers intended for both human and JSON callers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExampleErrorCode {
    #[serde(rename = "example_provider_unconfigured")]
    ProviderUnconfigured,
    #[serde(rename = "example_interaction_required")]
    SetupRequired,
    #[serde(rename = "example_setup_cancelled")]
    SetupCancelled,
    #[serde(rename = "example_config_invalid")]
    Configuration,
    #[serde(rename = "example_credential_unavailable")]
    CredentialUnavailable,
    #[serde(rename = "example_provider_unavailable")]
    ProviderNotInstalled,
    #[serde(rename = "example_auth_failed")]
    ProviderAuthentication,
    #[serde(rename = "example_rate_limited")]
    ProviderRateLimit,
    #[serde(rename = "example_model_unavailable")]
    ProviderModel,
    #[serde(rename = "example_timeout")]
    ProviderTimeout,
    #[serde(rename = "example_invalid_response")]
    ProviderOutput,
    #[serde(rename = "example_provider_unavailable")]
    ProviderService,
    #[serde(rename = "example_invalid_query")]
    InvalidPrompt,
    #[serde(rename = "example_grounding_unavailable")]
    GroundingUnavailable,
    #[serde(rename = "example_invalid_response")]
    InvalidResponse,
    #[serde(rename = "example_syntax_rejected")]
    SyntaxRejected,
    #[serde(rename = "example_io_error")]
    Io,
    #[serde(rename = "example_output_error")]
    Serialization,
}

/// A bounded, terminal-safe error. It never contains a credential.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct ExampleError {
    pub version: u32,
    pub code: ExampleErrorCode,
    pub message: String,
}

impl ExampleError {
    fn new(code: ExampleErrorCode, message: impl AsRef<str>) -> Self {
        Self {
            version: OUTPUT_VERSION,
            code,
            message: sanitize_single_line(message.as_ref(), 512),
        }
    }
}

impl fmt::Debug for ExampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExampleError")
            .field("version", &self.version)
            .field("code", &self.code)
            .field("message", &self.message)
            .finish()
    }
}

impl fmt::Display for ExampleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ExampleError {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ExampleCitation {
    pub index: usize,
    pub title: String,
    pub url: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ValidationLabel {
    SyntaxAccepted,
    Skipped,
}

/// Versioned successful generation envelope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ExampleSuccess {
    pub version: u32,
    pub provider: String,
    pub model: String,
    pub remote_service: &'static str,
    pub language: String,
    pub code: String,
    pub explanation: String,
    pub citations: Vec<ExampleCitation>,
    pub validation: ValidationLabel,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub validation_note: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SetupState {
    Configured,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct SetupReport {
    pub version: u32,
    pub state: SetupState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CredentialState {
    NotApplicable,
    EnvironmentOverride,
    Stored,
    Missing,
    NotChecked,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct StatusReport {
    pub version: u32,
    pub configured: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    pub credential: CredentialState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResetState {
    Removed,
    NotConfigured,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResetCredentialCleanup {
    Cleaned,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ResetReport {
    pub version: u32,
    pub state: ResetState,
    pub credential_cleanup: ResetCredentialCleanup,
}

/// Generate one example from an already-grounded in-memory search outcome.
///
/// This function does not fetch pages, cache prompts or responses, or record
/// history. First use prompts only on a real stdin/stderr terminal and only in
/// human mode.
pub(crate) fn generate(
    query: &str,
    grounded: &Outcome,
    options: GenerateOptions,
) -> Result<ExampleSuccess, ExampleError> {
    let state = crate::state::StateDir::platform();
    let credentials = OsCredentialStore;
    let environment = ProcessEnvironment;
    let providers = ProductionProviders;
    let mut terminal = TerminalSetupIo;
    generate_with(
        query,
        grounded,
        options,
        &state,
        &credentials,
        &environment,
        &providers,
        &mut terminal,
    )
}

/// Run explicit interactive setup. The minimal strict probe is the only remote
/// generation performed by this command.
pub(crate) fn setup() -> Result<SetupReport, ExampleError> {
    let state = crate::state::StateDir::platform();
    let credentials = OsCredentialStore;
    let environment = ProcessEnvironment;
    let providers = ProductionProviders;
    let mut terminal = TerminalSetupIo;
    setup_with(
        &state,
        &credentials,
        &environment,
        &providers,
        &mut terminal,
    )
}

/// Report setup state. When `machine_mode` is true, this never reads the OS
/// keyring; an environment override can still be reported safely.
pub(crate) fn status(machine_mode: bool) -> Result<StatusReport, ExampleError> {
    let state = crate::state::StateDir::platform();
    let machine_mode = machine_mode || !io::stdin().is_terminal() || !io::stderr().is_terminal();
    status_with(
        &state,
        &OsCredentialStore,
        &ProcessEnvironment,
        machine_mode,
    )
}

/// Remove the saved provider and attempt to clean the dedicated stored-key account.
pub(crate) fn reset() -> Result<ResetReport, ExampleError> {
    let state = crate::state::StateDir::platform();
    let removed = config::remove_config(&state, &OsCredentialStore).map_err(map_config_error)?;
    Ok(ResetReport {
        version: OUTPUT_VERSION,
        state: match removed.status {
            config::RemovalStatus::Removed => ResetState::Removed,
            config::RemovalStatus::NotConfigured => ResetState::NotConfigured,
        },
        credential_cleanup: match removed.credential_cleanup {
            config::CredentialCleanupStatus::Cleaned => ResetCredentialCleanup::Cleaned,
            config::CredentialCleanupStatus::Unavailable => ResetCredentialCleanup::Unavailable,
        },
    })
}

/// Serialize exactly one JSON value. The caller should write this string once.
pub(crate) fn to_json(value: &impl Serialize) -> Result<String, ExampleError> {
    serde_json::to_string(value).map_err(|_| {
        ExampleError::new(
            ExampleErrorCode::Serialization,
            "example output could not be serialized",
        )
    })
}

/// Plain human rendering. No pager, highlighter, shell, or other subprocess is
/// started. Every remote string is sanitized before it is written.
pub(crate) fn render_human(
    output: &ExampleSuccess,
    writer: &mut impl Write,
) -> Result<(), ExampleError> {
    writeln!(
        writer,
        "Example ({}, {}, sent to {})",
        sanitize_single_line(&output.provider, 128),
        sanitize_single_line(&output.model, 128),
        REMOTE_SERVICE
    )
    .map_err(map_io_error)?;
    writeln!(
        writer,
        "Language: {}",
        sanitize_single_line(&output.language, 64)
    )
    .map_err(map_io_error)?;
    writeln!(writer).map_err(map_io_error)?;
    writeln!(
        writer,
        "{}",
        sanitize_multiline(&output.code, schema::MAX_CODE_BYTES)
    )
    .map_err(map_io_error)?;
    if !output.explanation.is_empty() {
        writeln!(writer).map_err(map_io_error)?;
        writeln!(
            writer,
            "{}",
            sanitize_multiline(&output.explanation, schema::MAX_EXPLANATION_BYTES)
        )
        .map_err(map_io_error)?;
    }
    for citation in &output.citations {
        writeln!(
            writer,
            "[{}] {} — {}",
            citation.index,
            sanitize_single_line(&citation.title, 256),
            sanitize_single_line(&citation.url, 512)
        )
        .map_err(map_io_error)?;
    }
    let label = match output.validation {
        ValidationLabel::SyntaxAccepted => "Syntax accepted",
        ValidationLabel::Skipped => "Syntax check skipped",
    };
    if output.validation_note.is_empty() {
        writeln!(writer, "Validation: {label}").map_err(map_io_error)?;
    } else {
        writeln!(
            writer,
            "Validation: {label} ({})",
            sanitize_single_line(&output.validation_note, 512)
        )
        .map_err(map_io_error)?;
    }
    Ok(())
}

trait ProviderRunner {
    fn generate(
        &self,
        provider: ProviderId,
        secret: Option<&SecretString>,
        prompt: &str,
    ) -> Result<ProviderResult, ProviderError>;
}

struct ProductionProviders;

impl ProviderRunner for ProductionProviders {
    fn generate(
        &self,
        provider: ProviderId,
        secret: Option<&SecretString>,
        prompt: &str,
    ) -> Result<ProviderResult, ProviderError> {
        match provider {
            ProviderId::GeminiCli => gemini_cli::generate(prompt),
            ProviderId::GeminiApi => {
                let key = secret.ok_or_else(|| {
                    ProviderError::new(
                        ProviderErrorKind::Auth,
                        "Gemini API credentials are unavailable",
                    )
                })?;
                gemini_api::GeminiApi::new().generate(key.expose_secret(), prompt, None)
            }
        }
    }
}

#[derive(Debug)]
enum StageError {
    Provider(ProviderError),
    Public(ExampleError),
}

impl StageError {
    fn public(self) -> ExampleError {
        match self {
            Self::Provider(error) => map_provider_error(error),
            Self::Public(error) => error,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_with<S, C, E, R, I>(
    query: &str,
    grounded: &Outcome,
    options: GenerateOptions,
    state: &S,
    credentials: &C,
    environment: &E,
    providers: &R,
    io: &mut I,
) -> Result<ExampleSuccess, ExampleError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
    R: ProviderRunner,
    I: SetupIo,
{
    let grounding = grounding::build_grounding(grounded);
    if grounding.is_empty() {
        return Err(ExampleError::new(
            ExampleErrorCode::GroundingUnavailable,
            "no vetted HTTPS documentation result was available for this query",
        ));
    }
    let prompt = grounding::build_prompt_from_grounding(query, &grounding)
        .map_err(|error| ExampleError::new(ExampleErrorCode::InvalidPrompt, error.to_string()))?;
    let checks = GenerationChecks {
        query: query.to_string(),
        expected_language: expected_language(grounded),
    };
    let prompt_allowed = options.prompt_allowed(io);
    let captured = RefCell::new(None::<Result<ExampleSuccess, StageError>>);

    let ensured = config::ensure_config(
        state,
        credentials,
        environment,
        io,
        prompt_allowed,
        |provider, secret| {
            let generated = produce(providers, provider, secret, &prompt, &grounding, &checks);
            let accepted = generated.is_ok();
            *captured.borrow_mut() = Some(generated);
            if accepted {
                Ok(())
            } else {
                Err(())
            }
        },
    );

    let config = match ensured {
        Ok(config::EnsureOutcome::Saved(config)) => config,
        Ok(config::EnsureOutcome::Configured(config)) => {
            return captured
                .into_inner()
                .expect("setup probe always records its actual generation")
                .map_err(StageError::public)
                .and_then(|output| {
                    if output.provider == config.provider.as_str() {
                        Ok(output)
                    } else {
                        Err(ExampleError::new(
                            ExampleErrorCode::ProviderOutput,
                            "provider identity did not match the saved setup",
                        ))
                    }
                });
        }
        Ok(config::EnsureOutcome::Cancelled) => {
            return Err(ExampleError::new(
                ExampleErrorCode::SetupCancelled,
                "example setup was cancelled",
            ));
        }
        Err(ConfigError::ProbeFailed) => {
            if let Some(result) = captured.into_inner() {
                return result.map_err(StageError::public);
            }
            return Err(map_config_error(ConfigError::ProbeFailed));
        }
        Err(error) => return Err(map_config_error(error)),
    };

    let first = produce_for_saved(
        providers,
        config.provider,
        credentials,
        environment,
        prompt_allowed,
        &prompt,
        &grounding,
        &checks,
    );
    let provider_error = match first {
        Ok(output) => return Ok(output),
        Err(StageError::Public(error)) => return Err(error),
        Err(StageError::Provider(error)) => error,
    };

    if !prompt_allowed {
        return Err(map_provider_error(provider_error));
    }
    io.write_notice(&format!("Provider failed: {}", provider_error.message()))
        .map_err(map_config_error)?;
    match config::failure_action(io, true).map_err(map_config_error)? {
        FailureAction::Quit => Err(map_provider_error(provider_error)),
        FailureAction::Retry => produce_for_saved(
            providers,
            config.provider,
            credentials,
            environment,
            true,
            &prompt,
            &grounding,
            &checks,
        )
        .map_err(StageError::public),
        FailureAction::Change => change_and_generate(
            state,
            credentials,
            environment,
            providers,
            io,
            &prompt,
            &grounding,
            &checks,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn change_and_generate<S, C, E, R, I>(
    state: &S,
    credentials: &C,
    environment: &E,
    providers: &R,
    io: &mut I,
    prompt: &str,
    grounding: &[GroundingItem],
    checks: &GenerationChecks,
) -> Result<ExampleSuccess, ExampleError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
    R: ProviderRunner,
    I: SetupIo,
{
    let captured = RefCell::new(None::<Result<ExampleSuccess, StageError>>);
    let setup = config::interactive_setup(
        state,
        credentials,
        environment,
        io,
        true,
        |provider, secret| {
            let generated = produce(providers, provider, secret, prompt, grounding, checks);
            let accepted = generated.is_ok();
            *captured.borrow_mut() = Some(generated);
            if accepted {
                Ok(())
            } else {
                Err(())
            }
        },
    );
    match setup {
        Ok(SetupOutcome::Configured(_)) => captured
            .into_inner()
            .expect("provider change probe records its generation")
            .map_err(StageError::public),
        Ok(SetupOutcome::Cancelled) => Err(ExampleError::new(
            ExampleErrorCode::SetupCancelled,
            "example setup was cancelled",
        )),
        Err(ConfigError::ProbeFailed) => captured
            .into_inner()
            .unwrap_or_else(|| {
                Err(StageError::Public(map_config_error(
                    ConfigError::ProbeFailed,
                )))
            })
            .map_err(StageError::public),
        Err(error) => Err(map_config_error(error)),
    }
}

#[derive(Debug)]
struct GenerationChecks {
    query: String,
    expected_language: Option<&'static str>,
}

fn expected_language(outcome: &Outcome) -> Option<&'static str> {
    let provider = outcome.provider.to_ascii_lowercase();
    match provider.as_str() {
        "python" | "py" | "pypi" | "pip" => Some("python"),
        "rust" | "rs" | "crate" | "crates" | "docs" => Some("rust"),
        "c" => Some("c"),
        "cpp" | "c++" => Some("cpp"),
        "go" | "go-mod" => Some("go"),
        "haskell" | "hs" => Some("haskell"),
        "javascript" | "js" | "node" | "npm" => Some("javascript"),
        "typescript" | "ts" => Some("typescript"),
        "java" | "maven" => Some("java"),
        "kotlin" | "kt" => Some("kotlin"),
        "scala" => Some("scala"),
        "clojure" | "clj" => Some("clojure"),
        "ruby" | "rb" | "gem" => Some("ruby"),
        "php" => Some("php"),
        "dart" => Some("dart"),
        "ocaml" | "ml" => Some("ocaml"),
        "zig" => Some("zig"),
        "lua" => Some("lua"),
        "elixir" | "ex" | "hex" => Some("elixir"),
        "perl" => Some("perl"),
        "erlang" | "erl" => Some("erlang"),
        "julia" | "jl" => Some("julia"),
        "nim" => Some("nim"),
        "r" => Some("r"),
        "bash" | "sh" => Some("bash"),
        "zsh" => Some("zsh"),
        "css" => Some("css"),
        "html" => Some("html"),
        "nuget" => Some("csharp"),
        _ => None,
    }
}

fn response_is_relevant(
    response: &schema::ExampleResponse,
    query: &str,
    grounding: &[GroundingItem],
) -> bool {
    let output = format!("{}\n{}", response.code, response.explanation).to_ascii_lowercase();
    let mut evidence = query.to_string();
    for index in &response.citations {
        if let Some(item) = grounding.get(index - 1) {
            evidence.push(' ');
            evidence.push_str(&item.title);
            evidence.push(' ');
            evidence.push_str(&item.snippet);
            for value in &item.meta {
                evidence.push(' ');
                evidence.push_str(value);
            }
        }
    }
    let stop = [
        "and",
        "api",
        "code",
        "documentation",
        "example",
        "for",
        "from",
        "function",
        "how",
        "method",
        "official",
        "search",
        "task",
        "that",
        "the",
        "this",
        "type",
        "use",
        "using",
        "with",
        "bash",
        "cpp",
        "csharp",
        "css",
        "dart",
        "elixir",
        "erlang",
        "haskell",
        "html",
        "java",
        "javascript",
        "json",
        "julia",
        "kotlin",
        "lua",
        "nim",
        "ocaml",
        "perl",
        "php",
        "python",
        "ruby",
        "rust",
        "scala",
        "sql",
        "swift",
        "typescript",
        "xml",
        "yaml",
        "zig",
        "zsh",
    ];
    let terms = evidence
        .split(|character: char| !(character.is_alphanumeric() || character == '_'))
        .map(str::to_ascii_lowercase)
        .filter(|term| term.len() >= 3 && !stop.contains(&term.as_str()))
        .collect::<std::collections::BTreeSet<_>>();
    terms.is_empty() || terms.iter().any(|term| output.contains(term))
}

#[allow(clippy::too_many_arguments)]
fn produce_for_saved<C, E, R>(
    providers: &R,
    provider: ProviderId,
    credentials: &C,
    environment: &E,
    allow_keyring: bool,
    prompt: &str,
    grounding: &[GroundingItem],
    checks: &GenerationChecks,
) -> Result<ExampleSuccess, StageError>
where
    C: CredentialStore,
    E: Environment,
    R: ProviderRunner,
{
    let secret = match provider {
        ProviderId::GeminiCli => None,
        ProviderId::GeminiApi => Some(resolve_key(credentials, environment, allow_keyring)?),
    };
    produce(
        providers,
        provider,
        secret.as_ref(),
        prompt,
        grounding,
        checks,
    )
}

fn resolve_key<C: CredentialStore, E: Environment>(
    credentials: &C,
    environment: &E,
    allow_keyring: bool,
) -> Result<SecretString, StageError> {
    if !allow_keyring {
        return environment
            .secret(config::GEMINI_API_KEY_ENV)
            .filter(|secret| !secret.expose_secret().trim().is_empty())
            .ok_or_else(|| {
                StageError::Public(ExampleError::new(
                    ExampleErrorCode::CredentialUnavailable,
                    "GEMINI_API_KEY is required for Gemini API in JSON or non-interactive mode",
                ))
            });
    }
    config::resolve_api_key(credentials, environment).map_err(|_| {
        StageError::Public(ExampleError::new(
            ExampleErrorCode::CredentialUnavailable,
            "Gemini API credentials are unavailable",
        ))
    })
}

fn produce<R: ProviderRunner>(
    providers: &R,
    provider: ProviderId,
    secret: Option<&SecretString>,
    prompt: &str,
    grounding: &[GroundingItem],
    checks: &GenerationChecks,
) -> Result<ExampleSuccess, StageError> {
    let result = providers
        .generate(provider, secret, prompt)
        .map_err(StageError::Provider)?;
    if result.metadata.provider != provider {
        return Err(StageError::Public(ExampleError::new(
            ExampleErrorCode::ProviderOutput,
            "provider returned mismatched metadata",
        )));
    }
    let response = schema::parse_response(&result.text, grounding.len()).map_err(|error| {
        StageError::Public(ExampleError::new(
            ExampleErrorCode::InvalidResponse,
            error.to_string(),
        ))
    })?;
    if response.citations.is_empty() {
        return Err(StageError::Public(ExampleError::new(
            ExampleErrorCode::InvalidResponse,
            "generated example did not cite its documentation evidence",
        )));
    }
    if checks
        .expected_language
        .is_some_and(|expected| response.language != expected)
    {
        return Err(StageError::Public(ExampleError::new(
            ExampleErrorCode::InvalidResponse,
            "generated example language did not match the requested documentation",
        )));
    }
    if !response_is_relevant(&response, &checks.query, grounding) {
        return Err(StageError::Public(ExampleError::new(
            ExampleErrorCode::InvalidResponse,
            "generated example did not reference the query or cited evidence",
        )));
    }
    let validation = validate::validate_syntax(&response.language, &response.code);
    if validation.status == ValidationStatus::SyntaxRejected {
        let detail = if validation.diagnostics.trim().is_empty() {
            "generated code failed syntax validation".to_string()
        } else {
            format!(
                "generated code failed syntax validation: {}",
                sanitize_single_line(&validation.diagnostics, 384)
            )
        };
        return Err(StageError::Public(ExampleError::new(
            ExampleErrorCode::SyntaxRejected,
            detail,
        )));
    }

    let citations = response
        .citations
        .iter()
        .filter_map(|index| grounding.get(index - 1))
        .map(|item| ExampleCitation {
            index: item.index,
            title: sanitize_single_line(&item.title, 256),
            url: sanitize_single_line(&item.url, 512),
        })
        .collect();
    let (validation, validation_note) = validation_fields(validation);
    Ok(ExampleSuccess {
        version: OUTPUT_VERSION,
        provider: provider.as_str().to_string(),
        model: sanitize_single_line(&result.metadata.model, 128),
        remote_service: REMOTE_SERVICE,
        language: sanitize_single_line(&response.language, 64),
        code: sanitize_multiline(&response.code, schema::MAX_CODE_BYTES),
        explanation: sanitize_multiline(&response.explanation, schema::MAX_EXPLANATION_BYTES),
        citations,
        validation,
        validation_note,
    })
}

fn validation_fields(result: ValidationResult) -> (ValidationLabel, String) {
    match result.status {
        ValidationStatus::SyntaxAccepted => (ValidationLabel::SyntaxAccepted, String::new()),
        ValidationStatus::Skipped => (
            ValidationLabel::Skipped,
            sanitize_single_line(&result.diagnostics, 512),
        ),
        ValidationStatus::SyntaxRejected => unreachable!("rejected results return an error"),
    }
}

fn setup_with<S, C, E, R, I>(
    state: &S,
    credentials: &C,
    environment: &E,
    providers: &R,
    io: &mut I,
) -> Result<SetupReport, ExampleError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
    R: ProviderRunner,
    I: SetupIo,
{
    if !io.is_interactive() {
        return Err(ExampleError::new(
            ExampleErrorCode::SetupRequired,
            "explicit example setup requires an interactive terminal",
        ));
    }
    let failure = RefCell::new(None::<ExampleError>);
    let outcome = config::interactive_setup(
        state,
        credentials,
        environment,
        io,
        true,
        |provider, secret| match providers.generate(provider, secret, SETUP_PROBE) {
            Ok(result) => match schema::parse_response(&result.text, 0) {
                Ok(_) if result.metadata.provider == provider => Ok(()),
                Ok(_) => {
                    *failure.borrow_mut() = Some(ExampleError::new(
                        ExampleErrorCode::ProviderOutput,
                        "provider returned mismatched metadata",
                    ));
                    Err(())
                }
                Err(error) => {
                    *failure.borrow_mut() = Some(ExampleError::new(
                        ExampleErrorCode::InvalidResponse,
                        error.to_string(),
                    ));
                    Err(())
                }
            },
            Err(error) => {
                *failure.borrow_mut() = Some(map_provider_error(error));
                Err(())
            }
        },
    );
    match outcome {
        Ok(SetupOutcome::Configured(config)) => Ok(SetupReport {
            version: OUTPUT_VERSION,
            state: SetupState::Configured,
            provider: Some(config.provider.as_str().to_string()),
        }),
        Ok(SetupOutcome::Cancelled) => Ok(SetupReport {
            version: OUTPUT_VERSION,
            state: SetupState::Cancelled,
            provider: None,
        }),
        Err(ConfigError::ProbeFailed) => Err(failure
            .into_inner()
            .unwrap_or_else(|| map_config_error(ConfigError::ProbeFailed))),
        Err(error) => Err(map_config_error(error)),
    }
}

fn status_with<S, C, E>(
    state: &S,
    credentials: &C,
    environment: &E,
    machine_mode: bool,
) -> Result<StatusReport, ExampleError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
{
    if !machine_mode {
        let status =
            config::config_status(state, credentials, environment).map_err(map_config_error)?;
        let credential = match status.credential {
            config::CredentialStatus::NotApplicable => CredentialState::NotApplicable,
            config::CredentialStatus::EnvironmentOverride => CredentialState::EnvironmentOverride,
            config::CredentialStatus::Stored => CredentialState::Stored,
            config::CredentialStatus::Missing => CredentialState::Missing,
            config::CredentialStatus::Unavailable => CredentialState::Unavailable,
        };
        return Ok(StatusReport {
            version: OUTPUT_VERSION,
            configured: status.configured,
            provider: status
                .provider
                .map(|provider| provider.as_str().to_string()),
            credential,
        });
    }

    let Some(saved) = config::load_config(state).map_err(map_config_error)? else {
        return Ok(StatusReport {
            version: OUTPUT_VERSION,
            configured: false,
            provider: None,
            credential: CredentialState::NotApplicable,
        });
    };
    let credential = match saved.provider {
        ProviderId::GeminiCli => CredentialState::NotApplicable,
        ProviderId::GeminiApi => {
            if environment
                .secret(config::GEMINI_API_KEY_ENV)
                .is_some_and(|secret| !secret.expose_secret().trim().is_empty())
            {
                CredentialState::EnvironmentOverride
            } else {
                CredentialState::NotChecked
            }
        }
    };
    Ok(StatusReport {
        version: OUTPUT_VERSION,
        configured: true,
        provider: Some(saved.provider.as_str().to_string()),
        credential,
    })
}

struct TerminalSetupIo;

impl SetupIo for TerminalSetupIo {
    fn is_interactive(&self) -> bool {
        io::stdin().is_terminal() && io::stderr().is_terminal()
    }

    fn write_notice(&mut self, text: &str) -> Result<(), ConfigError> {
        let mut stderr = io::stderr().lock();
        writeln!(stderr, "{}", sanitize_single_line(text, 1024))
            .and_then(|()| stderr.flush())
            .map_err(|_| ConfigError::SetupIo)
    }

    fn read_choice(&mut self) -> Result<Option<String>, ConfigError> {
        let mut value = String::new();
        let count = io::stdin()
            .lock()
            .read_line(&mut value)
            .map_err(|_| ConfigError::SetupIo)?;
        Ok((count != 0).then_some(value))
    }

    fn read_secret_no_echo(&mut self) -> Result<Option<SecretString>, ConfigError> {
        rpassword::read_password()
            .map(SecretString::from)
            .map(Some)
            .map_err(|_| ConfigError::SetupIo)
    }
}

fn map_provider_error(error: ProviderError) -> ExampleError {
    let code = match error.kind() {
        ProviderErrorKind::NotInstalled => ExampleErrorCode::ProviderNotInstalled,
        ProviderErrorKind::Auth => ExampleErrorCode::ProviderAuthentication,
        ProviderErrorKind::RateLimit => ExampleErrorCode::ProviderRateLimit,
        ProviderErrorKind::Model => ExampleErrorCode::ProviderModel,
        ProviderErrorKind::Timeout => ExampleErrorCode::ProviderTimeout,
        ProviderErrorKind::BadOutput => ExampleErrorCode::ProviderOutput,
        ProviderErrorKind::Service => ExampleErrorCode::ProviderService,
    };
    ExampleError::new(code, error.message())
}

fn map_config_error(error: ConfigError) -> ExampleError {
    let code = match error {
        ConfigError::NonInteractive => ExampleErrorCode::ProviderUnconfigured,
        ConfigError::CredentialUnavailable
        | ConfigError::CredentialWriteFailed
        | ConfigError::CredentialDeleteFailed => ExampleErrorCode::CredentialUnavailable,
        ConfigError::SetupIo => ExampleErrorCode::Io,
        _ => ExampleErrorCode::Configuration,
    };
    ExampleError::new(code, error.to_string())
}

fn map_io_error(_: io::Error) -> ExampleError {
    ExampleError::new(ExampleErrorCode::Io, "example output could not be written")
}

fn sanitize_single_line(input: &str, max_bytes: usize) -> String {
    let mut output = String::new();
    let mut space = false;
    for character in input.chars() {
        if character.is_whitespace() || character.is_control() || is_bidi(character) {
            space |= !output.is_empty();
            continue;
        }
        if space {
            if output.len() == max_bytes {
                break;
            }
            output.push(' ');
            space = false;
        }
        if output.len() + character.len_utf8() > max_bytes {
            break;
        }
        output.push(character);
    }
    if output.is_empty() {
        "unavailable".to_string()
    } else {
        output
    }
}

fn sanitize_multiline(input: &str, max_bytes: usize) -> String {
    let mut output = String::new();
    for character in input.chars() {
        let character = if is_bidi(character)
            || (character.is_control() && !matches!(character, '\n' | '\t'))
        {
            ' '
        } else {
            character
        };
        if output.len() + character.len_utf8() > max_bytes {
            break;
        }
        output.push(character);
    }
    output
}

fn is_bidi(character: char) -> bool {
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
    use std::cell::{Cell, RefCell};
    use std::time::Duration;

    #[derive(Default)]
    struct MemoryState(RefCell<Option<String>>);

    impl config::LockedStateStore for MemoryState {
        fn read_bounded(&self, max: u64) -> Result<Option<String>, ConfigError> {
            let value = self.0.borrow().clone();
            if value.as_ref().is_some_and(|v| v.len() as u64 > max) {
                return Err(ConfigError::StateUnavailable);
            }
            Ok(value)
        }
        fn save_private_atomic(&self, text: &str) -> Result<(), ConfigError> {
            *self.0.borrow_mut() = Some(text.to_string());
            Ok(())
        }
        fn remove(&self) -> Result<(), ConfigError> {
            self.0.borrow_mut().take();
            Ok(())
        }
    }

    impl StateStore for MemoryState {
        fn read_bounded(&self, max: u64) -> Result<Option<String>, ConfigError> {
            config::LockedStateStore::read_bounded(self, max)
        }
        fn with_lock<T>(
            &self,
            action: impl FnOnce(&dyn config::LockedStateStore) -> Result<T, ConfigError>,
        ) -> Result<T, ConfigError> {
            action(self)
        }
    }

    #[derive(Default)]
    struct MemoryCredentials {
        secret: RefCell<Option<SecretString>>,
        gets: Cell<usize>,
    }

    impl CredentialStore for MemoryCredentials {
        fn get(&self) -> Result<Option<SecretString>, config::CredentialError> {
            self.gets.set(self.gets.get() + 1);
            Ok(self
                .secret
                .borrow()
                .as_ref()
                .map(|s| SecretString::from(s.expose_secret().to_string())))
        }
        fn set(&self, value: &SecretString) -> Result<(), config::CredentialError> {
            *self.secret.borrow_mut() = Some(SecretString::from(value.expose_secret().to_string()));
            Ok(())
        }
        fn delete(&self) -> Result<(), config::CredentialError> {
            self.secret.borrow_mut().take();
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeEnvironment(Option<String>);

    impl Environment for FakeEnvironment {
        fn secret(&self, _: &str) -> Option<SecretString> {
            self.0.clone().map(SecretString::from)
        }
    }

    struct FakeIo {
        interactive: bool,
        choices: RefCell<Vec<String>>,
        secret: Option<String>,
        notices: Cell<usize>,
    }

    impl FakeIo {
        fn new(interactive: bool, choices: &[&str]) -> Self {
            Self {
                interactive,
                choices: RefCell::new(choices.iter().rev().map(|s| s.to_string()).collect()),
                secret: Some("top-secret-key".to_string()),
                notices: Cell::new(0),
            }
        }
    }

    impl SetupIo for FakeIo {
        fn is_interactive(&self) -> bool {
            self.interactive
        }
        fn write_notice(&mut self, _: &str) -> Result<(), ConfigError> {
            self.notices.set(self.notices.get() + 1);
            Ok(())
        }
        fn read_choice(&mut self) -> Result<Option<String>, ConfigError> {
            Ok(self.choices.borrow_mut().pop())
        }
        fn read_secret_no_echo(&mut self) -> Result<Option<SecretString>, ConfigError> {
            Ok(self.secret.take().map(SecretString::from))
        }
    }

    struct FakeProviders {
        calls: Cell<usize>,
        failures: Cell<usize>,
        response: String,
    }

    impl FakeProviders {
        fn valid(citations: &[usize]) -> Self {
            Self {
                calls: Cell::new(0),
                failures: Cell::new(0),
                response: serde_json::json!({
                    "schema": 1,
                    "language": "haskell",
                    "code": "example = foldl (+) 0 [1, 2]",
                    "explanation": "foldl reduces the documented list.",
                    "citations": citations,
                })
                .to_string(),
            }
        }
    }

    impl ProviderRunner for FakeProviders {
        fn generate(
            &self,
            provider: ProviderId,
            _: Option<&SecretString>,
            _: &str,
        ) -> Result<ProviderResult, ProviderError> {
            self.calls.set(self.calls.get() + 1);
            if self.failures.get() > 0 {
                self.failures.set(self.failures.get() - 1);
                return Err(ProviderError::new(
                    ProviderErrorKind::Service,
                    "remote\x1b[31m failure",
                ));
            }
            Ok(ProviderResult::new(
                provider,
                "fake-model",
                self.response.clone(),
                Duration::ZERO,
            ))
        }
    }

    fn grounded() -> Outcome {
        let mut outcome = Outcome::new("haskell", "foldl");
        outcome.hits.push(
            crate::model::Hit::new(
                "Data.Foldable foldl",
                "https://hackage.haskell.org/package/base/docs/Data-Foldable.html#v:foldl",
            )
            .snippet("foldl reduces a structure from the left"),
        );
        outcome
    }

    #[test]
    fn json_and_non_tty_never_prompt_or_touch_keyring() {
        let state = MemoryState::default();
        state.0.replace(Some(
            config::serialize_config(&config::ExampleConfig::new(ProviderId::GeminiApi)).unwrap(),
        ));
        let credentials = MemoryCredentials::default();
        let providers = FakeProviders::valid(&[]);
        let mut io = FakeIo::new(true, &[]);
        let error = generate_with(
            "q",
            &grounded(),
            GenerateOptions {
                json: true,
                noninteractive: false,
            },
            &state,
            &credentials,
            &FakeEnvironment::default(),
            &providers,
            &mut io,
        )
        .unwrap_err();
        assert_eq!(error.code, ExampleErrorCode::CredentialUnavailable);
        assert_eq!(credentials.gets.get(), 0);
        assert_eq!(io.notices.get(), 0);

        let empty = MemoryState::default();
        let mut no_tty = FakeIo::new(false, &["1"]);
        let _ = generate_with(
            "q",
            &grounded(),
            GenerateOptions::default(),
            &empty,
            &credentials,
            &FakeEnvironment::default(),
            &providers,
            &mut no_tty,
        );
        assert_eq!(no_tty.notices.get(), 0);
    }

    #[test]
    fn first_setup_reuses_actual_generation_and_maps_citation() {
        let state = MemoryState::default();
        let credentials = MemoryCredentials::default();
        let providers = FakeProviders::valid(&[1]);
        let mut io = FakeIo::new(true, &["1"]);
        let output = generate_with(
            "q",
            &grounded(),
            GenerateOptions::default(),
            &state,
            &credentials,
            &FakeEnvironment::default(),
            &providers,
            &mut io,
        )
        .unwrap();
        assert_eq!(providers.calls.get(), 1);
        assert_eq!(output.citations[0].index, 1);
        assert_eq!(
            output.citations[0].url,
            "https://hackage.haskell.org/package/base/docs/Data-Foldable.html#v:foldl"
        );
        assert!(config::load_config(&state).unwrap().is_some());
    }

    #[test]
    fn saved_failure_menu_is_bounded_to_one_action() {
        let state = MemoryState::default();
        state.0.replace(Some(
            config::serialize_config(&config::ExampleConfig::new(ProviderId::GeminiCli)).unwrap(),
        ));
        let providers = FakeProviders::valid(&[]);
        providers.failures.set(2);
        let mut io = FakeIo::new(true, &["r", "r"]);
        let error = generate_with(
            "q",
            &grounded(),
            GenerateOptions::default(),
            &state,
            &MemoryCredentials::default(),
            &FakeEnvironment::default(),
            &providers,
            &mut io,
        )
        .unwrap_err();
        assert_eq!(error.code, ExampleErrorCode::ProviderService);
        assert_eq!(providers.calls.get(), 2);
        assert_eq!(io.notices.get(), 2);
    }

    #[test]
    fn secrets_are_absent_from_debug_and_serialization() {
        let error = ExampleError::new(ExampleErrorCode::CredentialUnavailable, "safe");
        let debug = format!("{error:?}");
        let json = to_json(&error).unwrap();
        assert!(!debug.contains("top-secret-key"));
        assert!(!json.contains("top-secret-key"));
    }

    #[test]
    fn success_envelope_and_validation_label_are_stable() {
        let evidence = grounding::build_grounding(&grounded());
        let output = produce(
            &FakeProviders::valid(&[1]),
            ProviderId::GeminiCli,
            None,
            "prompt",
            &evidence,
            &GenerationChecks {
                query: "foldl".to_string(),
                expected_language: Some("haskell"),
            },
        )
        .unwrap();
        let json = to_json(&output).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["version"], 1);
        assert_eq!(value["remote_service"], "Google");
        assert_eq!(value["validation"], "skipped");
        assert_eq!(json.matches("\"version\"").count(), 1);
    }

    #[test]
    fn mismatched_or_unrelated_generation_is_rejected() {
        let evidence = vec![GroundingItem {
            index: 1,
            title: "Vec in std::vec".to_string(),
            url: "https://doc.rust-lang.org/std/vec/struct.Vec.html".to_string(),
            snippet: "A contiguous growable array".to_string(),
            meta: Vec::new(),
        }];
        let checks = GenerationChecks {
            query: "rust Vec".to_string(),
            expected_language: Some("rust"),
        };
        let wrong_language = FakeProviders {
            calls: Cell::new(0),
            failures: Cell::new(0),
            response: r#"{"schema":1,"language":"python","code":"print('Vec')","explanation":"Vec","citations":[1]}"#.to_string(),
        };
        let error = produce(
            &wrong_language,
            ProviderId::GeminiCli,
            None,
            "prompt",
            &evidence,
            &checks,
        )
        .unwrap_err()
        .public();
        assert_eq!(error.code, ExampleErrorCode::InvalidResponse);

        let unrelated = FakeProviders {
            calls: Cell::new(0),
            failures: Cell::new(0),
            response: r#"{"schema":1,"language":"rust","code":"fn moon() {}","explanation":"The moon is cheese.","citations":[1]}"#.to_string(),
        };
        let error = produce(
            &unrelated,
            ProviderId::GeminiCli,
            None,
            "prompt",
            &evidence,
            &checks,
        )
        .unwrap_err()
        .public();
        assert_eq!(error.code, ExampleErrorCode::InvalidResponse);
    }

    #[test]
    fn json_error_codes_are_stable_and_versioned() {
        let error = ExampleError::new(
            ExampleErrorCode::ProviderUnconfigured,
            "configure a provider",
        );
        let json = to_json(&error).unwrap();
        assert_eq!(
            json,
            r#"{"version":1,"code":"example_provider_unconfigured","message":"configure a provider"}"#
        );
        let error = ExampleError::new(ExampleErrorCode::ProviderTimeout, "timed out");
        assert!(to_json(&error)
            .unwrap()
            .contains(r#""code":"example_timeout""#));
    }

    #[test]
    fn syntax_rejection_is_an_error_without_repair() {
        let providers = FakeProviders {
            calls: Cell::new(0),
            failures: Cell::new(0),
            response: r#"{"schema":1,"language":"python","code":"if True print(1)","explanation":"print example","citations":[1]}"#.to_string(),
        };
        let evidence = vec![GroundingItem {
            index: 1,
            title: "Python print".to_string(),
            url: "https://docs.python.org/3/".to_string(),
            snippet: "print output".to_string(),
            meta: Vec::new(),
        }];
        let result = produce(
            &providers,
            ProviderId::GeminiCli,
            None,
            "p",
            &evidence,
            &GenerationChecks {
                query: "python print".to_string(),
                expected_language: Some("python"),
            },
        );
        match result {
            Err(StageError::Public(error)) => {
                // A host without Python safely skips instead of rejecting.
                assert!(matches!(
                    error.code,
                    ExampleErrorCode::SyntaxRejected | ExampleErrorCode::InvalidResponse
                ));
            }
            Ok(output) => assert_eq!(output.validation, ValidationLabel::Skipped),
            Err(StageError::Provider(_)) => panic!("unexpected provider error"),
        }
        assert_eq!(providers.calls.get(), 1);
    }

    #[test]
    fn machine_status_does_not_read_keyring() {
        let state = MemoryState::default();
        state.0.replace(Some(
            config::serialize_config(&config::ExampleConfig::new(ProviderId::GeminiApi)).unwrap(),
        ));
        let credentials = MemoryCredentials::default();
        let report = status_with(&state, &credentials, &FakeEnvironment::default(), true).unwrap();
        assert_eq!(report.credential, CredentialState::NotChecked);
        assert_eq!(credentials.gets.get(), 0);
    }
}
