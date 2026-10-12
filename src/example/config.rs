//! Provider preference, credential handling, and interactive setup state.
//!
//! The on-disk file contains only `version` and `provider`. API keys are held
//! in the process environment or the operating-system credential store.

use std::fmt;

use secrecy::{ExposeSecret, SecretString};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::provider::ProviderId;

pub(crate) const CONFIG_VERSION: u32 = 1;
pub(crate) const CONFIG_LEAF: &str = "example.json";
pub(crate) const MAX_CONFIG_BYTES: u64 = 4 * 1024;
pub(crate) const KEYRING_SERVICE: &str = "io.github.pragundamani.quicksearch";
pub(crate) const KEYRING_ACCOUNT: &str = "example/gemini-api";
pub(crate) const GEMINI_API_KEY_ENV: &str = "GEMINI_API_KEY";

impl Serialize for ProviderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ProviderId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProviderVisitor;

        impl Visitor<'_> for ProviderVisitor {
            type Value = ProviderId;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("gemini-cli or gemini-api")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "gemini-cli" => Ok(ProviderId::GeminiCli),
                    "gemini-api" => Ok(ProviderId::GeminiApi),
                    _ => Err(E::unknown_variant(value, &["gemini-cli", "gemini-api"])),
                }
            }
        }

        deserializer.deserialize_str(ProviderVisitor)
    }
}

/// V1's complete non-secret configuration schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExampleConfig {
    pub version: u32,
    pub provider: ProviderId,
}

