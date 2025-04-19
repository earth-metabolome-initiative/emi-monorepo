//! Submodule providing errors for the migrations utility.

use std::fmt::Display;

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
    /// When the migrations directory contains circular dependencies
    CircularDependency(Vec<(u64, String)>),
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidMigration(msg) => write!(f, "Invalid migration: {}", msg),
            Error::MissingUpMigration(num) => write!(f, "Missing up migration: {}", num),
            Error::MissingDownMigration(num) => write!(f, "Missing down migration: {}", num),
            Error::DuplicateMigrationNumber(num) => {
                write!(f, "Duplicate migration number: {}", num)
            }
            Error::InvalidSql(num, kind, msg) => {
                write!(f, "Invalid SQL in migration {} ({}): {}", num, kind, msg)
            }
            Error::MovingMigrationFailed { source, destination } => {
                write!(f, "Moving migration from {} to {} failed", source, destination)
            }
            Error::ReadingMigrationFailed(num, kind, msg) => {
                write!(f, "Reading migration {} ({}): {}", num, kind, msg)
            }
            Error::ConnectionFailed(err) => write!(f, "Connection failed: {}", err),
            Error::ExecutingMigrationFailed(num, kind, err) => {
                write!(f, "Executing migration {} ({}): {}", num, kind, err)
            }
            Error::ParsingMigrationFailed(num, kind, err) => {
                write!(f, "Parsing migration {} ({}): {}", num, kind, err)
            }
            Error::NotDAG => write!(f, "The migrations directory does not form a DAG"),
            Error::CircularDependency(deps) => {
                write!(f, "Circular dependencies detected: {:?}", deps)
            }
        }
    }
}

impl core::error::Error for Error {}
