//! This module contains the API path for the JWT-cookies endpoint.
use super::FULL_ENDPOINT as PARENT_ENDPOINT;

/// Endpoint to refresh the JWT-cookies
pub const REFRESH_ENDPOINT: &str = "/refresh";
/// Full endpoint to refresh the JWT-cookies
pub const FULL_REFRESH_ENDPOINT: &str =
    const_format::formatcp!("{PARENT_ENDPOINT}{REFRESH_ENDPOINT}");

/// Endpoint to delete the JWT-cookies
pub const LOGOUT_ENDPOINT: &str = "/logout";
/// Full endpoint to delete the JWT-cookies
pub const FULL_LOGOUT_ENDPOINT: &str =
    const_format::formatcp!("{PARENT_ENDPOINT}{LOGOUT_ENDPOINT}");

/// Name of the cookie storing whether the user is online
pub const USER_ONLINE_COOKIE_NAME: &str = "user_online";

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
/// Struct representing the access token used to authenticate the user.
pub struct AccessToken {
    token: String,
}

impl AccessToken {
    #[must_use]
    /// Creates a new access token.
    pub fn new(token: String) -> Self {
        Self { token }
    }

    #[must_use]
    /// Creates a new access token from a string slice.
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    /// Creates a new access token from a string slice.
    pub fn header(&self) -> (&str, String) {
        ("Authorization", format!("Bearer {}", self.token))
    }
}
