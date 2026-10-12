//! Deterministic query analysis for semantic search.
//!
//! This module deliberately describes concepts, not library APIs.  Ranking and
//! provider-specific mappings belong in later layers.
#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Facet {
    Direction(Direction),
    Ownership(Ownership),
    Arithmetic(Arithmetic),
    Representation(Representation),
    Effect(Effect),
    Cardinality(Cardinality),
    Comparison(Comparison),
    Archive(Archive),
    Operation(Operation),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Direction {
    First,
    Last,
    Prefix,
    Suffix,
    Forward,
    Backward,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Ownership {
    Owning,
    Unique,
    Shared,
    Weak,
    Borrowed,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Arithmetic {
    Checked,
    Wrapping,
    Saturating,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Representation {
    Text,
    Bytes,
    Regex,
    Shell,
    Url,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Effect {
    Pure,
    Monadic,
    Async,
    Blocking,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Cardinality {
    One,
    All,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Comparison {
    CaseSensitive,
    CaseInsensitive,
    ConstantTime,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Archive {
    Gzip,
    Zip,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Operation {
    Find,
    Remove,
    Read,
    Write,
    Split,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct QueryPlan {
    pub positive_terms: Vec<String>,
    pub negative_terms: Vec<String>,
    pub positive_facets: Vec<Facet>,
    pub negative_facets: Vec<Facet>,
    pub phrases: Vec<String>,
    pub identifiers: Vec<String>,
}

#[derive(Debug)]
struct Word {
    text: String,
    boundary_before: bool,
    negation_operator: bool,
}

/// Build a stable, provider-independent description of a programmer query.
pub(crate) fn plan_query(query: &str) -> QueryPlan {
    let (words, phrases, identifiers) = lex(query);
    let mut negative = vec![false; words.len()];
    let mut excluded = vec![false; words.len()];

    let mut index = 0;
    while index < words.len() {
        if !words[index].negation_operator {
            index += 1;
            continue;
        }
        let one = words[index].text.as_str();
        let two = words.get(index + 1).map(|word| word.text.as_str());
        let (operator_len, is_not_only) = match (one, two) {
            ("not", Some("only")) => (2, true),
            ("rather", Some("than")) | ("instead", Some("of")) | ("but", Some("not")) => (2, false),
            ("not" | "without", _) => (1, false),
            _ => (0, false),
        };
        if operator_len == 0 {
            index += 1;
            continue;
        }
        for item in excluded.iter_mut().skip(index).take(operator_len) {
            *item = true;
        }
        index += operator_len;
        if is_not_only {
            continue;
        }

        let start = index;
        while index < words.len() {
            if index > start
                && (words[index].boundary_before || is_span_delimiter(&words[index].text))
            {
                break;
            }
            negative[index] = true;
            index += 1;
        }
    }

    let mut plan = QueryPlan {
        phrases,
        identifiers,
        ..QueryPlan::default()
    };
    for (index, word) in words.iter().enumerate() {
        if excluded[index] || is_noise(&word.text) {
            continue;
        }
        let terms = if negative[index] {
            &mut plan.negative_terms
        } else {
            &mut plan.positive_terms
        };
        push_unique(terms, word.text.clone());
    }

    for index in 0..words.len() {
        if excluded[index] {
            continue;
        }
        if let Some((facet, width)) = facet_at(&words, index) {
            if (index..index + width)
                .all(|item| negative[item] == negative[index] && !excluded[item])
            {
                let facets = if negative[index] {
                    &mut plan.negative_facets
                } else {
                    &mut plan.positive_facets
                };
                push_unique(facets, facet);
            }
        }
    }
    plan
}

fn lex(query: &str) -> (Vec<Word>, Vec<String>, Vec<String>) {
    let mut words = Vec::new();
    let mut phrases = Vec::new();
    let mut identifiers = Vec::new();
    let chars: Vec<char> = query.chars().collect();
    let mut index = 0;
    let mut boundary = false;
    while index < chars.len() {
        if chars[index] == '"' || chars[index] == '\'' {
            let quote = chars[index];
            index += 1;
            let start = index;
            while index < chars.len() && chars[index] != quote {
                index += 1;
            }
            let phrase: String = chars[start..index].iter().collect();
            let phrase_words = split_words(&phrase);
            if !phrase_words.is_empty() {
                push_unique(&mut phrases, phrase_words.join(" "));
            }
            for text in phrase_words {
                words.push(Word {
                    text,
                    boundary_before: std::mem::take(&mut boundary),
                    negation_operator: true,
                });
            }
            index += usize::from(index < chars.len());
        } else if chars[index].is_alphanumeric() || chars[index] == '_' {
            let start = index;
            index += 1;
            while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_') {
                index += 1;
            }
            let raw: String = chars[start..index].iter().collect();
            if looks_like_identifier(&raw) {
                push_unique(&mut identifiers, raw.clone());
            }
            let components = split_identifier(&raw);
            let can_negate = components.len() == 1;
            for text in components {
                words.push(Word {
                    text,
                    boundary_before: std::mem::take(&mut boundary),
                    negation_operator: can_negate,
                });
            }
        } else {
            if matches!(
                chars[index],
                ',' | ';' | '.' | '!' | '?' | '(' | ')' | '[' | ']' | '{' | '}'
            ) {
                boundary = true;
            }
            index += 1;
        }
    }
    (words, phrases, identifiers)
}

fn split_words(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    for character in text.chars() {
        if character.is_alphanumeric() || character == '_' {
            current.push(character);
        } else if !current.is_empty() {
            result.extend(split_identifier(&current));
            current.clear();
        }
    }
    if !current.is_empty() {
        result.extend(split_identifier(&current));
    }
    result
}

fn split_identifier(raw: &str) -> Vec<String> {
    let chars: Vec<char> = raw.chars().collect();
    let mut parts = Vec::new();
    let mut start = 0;
    for index in 0..=chars.len() {
        let split = index == chars.len()
            || chars[index] == '_'
            || (index > start
                && chars[index].is_uppercase()
                && (chars[index - 1].is_lowercase()
                    || (index + 1 < chars.len() && chars[index + 1].is_lowercase())));
        if split {
            if start < index {
                parts.push(
                    chars[start..index]
                        .iter()
                        .collect::<String>()
                        .to_lowercase(),
                );
            }
            start = index + usize::from(index < chars.len() && chars[index] == '_');
        }
    }
    parts
}

fn looks_like_identifier(raw: &str) -> bool {
    raw.contains('_') || raw.chars().skip(1).any(char::is_uppercase)
}

fn is_span_delimiter(word: &str) -> bool {
    matches!(
        word,
        "and" | "but" | "then" | "while" | "with" | "using" | "for" | "rather" | "instead"
    )
}

fn is_noise(word: &str) -> bool {
    matches!(
        word,
        "a" | "an"
            | "the"
            | "to"
            | "of"
            | "than"
            | "but"
            | "also"
            | "only"
            | "and"
            | "or"
            | "with"
            | "using"
            | "for"
            | "then"
            | "while"
            | "please"
    )
}

fn facet_at(words: &[Word], index: usize) -> Option<(Facet, usize)> {
    let word = words[index].text.as_str();
    let next = words.get(index + 1).map(|item| item.text.as_str());
    let compound = match (word, next) {
        ("case", Some("sensitive")) => Some(Facet::Comparison(Comparison::CaseSensitive)),
        ("case", Some("insensitive")) => Some(Facet::Comparison(Comparison::CaseInsensitive)),
        ("constant", Some("time")) => Some(Facet::Comparison(Comparison::ConstantTime)),
        ("move", Some("only")) => Some(Facet::Ownership(Ownership::Unique)),
        ("exactly", Some("one")) | ("at", Some("most")) => {
            Some(Facet::Cardinality(Cardinality::One))
        }
        ("begins", Some("with")) | ("starts", Some("with")) => {
            Some(Facet::Direction(Direction::Prefix))
        }
        _ => None,
    };
    if let Some(facet) = compound {
        return Some((facet, 2));
    }
    let facet = match word {
        "first" => Facet::Direction(Direction::First),
        "last" => Facet::Direction(Direction::Last),
        "prefix" => Facet::Direction(Direction::Prefix),
        "suffix" => Facet::Direction(Direction::Suffix),
        "forward" => Facet::Direction(Direction::Forward),
        "backward" => Facet::Direction(Direction::Backward),
        "own" | "owning" | "owner" | "ownership" => Facet::Ownership(Ownership::Owning),
        "unique" | "exclusive" => Facet::Ownership(Ownership::Unique),
        "shared" => Facet::Ownership(Ownership::Shared),
        "weak" => Facet::Ownership(Ownership::Weak),
        "borrowed" => Facet::Ownership(Ownership::Borrowed),
        "checked" => Facet::Arithmetic(Arithmetic::Checked),
        "wrapping" => Facet::Arithmetic(Arithmetic::Wrapping),
        "saturating" => Facet::Arithmetic(Arithmetic::Saturating),
        "text" => Facet::Representation(Representation::Text),
        "bytes" => Facet::Representation(Representation::Bytes),
        "regex" => Facet::Representation(Representation::Regex),
        "shell" => Facet::Representation(Representation::Shell),
        "url" => Facet::Representation(Representation::Url),
        "pure" => Facet::Effect(Effect::Pure),
        "monadic" => Facet::Effect(Effect::Monadic),
        "async" => Facet::Effect(Effect::Async),
        "blocking" => Facet::Effect(Effect::Blocking),
        "single" => Facet::Cardinality(Cardinality::One),
        "all" => Facet::Cardinality(Cardinality::All),
        "gzip" => Facet::Archive(Archive::Gzip),
        "zip" => Facet::Archive(Archive::Zip),
        "find" | "locate" | "search" => Facet::Operation(Operation::Find),
        "remove" | "strip" | "delete" => Facet::Operation(Operation::Remove),
        "read" | "load" => Facet::Operation(Operation::Read),
        "write" | "save" => Facet::Operation(Operation::Write),
        "split" | "tokenize" => Facet::Operation(Operation::Split),
        _ => return None,
    };
    Some((facet, 1))
}

fn push_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
    if !items.contains(&item) {
        items.push(item);
    }
}

/// Return the longest API-like qualified identifier embedded in prose.
pub(crate) fn extract_qualified_identifier(query: &str) -> Option<String> {
    query
        .split_whitespace()
        .filter_map(|raw| {
            let token = raw
                .trim_matches(|ch: char| {
                    !ch.is_ascii_alphanumeric() && !matches!(ch, '_' | ':' | '.')
                })
                .trim_end_matches(['.', ':']);
            let separator = if token.contains("::") {
                "::"
            } else if token.contains('.') {
                "."
            } else {
                return None;
            };
            let parts: Vec<_> = token.split(separator).collect();
            let valid_parts = parts.len() >= 2
                && parts.iter().all(|part| {
                    !part.is_empty()
                        && part
                            .chars()
                            .next()
                            .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
                        && part
                            .chars()
                            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
                });
            let looks_like_domain = separator == "."
                && parts.last().is_some_and(|part| {
                    matches!(
                        part.to_ascii_lowercase().as_str(),
                        "com" | "org" | "net" | "io" | "dev" | "edu" | "gov"
                    )
                });
            (valid_parts && !looks_like_domain).then(|| token.to_string())
        })
        .max_by_key(String::len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualified_identifier_is_extracted_from_surplus_prose() {
        assert_eq!(
            extract_qualified_identifier("official docs show std::path::Path::ancestors now")
                .as_deref(),
            Some("std::path::Path::ancestors")
        );
        assert_eq!(
            extract_qualified_identifier("quick example Data.Set.fromList.").as_deref(),
            Some("Data.Set.fromList")
        );
        assert_eq!(
            extract_qualified_identifier("see pathlib.Path.read_text:").as_deref(),
            Some("pathlib.Path.read_text")
        );
        assert_eq!(
            extract_qualified_identifier("quick example Data.Set.fromList please").as_deref(),
            Some("Data.Set.fromList")
        );
        assert_eq!(extract_qualified_identifier("version 3.15 docs"), None);
        assert_eq!(extract_qualified_identifier("visit example.com now"), None);
        assert_eq!(
            extract_qualified_identifier("broken foo..bar foo::::bar"),
            None
        );
    }

    #[test]
    fn camel_case_not_is_not_a_negation_operator() {
        let plan = plan_query("kotlin.collections.mapNotNull");
        assert!(plan.negative_terms.is_empty());
        assert!(plan.positive_terms.contains(&"not".to_string()));
        assert!(plan.positive_terms.contains(&"null".to_string()));

        let natural = plan_query("array method not map");
        assert_eq!(natural.negative_terms, vec!["map"]);
    }

    #[test]
    fn contrast_forms_have_bounded_negative_spans() {
        let cases = [
            (
                "find first, not last, async",
                &["last"][..],
                &["find", "first", "async"][..],
            ),
            (
                "forward rather than backward with bytes",
                &["backward"],
                &["forward", "bytes"],
            ),
            (
                "prefix instead of suffix; pure",
                &["suffix"],
                &["prefix", "pure"],
            ),
            (
                "read but not write then save",
                &["write"],
                &["read", "save"],
            ),
            (
                "text without regex and shell",
                &["regex"],
                &["text", "shell"],
            ),
        ];
        for (query, negative, positive) in cases {
            let plan = plan_query(query);
            assert_eq!(plan.negative_terms, negative, "{query}");
            assert_eq!(plan.positive_terms, positive, "{query}");
        }
    }

    #[test]
    fn not_only_is_positive() {
        let plan = plan_query("not only checked but also saturating");
        assert!(plan.negative_terms.is_empty());
        assert_eq!(plan.positive_terms, ["checked", "saturating"]);
        assert_eq!(
            plan.positive_facets,
            [
                Facet::Arithmetic(Arithmetic::Checked),
                Facet::Arithmetic(Arithmetic::Saturating)
            ]
        );
    }

    #[test]
    fn camel_case_underscores_punctuation_and_phrases_are_tokenized() {
        let plan = plan_query("parseURL_value, findFirst 'constant time'");
        assert_eq!(plan.identifiers, ["parseURL_value", "findFirst"]);
        assert_eq!(plan.phrases, ["constant time"]);
        assert_eq!(
            plan.positive_terms,
            ["parse", "url", "value", "find", "first", "constant", "time"]
        );
        assert!(plan
            .positive_facets
            .contains(&Facet::Direction(Direction::First)));
        assert!(plan
            .positive_facets
            .contains(&Facet::Comparison(Comparison::ConstantTime)));
    }

    #[test]
    fn generic_ontology_maps_each_family() {
        let cases = [
            ("last", Facet::Direction(Direction::Last)),
            ("weak", Facet::Ownership(Ownership::Weak)),
            ("move-only", Facet::Ownership(Ownership::Unique)),
            ("wrapping", Facet::Arithmetic(Arithmetic::Wrapping)),
            ("bytes", Facet::Representation(Representation::Bytes)),
            ("monadic", Facet::Effect(Effect::Monadic)),
            ("all", Facet::Cardinality(Cardinality::All)),
            (
                "case-insensitive",
                Facet::Comparison(Comparison::CaseInsensitive),
            ),
            ("gzip", Facet::Archive(Archive::Gzip)),
            ("locate", Facet::Operation(Operation::Find)),
            ("strip", Facet::Operation(Operation::Remove)),
            ("load", Facet::Operation(Operation::Read)),
            ("save", Facet::Operation(Operation::Write)),
            ("tokenize", Facet::Operation(Operation::Split)),
        ];
        for (query, facet) in cases {
            assert_eq!(plan_query(query).positive_facets, [facet], "{query}");
        }
    }

    #[test]
    fn held_out_style_contrasts_keep_facet_polarity() {
        let cases = [
            (
                "owning not borrowed",
                Facet::Ownership(Ownership::Owning),
                Facet::Ownership(Ownership::Borrowed),
            ),
            (
                "unique rather than shared",
                Facet::Ownership(Ownership::Unique),
                Facet::Ownership(Ownership::Shared),
            ),
            (
                "saturating rather than checked",
                Facet::Arithmetic(Arithmetic::Saturating),
                Facet::Arithmetic(Arithmetic::Checked),
            ),
            (
                "async instead of blocking",
                Facet::Effect(Effect::Async),
                Facet::Effect(Effect::Blocking),
            ),
            (
                "single but not all",
                Facet::Cardinality(Cardinality::One),
                Facet::Cardinality(Cardinality::All),
            ),
            (
                "zip without gzip",
                Facet::Archive(Archive::Zip),
                Facet::Archive(Archive::Gzip),
            ),
        ];
        for (query, positive, negative) in cases {
            let plan = plan_query(query);
            assert!(plan.positive_facets.contains(&positive), "{query}");
            assert!(plan.negative_facets.contains(&negative), "{query}");
        }
    }

    #[test]
    fn operation_synonyms_share_facets() {
        for family in [
            ["find", "locate", "search"],
            ["remove", "strip", "delete"],
            ["read", "load", "read"],
            ["write", "save", "write"],
            ["split", "tokenize", "split"],
        ] {
            let expected = plan_query(family[0]).positive_facets;
            for word in family {
                assert_eq!(plan_query(word).positive_facets, expected, "{word}");
            }
        }
    }
}
