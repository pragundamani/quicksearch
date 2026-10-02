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
