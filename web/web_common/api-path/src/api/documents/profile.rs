//! This module contains the API path for the profile endpoint.
pub mod picture;
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for the profile API
pub const ENDPOINT: &str = "/profile";
/// Full endpoint for the profile API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");
