use std::process::{Command, Stdio};
use std::time::UNIX_EPOCH;

use crate::model::{Hit, Outcome};
use crate::providers::langs::cache_dir;
use crate::providers::man::open_page;

const TOOLS: &[&str] = &[
    "apt-get", "dnf", "apt", "pacman", "brew", "cargo", "rpm", "flatpak", "zypper", "snap",
    "podman",
];

const SUBCOMMANDS: &[&str] = &[
    "add", "attach", "autoremove", "bench", "build", "check", "clean", "clippy", "config",
    "create", "delete", "distro-sync", "doc", "download", "erase", "exec", "fetch", "fmt",
    "generate", "group", "help", "history", "images", "info", "init", "inspect", "install",
    "kill", "list", "login", "logs", "makecache", "metadata", "module", "new", "override",
    "owner", "package", "pause", "pin", "ps", "publish", "pull", "push", "query", "read",
    "reinstall", "remote", "remove", "repair", "repoquery", "rm", "rmi", "run", "search",
    "show", "start", "stop", "test", "tree", "uninstall", "unpause", "update", "upgrade",
    "vendor", "version", "yank",
];

pub enum CommandRoute {
    Manual(String),
    Package { tool: String, name: String },
}

pub fn route_command(name: &str, inner: &str) -> Option<CommandRoute> {
    let tool = TOOLS
        .iter()
        .copied()
        .find(|tool| tool.eq_ignore_ascii_case(name))?;
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(CommandRoute::Manual(tool.to_string()));
    }
    let (first, rest) = split_first(inner);
    if first.starts_with('-') {
        return Some(CommandRoute::Manual(tool.to_string()));
    }
    let page = format!("{tool}-{first}");
    let subcommand = SUBCOMMANDS.iter().any(|word| word.eq_ignore_ascii_case(first))
        || man_page_exists(&page);
    if subcommand && rest.is_empty() {
        Some(CommandRoute::Manual(page))
    } else if subcommand {
        Some(CommandRoute::Package {
            tool: tool.to_string(),
            name: rest.to_string(),
        })
    } else {
        Some(CommandRoute::Package {
            tool: tool.to_string(),
            name: inner.to_string(),
        })
    }
}

pub fn manual(page: &str) -> Result<Outcome, String> {
    if let Some(outcome) = open_page(page) {
        return Ok(outcome);
    }
    let tool = tool_of(page);
    if tool != page {
        if let Some(outcome) = open_page(tool) {
            return Ok(outcome);
        }
    }
    if let Some(url) = published_manual(page) {
        let mut outcome = Outcome::new("man", page);
        outcome.hits.push(Hit::new(page, url));
        outcome.summary = Some(format!("No local manual for {page}."));
        return Ok(outcome);
    }
    Err(format!("no manual for {page}"))
}

pub fn package(tool: &str, name: &str) -> Result<Outcome, String> {
    if let Some(outcome) = read_package_cache(tool, name) {
        return Ok(outcome);
    }
    let body = match tool {
        "dnf" | "rpm" => dnf_info(name)?,
        "apt" | "apt-get" => command_text("apt", &["show", name])?,
        "pacman" => command_text("pacman", &["-Si", name])?,
        "brew" => command_text("brew", &["info", name])?,
        "flatpak" => flatpak_info(name)?,
        "zypper" => command_text("zypper", &["info", name])?,
        "cargo" | "snap" | "podman" => {
            return Err(format!("{tool} has no package info for {name}"));
        }
        other => return Err(format!("no package info via {other}")),
    };
    let mut outcome = Outcome::new(tool, name);
    outcome.body = Some(body.trim().to_string());
    write_package_cache(tool, name, &outcome.body.clone().unwrap_or_default());
    Ok(outcome)
}

