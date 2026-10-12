// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Opt-in, syntax-only checks for generated examples.
//!
//! Validators receive source as data. This module never runs the source, a
//! generated artifact, a test, a build system, a package manager, or a network
//! command. A syntax result is not behavioral verification.

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::schema::{normalize_language, MAX_CODE_BYTES};

const TIMEOUT: Duration = Duration::from_secs(5);
const MAX_DIAGNOSTIC_BYTES: u64 = 4 * 1024;
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    /// The fixed parser accepted the syntax. This says nothing about behavior.
    SyntaxAccepted,
    SyntaxRejected,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub diagnostics: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatorMode {
    Stdin,
    JavaScriptFile,
    JavaFile,
    Disabled,
}

/// Public description of the fixed validator allowlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatorSpec {
    pub language: &'static str,
    pub programs: &'static [&'static str],
    pub args: &'static [&'static str],
    pub mode: ValidatorMode,
}

const PYTHON_PROGRAMS: &[&str] = &["/usr/bin/python3", "/usr/local/bin/python3"];
const GOFMT_PROGRAMS: &[&str] = &["/usr/local/go/bin/gofmt", "/usr/bin/gofmt"];
const NODE_PROGRAMS: &[&str] = &["/usr/bin/node", "/usr/local/bin/node"];
const BASH_PROGRAMS: &[&str] = &["/usr/bin/bash", "/bin/bash"];
const CC_PROGRAMS: &[&str] = &["/usr/bin/cc", "/usr/bin/gcc", "/usr/local/bin/cc"];
const CXX_PROGRAMS: &[&str] = &["/usr/bin/c++", "/usr/bin/g++", "/usr/local/bin/c++"];
const JAVAC_PROGRAMS: &[&str] = &["/usr/bin/javac", "/usr/local/bin/javac"];

const PYTHON_ARGS: &[&str] = &[
    "-I",
    "-c",
    "import sys; compile(sys.stdin.read(), '<example>', 'exec', dont_inherit=True, optimize=0)",
];
const GOFMT_ARGS: &[&str] = &["-e"];
const NODE_ARGS: &[&str] = &["--check"];
const BASH_ARGS: &[&str] = &["--noprofile", "--norc", "-n"];
const C_ARGS: &[&str] = &[
    "-fsyntax-only",
    "-x",
    "c",
    "-std=c17",
    "-fno-diagnostics-color",
];
const CPP_ARGS: &[&str] = &[
    "-fsyntax-only",
    "-x",
    "c++",
    "-std=c++17",
    "-fno-diagnostics-color",
];
const JAVAC_ARGS: &[&str] = &[
    "-proc:none",
    "-implicit:none",
    "-Xlint:none",
    "-encoding",
    "UTF-8",
];

pub const VALIDATOR_TABLE: &[ValidatorSpec] = &[
    ValidatorSpec {
        language: "python",
        programs: PYTHON_PROGRAMS,
        args: PYTHON_ARGS,
        mode: ValidatorMode::Stdin,
    },
    ValidatorSpec {
        language: "go",
        programs: GOFMT_PROGRAMS,
        args: GOFMT_ARGS,
        mode: ValidatorMode::Stdin,
    },
    ValidatorSpec {
        language: "javascript",
        programs: NODE_PROGRAMS,
        args: NODE_ARGS,
        mode: ValidatorMode::JavaScriptFile,
    },
    ValidatorSpec {
        language: "bash",
        programs: BASH_PROGRAMS,
        args: BASH_ARGS,
        mode: ValidatorMode::Stdin,
    },
    // C and C++ preprocessors can read attacker-selected files through source
    // directives. Without an OS sandbox, skipping is safer than pretending
    // that fixed compiler flags close that channel.
    ValidatorSpec {
        language: "c",
        programs: CC_PROGRAMS,
        args: C_ARGS,
        mode: ValidatorMode::Disabled,
    },
    ValidatorSpec {
        language: "cpp",
        programs: CXX_PROGRAMS,
        args: CPP_ARGS,
        mode: ValidatorMode::Disabled,
    },
    ValidatorSpec {
        language: "java",
        programs: JAVAC_PROGRAMS,
        args: JAVAC_ARGS,
        mode: ValidatorMode::JavaFile,
    },
];