impl ExampleConfig {
    pub(crate) const fn new(provider: ProviderId) -> Self {
        Self {
            version: CONFIG_VERSION,
            provider,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ConfigError {
    StateUnavailable,
    InvalidConfig,
    UnsupportedVersion,
    CredentialUnavailable,
    CredentialWriteFailed,
    CredentialDeleteFailed,
    RollbackFailed,
    ProbeFailed,
    NonInteractive,
    InvalidChoice,
    EmptyApiKey,
    SetupIo,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::StateUnavailable => "example settings are unavailable",
            Self::InvalidConfig => "example settings are invalid",
            Self::UnsupportedVersion => "example settings use an unsupported version",
            Self::CredentialUnavailable => "the operating-system credential store is unavailable",
            Self::CredentialWriteFailed => "the API credential could not be stored securely",
            Self::CredentialDeleteFailed => "the API credential could not be removed",
            Self::RollbackFailed => "the previous example setup could not be restored",
            Self::ProbeFailed => "the selected example provider failed its setup test",
            Self::NonInteractive => "interactive example setup is not available",
            Self::InvalidChoice => "invalid example setup choice",
            Self::EmptyApiKey => "the API key must not be empty",
            Self::SetupIo => "interactive example setup failed",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ConfigError {}

/// The minimum locked-file interface required by this module.
pub(crate) trait LockedStateStore {
    fn read_bounded(&self, max_bytes: u64) -> Result<Option<String>, ConfigError>;
    fn save_private_atomic(&self, text: &str) -> Result<(), ConfigError>;
    fn remove(&self) -> Result<(), ConfigError>;
}

/// A test seam over `crate::state`. Implementations must serialize writers.
pub(crate) trait StateStore {
    fn read_bounded(&self, max_bytes: u64) -> Result<Option<String>, ConfigError>;

    fn with_lock<T>(
        &self,
        action: impl FnOnce(&dyn LockedStateStore) -> Result<T, ConfigError>,
    ) -> Result<T, ConfigError>;
}

impl LockedStateStore for crate::state::LockedStateFile<'_> {
    fn read_bounded(&self, max_bytes: u64) -> Result<Option<String>, ConfigError> {
        self.read_text(max_bytes)
            .map_err(|_| ConfigError::StateUnavailable)
    }

    fn save_private_atomic(&self, text: &str) -> Result<(), ConfigError> {
        self.write_text_atomic(text)
            .map_err(|_| ConfigError::StateUnavailable)
    }

    fn remove(&self) -> Result<(), ConfigError> {
        crate::state::LockedStateFile::remove(self).map_err(|_| ConfigError::StateUnavailable)
    }
}

impl StateStore for crate::state::StateDir {
    fn read_bounded(&self, max_bytes: u64) -> Result<Option<String>, ConfigError> {
        self.read_text(CONFIG_LEAF, max_bytes)
            .map_err(|_| ConfigError::StateUnavailable)
    }

    fn with_lock<T>(
        &self,
        action: impl FnOnce(&dyn LockedStateStore) -> Result<T, ConfigError>,
    ) -> Result<T, ConfigError> {
        let mut outcome = None;
        crate::state::StateDir::with_lock(self, CONFIG_LEAF, |file| {
            outcome = Some(action(file));
            Ok(())
        })
        .map_err(|_| ConfigError::StateUnavailable)?;
        outcome.expect("state lock callback always runs")
    }
}

pub(crate) fn parse_config(text: &str) -> Result<ExampleConfig, ConfigError> {
    let config: ExampleConfig =
        serde_json::from_str(text).map_err(|_| ConfigError::InvalidConfig)?;
    if config.version != CONFIG_VERSION {
        return Err(ConfigError::UnsupportedVersion);
    }
    Ok(config)
}

pub(crate) fn serialize_config(config: &ExampleConfig) -> Result<String, ConfigError> {
    serde_json::to_string(config).map_err(|_| ConfigError::InvalidConfig)
}

pub(crate) fn load_config<S: StateStore>(state: &S) -> Result<Option<ExampleConfig>, ConfigError> {
    state
        .read_bounded(MAX_CONFIG_BYTES)?
        .as_deref()
        .map(parse_config)
        .transpose()
}

fn load_locked(state: &dyn LockedStateStore) -> Result<Option<ExampleConfig>, ConfigError> {
    state
        .read_bounded(MAX_CONFIG_BYTES)?
        .as_deref()
        .map(parse_config)
        .transpose()
}

fn save_locked(state: &dyn LockedStateStore, config: &ExampleConfig) -> Result<(), ConfigError> {
    state.save_private_atomic(&serialize_config(config)?)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CredentialError {
    Unavailable,
}

impl fmt::Display for CredentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("secure credential storage is unavailable")
    }
}

impl std::error::Error for CredentialError {}

/// Fixed-account credential access. Returned values remain secret wrappers.
pub(crate) trait CredentialStore {
    fn get(&self) -> Result<Option<SecretString>, CredentialError>;
    fn set(&self, secret: &SecretString) -> Result<(), CredentialError>;
    fn delete(&self) -> Result<(), CredentialError>;
}

/// Production OS-keyring adapter. It has no plaintext fallback.
#[derive(Clone, Copy, Default)]
pub(crate) struct OsCredentialStore;

impl OsCredentialStore {
    fn entry() -> Result<keyring::Entry, CredentialError> {
        keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT)
            .map_err(|_| CredentialError::Unavailable)
    }
}

impl CredentialStore for OsCredentialStore {
    fn get(&self) -> Result<Option<SecretString>, CredentialError> {
        match Self::entry()?.get_password() {
            Ok(value) => Ok(Some(SecretString::from(value))),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(CredentialError::Unavailable),
        }
    }

    fn set(&self, secret: &SecretString) -> Result<(), CredentialError> {
        Self::entry()?
            .set_password(secret.expose_secret())
            .map_err(|_| CredentialError::Unavailable)
    }

    fn delete(&self) -> Result<(), CredentialError> {
        match Self::entry()?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err(CredentialError::Unavailable),
        }
    }
}

pub(crate) trait Environment {
    fn secret(&self, name: &str) -> Option<SecretString>;
}

#[derive(Clone, Copy, Default)]
pub(crate) struct ProcessEnvironment;

impl Environment for ProcessEnvironment {
    fn secret(&self, name: &str) -> Option<SecretString> {
        std::env::var(name)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .map(SecretString::from)
    }
}

/// A proposed provider replacement. This type intentionally has no Debug,
/// Display, Serialize, or Clone implementation because it can own a secret.
pub(crate) enum ProviderReplacement {
    Cli,
    ApiPersistent(SecretString),
    ApiEnvironment(SecretString),
}

impl ProviderReplacement {
    fn provider(&self) -> ProviderId {
        match self {
            Self::Cli => ProviderId::GeminiCli,
            Self::ApiPersistent(_) | Self::ApiEnvironment(_) => ProviderId::GeminiApi,
        }
    }

