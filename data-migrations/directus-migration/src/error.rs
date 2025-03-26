//! Enumeration of errors that may occur during directus migration

use web_common_traits::database::InsertError;
use core_structures::codegen::structs_codegen::tables::insertables::InsertableUserAttributes;

#[derive(Debug)]
/// Enumeration of errors that may occur during directus migration
pub enum Error{
    /// Error when user doesn't have an email
    MissingEmail(uuid::Uuid),
    /// Missing first name
    MissingFirstName(uuid::Uuid),
    /// Missing last name
    MissingLastName(uuid::Uuid),
    /// Failed to establish database connection
    ConnectionFailed(diesel::ConnectionError),
    /// Failed to execute a query
    QueryFailed(diesel::result::Error),
    /// Failer to insert user
    UserInsertError(InsertError<InsertableUserAttributes>),
}

impl From<diesel::ConnectionError> for Error{
    fn from(value: diesel::ConnectionError) -> Self {
        Error::ConnectionFailed(value)
    }
}

impl From<diesel::result::Error> for Error{
    fn from(value: diesel::result::Error) -> Self {
        Error::QueryFailed(value)
    }
}

impl From<InsertError<InsertableUserAttributes>> for Error{
    fn from(value: InsertError<InsertableUserAttributes>) -> Self {
        Error::UserInsertError(value)
    }
}