//! This module contains the `OAuth2` API endpoints.
pub mod jwt_cookies;
pub mod providers;
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for the `OAuth2` API
pub const ENDPOINT: &str = "/oauth";
/// Full endpoint for the `OAuth2` API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");