    fn probe_secret(&self) -> Option<&SecretString> {
        match self {
            Self::Cli => None,
            Self::ApiPersistent(secret) | Self::ApiEnvironment(secret) => Some(secret),
        }
    }
}

fn restore_credential<C: CredentialStore>(
    credentials: &C,
    old: Option<&SecretString>,
) -> Result<(), ConfigError> {
    match old {
        Some(secret) => credentials.set(secret),
        None => credentials.delete(),
    }
    .map_err(|_| ConfigError::RollbackFailed)
}

/// Probe a candidate before committing its preference. Persistent API keys are
/// written only after the probe succeeds. A failed state write restores the
/// previous key while the old atomic config file remains in place.
pub(crate) fn replace_provider<S, C, P, E>(
    state: &S,
    credentials: &C,
    replacement: ProviderReplacement,
    probe: P,
) -> Result<ExampleConfig, ConfigError>
where
    S: StateStore,
    C: CredentialStore,
    P: FnOnce(ProviderId, Option<&SecretString>) -> Result<(), E>,
{
    if replacement
        .probe_secret()
        .is_some_and(|secret| secret.expose_secret().trim().is_empty())
    {
        return Err(ConfigError::EmptyApiKey);
    }

    let provider = replacement.provider();
    probe(provider, replacement.probe_secret()).map_err(|_| ConfigError::ProbeFailed)?;
    let new_config = ExampleConfig::new(provider);

    state.with_lock(|locked| {
        // Validate the current file while holding the same lock used to replace it.
        let old_config = load_locked(locked)?;
        match &replacement {
            ProviderReplacement::ApiPersistent(new_key) => {
                // Reading first makes locked/unavailable keyrings fail closed and
                // gives us an exact rollback value.
                let old_key = credentials
                    .get()
                    .map_err(|_| ConfigError::CredentialUnavailable)?;
                credentials
                    .set(new_key)
                    .map_err(|_| ConfigError::CredentialWriteFailed)?;
                if let Err(error) = save_locked(locked, &new_config) {
                    restore_credential(credentials, old_key.as_ref())?;
                    return Err(error);
                }
            }
            ProviderReplacement::ApiEnvironment(_) => {
                // An environment override is ephemeral. Keep any existing stored
                // API credential unchanged so one-run overrides cannot erase it.
                save_locked(locked, &new_config)?;
            }
            ProviderReplacement::Cli => {
                if old_config
                    .as_ref()
                    .is_some_and(|config| config.provider == ProviderId::GeminiApi)
                {
                    let old_key = credentials
                        .get()
                        .map_err(|_| ConfigError::CredentialUnavailable)?;
                    credentials
                        .delete()
                        .map_err(|_| ConfigError::CredentialDeleteFailed)?;
                    if let Err(error) = save_locked(locked, &new_config) {
                        restore_credential(credentials, old_key.as_ref())?;
                        return Err(error);
                    }
                } else {
                    save_locked(locked, &new_config)?;
                }
            }
        }
        Ok(new_config)
    })
}

