//! This module contains the API path for the profile picture endpoint.
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for the profile picture API
pub const ENDPOINT: &str = "/picture";
/// Full endpoint for the profile picture API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");
