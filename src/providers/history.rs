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

use crate::model::{Hit, Outcome};
use crate::providers::langs::cache_dir;
use crate::state::{LockedStateFile, StateDir};

const HISTORY_FILE: &str = "history";
const HISTORY_LIMIT: u64 = 1024 * 1024;
const KEEP: usize = 100;

pub fn migrate_legacy() {
    let state = StateDir::platform();
    // Preserve an existing state file. Unsafe leaves and inaccessible roots
    // also stop migration rather than risking replacement at the wrong place.
    if state.exists(HISTORY_FILE).unwrap_or(true) {
        return;
    }
    let legacy = cache_dir().join("history");
    let Ok(Some(text)) = crate::cache::read_cache_text(&legacy, HISTORY_LIMIT) else {
        return;
    };
    let _ = state.with_lock(HISTORY_FILE, |file| {
        if file.exists()? {
            return Ok(());
        }
        file.write_text_atomic(&text)?;
        crate::cache::remove_cache_file(&legacy)
    });
}

pub fn record(query: &str) {
    let query = normalize_query(query);
    if query.is_empty() {
        return;
    }
    let state = StateDir::platform();
    let _ = state.with_lock(HISTORY_FILE, |file| {
        let mut lines = read_history_text_locked(file)
            .lines()
            .map(str::to_string)
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>();
        lines.push(query.clone());
        if lines.len() > KEEP {
            lines = lines.split_off(lines.len() - KEEP);
        }
        file.write_text_atomic(&(lines.join("\n") + "\n"))
    });
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
        outcome
            .hits
            .push(Hit::new(query.as_str(), "").meta_line(format!("qs history {number}")));
    }
    Ok(outcome)
}

pub fn rerun_query(which: &str) -> Option<String> {
    let index = which.trim().parse::<usize>().ok()?;
    let lines = read_lines();
    let newest_first = lines.into_iter().rev().collect::<Vec<_>>();
    newest_first.get(index.wrapping_sub(1)).cloned()
}

fn normalize_query(query: &str) -> String {
    // One stored query is always one safe line. Arguments can contain literal
    // CR/LF, terminal controls, and bidi overrides.
    crate::text::sanitize_terminal_line(query)
}

fn read_lines() -> Vec<String> {
    read_history_text()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn read_history_text() -> String {
    StateDir::platform()
        .read_text(HISTORY_FILE, HISTORY_LIMIT)
        .ok()
        .flatten()
        .unwrap_or_else(read_legacy_history)
}

fn read_history_text_locked(file: &LockedStateFile<'_>) -> String {
    file.read_text(HISTORY_LIMIT)
        .ok()
        .flatten()
        .unwrap_or_else(read_legacy_history)
}

fn read_legacy_history() -> String {
    // Read the legacy location until the next successful record migrates it.
    crate::cache::read_cache_text(&cache_dir().join("history"), HISTORY_LIMIT)
        .ok()
        .flatten()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::normalize_query;

    #[test]
    fn history_query_cannot_inject_lines_or_terminal_controls() {
        assert_eq!(normalize_query("one\r\ntwo\0 three"), "one two three");
        assert_eq!(normalize_query("  one\t two  "), "one two");
        assert_eq!(normalize_query("left\u{202e}right"), "left right");
    }
}
