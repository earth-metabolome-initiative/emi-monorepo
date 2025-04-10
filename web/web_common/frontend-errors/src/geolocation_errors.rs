//! This module defines the `GeolocationError` enum, which represents various errors that can occur
//! when interacting with geolocation services in a web application.

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
/// An enum representing various geolocation errors.
pub enum GeolocationError {
    /// Indicates that geolocation is not supported by the device or browser.
    NotSupported,
    /// Indicates that permission to access geolocation was denied.
    PermissionDenied,
    /// Indicates that the geolocation position is unavailable.
    PositionUnavailable,
    /// Indicates that the geolocation request timed out.
    Timeout,
    /// Indicates an unknown error occurred while trying to access geolocation.
    Unknown(String),
}

impl ToString for GeolocationError {
    fn to_string(&self) -> String {
        match self {
            GeolocationError::NotSupported => "Geolocation is not supported.".to_string(),
            GeolocationError::PermissionDenied => "Permission to geolocate was denied - You may need to authorize this in your device privacy settings.".to_string(),
            GeolocationError::PositionUnavailable => "Position unavailable.".to_string(),
            GeolocationError::Timeout => "Geolocation request timed out.".to_string(),
            GeolocationError::Unknown(e) => format!("Geolocation error: {}", e),
        }
    }
}

impl From<web_sys::PositionError> for GeolocationError {
    fn from(e: web_sys::PositionError) -> Self {
        match e.code() {
            web_sys::PositionError::PERMISSION_DENIED => GeolocationError::PermissionDenied,
            web_sys::PositionError::POSITION_UNAVAILABLE => GeolocationError::PositionUnavailable,
            web_sys::PositionError::TIMEOUT => GeolocationError::Timeout,
            unknown => GeolocationError::Unknown(format!("Position error code: {:?}", unknown)),
        }
    }
}
