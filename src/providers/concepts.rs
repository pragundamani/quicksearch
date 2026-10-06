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

use crate::http::Client;
use crate::model::{Hit, Outcome};
use crate::providers::langs::fuzzy_score;

pub struct Concept {
    pub words: &'static [&'static str],
    pub label: &'static str,
    pub blurb: &'static str,
    pub docsets: &'static [&'static str],
    pub man: bool,
}

const CONCEPTS: &[Concept] = &[
    Concept {
        words: &["linux", "lin"],
        label: "Linux",
        blurb: "commands, filesystems, processes, packages, and shell manuals",
        docsets: &["bash", "zsh"],
        man: true,
    },
    Concept {
        words: &["networking", "network", "net"],
        label: "Networking",
        blurb: "dns, http, tcp, firewalls, proxies, nginx, and sockets",
        docsets: &["http", "nginx", "haproxy"],
        man: true,
    },
    Concept {
        words: &["security", "sec"],
        label: "Security",
        blurb: "ssh, tls, crypto, permissions, sudo, keys, and headers",
        docsets: &["http"],
        man: true,
    },
    Concept {
        words: &["databases", "database", "db", "sql"],
        label: "Databases",
        blurb: "sql, postgres, mysql, sqlite, and redis",
        docsets: &["postgresql", "mariadb", "sqlite", "redis"],
        man: false,
    },
    Concept {
        words: &["containers", "container"],
        label: "Containers",
        blurb: "docker, kubernetes, pods, and images",
        docsets: &["docker", "kubernetes", "kubectl"],
        man: true,
    },
    Concept {
        words: &["infra", "ops", "sre"],
        label: "Infrastructure",
        blurb: "ansible, terraform, servers, and automation",
        docsets: &["ansible", "terraform", "nginx", "docker"],
        man: true,
    },
    Concept {
        words: &["shell"],
        label: "Shell",
        blurb: "bash, zsh, fish, and scripting",
        docsets: &["bash", "zsh", "fish"],
        man: true,
    },
    Concept {
        words: &["git", "vcs"],
        label: "Git",
        blurb: "commits, branches, rebase, and history",
        docsets: &["git"],
        man: true,
    },
    Concept {
        words: &["frontend", "webdev"],
        label: "Frontend",
        blurb: "html, css, javascript, and http",
        docsets: &["html", "css", "javascript", "http"],
        man: false,
    },
    Concept {
        words: &["observability", "logs", "metrics", "monitor", "monitoring"],
        label: "Observability",
        blurb: "logs, metrics, grafana, prometheus, graphite, and journalctl",
        docsets: &["graphite"],
        man: true,
    },
    Concept {
        words: &["filesystem", "disk"],
        label: "Filesystems",
        blurb: "disks, mounts, permissions, and files",
        docsets: &[],
        man: true,
    },
    Concept {
        words: &["cloud", "aws", "gcp", "azure"],
        label: "Cloud",
        blurb: "aws, gcp, azure, iam, vpc, s3, terraform, and opentofu",
        docsets: &["terraform", "opentofu", "ansible"],
        man: true,
    },
    Concept {
        words: &["crypto", "openssl", "gpg", "gnupg"],
        label: "Crypto",
        blurb: "openssl, gpg, certificates, x509, keys, and signatures",
        docsets: &[],
        man: true,
    },
    Concept {
        words: &["mail", "email", "smtp", "postfix", "imap", "dovecot"],
        label: "Mail",
        blurb: "postfix, dovecot, smtp, imap, and aliases",
        docsets: &[],
        man: true,
    },
    Concept {
        words: &["editors", "editor", "vim", "nvim", "neovim", "emacs", "helix"],
        label: "Editors",
        blurb: "vim, neovim, emacs, helix, buffers, and registers",
        docsets: &["elisp"],
        man: true,
    },
    Concept {
        words: &["webserver", "httpd", "apache"],
        label: "Web servers",
        blurb: "apache, nginx, vhosts, and tls",
        docsets: &["apache_http_server", "nginx"],
        man: true,
    },
    Concept {
        words: &["build"],
        label: "Build",
        blurb: "cmake, make, bazel, and ninja",
        docsets: &["cmake", "gnu_make", "bazel"],
        man: true,
    },
    Concept {
        words: &["packages", "package", "pkg"],
        label: "Packages",
        blurb: "dnf, apt, pacman, rpm, and flatpak",
        docsets: &[],
        man: true,
    },
    Concept {
        words: &["virt", "vm", "qemu", "kvm", "libvirt"],
        label: "Virtual machines",
        blurb: "qemu, kvm, libvirt, and virsh",
        docsets: &[],
        man: true,
    },
    Concept {
        words: &["queue", "queues", "mq", "amqp"],
        label: "Queues",
        blurb: "rabbitmq, amqp, and celery",
        docsets: &["rabbitmq", "celery"],
        man: false,
    },
];

pub fn find(word: &str) -> Option<&'static Concept> {
    CONCEPTS.iter().find(|concept| {
        concept
            .words
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(word))
    })
}

pub fn list(pattern: &str, limit: usize) -> Result<Outcome, String> {
    let ranked = rank_concepts(pattern);
    if ranked.is_empty() {
        return Err(format!("no concept matches {pattern}"));
    }
    let shown_name = if pattern.trim().is_empty() {
        "concepts"
    } else {
        pattern
    };
    let cap = if pattern.trim().is_empty() {
        ranked.len()
    } else {
        limit.clamp(1, 20).min(ranked.len())
    };
    let hidden = ranked.len().saturating_sub(cap);
    let mut outcome = Outcome::new("topic", shown_name);
    if pattern.trim().is_empty() {
        outcome.summary = Some(
            "Fuzzy-find a concept with qs topic <pattern>. Then search with qs <shorthand> <topic>."
                .to_string(),
        );
    } else if hidden > 0 {
        outcome.summary = Some(format!("{hidden} more. pass -n to show them."));
    }
    for concept in ranked.into_iter().take(cap) {
        let primary = concept.words[0];
        let sources = source_line(concept);
        outcome.hits.push(
            Hit::new(concept.words.join(", "), "")
                .snippet(format!("{}  ·  {}", concept.label, concept.blurb))
                .meta_line(format!("qs {primary} <topic>  ·  {sources}")),
        );
    }
    Ok(outcome)
}

