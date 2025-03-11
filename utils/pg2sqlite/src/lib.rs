#![doc = include_str!("../README.md")]

pub mod errors;
pub mod traits;
pub mod impls;
pub mod pg2sqlite;

/// Prelude module for the library.
pub mod prelude {
    pub use crate::pg2sqlite::Pg2Sqlite;
    pub use crate::traits::{Translator, Schema};
}