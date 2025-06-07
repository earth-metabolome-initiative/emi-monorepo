//! API path for authentication

use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for authentication API
pub const ENDPOINT: &str = "/auth";
/// Full endpoint for authentication API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");
