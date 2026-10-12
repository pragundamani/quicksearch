#![allow(dead_code)]

#[path = "../src/cache.rs"]
mod cache;
#[path = "../src/http.rs"]
mod http;
#[path = "../src/model.rs"]
mod model;
#[path = "../src/text.rs"]
mod text;

mod providers {
    pub mod langs {
        use std::path::PathBuf;
        pub(crate) fn cache_dir() -> PathBuf {
            std::env::temp_dir().join("quicksearch-haskell-api-test")
        }
    }
}

#[path = "../src/providers/haskell_api.rs"]
mod haskell_api;