/// Explicitly opt in to a bounded syntax check.
///
/// Calling this function is the opt-in. No validation runs during parsing,
/// prompting, rendering, or any other implicit path.
pub fn validate_syntax(language: &str, code: &str) -> ValidationResult {
    let Some(language) = normalize_language(language) else {
        return skipped("unsupported language; no command was run");
    };
    if code.len() > MAX_CODE_BYTES {
        return skipped("source exceeds the validation input limit; no command was run");
    }
    let Some(spec) = VALIDATOR_TABLE
        .iter()
        .find(|spec| spec.language == language)
    else {
        return skipped("no allowlisted syntax validator; no command was run");
    };
    if spec.mode == ValidatorMode::Disabled {
        return skipped(
            "validator disabled because safe isolation is unavailable; no command was run",
        );
    }
    let Some(program) = spec.programs.iter().find_map(|path| trusted_program(path)) else {
        return skipped("allowlisted validator is absent; no command was run");
    };

    match run_validator(&program, spec, code) {
        Ok(result) => result,
        Err(message) => skipped(&format!("validator skipped: {message}")),
    }
}

fn trusted_program(candidate: &str) -> Option<PathBuf> {
    let canonical = fs::canonicalize(candidate).ok()?;
    let metadata = fs::metadata(&canonical).ok()?;
    if !metadata.is_file() {
        return None;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        // Follow only a root-owned, non-writable system path. This permits
        // distribution-managed alternatives without trusting user symlinks.
        if metadata.uid() != 0 || metadata.mode() & 0o022 != 0 {
            return None;
        }
        let mut ancestor = canonical.parent();
        while let Some(path) = ancestor {
            let metadata = fs::metadata(path).ok()?;
            if metadata.uid() != 0 || metadata.mode() & 0o022 != 0 {
                return None;
            }
            ancestor = path.parent();
        }
    }
    #[cfg(not(unix))]
    if Path::new(candidate).is_symlink() {
        return None;
    }
    Some(canonical)
}

fn java_source_name(code: &str) -> String {
    let words = code
        .split(|character: char| {
            !(character.is_ascii_alphanumeric() || matches!(character, '_' | '$'))
        })
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    for window in words.windows(3) {
        if window[0] == "public"
            && matches!(window[1], "class" | "interface" | "enum" | "record")
            && window[2]
                .bytes()
                .next()
                .is_some_and(|byte| byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$'))
            && window[2].len() <= 128
        {
            return format!("{}.java", window[2]);
        }
    }
    "Example.java".to_string()
}

fn run_validator(
    program: &Path,
    spec: &ValidatorSpec,
    code: &str,
) -> Result<ValidationResult, String> {
    let temp = PrivateTempDir::create()?;
    let input_name = match spec.mode {
        ValidatorMode::JavaScriptFile => "example.js".to_string(),
        ValidatorMode::JavaFile => java_source_name(code),
        ValidatorMode::Stdin => "input.txt".to_string(),
        ValidatorMode::Disabled => return Err("disabled validator".to_string()),
    };
    let input_path = temp.path.join(&input_name);
    let stdout_path = temp.path.join("stdout.txt");
    let stderr_path = temp.path.join("stderr.txt");
    write_private(&input_path, code.as_bytes())?;
    let input = File::open(&input_path).map_err(|error| error.to_string())?;
    let stdout = create_private(&stdout_path)?;
    let stderr = create_private(&stderr_path)?;

    let mut command = Command::new(program);
    command
        .args(spec.args)
        .env_clear()
        .current_dir(&temp.path)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    match spec.mode {
        ValidatorMode::Stdin => {
            command.stdin(Stdio::from(input));
        }
        ValidatorMode::JavaScriptFile => {
            command.arg(&input_name).stdin(Stdio::null());
        }
        ValidatorMode::JavaFile => {
            fs::create_dir(temp.path.join("classes")).map_err(|error| error.to_string())?;
            command
                .args(["-classpath", "classes", "-d", "classes", &input_name])
                .stdin(Stdio::null());
        }
        ValidatorMode::Disabled => unreachable!(),
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }

    let mut child = command.spawn().map_err(|error| error.to_string())?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait().map_err(|error| error.to_string())? {
            Some(status) => break status,
            None if started.elapsed() < TIMEOUT => thread::sleep(Duration::from_millis(10)),
            None => {
                kill_process_tree(&mut child);
                let _ = child.wait();
                return Ok(ValidationResult {
                    status: ValidationStatus::Skipped,
                    diagnostics: "syntax validator timed out after 5 seconds; no behavioral verification was performed".to_string(),
                });
            }
        }
    };

    let diagnostics = if status.success() {
        String::new()
    } else {
        let mut value = read_bounded(&stderr_path)?;
        if value.trim().is_empty() {
            value = read_bounded(&stdout_path)?;
        }
        sanitize_diagnostic(&value)
    };
    Ok(ValidationResult {
        status: if status.success() {
            ValidationStatus::SyntaxAccepted
        } else {
            ValidationStatus::SyntaxRejected
        },
        diagnostics,
    })
}