pub(crate) fn resolve_api_key<C: CredentialStore, E: Environment>(
    credentials: &C,
    environment: &E,
) -> Result<SecretString, ConfigError> {
    if let Some(secret) = environment.secret(GEMINI_API_KEY_ENV) {
        if !secret.expose_secret().trim().is_empty() {
            return Ok(secret);
        }
    }
    credentials
        .get()
        .map_err(|_| ConfigError::CredentialUnavailable)?
        .filter(|secret| !secret.expose_secret().trim().is_empty())
        .ok_or(ConfigError::CredentialUnavailable)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CredentialStatus {
    NotApplicable,
    EnvironmentOverride,
    Stored,
    Missing,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConfigStatus {
    pub configured: bool,
    pub provider: Option<ProviderId>,
    pub credential: CredentialStatus,
}

pub(crate) fn config_status<S, C, E>(
    state: &S,
    credentials: &C,
    environment: &E,
) -> Result<ConfigStatus, ConfigError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
{
    let Some(config) = load_config(state)? else {
        return Ok(ConfigStatus {
            configured: false,
            provider: None,
            credential: CredentialStatus::NotApplicable,
        });
    };
    let credential = match config.provider {
        ProviderId::GeminiCli => CredentialStatus::NotApplicable,
        ProviderId::GeminiApi => {
            if environment
                .secret(GEMINI_API_KEY_ENV)
                .is_some_and(|secret| !secret.expose_secret().trim().is_empty())
            {
                CredentialStatus::EnvironmentOverride
            } else {
                match credentials.get() {
                    Ok(Some(_)) => CredentialStatus::Stored,
                    Ok(None) => CredentialStatus::Missing,
                    Err(_) => CredentialStatus::Unavailable,
                }
            }
        }
    };
    Ok(ConfigStatus {
        configured: true,
        provider: Some(config.provider),
        credential,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RemovalStatus {
    NotConfigured,
    Removed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CredentialCleanupStatus {
    Cleaned,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RemovalResult {
    pub status: RemovalStatus,
    pub credential_cleanup: CredentialCleanupStatus,
}

/// Remove the preference and, when applicable, its stored API key. The key is
/// restored if removing the state file fails.
pub(crate) fn remove_config<S: StateStore, C: CredentialStore>(
    state: &S,
    credentials: &C,
) -> Result<RemovalResult, ConfigError> {
    state.with_lock(|locked| {
        // Always attempt to clean the dedicated keyring account. This recovers
        // an orphan from an interrupted setup. An unavailable keyring must not
        // prevent removal of provider state; a later reset retries cleanup.
        let configured = !matches!(locked.read_bounded(MAX_CONFIG_BYTES), Ok(None));
        let (old_key, credential_cleanup) = match credentials.get() {
            Ok(old_key) => match credentials.delete() {
                Ok(()) => (Some(old_key), CredentialCleanupStatus::Cleaned),
                Err(_) => (None, CredentialCleanupStatus::Unavailable),
            },
            Err(_) => (None, CredentialCleanupStatus::Unavailable),
        };
        if configured {
            if let Err(error) = locked.remove() {
                if let Some(old_key) = old_key.as_ref() {
                    restore_credential(credentials, old_key.as_ref())?;
                }
                return Err(error);
            }
        }
        Ok(RemovalResult {
            status: if configured {
                RemovalStatus::Removed
            } else {
                RemovalStatus::NotConfigured
            },
            credential_cleanup,
        })
    })
}

/// Interactive setup is isolated behind this interface. Production key input
/// must call an implementation backed by `rpassword`, so terminal echo stays off.
pub(crate) trait SetupIo {
    fn is_interactive(&self) -> bool;
    fn write_notice(&mut self, text: &str) -> Result<(), ConfigError>;
    fn read_choice(&mut self) -> Result<Option<String>, ConfigError>;
    fn read_secret_no_echo(&mut self) -> Result<Option<SecretString>, ConfigError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SetupOutcome {
    Configured(ExampleConfig),
    Cancelled,
}

fn parse_provider_choice(value: &str) -> Result<Option<ProviderId>, ConfigError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "cli" | "gemini-cli" => Ok(Some(ProviderId::GeminiCli)),
        "2" | "api" | "gemini-api" => Ok(Some(ProviderId::GeminiApi)),
        "q" | "quit" | "cancel" => Ok(None),
        _ => Err(ConfigError::InvalidChoice),
    }
}

/// Run at most one setup choice cycle. `prompt_allowed` must be false for JSON
/// output and other non-prompting call paths.
pub(crate) fn interactive_setup<S, C, E, I, P, PE>(
    state: &S,
    credentials: &C,
    environment: &E,
    io: &mut I,
    prompt_allowed: bool,
    probe: P,
) -> Result<SetupOutcome, ConfigError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
    I: SetupIo,
    P: FnOnce(ProviderId, Option<&SecretString>) -> Result<(), PE>,
{
    if !prompt_allowed || !io.is_interactive() {
        return Err(ConfigError::NonInteractive);
    }
    io.write_notice(
        "Choose an example provider: [1] Gemini CLI or [2] Gemini API. Your query and grounding context will be sent to Google. [q] Quit.",
    )?;
    let Some(choice) = io.read_choice()? else {
        return Ok(SetupOutcome::Cancelled);
    };
    let Some(provider) = parse_provider_choice(&choice)? else {
        return Ok(SetupOutcome::Cancelled);
    };

    let replacement = match provider {
        ProviderId::GeminiCli => ProviderReplacement::Cli,
        ProviderId::GeminiApi => {
            if let Some(secret) = environment.secret(GEMINI_API_KEY_ENV) {
                ProviderReplacement::ApiEnvironment(secret)
            } else {
                io.write_notice("Enter the Gemini API key (input is hidden).")?;
                let Some(secret) = io.read_secret_no_echo()? else {
                    return Ok(SetupOutcome::Cancelled);
                };
                ProviderReplacement::ApiPersistent(secret)
            }
        }
    };
    replace_provider(state, credentials, replacement, probe).map(SetupOutcome::Configured)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EnsureOutcome {
    Saved(ExampleConfig),
    Configured(ExampleConfig),
    Cancelled,
}

pub(crate) fn ensure_config<S, C, E, I, P, PE>(
    state: &S,
    credentials: &C,
    environment: &E,
    io: &mut I,
    prompt_allowed: bool,
    probe: P,
) -> Result<EnsureOutcome, ConfigError>
where
    S: StateStore,
    C: CredentialStore,
    E: Environment,
    I: SetupIo,
    P: FnOnce(ProviderId, Option<&SecretString>) -> Result<(), PE>,
{
    if let Some(config) = load_config(state)? {
        return Ok(EnsureOutcome::Saved(config));
    }
    match interactive_setup(state, credentials, environment, io, prompt_allowed, probe)? {
        SetupOutcome::Configured(config) => Ok(EnsureOutcome::Configured(config)),
        SetupOutcome::Cancelled => Ok(EnsureOutcome::Cancelled),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FailureAction {
    Retry,
    Change,
    Quit,
}

/// Read one failure action. Invalid input is returned to the caller and is not
/// reprompted, which bounds this function to one cycle.
pub(crate) fn failure_action<I: SetupIo>(
    io: &mut I,
    prompt_allowed: bool,
) -> Result<FailureAction, ConfigError> {
    if !prompt_allowed || !io.is_interactive() {
        return Ok(FailureAction::Quit);
    }
    io.write_notice("Choose [r] Retry, [c] Change provider, or [q] Quit.")?;
    let Some(choice) = io.read_choice()? else {
        return Ok(FailureAction::Quit);
    };
    match choice.trim().to_ascii_lowercase().as_str() {
        "r" | "retry" => Ok(FailureAction::Retry),
        "c" | "change" => Ok(FailureAction::Change),
        "q" | "quit" => Ok(FailureAction::Quit),
        _ => Err(ConfigError::InvalidChoice),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Default)]
    struct FakeState {
        value: RefCell<Option<String>>,
        save_fails: Cell<bool>,
        remove_fails: Cell<bool>,
        locks: Cell<usize>,
    }

    impl LockedStateStore for FakeState {
        fn read_bounded(&self, max_bytes: u64) -> Result<Option<String>, ConfigError> {
            let value = self.value.borrow().clone();
            if value
                .as_ref()
                .is_some_and(|text| text.len() as u64 > max_bytes)
            {
                return Err(ConfigError::StateUnavailable);
            }
            Ok(value)
        }

        fn save_private_atomic(&self, text: &str) -> Result<(), ConfigError> {
            if self.save_fails.get() {
                return Err(ConfigError::StateUnavailable);
            }
            *self.value.borrow_mut() = Some(text.to_owned());
            Ok(())
        }

        fn remove(&self) -> Result<(), ConfigError> {
            if self.remove_fails.get() {
                return Err(ConfigError::StateUnavailable);
            }
            self.value.borrow_mut().take();
            Ok(())
        }
    }

    impl StateStore for FakeState {
        fn read_bounded(&self, max_bytes: u64) -> Result<Option<String>, ConfigError> {
            LockedStateStore::read_bounded(self, max_bytes)
        }

        fn with_lock<T>(
            &self,
            action: impl FnOnce(&dyn LockedStateStore) -> Result<T, ConfigError>,
        ) -> Result<T, ConfigError> {
            self.locks.set(self.locks.get() + 1);
            action(self)
        }
    }

    #[derive(Default)]
    struct FakeCredentials {
        value: RefCell<Option<SecretString>>,
        get_fails: Cell<bool>,
        set_fails: Cell<bool>,
        delete_fails: Cell<bool>,
    }

    impl CredentialStore for FakeCredentials {
        fn get(&self) -> Result<Option<SecretString>, CredentialError> {
            if self.get_fails.get() {
                return Err(CredentialError::Unavailable);
            }
            Ok(self.value.borrow().clone())
        }

        fn set(&self, secret: &SecretString) -> Result<(), CredentialError> {
            if self.set_fails.get() {
                return Err(CredentialError::Unavailable);
            }
            *self.value.borrow_mut() = Some(secret.clone());
            Ok(())
        }

        fn delete(&self) -> Result<(), CredentialError> {
            if self.delete_fails.get() {
                return Err(CredentialError::Unavailable);
            }
            self.value.borrow_mut().take();
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeEnvironment(RefCell<Option<SecretString>>);

    impl Environment for FakeEnvironment {
        fn secret(&self, name: &str) -> Option<SecretString> {
            assert_eq!(name, GEMINI_API_KEY_ENV);
            self.0.borrow().clone()
        }
    }

    struct FakeIo {
        interactive: bool,
        choice: Option<String>,
        secret: Option<SecretString>,
        reads: usize,
        secret_reads: usize,
        notices: Vec<String>,
    }

    impl FakeIo {
        fn with_choice(choice: Option<&str>) -> Self {
            Self {
                interactive: true,
                choice: choice.map(str::to_owned),
                secret: None,
                reads: 0,
                secret_reads: 0,
                notices: Vec::new(),
            }
        }
    }

    impl SetupIo for FakeIo {
        fn is_interactive(&self) -> bool {
            self.interactive
        }

        fn write_notice(&mut self, text: &str) -> Result<(), ConfigError> {
            self.notices.push(text.to_owned());
            Ok(())
        }

        fn read_choice(&mut self) -> Result<Option<String>, ConfigError> {
            self.reads += 1;
            Ok(self.choice.take())
        }

        fn read_secret_no_echo(&mut self) -> Result<Option<SecretString>, ConfigError> {
            self.secret_reads += 1;
            Ok(self.secret.take())
        }
    }

    fn saved(state: &FakeState, provider: ProviderId) {
        *state.value.borrow_mut() = Some(serialize_config(&ExampleConfig::new(provider)).unwrap());
    }

    #[test]
    fn schema_is_exact_and_rejects_unknown_fields() {
        let config = ExampleConfig::new(ProviderId::GeminiApi);
        assert_eq!(
            serialize_config(&config).unwrap(),
            r#"{"version":1,"provider":"gemini-api"}"#
        );
        assert_eq!(
            parse_config(r#"{"version":1,"provider":"gemini-cli"}"#)
                .unwrap()
                .provider,
            ProviderId::GeminiCli
        );
        assert_eq!(
            parse_config(r#"{"version":1,"provider":"gemini-cli","token":"x"}"#),
            Err(ConfigError::InvalidConfig)
        );
        assert_eq!(
            parse_config(r#"{"version":2,"provider":"gemini-cli"}"#),
            Err(ConfigError::UnsupportedVersion)
        );
    }

    #[test]
    fn first_use_configures_cli_and_discloses_google() {
        let state = FakeState::default();
        let credentials = FakeCredentials::default();
        let environment = FakeEnvironment::default();
        let mut io = FakeIo::with_choice(Some("1"));
        let result = ensure_config(
            &state,
            &credentials,
            &environment,
            &mut io,
            true,
            |provider, key| {
                assert_eq!(provider, ProviderId::GeminiCli);
                assert!(key.is_none());
                Ok::<_, ()>(())
            },
        )
        .unwrap();
        assert_eq!(
            result,
            EnsureOutcome::Configured(ExampleConfig::new(ProviderId::GeminiCli))
        );
        assert!(io.notices.join(" ").contains("sent to Google"));
        assert_eq!(state.locks.get(), 1);
    }

    #[test]
    fn cancellation_and_invalid_choice_do_not_write() {
        let state = FakeState::default();
        let credentials = FakeCredentials::default();
        let environment = FakeEnvironment::default();
        let mut cancelled = FakeIo::with_choice(Some("q"));
        let outcome = interactive_setup(
            &state,
            &credentials,
            &environment,
            &mut cancelled,
            true,
            |_, _| Ok::<_, ()>(()),
        )
        .unwrap();
        assert_eq!(outcome, SetupOutcome::Cancelled);
        assert!(state.value.borrow().is_none());

        let mut invalid = FakeIo::with_choice(Some("other"));
        assert_eq!(
            interactive_setup(
                &state,
                &credentials,
                &environment,
                &mut invalid,
                true,
                |_, _| Ok::<_, ()>(())
            ),
            Err(ConfigError::InvalidChoice)
        );
        assert_eq!(invalid.reads, 1);
        assert!(state.value.borrow().is_none());
    }

    #[test]
    fn noninteractive_path_does_not_perform_io() {
        let state = FakeState::default();
        let credentials = FakeCredentials::default();
        let environment = FakeEnvironment::default();
        let mut io = FakeIo::with_choice(Some("1"));
        assert_eq!(
            interactive_setup(
                &state,
                &credentials,
                &environment,
                &mut io,
                false,
                |_, _| Ok::<_, ()>(())
            ),
            Err(ConfigError::NonInteractive)
        );
        assert_eq!(io.reads, 0);
        assert!(io.notices.is_empty());
    }

    #[test]
    fn failed_probe_preserves_old_config_and_key() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiApi);
        let old_text = state.value.borrow().clone();
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("old-secret"));
        let result = replace_provider(
            &state,
            &credentials,
            ProviderReplacement::ApiPersistent(SecretString::from("new-secret")),
            |_, _| Err::<(), _>(()),
        );
        assert_eq!(result, Err(ConfigError::ProbeFailed));
        assert_eq!(*state.value.borrow(), old_text);
        assert_eq!(
            credentials.value.borrow().as_ref().unwrap().expose_secret(),
            "old-secret"
        );
        assert_eq!(state.locks.get(), 0);
    }

    #[test]
    fn failed_config_commit_rolls_back_key() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiApi);
        state.save_fails.set(true);
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("old-secret"));
        let result = replace_provider(
            &state,
            &credentials,
            ProviderReplacement::ApiPersistent(SecretString::from("new-secret")),
            |_, _| Ok::<_, ()>(()),
        );
        assert_eq!(result, Err(ConfigError::StateUnavailable));
        assert_eq!(
            credentials.value.borrow().as_ref().unwrap().expose_secret(),
            "old-secret"
        );
    }

    #[test]
    fn switching_from_api_to_cli_removes_the_stored_key() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiApi);
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("old-secret"));
        let configured =
            replace_provider(&state, &credentials, ProviderReplacement::Cli, |_, _| {
                Ok::<_, ()>(())
            })
            .unwrap();
        assert_eq!(configured.provider, ProviderId::GeminiCli);
        assert!(credentials.value.borrow().is_none());
    }

    #[test]
    fn failed_cli_switch_restores_the_stored_key() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiApi);
        state.save_fails.set(true);
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("old-secret"));
        let result = replace_provider(&state, &credentials, ProviderReplacement::Cli, |_, _| {
            Ok::<_, ()>(())
        });
        assert_eq!(result, Err(ConfigError::StateUnavailable));
        assert_eq!(
            credentials.value.borrow().as_ref().unwrap().expose_secret(),
            "old-secret"
        );
    }

    #[test]
    fn saved_preference_skips_prompt_and_probe() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiCli);
        let credentials = FakeCredentials::default();
        let environment = FakeEnvironment::default();
        let mut io = FakeIo::with_choice(Some("2"));
        let probed = Cell::new(false);
        let result = ensure_config(&state, &credentials, &environment, &mut io, true, |_, _| {
            probed.set(true);
            Ok::<_, ()>(())
        })
        .unwrap();
        assert_eq!(
            result,
            EnsureOutcome::Saved(ExampleConfig::new(ProviderId::GeminiCli))
        );
        assert!(!probed.get());
        assert_eq!(io.reads, 0);
    }

    #[test]
    fn environment_key_overrides_keyring_without_persistence() {
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("stored-secret"));
        let environment =
            FakeEnvironment(RefCell::new(Some(SecretString::from("environment-secret"))));
        let key = resolve_api_key(&credentials, &environment).unwrap();
        assert_eq!(key.expose_secret(), "environment-secret");

        let state = FakeState::default();
        let mut io = FakeIo::with_choice(Some("2"));
        interactive_setup(
            &state,
            &credentials,
            &environment,
            &mut io,
            true,
            |_, key| {
                assert_eq!(key.unwrap().expose_secret(), "environment-secret");
                Ok::<_, ()>(())
            },
        )
        .unwrap();
        assert_eq!(io.secret_reads, 0);
        assert_eq!(
            credentials.value.borrow().as_ref().unwrap().expose_secret(),
            "stored-secret"
        );
    }

    #[test]
    fn keyring_errors_fail_closed() {
        let state = FakeState::default();
        let credentials = FakeCredentials::default();
        credentials.get_fails.set(true);
        let result = replace_provider(
            &state,
            &credentials,
            ProviderReplacement::ApiPersistent(SecretString::from("secret")),
            |_, _| Ok::<_, ()>(()),
        );
        assert_eq!(result, Err(ConfigError::CredentialUnavailable));
        assert!(state.value.borrow().is_none());

        let environment = FakeEnvironment::default();
        assert!(matches!(
            resolve_api_key(&credentials, &environment),
            Err(ConfigError::CredentialUnavailable)
        ));
    }

    #[test]
    fn status_removal_and_errors_never_expose_secret() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiApi);
        let credentials = FakeCredentials::default();
        let marker = "do-not-leak-7ca9";
        *credentials.value.borrow_mut() = Some(SecretString::from(marker));
        let environment = FakeEnvironment::default();

        let status = config_status(&state, &credentials, &environment).unwrap();
        assert_eq!(status.credential, CredentialStatus::Stored);
        assert!(!format!("{status:?}").contains(marker));
        assert!(!state.value.borrow().as_ref().unwrap().contains(marker));
        for error in [
            ConfigError::CredentialUnavailable,
            ConfigError::ProbeFailed,
            ConfigError::RollbackFailed,
        ] {
            assert!(!format!("{error:?} {error}").contains(marker));
        }
        assert_eq!(
            remove_config(&state, &credentials).unwrap().status,
            RemovalStatus::Removed
        );
        assert!(credentials.value.borrow().is_none());
        assert!(state.value.borrow().is_none());
    }

    #[test]
    fn reset_cleans_an_orphaned_key_without_configuration() {
        let state = FakeState::default();
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("orphaned-secret"));
        assert_eq!(
            remove_config(&state, &credentials).unwrap().status,
            RemovalStatus::NotConfigured
        );
        assert!(credentials.value.borrow().is_none());
    }

    #[test]
    fn reset_removes_state_when_the_keyring_is_unavailable() {
        let state = FakeState::default();
        saved(&state, ProviderId::GeminiCli);
        let credentials = FakeCredentials::default();
        credentials.get_fails.set(true);
        let result = remove_config(&state, &credentials).unwrap();
        assert_eq!(result.status, RemovalStatus::Removed);
        assert_eq!(
            result.credential_cleanup,
            CredentialCleanupStatus::Unavailable
        );
        assert!(state.value.borrow().is_none());
    }

    #[test]
    fn reset_removes_corrupt_config_and_dedicated_key() {
        let state = FakeState::default();
        *state.value.borrow_mut() = Some("{not json".to_string());
        let credentials = FakeCredentials::default();
        *credentials.value.borrow_mut() = Some(SecretString::from("stale-secret"));
        assert_eq!(
            remove_config(&state, &credentials).unwrap().status,
            RemovalStatus::Removed
        );
        assert!(state.value.borrow().is_none());
        assert!(credentials.value.borrow().is_none());
    }

    #[test]
    fn failure_action_is_one_cycle_and_noninteractive_quits() {
        let mut io = FakeIo::with_choice(Some("retry"));
        assert_eq!(failure_action(&mut io, true).unwrap(), FailureAction::Retry);
        assert_eq!(io.reads, 1);

        let mut json_io = FakeIo::with_choice(Some("change"));
        assert_eq!(
            failure_action(&mut json_io, false).unwrap(),
            FailureAction::Quit
        );
        assert_eq!(json_io.reads, 0);
        assert!(json_io.notices.is_empty());
    }
}
