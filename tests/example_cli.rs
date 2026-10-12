use std::fs;
#[cfg(unix)]
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "qs-example-cli-{label}-{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_qs"));
        command
            .env("HOME", &self.0)
            .env("USERPROFILE", &self.0)
            .env("XDG_STATE_HOME", self.0.join("state"))
            .env("XDG_CACHE_HOME", self.0.join("cache"))
            .env("XDG_CONFIG_HOME", self.0.join("config"));
        command
    }

    fn state_dir(&self) -> PathBuf {
        self.0.join("state/quicksearch")
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn json(output: &Output) -> serde_json::Value {
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn status_is_one_json_value_and_does_not_create_configuration() {
    let root = TestRoot::new("status");
    let output = root
        .command()
        .args(["--json", "--example-status"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let value = json(&output);
    assert_eq!(value["version"], 1);
    assert_eq!(value["configured"], false);
    assert!(!root.state_dir().join("example.json").exists());
}

#[test]
fn machine_setup_reset_and_early_failures_share_the_example_error_contract() {
    for arguments in [
        vec!["--json", "--example-setup"],
        vec!["--json", "--example-reset"],
        vec!["--json", "--example"],
        vec!["--json", "--example", "--limit", "0", "rust", "Vec"],
        vec!["--json", "--limit", "bad", "e", "py", "x"],
        vec!["--json", "e", "--open", "py", "x"],
        vec!["--json", "--example", "cache", "clear"],
        vec!["--json", "--example", "qs"],
    ] {
        let root = TestRoot::new("errors");
        let output = root.command().args(&arguments).output().unwrap();
        assert!(
            !output.status.success(),
            "unexpected success for {arguments:?}"
        );
        let value = json(&output);
        assert_eq!(value["version"], 1, "arguments: {arguments:?}");
        assert!(value["code"]
            .as_str()
            .is_some_and(|code| code.starts_with("example_")));
        assert!(value["message"].is_string());
        assert!(!root.state_dir().join("example.json").exists());
    }
}

#[cfg(unix)]
#[test]
fn fake_cli_generation_is_grounded_and_never_enters_history_or_cache() {
    use std::io::Write;
    use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};

    let root = TestRoot::new("generate");
    let state = root.state_dir();
    let mut builder = fs::DirBuilder::new();
    builder.mode(0o700).recursive(true).create(&state).unwrap();
    let mut config = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(state.join("example.json"))
        .unwrap();
    config
        .write_all(b"{\"version\":1,\"provider\":\"gemini-cli\"}\n")
        .unwrap();

    let bin = root.0.join("bin");
    fs::create_dir(&bin).unwrap();
    fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap();
    let gemini = bin.join("gemini");
    let marker = "generated-marker-4f931";
    let script = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then printf '0.63.0\\n'; exit 0; fi\ncat >/dev/null\nprintf '%s\\n' '{{\"response\":\"{{\\\"schema\\\":1,\\\"language\\\":\\\"rust\\\",\\\"code\\\":\\\"fn {marker}() {{}}\\\",\\\"explanation\\\":\\\"Uses the documented Vec type.\\\",\\\"citations\\\":[1]}}\"}}'\n"
    );
    fs::write(&gemini, script).unwrap();
    fs::set_permissions(&gemini, fs::Permissions::from_mode(0o700)).unwrap();

    let output = root
        .command()
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .args(["--json", "--example", "rust", "Vec"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value = json(&output);
    assert_eq!(value["version"], 1);
    assert_eq!(value["provider"], "gemini-cli");
    assert_eq!(value["citations"][0]["index"], 1);
    assert!(value["code"].as_str().unwrap().contains(marker));

    let alias_output = root
        .command()
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .args(["e", "--json", "rust", "Vec"])
        .output()
        .unwrap();
    assert!(alias_output.status.success());
    let alias_value = json(&alias_output);
    assert_eq!(alias_value["provider"], "gemini-cli");
    assert!(alias_value["code"].as_str().unwrap().contains(marker));

    let provider_alias_output = root
        .command()
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .args(["--json", "-p", "rust", "e", "Vec"])
        .output()
        .unwrap();
    assert!(provider_alias_output.status.success());
    let provider_alias_value = json(&provider_alias_output);
    assert_eq!(provider_alias_value["provider"], "gemini-cli");

    assert!(!state.join("history").exists());
    assert!(!tree_contains(&root.0.join("cache"), marker.as_bytes()));
}

#[cfg(unix)]
fn tree_contains(root: &Path, needle: &[u8]) -> bool {
    let Ok(entries) = fs::read_dir(root) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if tree_contains(&path, needle) {
                return true;
            }
        } else if fs::read(&path)
            .ok()
            .is_some_and(|bytes| bytes.windows(needle.len()).any(|window| window == needle))
        {
            return true;
        }
    }
    false
}
