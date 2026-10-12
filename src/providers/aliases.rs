// qs looks things up from the terminal.
// Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Small, deliberately conservative cross-language alias table.
//!
//! This module only identifies a semantic intent and suggests search terms. It
//! does not fetch documentation. Callers may use `landing` directly when it is
//! present, or pass `target_terms` to their normal documentation provider.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Go,
    Python,
}

impl Language {
    #[cfg(test)]
    pub const ALL: [Self; 4] = [Self::C, Self::Cpp, Self::Go, Self::Python];
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Intent {
    ParseInteger,
    ReadFile,
    WriteFile,
    RemoveFile,
    TemporaryFile,
    WalkFiles,
    SortByKey,
    EraseMapKey,
    MergeMaps,
    ElapsedTime,
    CommandLineArguments,
    DefineRecord,
    StorageDuration,
    SortSequence,
    FormatDateTime,
}

impl Intent {
    #[cfg(test)]
    pub const ALL: [Self; 15] = [
        Self::ParseInteger,
        Self::ReadFile,
        Self::WriteFile,
        Self::RemoveFile,
        Self::TemporaryFile,
        Self::WalkFiles,
        Self::SortByKey,
        Self::EraseMapKey,
        Self::MergeMaps,
        Self::ElapsedTime,
        Self::CommandLineArguments,
        Self::DefineRecord,
        Self::StorageDuration,
        Self::SortSequence,
        Self::FormatDateTime,
    ];
}

/// Each non-empty slot must match at least one complete query token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticCues {
    pub action: &'static [&'static str],
    pub object: &'static [&'static str],
    pub qualifier: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Confidence {
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Alias {
    pub language: Language,
    pub intent: Intent,
    pub cues: SemanticCues,
    pub target_terms: &'static [&'static str],
    /// An authoritative language or standard-library documentation page.
    pub landing: Option<&'static str>,
    pub confidence: Confidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AliasMatch {
    pub alias: &'static Alias,
    pub matched_slots: u8,
}

const PARSE_INTEGER: SemanticCues = SemanticCues {
    action: &["parse", "convert"],
    object: &["integer", "int"],
    qualifier: &[],
};
const READ_FILE: SemanticCues = SemanticCues {
    action: &["read", "load"],
    object: &["file"],
    qualifier: &[],
};
const WRITE_FILE: SemanticCues = SemanticCues {
    action: &["write", "save"],
    object: &["file"],
    qualifier: &[],
};
const REMOVE_FILE: SemanticCues = SemanticCues {
    action: &["remove", "delete", "unlink"],
    object: &["file"],
    qualifier: &[],
};
const TEMPORARY_FILE: SemanticCues = SemanticCues {
    action: &["temporary", "temp"],
    object: &["file"],
    qualifier: &[],
};
const WALK_FILES: SemanticCues = SemanticCues {
    action: &["walk", "traverse"],
    object: &["file", "directory", "tree"],
    qualifier: &[],
};
const SORT_BY_KEY: SemanticCues = SemanticCues {
    action: &["sort", "order"],
    object: &["key"],
    qualifier: &[],
};
const ERASE_MAP_KEY: SemanticCues = SemanticCues {
    action: &["erase", "remove", "delete"],
    object: &["map", "dictionary", "dict"],
    qualifier: &["key", "entry"],
};
const MERGE_MAPS: SemanticCues = SemanticCues {
    action: &["merge", "combine", "update"],
    object: &["map", "dictionary", "dict"],
    qualifier: &[],
};
const ELAPSED_TIME: SemanticCues = SemanticCues {
    action: &["elapsed", "duration"],
    object: &["time", "timer"],
    qualifier: &[],
};
const COMMAND_LINE_ARGUMENTS: SemanticCues = SemanticCues {
    action: &[],
    object: &["argv", "argument", "args"],
    qualifier: &["commandline", "cli", "command"],
};
const DEFINE_RECORD: SemanticCues = SemanticCues {
    action: &[],
    object: &["struct", "structure", "record"],
    qualifier: &[],
};
const STORAGE_DURATION: SemanticCues = SemanticCues {
    action: &[],
    object: &["storage"],
    qualifier: &["duration", "lifetime"],
};
const SORT_SEQUENCE: SemanticCues = SemanticCues {
    action: &["sort", "order"],
    object: &["array", "sequence", "elements"],
    qualifier: &["comparator", "comparison", "callback"],
};
const FORMAT_DATE_TIME: SemanticCues = SemanticCues {
    action: &["format", "formatting"],
    object: &["date", "calendar"],
    qualifier: &["time"],
};

macro_rules! alias {
    ($language:ident, $intent:ident, $cues:ident, [$($term:literal),+], $landing:expr, $confidence:ident) => {
        Alias { language: Language::$language, intent: Intent::$intent, cues: $cues,
            target_terms: &[$($term),+], landing: $landing, confidence: Confidence::$confidence }
    };
}

/// The table is rectangular: every supported intent has one entry per language.
pub const ALIASES: &[Alias] = &[
    alias!(
        C,
        ParseInteger,
        PARSE_INTEGER,
        ["strtol", "strtoul"],
        Some("https://en.cppreference.com/w/c/string/byte/strtol"),
        High
    ),
    alias!(
        Cpp,
        ParseInteger,
        PARSE_INTEGER,
        ["std::stoi", "std::from_chars"],
        Some("https://en.cppreference.com/w/cpp/string/basic_string/stol"),
        High
    ),
    alias!(
        Go,
        ParseInteger,
        PARSE_INTEGER,
        ["strconv.Atoi", "strconv.ParseInt"],
        Some("https://pkg.go.dev/strconv#ParseInt"),
        High
    ),
    alias!(
        Python,
        ParseInteger,
        PARSE_INTEGER,
        ["int"],
        Some("https://docs.python.org/3/library/functions.html#int"),
        High
    ),
    alias!(
        C,
        ReadFile,
        READ_FILE,
        ["fread", "fgets"],
        Some("https://en.cppreference.com/w/c/io/fread"),
        High
    ),
    alias!(
        Cpp,
        ReadFile,
        READ_FILE,
        ["std::ifstream", "std::filesystem"],
        Some("https://en.cppreference.com/w/cpp/io/basic_ifstream"),
        High
    ),
    alias!(
        Go,
        ReadFile,
        READ_FILE,
        ["os.ReadFile"],
        Some("https://pkg.go.dev/os#ReadFile"),
        High
    ),
    alias!(
        Python,
        ReadFile,
        READ_FILE,
        ["Path.read_text", "open"],
        Some("https://docs.python.org/3/library/pathlib.html#pathlib.Path.read_text"),
        High
    ),
    alias!(
        C,
        WriteFile,
        WRITE_FILE,
        ["fwrite", "fputs"],
        Some("https://en.cppreference.com/w/c/io/fwrite"),
        High
    ),
    alias!(
        Cpp,
        WriteFile,
        WRITE_FILE,
        ["std::ofstream"],
        Some("https://en.cppreference.com/w/cpp/io/basic_ofstream"),
        High
    ),
    alias!(
        Go,
        WriteFile,
        WRITE_FILE,
        ["os.WriteFile"],
        Some("https://pkg.go.dev/os#WriteFile"),
        High
    ),
    alias!(
        Python,
        WriteFile,
        WRITE_FILE,
        ["Path.write_text", "open"],
        Some("https://docs.python.org/3/library/pathlib.html#pathlib.Path.write_text"),
        High
    ),
    alias!(
        C,
        RemoveFile,
        REMOVE_FILE,
        ["remove"],
        Some("https://en.cppreference.com/w/c/io/remove"),
        High
    ),
    alias!(
        Cpp,
        RemoveFile,
        REMOVE_FILE,
        ["std::filesystem::remove"],
        Some("https://en.cppreference.com/w/cpp/filesystem/remove"),
        High
    ),
    alias!(
        Go,
        RemoveFile,
        REMOVE_FILE,
        ["os.Remove"],
        Some("https://pkg.go.dev/os#Remove"),
        High
    ),
    alias!(
        Python,
        RemoveFile,
        REMOVE_FILE,
        ["Path.unlink", "os.remove"],
        Some("https://docs.python.org/3/library/pathlib.html#pathlib.Path.unlink"),
        High
    ),
    alias!(
        C,
        TemporaryFile,
        TEMPORARY_FILE,
        ["tmpfile", "mkstemp"],
        Some("https://en.cppreference.com/w/c/io/tmpfile"),
        Medium
    ),
    alias!(
        Cpp,
        TemporaryFile,
        TEMPORARY_FILE,
        ["std::filesystem::temp_directory_path", "tmpfile"],
        Some("https://en.cppreference.com/w/cpp/filesystem/temp_directory_path"),
        Medium
    ),
    alias!(
        Go,
        TemporaryFile,
        TEMPORARY_FILE,
        ["os.CreateTemp", "os.MkdirTemp"],
        Some("https://pkg.go.dev/os#CreateTemp"),
        High
    ),
    alias!(
        Python,
        TemporaryFile,
        TEMPORARY_FILE,
        ["tempfile.TemporaryFile", "tempfile.NamedTemporaryFile"],
        Some("https://docs.python.org/3/library/tempfile.html"),
        High
    ),
    alias!(
        C,
        WalkFiles,
        WALK_FILES,
        ["opendir", "readdir", "nftw"],
        Some("https://pubs.opengroup.org/onlinepubs/9699919799/functions/nftw.html"),
        Medium
    ),
    alias!(
        Cpp,
        WalkFiles,
        WALK_FILES,
        ["std::filesystem::recursive_directory_iterator"],
        Some("https://en.cppreference.com/w/cpp/filesystem/recursive_directory_iterator"),
        High
    ),
    alias!(
        Go,
        WalkFiles,
        WALK_FILES,
        ["filepath.WalkDir"],
        Some("https://pkg.go.dev/path/filepath#WalkDir"),
        High
    ),
    alias!(
        Python,
        WalkFiles,
        WALK_FILES,
        ["os.walk", "Path.rglob"],
        Some("https://docs.python.org/3/library/os.html#os.walk"),
        High
    ),
    alias!(
        C,
        SortByKey,
        SORT_BY_KEY,
        ["qsort comparator"],
        Some("https://en.cppreference.com/w/c/algorithm/qsort"),
        Medium
    ),
    alias!(
        Cpp,
        SortByKey,
        SORT_BY_KEY,
        ["std::sort comparator", "std::ranges::sort projection"],
        Some("https://en.cppreference.com/w/cpp/algorithm/ranges/sort"),
        High
    ),
    alias!(
        Go,
        SortByKey,
        SORT_BY_KEY,
        ["slices.SortFunc", "sort.Slice"],
        Some("https://pkg.go.dev/slices#SortFunc"),
        High
    ),
    alias!(
        Python,
        SortByKey,
        SORT_BY_KEY,
        ["sorted key", "list.sort key"],
        Some("https://docs.python.org/3/howto/sorting.html#key-functions"),
        High
    ),
    alias!(
        C,
        EraseMapKey,
        ERASE_MAP_KEY,
        ["hash table delete"],
        None,
        Medium
    ),
    alias!(
        Cpp,
        EraseMapKey,
        ERASE_MAP_KEY,
        ["std::map::erase", "std::unordered_map::erase"],
        Some("https://en.cppreference.com/w/cpp/container/map/erase"),
        High
    ),
    alias!(
        Go,
        EraseMapKey,
        ERASE_MAP_KEY,
        ["delete"],
        Some("https://go.dev/ref/spec#Deletion_of_map_elements"),
        High
    ),
    alias!(
        Python,
        EraseMapKey,
        ERASE_MAP_KEY,
        ["del mapping[key]", "dict.pop"],
        Some("https://docs.python.org/3/library/stdtypes.html#mapping-types-dict"),
        High
    ),
    alias!(C, MergeMaps, MERGE_MAPS, ["hash table merge"], None, Medium),
    alias!(
        Cpp,
        MergeMaps,
        MERGE_MAPS,
        ["std::map::merge", "std::unordered_map::merge"],
        Some("https://en.cppreference.com/w/cpp/container/map/merge"),
        High
    ),
    alias!(
        Go,
        MergeMaps,
        MERGE_MAPS,
        ["for range map assignment"],
        Some("https://go.dev/ref/spec#For_statements"),
        Medium
    ),
    alias!(
        Python,
        MergeMaps,
        MERGE_MAPS,
        ["dict.update", "mapping union"],
        Some("https://docs.python.org/3/library/stdtypes.html#dict.update"),
        High
    ),
    alias!(
        C,
        ElapsedTime,
        ELAPSED_TIME,
        ["clock_gettime", "timespec"],
        Some("https://pubs.opengroup.org/onlinepubs/9699919799/functions/clock_gettime.html"),
        Medium
    ),
    alias!(
        Cpp,
        ElapsedTime,
        ELAPSED_TIME,
        ["std::chrono::steady_clock", "duration"],
        Some("https://en.cppreference.com/w/cpp/chrono/steady_clock"),
        High
    ),
    alias!(
        Go,
        ElapsedTime,
        ELAPSED_TIME,
        ["time.Since", "time.Now"],
        Some("https://pkg.go.dev/time#Since"),
        High
    ),
    alias!(
        Python,
        ElapsedTime,
        ELAPSED_TIME,
        ["time.monotonic", "time.perf_counter"],
        Some("https://docs.python.org/3/library/time.html#time.perf_counter"),
        High
    ),
    alias!(
        C,
        CommandLineArguments,
        COMMAND_LINE_ARGUMENTS,
        ["argc", "argv"],
        Some("https://en.cppreference.com/w/c/language/main_function"),
        High
    ),
    alias!(
        Cpp,
        CommandLineArguments,
        COMMAND_LINE_ARGUMENTS,
        ["argc", "argv"],
        Some("https://en.cppreference.com/w/cpp/language/main_function"),
        High
    ),
    alias!(
        Go,
        CommandLineArguments,
        COMMAND_LINE_ARGUMENTS,
        ["os.Args", "flag"],
        Some("https://pkg.go.dev/os#pkg-variables"),
        High
    ),
    alias!(
        Python,
        CommandLineArguments,
        COMMAND_LINE_ARGUMENTS,
        ["sys.argv", "argparse"],
        Some("https://docs.python.org/3/library/argparse.html"),
        High
    ),
    alias!(
        C,
        DefineRecord,
        DEFINE_RECORD,
        ["Struct declaration", "struct"],
        Some("https://en.cppreference.com/w/c/language/struct"),
        High
    ),
    alias!(
        Cpp,
        DefineRecord,
        DEFINE_RECORD,
        ["class declaration", "struct declaration"],
        Some("https://en.cppreference.com/w/cpp/language/class"),
        High
    ),
    alias!(
        Go,
        DefineRecord,
        DEFINE_RECORD,
        ["struct types"],
        Some("https://go.dev/ref/spec#Struct_types"),
        High
    ),
    alias!(
        Python,
        DefineRecord,
        DEFINE_RECORD,
        ["dataclasses", "namedtuple"],
        Some("https://docs.python.org/3/library/dataclasses.html"),
        Medium
    ),
    alias!(
        C,
        StorageDuration,
        STORAGE_DURATION,
        [
            "automatic, static, thread, and allocated storage duration",
            "Static storage duration",
            "Thread storage duration"
        ],
        Some("https://en.cppreference.com/w/c/language/storage_duration"),
        High
    ),
    alias!(
        Cpp,
        StorageDuration,
        STORAGE_DURATION,
        ["storage duration"],
        Some("https://en.cppreference.com/w/cpp/language/storage_duration"),
        High
    ),
    alias!(
        Go,
        StorageDuration,
        STORAGE_DURATION,
        ["variable lifetime"],
        Some("https://go.dev/ref/spec#Variables"),
        Medium
    ),
    alias!(
        Python,
        StorageDuration,
        STORAGE_DURATION,
        ["object lifetime"],
        Some("https://docs.python.org/3/reference/datamodel.html#objects-values-and-types"),
        Medium
    ),
    alias!(
        C,
        SortSequence,
        SORT_SEQUENCE,
        ["qsort", "qsort comparator"],
        Some("https://en.cppreference.com/w/c/algorithm/qsort"),
        High
    ),
    alias!(
        Cpp,
        SortSequence,
        SORT_SEQUENCE,
        ["std::sort comparator"],
        Some("https://en.cppreference.com/w/cpp/algorithm/sort"),
        High
    ),
    alias!(
        Go,
        SortSequence,
        SORT_SEQUENCE,
        ["slices.SortFunc"],
        Some("https://pkg.go.dev/slices#SortFunc"),
        High
    ),
    alias!(
        Python,
        SortSequence,
        SORT_SEQUENCE,
        ["sorted key"],
        Some("https://docs.python.org/3/howto/sorting.html#key-functions"),
        High
    ),
    alias!(
        C,
        FormatDateTime,
        FORMAT_DATE_TIME,
        ["strftime"],
        Some("https://en.cppreference.com/w/c/chrono/strftime"),
        High
    ),
    alias!(
        Cpp,
        FormatDateTime,
        FORMAT_DATE_TIME,
        ["std::put_time", "std::strftime"],
        Some("https://en.cppreference.com/w/cpp/io/manip/put_time"),
        High
    ),
    alias!(
        Go,
        FormatDateTime,
        FORMAT_DATE_TIME,
        ["time.Time.Format"],
        Some("https://pkg.go.dev/time#Time.Format"),
        High
    ),
    alias!(
        Python,
        FormatDateTime,
        FORMAT_DATE_TIME,
        ["datetime.strftime"],
        Some("https://docs.python.org/3/library/datetime.html#datetime.datetime.strftime"),
        High
    ),
];

/// Split ASCII identifiers at punctuation and lower-to-upper camel-case edges.
/// It intentionally does not do substring or edit-distance matching.
pub fn identifier_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut previous_lower_or_digit = false;
    for ch in input.chars() {
        if !ch.is_ascii_alphanumeric() {
            if !token.is_empty() {
                tokens.push(std::mem::take(&mut token));
            }
            previous_lower_or_digit = false;
            continue;
        }
        if ch.is_ascii_uppercase() && previous_lower_or_digit && !token.is_empty() {
            tokens.push(std::mem::take(&mut token));
        }
        token.push(ch.to_ascii_lowercase());
        previous_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

const WORD_FAMILIES: &[&[&str]] = &[
    &["argument", "arguments", "arg", "args"],
    &["combine", "combined", "combining"],
    &["convert", "converted", "converting", "conversion"],
    &["delete", "deleted", "deleting", "deletion"],
    &["dictionary", "dictionaries", "dict", "dicts"],
    &["directory", "directories", "dir", "dirs"],
    &["duration", "durations"],
    &["elapsed", "elapse"],
    &["entry", "entries"],
    &["file", "files"],
    &["integer", "integers", "int", "ints"],
    &["key", "keys"],
    &["load", "loaded", "loading"],
    &["map", "maps", "mapping", "mappings"],
    &["merge", "merged", "merging"],
    &["order", "ordered", "ordering"],
    &["parse", "parsed", "parsing"],
    &["read", "reads", "reading"],
    &["remove", "removed", "removing", "removal"],
    &["save", "saved", "saving"],
    &["sort", "sorted", "sorting"],
    &[
        "struct",
        "structs",
        "structure",
        "structures",
        "record",
        "records",
    ],
    &["temporary", "temp"],
    &["time", "times", "timer", "timers"],
    &["traverse", "traversed", "traversing", "traversal"],
    &["update", "updated", "updating"],
    &["walk", "walked", "walking"],
    &["write", "writes", "writing", "written"],
];

pub fn word_family_matches(left: &str, right: &str) -> bool {
    if left.eq_ignore_ascii_case(right) {
        return true;
    }
    WORD_FAMILIES.iter().any(|family| {
        family.iter().any(|word| word.eq_ignore_ascii_case(left))
            && family.iter().any(|word| word.eq_ignore_ascii_case(right))
    })
}

fn slot_matches(tokens: &[String], slot: &[&str]) -> bool {
    slot.is_empty()
        || slot
            .iter()
            .any(|cue| tokens.iter().any(|token| word_family_matches(token, cue)))
}

fn alias_has_unsafe_surplus(tokens: &[String], cues: SemanticCues) -> bool {
    const STOP_WORDS: &[&str] = &["a", "an", "the", "to", "from", "of", "by", "with", "and"];
    const AMBIGUOUS_MODIFIERS: &[&str] = &[
        "permission",
        "permissions",
        "mode",
        "owner",
        "ownership",
        "metadata",
        "flags",
    ];
    let slots = [cues.action, cues.object, cues.qualifier];
    let unexplained: Vec<&str> = tokens
        .iter()
        .map(String::as_str)
        .filter(|token| !STOP_WORDS.contains(token))
        .filter(|token| {
            !slots
                .iter()
                .flat_map(|slot| slot.iter())
                .any(|cue| word_family_matches(token, cue))
        })
        .collect();
    let maximum_unexplained = if slots.iter().filter(|slot| !slot.is_empty()).count() == 1 {
        0
    } else {
        2
    };
    unexplained.len() > maximum_unexplained
        || unexplained
            .iter()
            .any(|token| AMBIGUOUS_MODIFIERS.contains(token))
}

/// Return all rules whose non-empty semantic slots match complete tokens.
pub fn find_aliases(language: Language, query: &str) -> Vec<AliasMatch> {
    let tokens = identifier_tokens(query);
    if tokens.is_empty() {
        return Vec::new();
    }
    ALIASES
        .iter()
        .filter(|alias| alias.language == language)
        .filter_map(|alias| {
            let slots = [alias.cues.action, alias.cues.object, alias.cues.qualifier];
            if slots.iter().all(|slot| slot_matches(&tokens, slot))
                && !alias_has_unsafe_surplus(&tokens, alias.cues)
            {
                Some(AliasMatch {
                    alias,
                    matched_slots: slots.iter().filter(|slot| !slot.is_empty()).count() as u8,
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn find_alias(language: Language, query: &str) -> Option<AliasMatch> {
    find_aliases(language, query)
        .into_iter()
        .max_by_key(|found| (found.matched_slots, found.alias.confidence))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GoLanding {
    pub name: &'static str,
    pub landing: &'static str,
}

/// Exact names of Go predeclared functions. Types, constants, and aliases are
/// omitted so a caller can label these specifically as callable built-ins.
pub const GO_BUILTINS: &[GoLanding] = &[
    GoLanding {
        name: "append",
        landing: "https://pkg.go.dev/builtin#append",
    },
    GoLanding {
        name: "cap",
        landing: "https://pkg.go.dev/builtin#cap",
    },
    GoLanding {
        name: "clear",
        landing: "https://pkg.go.dev/builtin#clear",
    },
    GoLanding {
        name: "close",
        landing: "https://pkg.go.dev/builtin#close",
    },
    GoLanding {
        name: "complex",
        landing: "https://pkg.go.dev/builtin#complex",
    },
    GoLanding {
        name: "copy",
        landing: "https://pkg.go.dev/builtin#copy",
    },
    GoLanding {
        name: "delete",
        landing: "https://pkg.go.dev/builtin#delete",
    },
    GoLanding {
        name: "imag",
        landing: "https://pkg.go.dev/builtin#imag",
    },
    GoLanding {
        name: "len",
        landing: "https://pkg.go.dev/builtin#len",
    },
    GoLanding {
        name: "make",
        landing: "https://pkg.go.dev/builtin#make",
    },
    GoLanding {
        name: "max",
        landing: "https://pkg.go.dev/builtin#max",
    },
    GoLanding {
        name: "min",
        landing: "https://pkg.go.dev/builtin#min",
    },
    GoLanding {
        name: "new",
        landing: "https://pkg.go.dev/builtin#new",
    },
    GoLanding {
        name: "panic",
        landing: "https://pkg.go.dev/builtin#panic",
    },
    GoLanding {
        name: "print",
        landing: "https://pkg.go.dev/builtin#print",
    },
    GoLanding {
        name: "println",
        landing: "https://pkg.go.dev/builtin#println",
    },
    GoLanding {
        name: "real",
        landing: "https://pkg.go.dev/builtin#real",
    },
    GoLanding {
        name: "recover",
        landing: "https://pkg.go.dev/builtin#recover",
    },
];

pub fn go_builtin(name: &str) -> Option<&'static GoLanding> {
    GO_BUILTINS.iter().find(|item| item.name == name)
}

pub const GO_CONCEPTS: &[GoLanding] = &[
    GoLanding {
        name: "array",
        landing: "https://go.dev/ref/spec#Array_types",
    },
    GoLanding {
        name: "channel",
        landing: "https://go.dev/ref/spec#Channel_types",
    },
    GoLanding {
        name: "defer",
        landing: "https://go.dev/ref/spec#Defer_statements",
    },
    GoLanding {
        name: "goroutine",
        landing: "https://go.dev/ref/spec#Go_statements",
    },
    GoLanding {
        name: "interface",
        landing: "https://go.dev/ref/spec#Interface_types",
    },
    GoLanding {
        name: "map",
        landing: "https://go.dev/ref/spec#Map_types",
    },
    GoLanding {
        name: "select",
        landing: "https://go.dev/ref/spec#Select_statements",
    },
    GoLanding {
        name: "slice",
        landing: "https://go.dev/ref/spec#Slice_types",
    },
];

pub fn go_concept(name: &str) -> Option<&'static GoLanding> {
    GO_CONCEPTS.iter().find(|item| item.name == name)
}

/// A Go language concept and the complete query forms that identify it.
///
/// Query forms are deliberately explicit. [`find_go_language_concept`] does
/// not ignore extra tokens, so package APIs such as `range.Int` cannot be
/// mistaken for the `range` language construct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GoConcept {
    /// A stable, machine-readable identifier.
    pub name: &'static str,
    /// A short label suitable for a search result title.
    pub label: &'static str,
    /// Official Go specification or tutorial documentation.
    pub landing: &'static str,
    /// Accepted query forms. Matching is case-insensitive after identifier
    /// tokenization, but every token must be present and no surplus is allowed.
    pub queries: &'static [&'static str],
}

macro_rules! go_language_concept {
    ($name:literal, $label:literal, $landing:literal, [$($query:literal),+ $(,)?]) => {
        GoConcept {
            name: $name,
            label: $label,
            landing: $landing,
            queries: &[$($query),+],
        }
    };
}

/// Conservative routes for Go language concepts.
///
/// Both singular and plural forms are listed when they are natural searches.
/// Multiword forms are matched as a whole rather than as an unordered bag of
/// cues. This keeps short keywords such as `range` and `select` from claiming
/// package API searches.
pub const GO_LANGUAGE_CONCEPTS: &[GoConcept] = &[
    go_language_concept!(
        "array",
        "arrays",
        "https://go.dev/ref/spec#Array_types",
        ["array", "arrays"]
    ),
    go_language_concept!(
        "buffered-channel",
        "buffered channels",
        "https://go.dev/ref/spec#Channel_types",
        ["buffered channel", "buffered channels"]
    ),
    go_language_concept!(
        "channel",
        "channels",
        "https://go.dev/ref/spec#Channel_types",
        ["channel", "channels"]
    ),
    go_language_concept!(
        "defer",
        "defer statements",
        "https://go.dev/ref/spec#Defer_statements",
        ["defer", "defer statement", "defer statements"]
    ),
    go_language_concept!(
        "goroutine",
        "goroutines",
        "https://go.dev/ref/spec#Go_statements",
        ["goroutine", "goroutines", "go routine", "go routines"]
    ),
    go_language_concept!(
        "embedded-field",
        "embedded fields",
        "https://go.dev/ref/spec#Struct_types",
        ["embedded field", "embedded fields"]
    ),
    go_language_concept!(
        "interface",
        "interfaces",
        "https://go.dev/ref/spec#Interface_types",
        [
            "interface",
            "interfaces",
            "interface type",
            "interface types",
            "interface embedding",
            "embedded interface",
            "embedded interfaces"
        ]
    ),
    go_language_concept!("iota", "iota", "https://go.dev/ref/spec#Iota", ["iota"]),
    go_language_concept!(
        "map",
        "maps",
        "https://go.dev/ref/spec#Map_types",
        ["map", "maps", "map type", "map types"]
    ),
    go_language_concept!(
        "nil",
        "predeclared nil value",
        "https://go.dev/ref/spec#Predeclared_identifiers",
        ["nil", "nil value"]
    ),
    go_language_concept!(
        "multiple-returns",
        "multiple return values",
        "https://go.dev/doc/effective_go#multiple-returns",
        [
            "multiple return",
            "multiple returns",
            "multiple return value",
            "multiple return values"
        ]
    ),
    go_language_concept!(
        "range",
        "range clauses",
        "https://go.dev/ref/spec#For_range",
        [
            "range",
            "range clause",
            "range clauses",
            "range loop",
            "range loops",
            "for range"
        ]
    ),
    go_language_concept!(
        "receiver",
        "method receivers",
        "https://go.dev/ref/spec#Method_declarations",
        [
            "receiver",
            "receivers",
            "method receiver",
            "method receivers",
            "pointer receiver",
            "pointer receivers",
            "method pointer receiver",
            "methods pointer receivers"
        ]
    ),
    go_language_concept!(
        "select",
        "select statements",
        "https://go.dev/ref/spec#Select_statements",
        ["select", "select statement", "select statements"]
    ),
    go_language_concept!(
        "slice",
        "slices",
        "https://go.dev/ref/spec#Slice_types",
        ["slice", "slices", "slice type", "slice types"]
    ),
    go_language_concept!(
        "struct-tag",
        "struct tags",
        "https://go.dev/ref/spec#Struct_types",
        ["struct tag", "struct tags", "field tag", "field tags"]
    ),
    go_language_concept!(
        "type-assertion",
        "type assertions",
        "https://go.dev/ref/spec#Type_assertions",
        ["type assertion", "type assertions"]
    ),
    go_language_concept!(
        "type-parameter",
        "type parameters and generics",
        "https://go.dev/ref/spec#Type_parameter_declarations",
        [
            "generic",
            "generics",
            "type parameter",
            "type parameters",
            "type parameter declaration",
            "type parameter declarations",
            "generic constraint",
            "generic constraints",
            "constraint interface",
            "constraint interfaces"
        ]
    ),
    go_language_concept!(
        "type-set",
        "interface type sets",
        "https://go.dev/ref/spec#Interface_types",
        [
            "type set",
            "type sets",
            "interface type set",
            "interface type sets"
        ]
    ),
    go_language_concept!(
        "type-switch",
        "type switches",
        "https://go.dev/ref/spec#Type_switches",
        ["type switch", "type switches"]
    ),
    go_language_concept!(
        "variadic",
        "variadic functions",
        "https://go.dev/ref/spec#Function_types",
        [
            "variadic",
            "variadics",
            "variadic function",
            "variadic functions",
            "variadic parameter",
            "variadic parameters"
        ]
    ),
];

/// Match an exact single- or multiword Go concept query.
///
/// Punctuation and camel-case boundaries are normalized by
/// [`identifier_tokens`]. Unlisted words cause the match to fail.
pub fn find_go_language_concept(query: &str) -> Option<&'static GoConcept> {
    let tokens = identifier_tokens(query);
    if tokens.is_empty() {
        return None;
    }
    GO_LANGUAGE_CONCEPTS.iter().find(|concept| {
        concept
            .queries
            .iter()
            .any(|accepted| identifier_tokens(accepted) == tokens)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn table_is_complete_unique_and_well_formed() {
        assert_eq!(ALIASES.len(), Language::ALL.len() * Intent::ALL.len());
        let mut pairs = HashSet::new();
        for alias in ALIASES {
            assert!(
                pairs.insert((alias.language, alias.intent)),
                "duplicate alias: {:?} {:?}",
                alias.language,
                alias.intent
            );
            assert!(!alias.target_terms.is_empty());
            assert!(alias
                .target_terms
                .iter()
                .all(|term| !term.trim().is_empty()));
            assert!(
                !alias.cues.action.is_empty()
                    || !alias.cues.object.is_empty()
                    || !alias.cues.qualifier.is_empty()
            );
            if let Some(url) = alias.landing {
                assert!(url.starts_with("https://"), "non-HTTPS landing: {url}");
            }
        }
        for language in Language::ALL {
            for intent in Intent::ALL {
                assert!(
                    pairs.contains(&(language, intent)),
                    "missing alias: {language:?} {intent:?}"
                );
            }
        }
    }

    #[test]
    fn tokenization_handles_identifiers_but_not_substrings() {
        assert_eq!(
            identifier_tokens("parseInteger/file_name"),
            ["parse", "integer", "file", "name"]
        );
        assert_eq!(identifier_tokens("HTTPServer"), ["httpserver"]);
        assert!(find_alias(Language::Go, "parseInteger").is_some());
        assert!(find_alias(Language::Go, "parsec integers").is_none());
        assert!(find_alias(Language::Python, "removeable file").is_none());
        assert!(find_alias(Language::Python, "profile reader").is_none());
    }

    #[test]
    fn matcher_requires_every_nonempty_slot() {
        assert!(find_alias(Language::Cpp, "sorting records by keys").is_some());
        assert!(find_alias(Language::Cpp, "sort records").is_none());
        assert!(find_alias(Language::Go, "delete map entry").is_some());
        assert!(find_alias(Language::Go, "delete map").is_none());
        assert!(find_alias(Language::Python, "command line arguments").is_some());
        assert!(find_alias(Language::Python, "arguments").is_none());
        assert!(find_alias(Language::Go, "read file permissions").is_none());
        assert!(find_alias(Language::Python, "remove file permissions").is_none());
        assert!(find_alias(Language::C, "read a file line by line").is_some());
        assert!(find_alias(Language::Cpp, "convert string to integer").is_some());
    }

    #[test]
    fn language_task_aliases_use_semantic_slots() {
        for (language, query, expected) in [
            (Language::C, "structures", "Struct declaration"),
            (
                Language::C,
                "storage lifetime",
                "automatic, static, thread, and allocated storage duration",
            ),
            (Language::C, "order a sequence using a callback", "qsort"),
            (Language::C, "format calendar and time", "strftime"),
            (
                Language::Cpp,
                "sort array with comparator",
                "std::sort comparator",
            ),
            (Language::Go, "format date and time", "time.Time.Format"),
        ] {
            let found = find_alias(language, query).unwrap_or_else(|| panic!("query: {query}"));
            assert_eq!(found.alias.target_terms[0], expected, "query: {query}");
            assert_eq!(found.alias.confidence, Confidence::High, "query: {query}");
        }
        assert!(find_alias(Language::C, "search array with comparator").is_none());
        assert!(find_alias(Language::C, "format a date").is_none());
    }

    #[test]
    fn word_families_are_explicit_and_conservative() {
        assert!(word_family_matches("writing", "write"));
        assert!(word_family_matches("dictionaries", "dict"));
        assert!(!word_family_matches("writer", "write"));
        assert!(!word_family_matches("mappington", "map"));
    }

    #[test]
    fn go_tables_use_exact_names_and_valid_landings() {
        assert_eq!(
            go_builtin("len").unwrap().landing,
            "https://pkg.go.dev/builtin#len"
        );
        assert!(go_builtin("Len").is_none());
        assert!(go_builtin("length").is_none());
        assert!(go_builtin("len()").is_none());
        assert!(go_concept("goroutine").is_some());
        assert!(go_concept("go routine").is_none());
        for table in [GO_BUILTINS, GO_CONCEPTS] {
            let mut names = HashSet::new();
            for item in table {
                assert!(names.insert(item.name), "duplicate Go name: {}", item.name);
                assert!(item.landing.starts_with("https://"));
            }
        }
    }

    #[test]
    fn go_language_concepts_cover_single_and_multiword_forms() {
        let cases = [
            ("type assertions", "type-assertion"),
            ("typeSwitch", "type-switch"),
            ("struct-tags", "struct-tag"),
            ("method receivers", "receiver"),
            ("type parameters", "type-parameter"),
            ("generics", "type-parameter"),
            ("multiple return values", "multiple-returns"),
            ("buffered_channels", "buffered-channel"),
            ("select statement", "select"),
            ("for range", "range"),
            ("iota", "iota"),
            ("variadic functions", "variadic"),
            ("go routines", "goroutine"),
        ];
        for (query, expected) in cases {
            assert_eq!(
                find_go_language_concept(query).map(|concept| concept.name),
                Some(expected),
                "query: {query}"
            );
        }
    }

    #[test]
    fn go_language_concepts_reject_package_apis_and_surplus_terms() {
        for query in [
            "range.Int",
            "select.Query",
            "reflect.Type assertion",
            "struct tag parser",
            "receiver function",
            "buffered channel capacity",
            "iota package",
            "variadic fmt Printf",
            "goroutine pool",
            "type switch helper",
        ] {
            assert!(
                find_go_language_concept(query).is_none(),
                "unexpected Go concept route: {query}"
            );
        }
    }

    #[test]
    fn go_language_concept_table_is_unique_and_official() {
        let mut names = HashSet::new();
        let mut queries = HashSet::new();
        for concept in GO_LANGUAGE_CONCEPTS {
            assert!(
                names.insert(concept.name),
                "duplicate concept: {}",
                concept.name
            );
            assert!(!concept.label.trim().is_empty());
            assert!(concept.landing.starts_with("https://go.dev/"));
            assert!(!concept.queries.is_empty());
            for query in concept.queries {
                let normalized = identifier_tokens(query).join(" ");
                assert!(!normalized.is_empty());
                assert!(
                    queries.insert(normalized),
                    "duplicate Go concept query: {query}"
                );
            }
        }
    }
}
