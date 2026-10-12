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

use std::process::{Command, Stdio};

use crate::model::{Hit, Outcome};
use crate::providers::fallback;
use crate::providers::man::open_page;

const TOOLS: &[&str] = &[
    "apt-get", "dnf", "apt", "pacman", "brew", "cargo", "rpm", "flatpak", "zypper", "snap",
    "podman", "winget",
];

const SUBCOMMANDS: &[&str] = &[
    "add",
    "attach",
    "autoremove",
    "bench",
    "build",
    "check",
    "clean",
    "clippy",
    "connections",
    "config",
    "create",
    "delete",
    "distro-sync",
    "doc",
    "download",
    "erase",
    "exec",
    "fetch",
    "fmt",
    "generate",
    "group",
    "help",
    "history",
    "images",
    "info",
    "init",
    "inspect",
    "install",
    "kube",
    "kill",
    "leaves",
    "list",
    "login",
    "logs",
    "makecache",
    "metadata",
    "module",
    "new",
    "override",
    "owner",
    "package",
    "pause",
    "policy",
    "pin",
    "ps",
    "publish",
    "pull",
    "push",
    "query",
    "read",
    "reinstall",
    "remotes",
    "remote",
    "remove",
    "repair",
    "repoquery",
    "rm",
    "rmi",
    "run",
    "search",
    "show",
    "start",
    "stop",
    "test",
    "tree",
    "uninstall",
    "unpause",
    "update",
    "upgrade",
    "vendor",
    "version",
    "yank",
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
    if tool == "podman" {
        if let Some(nested_page) = podman_nested_manual(first, rest) {
            return Some(CommandRoute::Manual(nested_page));
        }
    }
    let full_page = format!("{tool}-{}", inner.replace(char::is_whitespace, "-"));
    if !rest.is_empty() && man_page_exists(&full_page) {
        return Some(CommandRoute::Manual(full_page));
    }
    let subcommand = SUBCOMMANDS
        .iter()
        .any(|word| word.eq_ignore_ascii_case(first))
        || man_page_exists(&page);
    if subcommand && (rest.is_empty() || tool == "podman") {
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
    if let Some(url) = published_manual(page) {
        let mut outcome = Outcome::new("man", page);
        outcome.hits.push(Hit::new(page, url));
        outcome.summary = Some(format!("No local manual for {page}."));
        return Ok(outcome);
    }
    let tool = tool_of(page);
    if tool != page {
        if let Some(outcome) = open_page(tool) {
            return Ok(outcome);
        }
    }
    let url = format!(
        "https://manpages.debian.org/cgi-bin/search.py?q={}",
        crate::text::encode_query(page)
    );
    Ok(fallback::search(
        "man",
        page,
        format!("Search online manuals for {page}"),
        url,
        format!("no local manual for {page}"),
    ))
}

pub fn package(tool: &str, name: &str) -> Result<Outcome, String> {
    match package_live(tool, name) {
        Ok(outcome) => Ok(outcome),
        Err(reason) => {
            let url = package_search_url(tool, name);
            Ok(fallback::search(
                tool,
                name,
                format!("Search {tool} packages for {name}"),
                url,
                reason,
            ))
        }
    }
}

fn package_search_url(tool: &str, name: &str) -> String {
    let query = crate::text::encode_query(name);
    match tool {
        "apt" | "apt-get" => format!("https://packages.debian.org/search?keywords={query}"),
        "dnf" | "rpm" => format!("https://packages.fedoraproject.org/search?query={query}"),
        "pacman" => format!("https://archlinux.org/packages/?q={query}"),
        "brew" => format!("https://formulae.brew.sh/search/?q={query}"),
        "cargo" => format!("https://crates.io/search?q={query}"),
        "flatpak" => format!("https://flathub.org/apps/search?q={query}"),
        "snap" => format!("https://snapcraft.io/search?q={query}"),
        "winget" => format!("https://winstall.app/apps?q={query}"),
        _ => format!(
            "https://duckduckgo.com/?q={}+package+{}",
            crate::text::encode_query(tool),
            query
        ),
    }
}

fn validate_package_operand(name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty()
        || name.starts_with('-')
        || name.len() > 512
        || name.chars().any(char::is_control)
    {
        return Err("invalid package name".into());
    }
    Ok(())
}

fn package_live(tool: &str, name: &str) -> Result<Outcome, String> {
    validate_package_operand(name)?;
    let body = match tool {
        "dnf" | "rpm" => dnf_info(name)?,
        "apt" | "apt-get" => command_text("apt", &["show", "--", name])?,
        "pacman" => command_text("pacman", &["-Si", "--", name])?,
        "brew" => command_text("brew", &["info", "--", name])?,
        "winget" => command_text(
            "winget",
            &[
                "show",
                "--id",
                name,
                "--exact",
                "--disable-interactivity",
                "--accept-source-agreements",
            ],
        )?,
        "flatpak" => flatpak_info(name)?,
        "zypper" => command_text("zypper", &["info", "--", name])?,
        "cargo" | "snap" | "podman" => {
            return Err(format!("{tool} has no package info for {name}"));
        }
        other => return Err(format!("no package info via {other}")),
    };
    let mut outcome = Outcome::new(tool, name);
    outcome.body = Some(body.trim().to_string());
    Ok(outcome)
}

fn dnf_info(name: &str) -> Result<String, String> {
    let installed = Command::new("rpm")
        .args(["-q", "--info", "--", name])
        .output();
    if let Ok(output) = installed {
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        if output.status.success() && !text.to_ascii_lowercase().contains("is not installed") {
            return Ok(text);
        }
    }
    let output = Command::new("dnf")
        .args(["info", "-q", "--cacheonly", "--", name])
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
    if let Ok(text) = command_text("flatpak", &["info", "--", name]) {
        return Ok(text);
    }
    command_text("flatpak", &["search", "--", name])
}

fn podman_nested_manual(first: &str, rest: &str) -> Option<String> {
    let (second, _) = split_first(rest);
    let known = matches!(
        (first, second),
        ("kube", "play" | "apply" | "down" | "generate")
            | ("machine", "init" | "start" | "stop" | "ssh")
            | ("container", "cleanup" | "exists")
            | ("image", "prune")
            | ("network", "create")
            | ("pod", "create")
            | ("volume", "create")
    );
    known.then(|| format!("podman-{first}-{second}"))
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
        "winget" => Some("https://learn.microsoft.com/windows/package-manager/winget/".into()),
        "apt" | "apt-get" if sub == "policy" => {
            Some("https://manpages.debian.org/unstable/apt/apt-cache.8.en.html#policy".into())
        }
        "apt" | "apt-get" => Some("https://manpages.debian.org/unstable/apt/apt.8.en.html".into()),
        "pacman" => Some("https://man.archlinux.org/man/pacman.8".into()),
        "podman" if sub.is_empty() => Some("https://docs.podman.io/en/latest/Commands.html".into()),
        "podman" => Some(format!(
            "https://docs.podman.io/en/latest/markdown/{page}.1.html"
        )),
        "flatpak" => {
            Some("https://docs.flatpak.org/en/latest/flatpak-command-reference.html".into())
        }
        "snap" => Some("https://snapcraft.io/docs/command-reference".into()),
        "dnf" | "rpm" => Some("https://dnf5.readthedocs.io/en/latest/commands/index.html".into()),
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

fn split_first(text: &str) -> (&str, &str) {
    match text.split_once(char::is_whitespace) {
        Some((head, tail)) => (head, tail.trim()),
        None => (text, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_operands_cannot_be_command_options() {
        assert!(validate_package_operand("serde").is_ok());
        assert!(validate_package_operand("@scope/package").is_ok());
        assert!(validate_package_operand("--pipe=/usr/bin/id").is_err());
        assert!(validate_package_operand("-q").is_err());
        assert!(validate_package_operand("bad\nname").is_err());
    }

    #[test]
    fn apt_policy_uses_the_apt_cache_manual() {
        let url = published_manual("apt-policy").unwrap();
        assert!(url.contains("apt-cache.8"));
        assert!(url.ends_with("#policy"));
    }

    #[test]
    fn known_package_manager_commands_route_to_manuals() {
        assert!(
            matches!(route_command("apt", "policy"), Some(CommandRoute::Manual(page)) if page == "apt-policy")
        );
        assert!(
            matches!(route_command("brew", "leaves"), Some(CommandRoute::Manual(page)) if page == "brew-leaves")
        );
        assert!(
            matches!(route_command("snap", "connections"), Some(CommandRoute::Manual(page)) if page == "snap-connections")
        );
        assert!(
            matches!(route_command("podman", "kube play"), Some(CommandRoute::Manual(page)) if page == "podman-kube-play")
        );
        assert!(
            matches!(route_command("podman", "run alpine"), Some(CommandRoute::Manual(page)) if page == "podman-run")
        );
        assert!(
            matches!(route_command("podman", "kube play demo.yaml"), Some(CommandRoute::Manual(page)) if page == "podman-kube-play")
        );
        assert!(
            matches!(route_command("podman", "machine ssh vm"), Some(CommandRoute::Manual(page)) if page == "podman-machine-ssh")
        );
    }

    #[test]
    fn podman_nested_commands_do_not_include_operands_in_manual_names() {
        for query in ["machine ssh", "machine ssh vm", "machine ssh vm user"] {
            assert!(
                matches!(route_command("podman", query), Some(CommandRoute::Manual(page)) if page == "podman-machine-ssh"),
                "unexpected route for {query}"
            );
        }
    }

    #[test]
    fn package_search_fallbacks_are_source_specific() {
        assert_eq!(
            package_search_url("apt", "ripgrep"),
            "https://packages.debian.org/search?keywords=ripgrep"
        );
        assert_eq!(
            package_search_url("brew", "fd find"),
            "https://formulae.brew.sh/search/?q=fd%20find"
        );
    }
}
