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

use crate::http::redirect_location;
use crate::model::{Hit, Outcome};
use crate::text::encode_query;

pub fn search(name: &str, query: &str) -> Result<Outcome, String> {
    let target = format!(
        "https://duckduckgo.com/?q=!{}+{}",
        encode_query(name),
        encode_query(query)
    );
    let location = redirect_location(&target)?;
    let location = if location.starts_with("http") {
        location
    } else if location.starts_with("//") {
        format!("https:{location}")
    } else {
        format!("https://duckduckgo.com{location}")
    };
    let mut outcome = Outcome::new(name, query);
    outcome.source_url = Some(target);
    outcome.hits.push(Hit::new(query, location));
    Ok(outcome)
}
