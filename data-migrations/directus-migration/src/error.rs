//! Enumeration of errors that may occur during directus migration

use core_structures::{
    codegen::structs_codegen::tables::insertables::InsertableUserAttributes,
    tables::insertables::{
        InsertableAddressAttributes, InsertableBrandAttributes, InsertableCityAttributes, InsertableInstrumentAttributes, InsertableInstrumentLocationAttributes, InsertableInstrumentModelAttributes, InsertableRoomAttributes, InsertableUserEmailAttributes
    },
};
use web_common_traits::database::InsertError;

use crate::codegen::{
    Brand as DirectusBrand, DirectusUser, Instrument as DirectusInstrument,
    InstrumentModel as DirectusInstrumentModel, InstrumentType as DirectusInstrumentType,
    Room as DirectusRoom,
};

#[derive(Debug)]
#[allow(dead_code)]
/// Enumeration of errors that may occur during directus migration
pub enum Error {
    /// Error when user doesn't have an email
    MissingEmail(rosetta_uuid::Uuid),
    /// Missing first name
    MissingFirstName(rosetta_uuid::Uuid),
    /// Missing last name
    MissingLastName(rosetta_uuid::Uuid),
    /// Missing instrument name
    MissingInstrumentTypeName(Box<DirectusInstrumentType>),
    /// A brand is missing a user
    BrandWithMissingUser(Box<DirectusBrand>),
    /// Instrument model is missing the creator user
    InstrumentModelWithMissingUser(Box<DirectusInstrumentModel>),
    /// Instrument is missing the creator user
    InstrumentWithMissingUser(Box<DirectusInstrument>),
    /// Room is missing the creator user
    RoomWithMissingUser(Box<DirectusRoom>),
    /// Missing date
    MissingDate(String, String),
    /// Missing geolocation
    InvalidGeolocation(postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>),
    /// Unknown brand status
    UnknownBrandStatus(String),
    /// Unknown instrument type
    UnknownInstrumentType(Box<DirectusInstrumentType>),
    /// Unknown brand
    UnknownBrand(Box<DirectusBrand>),
    /// Unknown instrument state
    UnknownInstrumentState(String),
    /// Unknown country
    UnknownCountry(String),
    /// Unknown instrument model
    UnknownInstrumentModel(Box<DirectusInstrumentModel>),
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
    /// Failed to insert instrument
    InstrumentInsertError(InsertError<InsertableInstrumentAttributes>),
    /// Failed to insert a new city
    CityInsertError(InsertError<InsertableCityAttributes>),
    /// Failed to insert a new address
    AddressInsertError(InsertError<InsertableAddressAttributes>),
    /// Failed to insert a new room
    RoomInsertError(InsertError<InsertableRoomAttributes>),
    /// Failed to insert a new instrument location.
    InstrumentLocationInsertError(InsertError<InsertableInstrumentLocationAttributes>),
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

impl From<InsertError<InsertableInstrumentAttributes>> for Error {
    fn from(value: InsertError<InsertableInstrumentAttributes>) -> Self {
        Error::InstrumentInsertError(value)
    }
}

impl From<InsertError<InsertableCityAttributes>> for Error {
    fn from(value: InsertError<InsertableCityAttributes>) -> Self {
        Error::CityInsertError(value)
    }
}

impl From<InsertError<InsertableAddressAttributes>> for Error {
    fn from(value: InsertError<InsertableAddressAttributes>) -> Self {
        Error::AddressInsertError(value)
    }
}

impl From<InsertError<InsertableRoomAttributes>> for Error {
    fn from(value: InsertError<InsertableRoomAttributes>) -> Self {
        Error::RoomInsertError(value)
    }
}

impl From<InsertError<InsertableInstrumentLocationAttributes>> for Error {
    fn from(value: InsertError<InsertableInstrumentLocationAttributes>) -> Self {
        Error::InstrumentLocationInsertError(value)
    }
}
