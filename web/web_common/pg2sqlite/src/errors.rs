//! Submodule providing the error enumeration that may occur during the
//! translation between `PostgreSQL` and `SQLite`.

use sqlparser::parser::ParserError;

#[derive(Debug)]
/// Error enumeration that may occur during the translation between `PostgreSQL` and
/// `SQLite`.
pub enum Error {
    /// Error that may occur during the parsing of a SQL statement.
    ParserError(ParserError),
    /// Error that may occur during the reading of a file.
    IoError(std::io::Error),
    /// Error when a function is not available in the schema.
    UndefinedFunction(String),
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Self {
        Error::ParserError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}
