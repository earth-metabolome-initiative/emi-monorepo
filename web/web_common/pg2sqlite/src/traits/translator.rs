//! Submodule providing a trait to translate between a `PostgreSQL` entry and a
//! `SQLite` entry.

use super::Schema;
use crate::traits::TranslationOptions;

/// Trait to translate between a `PostgreSQL` entry and a `SQLite` entry.
pub trait Translator {
    /// The schema type to be used for the translation.
    type Schema: Schema;
    /// The translation options to be used for the translation.
    type Options: TranslationOptions;
    /// The `SQLite` entry type to be used for the translation.
    type SQLiteEntry;

    /// Translate a `PostgreSQL` entry to a `SQLite` entry.
    ///
    /// # Arguments
    ///
    /// * `schema` - The schema to be used for the translation.
    /// * `options` - The translation options to be used for the translation.
    ///
    /// # Errors
    ///
    /// * `crate::errors::Error` - If the translation fails.
    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error>;
}