fn skipped(message: &str) -> ValidationResult {
    ValidationResult {
        status: ValidationStatus::Skipped,
        diagnostics: message.to_string(),
    }
}

fn kill_process_tree(child: &mut std::process::Child) {
    #[cfg(unix)]
    unsafe {
        // The child starts in its own process group. Kill that group so a
        // validator cannot leave descendants after the deadline.
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    let _ = child.kill();
}

fn read_bounded(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    file.take(MAX_DIAGNOSTIC_BYTES)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn sanitize_diagnostic(input: &str) -> String {
    let mut output: String = input
        .chars()
        .filter(|ch| matches!(ch, '\n' | '\t') || !ch.is_control())
        .collect();
    let limit = MAX_DIAGNOSTIC_BYTES as usize;
    if output.len() > limit {
        let mut end = limit;
        while !output.is_char_boundary(end) {
            end -= 1;
        }
        output.truncate(end);
    }
    output
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let mut file = create_private(path)?;
    file.write_all(bytes).map_err(|error| error.to_string())
}

fn create_private(path: &Path) -> Result<File, String> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path).map_err(|error| error.to_string())
}

struct PrivateTempDir {
    path: PathBuf,
}

impl PrivateTempDir {
    fn create() -> Result<Self, String> {
        for _ in 0..16 {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let count = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("qs-example-{}-{nonce}-{count}", std::process::id()));
            #[cfg(unix)]
            let builder = {
                use std::os::unix::fs::DirBuilderExt;
                let mut builder = fs::DirBuilder::new();
                builder.mode(0o700);
                builder
            };
            #[cfg(not(unix))]
            let builder = fs::DirBuilder::new();
            match builder.create(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.to_string()),
            }
        }
        Err("could not create a private temporary directory".to_string())
    }
}

impl Drop for PrivateTempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_table_has_no_package_manager_or_shell_dispatch() {
        for spec in VALIDATOR_TABLE {
            for program in spec.programs {
                assert!(program.starts_with('/'));
                assert!(!["npm", "cargo", "pip", "go", "make", "mvn", "gradle"]
                    .iter()
                    .any(|name| program.ends_with(&format!("/{name}"))));
            }
        }
    }

    #[test]
    fn generated_programs_are_never_executed() {
        let marker = std::env::temp_dir().join(format!(
            "qs-example-must-not-exist-{}-{}",
            std::process::id(),
            TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let path = marker.to_string_lossy();
        let programs = [
            ("python", format!("open({path:?}, 'w').write('bad')")),
            ("bash", format!("touch {path}")),
            (
                "javascript",
                format!("require('fs').writeFileSync({path:?}, 'bad');"),
            ),
        ];
        for (language, code) in programs {
            let _ = validate_syntax(language, &code);
            assert!(!marker.exists(), "{language} source was executed");
        }
    }

    #[test]
    fn python_compile_rejects_context_invalid_syntax() {
        if trusted_program("/usr/bin/python3").is_some()
            || trusted_program("/usr/local/bin/python3").is_some()
        {
            assert_eq!(
                validate_syntax("python", "return 1").status,
                ValidationStatus::SyntaxRejected
            );
            assert_eq!(
                validate_syntax("python", "value = 1\n").status,
                ValidationStatus::SyntaxAccepted
            );
        }
    }

    #[test]
    fn java_filename_matches_the_public_type() {
        assert_eq!(java_source_name("public class Main {}"), "Main.java");
        assert_eq!(java_source_name("class Local {}"), "Example.java");
        if trusted_program("/usr/bin/javac").is_some()
            || trusted_program("/usr/local/bin/javac").is_some()
        {
            assert_eq!(
                validate_syntax("java", "public class Main {}").status,
                ValidationStatus::SyntaxAccepted
            );
        }
    }

    #[test]
    fn unsupported_and_compiler_risk_are_skipped() {
        assert_eq!(
            validate_syntax("typescript", "let x: number = 1;").status,
            ValidationStatus::Skipped
        );
        assert_eq!(
            validate_syntax("c", "int main(void) { return 0; }").status,
            ValidationStatus::Skipped
        );
    }

    #[test]
    fn parser_reports_syntax_only() {
        let result = validate_syntax("python", "if True print('x')");
        assert!(matches!(
            result.status,
            ValidationStatus::SyntaxRejected | ValidationStatus::Skipped
        ));
        assert!(!result
            .diagnostics
            .to_ascii_lowercase()
            .contains("behavior verified"));
    }
}
