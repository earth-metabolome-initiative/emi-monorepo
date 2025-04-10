//! Submodule for WebSocket API
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint for WebSocket API
pub const ENDPOINT: &str = "/ws";
/// Full endpoint for WebSocket API
pub const FULL_ENDPOINT: &str = const_format::formatcp!("{PARENT_ENDPOINT}{ENDPOINT}");

