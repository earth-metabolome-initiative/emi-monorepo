//! Submodule providing the enumeration of errors which may occur in the
//! Frontend.

use db_errors::DBError;

pub(crate) mod db_errors;
pub(crate) mod device_errors;
pub(crate) mod geolocation_errors;
pub(crate) mod ws_errors;

#[allow(dead_code)]
/// The errors which may occur in the Frontend.
pub(crate) enum FrontendError {
    /// An error related to device operations.
    Device(device_errors::DeviceError),
    /// An error related to geolocation operations.
    Geolocation(geolocation_errors::GeolocationError),
    /// An error related to the database.
    DB(db_errors::DBError),
    /// An error related to the WebSocket.
    WSError(ws_errors::WSError),
}

impl From<device_errors::DeviceError> for FrontendError {
    fn from(error: device_errors::DeviceError) -> Self {
        FrontendError::Device(error)
    }
}

impl From<geolocation_errors::GeolocationError> for FrontendError {
    fn from(error: geolocation_errors::GeolocationError) -> Self {
        FrontendError::Geolocation(error)
    }
}

impl From<db_errors::DBError> for FrontendError {
    fn from(error: db_errors::DBError) -> Self {
        FrontendError::DB(error)
    }
}

impl From<ws_errors::WSError> for FrontendError {
    fn from(error: ws_errors::WSError) -> Self {
        FrontendError::WSError(error)
    }
}

impl From<diesel::result::Error> for FrontendError {
    fn from(_err: diesel::result::Error) -> Self {
        FrontendError::DB(DBError::QueryFailed)
    }
}
