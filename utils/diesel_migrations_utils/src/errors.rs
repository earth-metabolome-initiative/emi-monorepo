//! Submodule providing errors for the migrations utility.

use crate::prelude::MigrationKind;

#[derive(Debug, PartialEq, Eq)]
/// Enumeration of errors that may occour with the migrations utility.
pub enum Error {
    /// Error raised when the directory is not a valid migration.
    InvalidMigration(String),
    /// Missing up migration.
    MissingUpMigration(u64),
    /// Missing down migration.
    MissingDownMigration(u64),
    /// Duplicate migration number.
    DuplicateMigrationNumber(u64),
    /// SQL within migration is invalid.
    InvalidSql(u64, MigrationKind, String),
    /// When moving a migration fails
    MovingMigrationFailed {
        /// Source directory.
        source: String,
        /// Destination directory.
        destination: String,
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::InvalidMigration(error.to_string())
    }
}