pub fn search(
    client: &Client,
    concept: &Concept,
    query: &str,
    limit: usize,
) -> Result<Outcome, String> {
    let mut ranked: Vec<(i32, Hit)> = Vec::new();
    let mut body = None;
    if concept.man {
        if let Ok(manual) = super::man::search(query, limit.max(8)) {
            body = manual.body;
            for (index, hit) in manual.hits.into_iter().enumerate() {
                let exact = hit
                    .title
                    .to_ascii_lowercase()
                    .starts_with(&format!("{} (", query.to_ascii_lowercase()));
                let score = if exact {
                    12_000 - index as i32
                } else {
                    700 - index as i32 * 15
                };
                ranked.push((score, hit.meta_line("manual")));
            }
        }
    }
    let per_docset = if concept.docsets.is_empty() {
        0
    } else {
        (limit / concept.docsets.len()).clamp(2, limit)
    };
    let fetch = body.is_none();
    let query_owned = query.to_string();
    let families = concept.docsets;
    let found = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for family in families {
            let agent = client.agent.clone();
            let query_owned = query_owned.clone();
            handles.push(scope.spawn(move || {
                let local = Client { agent };
                super::langs::family_hits(&local, family, &query_owned, per_docset, fetch)
            }));
        }
        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap_or_default())
            .collect::<Vec<_>>()
    });
    ranked.extend(found);
    ranked.sort_by(|left, right| right.0.cmp(&left.0));
    let mut seen = std::collections::HashSet::new();
    ranked.retain(|(_, hit)| seen.insert(hit.url.clone()));
    ranked.truncate(limit);

    if ranked.is_empty() && body.is_none() {
        return Err(format!("no {} results for {query}", concept.label));
    }
    let mut outcome = Outcome::new(concept.label, query);
    outcome.body = body;
    outcome.hits = ranked.into_iter().map(|(_, hit)| hit).collect();
    Ok(outcome)
}

fn keyword_score(pattern: &str, blurb: &str) -> Option<i32> {
    blurb
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .filter_map(|word| token_score(pattern, word))
        .max()
}

fn token_score(pattern: &str, token: &str) -> Option<i32> {
    let token = token.to_ascii_lowercase();
    if token == pattern {
        return Some(9_000 - token.len() as i32);
    }
    if token.starts_with(pattern) && pattern.len() >= 3 {
        return Some(7_000 - token.len() as i32);
    }
    if pattern.len() >= 4 {
        if let Some(at) = token.find(pattern) {
            return Some(5_000 - at as i32 - token.len() as i32);
        }
    }
    None
}

fn source_line(concept: &Concept) -> String {
    let mut parts = Vec::new();
    if concept.man {
        parts.push("man");
    }
    parts.extend(concept.docsets.iter().copied());
    if parts.is_empty() {
        "docs".to_string()
    } else {
        parts.join(", ")
    }
}

fn rank_concepts(pattern: &str) -> Vec<&'static Concept> {
    let pattern: String = pattern
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(|ch| ch.to_lowercase())
        .collect();
    let mut scored: Vec<(i32, &'static Concept)> = Vec::new();
    for concept in CONCEPTS {
        if pattern.is_empty() {
            scored.push((0, concept));
            continue;
        }
        let word_score = concept
            .words
            .iter()
            .filter_map(|word| fuzzy_score(&pattern, word))
            .max();
        let label_score = fuzzy_score(&pattern, concept.label);
        let blurb_score = keyword_score(&pattern, concept.blurb);
        let Some(score) = [word_score, label_score, blurb_score]
            .into_iter()
            .flatten()
            .max()
        else {
            continue;
        };
        scored.push((score, concept));
    }
    if pattern.is_empty() {
        scored.sort_by_key(|(_, concept)| {
            CONCEPTS
                .iter()
                .position(|item| std::ptr::eq(*concept, item))
                .unwrap_or(CONCEPTS.len())
        });
    } else {
        scored.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.label.cmp(right.1.label)));
    }
    scored.into_iter().map(|(_, concept)| concept).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_concept_aliases() {
        assert_eq!(find("net").unwrap().label, "Networking");
        assert_eq!(find("sec").unwrap().label, "Security");
        assert_eq!(find("lin").unwrap().label, "Linux");
        assert!(find("python").is_none());
        assert!(find("cmake").is_none());
        assert_eq!(find("aws").unwrap().label, "Cloud");
        assert_eq!(find("gpg").unwrap().label, "Crypto");
        assert_eq!(find("vim").unwrap().label, "Editors");
    }

    #[test]
    fn fuzzy_topics_match_blurbs_and_names() {
        let security = rank_concepts("tls");
        assert_eq!(security[0].label, "Security");
        let linux = rank_concepts("lin");
        assert_eq!(linux[0].label, "Linux");
        let data = rank_concepts("postgres");
        assert_eq!(data[0].label, "Databases");
        let mail = rank_concepts("smtp");
        assert_eq!(mail[0].label, "Mail");
        let observe = rank_concepts("prometheus");
        assert_eq!(observe[0].label, "Observability");
    }
}
