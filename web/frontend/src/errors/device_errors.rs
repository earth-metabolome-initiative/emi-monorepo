//! This module defines the `DeviceError` enum, which represents various errors
//! that can occur when interacting with devices in a web application.

use std::fmt::Display;

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

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::NoCameras => write!(f, "No cameras found on the device."),
            DeviceError::DeviceStoppedResponding => {
                write!(f, "The device stopped responding.")
            }
        }
    }
}
