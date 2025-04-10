//! This module contains the API paths for the documents endpoint.
pub mod profile;
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for the documents API
pub const ENDPOINT: &str = "/documents";
/// Full endpoint for the documents API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");
