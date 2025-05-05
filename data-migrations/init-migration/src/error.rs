//! Submodule providing the errors enumeration.

use core_structures::tables::insertables::InsertableLoginProviderAttributes;
use web_common_traits::database::InsertError;

#[derive(Debug)]
#[allow(dead_code)]
/// Error enumeration for the `init-migration` module.
pub enum Error {
    /// Failed to establish database connection
    ConnectionFailed(diesel::ConnectionError),
    /// Failed to execute a query
    QueryFailed(diesel::result::Error),
    /// Failed to insert a new login provider
    LoginProvider(InsertError<InsertableLoginProviderAttributes>),
}

impl From<diesel::ConnectionError> for Error {
    fn from(value: diesel::ConnectionError) -> Self {
        Error::ConnectionFailed(value)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::QueryFailed(value)
    }
}

impl From<InsertError<InsertableLoginProviderAttributes>> for Error {
    fn from(value: InsertError<InsertableLoginProviderAttributes>) -> Self {
        Error::LoginProvider(value)
    }
}
