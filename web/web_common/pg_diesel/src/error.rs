//! Submodule providing an error enumeration for the `pg_diesel` crate.

use std::fmt::Display;

use cached::DiskCacheError;

#[derive(Debug)]
/// This module defines a custom error type for the `pg_diesel` crate,
/// encapsulating errors from Diesel and standard I/O operations.
pub enum Error {
    /// An error originating from the Diesel ORM.
    Diesel(diesel::result::Error),
    /// An error originating from standard I/O operations.
    DiskCache(DiskCacheError),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Diesel(e)
    }
}

impl From<DiskCacheError> for Error {
    fn from(e: DiskCacheError) -> Self {
        Error::DiskCache(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Diesel(e) => write!(f, "Diesel error: {}", e),
            Error::DiskCache(e) => write!(f, "Disk cache error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Diesel(e) => Some(e),
            Error::DiskCache(e) => Some(e),
        }
    }
}
