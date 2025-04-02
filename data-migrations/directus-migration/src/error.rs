//! Enumeration of errors that may occur during directus migration

use crate::codegen::{
    Brand as DirectusBrand, DirectusUser, InstrumentModel as DirectusInstrumentModel,
    InstrumentType as DirectusInstrumentType,
};
use core_structures::{
    codegen::structs_codegen::tables::insertables::InsertableUserAttributes,
    tables::insertables::{InsertableBrandAttributes, InsertableInstrumentModelAttributes, InsertableUserEmailAttributes},
};
use web_common_traits::database::InsertError;

#[derive(Debug)]
/// Enumeration of errors that may occur during directus migration
pub enum Error {
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
    /// Instrument model is missing the creator user
    InstrumentModelWithMissingUser(Box<DirectusInstrumentModel>),
    /// Missing date
    MissingDate(String, String),
    /// Unknown brand status
    UnknownBrandStatus(String),
    /// Unknown instrument type
    UnknownInstrumentType(Box<DirectusInstrumentType>),
    /// Unknown brand
    UnknownBrand(Box<DirectusBrand>),
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
    /// Failed to insert instrument model.
    InstrumentModelInsertError(InsertError<InsertableInstrumentModelAttributes>),
    /// User never logged in
    UserNeverLoggedIn(Box<DirectusUser>),
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

impl From<InsertError<InsertableUserAttributes>> for Error {
    fn from(value: InsertError<InsertableUserAttributes>) -> Self {
        Error::UserInsertError(value)
    }
}

impl From<InsertError<InsertableUserEmailAttributes>> for Error {
    fn from(value: InsertError<InsertableUserEmailAttributes>) -> Self {
        Error::MailInsertError(value)
    }
}

impl From<InsertError<InsertableBrandAttributes>> for Error {
    fn from(value: InsertError<InsertableBrandAttributes>) -> Self {
        Error::BrandInsertError(value)
    }
}

impl From<InsertError<InsertableInstrumentModelAttributes>> for Error {
    fn from(value: InsertError<InsertableInstrumentModelAttributes>) -> Self {
        Error::InstrumentModelInsertError(value)
    }
}