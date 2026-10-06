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

use std::path::PathBuf;

use crate::model::{Hit, Outcome};
use crate::providers::langs::cache_dir;

const KEEP: usize = 100;

pub fn record(query: &str) {
    let query = query.trim();
    if query.is_empty() {
        return;
    }
    let path = history_path();
    let mut lines = std::fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    lines.push(query.to_string());
    if lines.len() > KEEP {
        lines = lines.split_off(lines.len() - KEEP);
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, lines.join("\n") + "\n");
}

pub fn list(which: &str, limit: usize) -> Result<Outcome, String> {
    let lines = read_lines();
    if lines.is_empty() {
        return Err("no search history".into());
    }
    let shown = lines.iter().rev().take(limit.max(1)).collect::<Vec<_>>();
    let mut outcome = Outcome::new("history", which);
    outcome.summary = Some("Run one again with qs history <number>.".into());
    for (offset, query) in shown.iter().enumerate() {
        let number = offset + 1;
        outcome.hits.push(
            Hit::new(query.as_str(), "")
                .meta_line(format!("qs history {number}")),
        );
    }
    Ok(outcome)
}

pub fn rerun_query(which: &str) -> Option<String> {
    let index = which.trim().parse::<usize>().ok()?;
    let lines = read_lines();
    let newest_first = lines.into_iter().rev().collect::<Vec<_>>();
    newest_first.get(index.wrapping_sub(1)).cloned()
}

fn read_lines() -> Vec<String> {
    std::fs::read_to_string(history_path())
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn history_path() -> PathBuf {
    cache_dir().join("history")
}
