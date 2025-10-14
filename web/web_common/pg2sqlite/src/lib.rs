#![doc = include_str!("../README.md")]

pub mod errors;
pub mod impls;
pub mod options;
pub mod pg2sqlite;
pub mod traits;

/// Prelude module for the library.
pub mod prelude {
    pub use crate::{
        options::Pg2SqliteOptions,
        pg2sqlite::Pg2Sqlite,
        traits::{Schema, TranslationOptions, Translator},
    };
}
