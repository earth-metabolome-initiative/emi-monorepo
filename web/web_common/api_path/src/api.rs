//! Submodule providing the root API endpoint and all submodules.

pub mod auth;
pub mod documents;
pub mod oauth;
pub mod ws;

/// Root API endpoint
pub const ENDPOINT: &str = "/api";
/// Full API endpoint
pub const FULL_ENDPOINT: &str = ENDPOINT;
