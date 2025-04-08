#![doc = include_str!("../README.md")]

pub mod errors;
pub mod impls;
pub mod pg2sqlite;
pub mod traits;

/// Prelude module for the library.
pub mod prelude {
    pub use crate::{
        pg2sqlite::Pg2Sqlite,
        traits::{Schema, Translator},
    };
}
