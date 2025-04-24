//! Submodule providing the enumeration of errors which may occur in the
//! Frontend.

pub(crate) mod device_errors;
pub(crate) mod geolocation_errors;

#[allow(dead_code)]
/// The errors which may occur in the Frontend.
pub(crate) enum FrontendError {
    /// An error related to device operations.
    DeviceError(device_errors::DeviceError),
    /// An error related to geolocation operations.
    GeolocationError(geolocation_errors::GeolocationError),
}

impl From<device_errors::DeviceError> for FrontendError {
    fn from(error: device_errors::DeviceError) -> Self {
        FrontendError::DeviceError(error)
    }
}
impl From<geolocation_errors::GeolocationError> for FrontendError {
    fn from(error: geolocation_errors::GeolocationError) -> Self {
        FrontendError::GeolocationError(error)
    }
}
