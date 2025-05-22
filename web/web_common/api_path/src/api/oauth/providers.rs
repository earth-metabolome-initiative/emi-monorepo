//! Struct defining an `OAuth2` login provider supported by the backend.
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for the `OAuth2` providers API
pub const ENDPOINT: &str = "/providers";
/// Full endpoint for the `OAuth2` providers API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");
