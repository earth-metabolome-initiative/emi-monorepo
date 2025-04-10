//! This module defines the `DeviceError` enum, which represents various errors
//! that can occur when interacting with devices in a web application.

#[derive(
    Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord,
)]
/// An enum representing various device errors.
pub enum DeviceError {
    /// Indicates that no cameras were found on the device.
    NoCameras,
    /// Indicates that the device stopped responding.
    DeviceStoppedResponding,
}

impl ToString for DeviceError {
    fn to_string(&self) -> String {
        match self {
            DeviceError::NoCameras => "No cameras found.".to_string(),
            DeviceError::DeviceStoppedResponding => "Device stopped responding.".to_string(),
        }
    }
}
