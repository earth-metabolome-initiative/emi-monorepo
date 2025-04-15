//! Submodule providing a trait to translate between a `PostgreSQL` entry and a
//! `SQLite` entry.

use super::Schema;

/// Trait to translate between a `PostgreSQL` entry and a `SQLite` entry.
pub trait Translator {
    /// The schema type to be used for the translation.
    type Schema: Schema;
    /// The `SQLite` entry type to be used for the translation.
    type SQLiteEntry;

    /// Translate a `PostgreSQL` entry to a `SQLite` entry.
    /// 
    /// # Arguments
    /// 
    /// * `schema` - The schema to be used for the translation.
    /// 
    /// # Errors
    /// 
    /// * `crate::errors::Error` - If the translation fails.
    /// 
    fn translate(&self, schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error>;
}
