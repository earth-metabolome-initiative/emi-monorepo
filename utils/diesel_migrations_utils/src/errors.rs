//! Submodule providing errors for the migrations utility.

use sqlparser::parser::ParserError;

use crate::prelude::MigrationKind;

#[derive(Debug, PartialEq)]
/// Enumeration of errors that may occur with the migrations utility.
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
    },
    /// When reading a migration fails
    ReadingMigrationFailed(u64, MigrationKind, String),
    /// Failed to connect to the database
    ConnectionFailed(diesel::ConnectionError),
    /// When executing a migration fails
    ExecutingMigrationFailed(u64, MigrationKind, diesel::result::Error),
    /// When parsing a migration fails
    ParsingMigrationFailed(u64, MigrationKind, ParserError),
    /// When the migrations directory order does not form a DAG
    NotDAG,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::InvalidMigration(error.to_string())
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(error: diesel::ConnectionError) -> Self {
        Error::ConnectionFailed(error)
    }
}
