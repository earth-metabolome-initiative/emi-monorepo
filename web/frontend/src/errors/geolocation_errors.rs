//! This module defines the `GeolocationError` enum, which represents various
//! errors that can occur when interacting with geolocation services in a web
//! application.

use std::fmt::Display;

#[derive(
    Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord,
)]
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

impl Display for GeolocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeolocationError::NotSupported => write!(f, "Geolocation is not supported."),
            GeolocationError::PermissionDenied => write!(f, "Permission denied."),
            GeolocationError::PositionUnavailable => write!(f, "Position unavailable."),
            GeolocationError::Timeout => write!(f, "Request timed out."),
            GeolocationError::Unknown(msg) => write!(f, "Unknown error: {msg}"),
        }
    }
}

impl From<web_sys::PositionError> for GeolocationError {
    fn from(e: web_sys::PositionError) -> Self {
        match e.code() {
            web_sys::PositionError::PERMISSION_DENIED => GeolocationError::PermissionDenied,
            web_sys::PositionError::POSITION_UNAVAILABLE => GeolocationError::PositionUnavailable,
            web_sys::PositionError::TIMEOUT => GeolocationError::Timeout,
            unknown => GeolocationError::Unknown(format!("Position error code: {unknown:?}")),
        }
    }
}
