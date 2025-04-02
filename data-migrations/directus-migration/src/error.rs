//! Enumeration of errors that may occur during directus migration

use crate::codegen::{DirectusUser, Brand as DirectusBrand, InstrumentType as DirectusInstrumentType};
use web_common_traits::database::InsertError;
use core_structures::{codegen::structs_codegen::tables::insertables::InsertableUserAttributes, tables::insertables::{InsertableBrandAttributes, InsertableUserEmailAttributes}};

#[derive(Debug)]
/// Enumeration of errors that may occur during directus migration
pub enum Error{
    /// Error when user doesn't have an email
    MissingEmail(uuid::Uuid),
    /// Missing first name
    MissingFirstName(uuid::Uuid),
    /// Missing last name
    MissingLastName(uuid::Uuid),
    /// Missing instrument name
    MissingInstrumentTypeName(Box<DirectusInstrumentType>),
    /// A brand is missing a user
    BrandWithMissingUser(Box<DirectusBrand>),
    /// Missing date
    MissingDate(String, String),
    /// Unknown brand status
    UnknownBrandStatus(String),
    /// Unknown instrument type
    UnknownInstrumentType(Box<DirectusInstrumentType>),
    /// Failed to establish database connection
    ConnectionFailed(diesel::ConnectionError),
    /// Failed to execute a query
    QueryFailed(diesel::result::Error),
    /// Failed to insert user
    UserInsertError(InsertError<InsertableUserAttributes>),
    /// Failed to insert Email
    MailInsertError(InsertError<InsertableUserEmailAttributes>),
    /// Failed to insert brand
    BrandInsertError(InsertError<InsertableBrandAttributes>),
    /// User never logged in
    UserNeverLoggedIn(Box<DirectusUser>)
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

impl From<InsertError<InsertableUserEmailAttributes>> for Error{
    fn from(value: InsertError<InsertableUserEmailAttributes>) -> Self {
        Error::MailInsertError(value)
    }
}

impl From<InsertError<InsertableBrandAttributes>> for Error{
    fn from(value: InsertError<InsertableBrandAttributes>) -> Self {
        Error::BrandInsertError(value)
    }
}