fn dnf_info(name: &str) -> Result<String, String> {
    let installed = Command::new("rpm")
        .args(["-q", "--info", name])
        .output();
    if let Ok(output) = installed {
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        if output.status.success() && !text.to_ascii_lowercase().contains("is not installed") {
            return Ok(text);
        }
    }
    let output = Command::new("dnf")
        .args(["info", "-q", "--cacheonly", name])
        .stdin(Stdio::null())
        .output()
        .map_err(|err| format!("dnf: {err}"))?;
    let text = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() || text.trim().is_empty() {
        let err = String::from_utf8_lossy(&output.stderr);
        let detail = err.trim();
        if detail.is_empty() {
            return Err(format!("no dnf info for {name}"));
        }
        return Err(format!("no dnf info for {name}: {detail}"));
    }
    Ok(pick_stanza(&text))
}

fn pick_stanza(text: &str) -> String {
    let stanzas: Vec<&str> = text
        .split("\n\n")
        .map(str::trim)
        .filter(|stanza| stanza.to_ascii_lowercase().contains("name"))
        .collect();
    if stanzas.is_empty() {
        return text.trim().to_string();
    }
    let arch = std::env::consts::ARCH;
    let prefer = |stanza: &str| {
        let lower = stanza.to_ascii_lowercase();
        if lower.contains(&format!("architecture   : {arch}"))
            || lower.contains(&format!("architecture: {arch}"))
        {
            0
        } else if lower.contains("architecture   : src") || lower.contains("architecture: src") {
            2
        } else {
            1
        }
    };
    stanzas
        .into_iter()
        .min_by_key(|stanza| prefer(stanza))
        .unwrap_or(text)
        .trim()
        .to_string()
}

fn flatpak_info(name: &str) -> Result<String, String> {
    if let Ok(text) = command_text("flatpak", &["info", name]) {
        return Ok(text);
    }
    command_text("flatpak", &["search", name])
}

fn command_text(bin: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(bin)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|err| format!("{bin}: {err}"))?;
    let text = String::from_utf8_lossy(&output.stdout).to_string();
    if output.status.success() && !text.trim().is_empty() {
        Ok(text)
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("{bin} {} failed: {}", args.join(" "), err.trim()))
    }
}

fn published_manual(page: &str) -> Option<String> {
    let tool = tool_of(page);
    let sub = page.strip_prefix(&format!("{tool}-")).unwrap_or("");
    match tool {
        "cargo" if sub.is_empty() => {
            Some("https://doc.rust-lang.org/cargo/commands/cargo.html".into())
        }
        "cargo" => Some(format!(
            "https://doc.rust-lang.org/cargo/commands/cargo-{sub}.html"
        )),
        "brew" => Some("https://docs.brew.sh/Manpage".into()),
        "apt" | "apt-get" => Some("https://manpages.debian.org/unstable/apt/apt.8.en.html".into()),
        "pacman" => Some("https://man.archlinux.org/man/pacman.8".into()),
        _ => None,
    }
}

fn tool_of(page: &str) -> &str {
    TOOLS
        .iter()
        .copied()
        .find(|tool| *tool == page || page.starts_with(&format!("{tool}-")))
        .unwrap_or(page)
}

fn man_page_exists(page: &str) -> bool {
    Command::new("man")
        .args(["-w", page])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn package_cache_path(tool: &str, name: &str) -> std::path::PathBuf {
    let safe: String = format!("{tool}-{name}")
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect();
    cache_dir().join("packages").join(safe)
}

fn cache_stamp() -> u64 {
    std::fs::metadata("/var/cache/libdnf5")
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn read_package_cache(tool: &str, name: &str) -> Option<Outcome> {
    let text = std::fs::read_to_string(package_cache_path(tool, name)).ok()?;
    let (stamp, body) = text.split_once('\n')?;
    let saved: u64 = stamp.strip_prefix("stamp ")?.parse().ok()?;
    if saved != cache_stamp() {
        return None;
    }
    let body = body.trim();
    if body.is_empty() {
        return None;
    }
    let mut outcome = Outcome::new(tool, name);
    outcome.body = Some(body.to_string());
    Some(outcome)
}

fn write_package_cache(tool: &str, name: &str, body: &str) {
    let path = package_cache_path(tool, name);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, format!("stamp {}\n{body}", cache_stamp()));
}

fn split_first(text: &str) -> (&str, &str) {
    match text.split_once(char::is_whitespace) {
        Some((head, tail)) => (head, tail.trim()),
        None => (text, ""),
    }
}
