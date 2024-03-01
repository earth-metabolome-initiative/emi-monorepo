use serde::{Deserialize, Serialize};
pub mod oauth;
pub mod auth;
use validator::ValidationErrors;


pub const ENDPOINT: &str = "/api";
pub const FULL_ENDPOINT: &str = ENDPOINT;

#[derive(Debug, Deserialize, Serialize)]
pub enum ApiError {
    Oauth(oauth::OauthErrors),
    BadGateway,
    BadRequest(String),
    InternalServerError,
}

impl ApiError {
    pub fn unauthorized() -> Self {
        Self::Oauth(oauth::OauthErrors::Refresh(oauth::jwt_cookies::RefreshError::Unauthorized))
    }

    pub fn expired_authorization() -> Self {
        Self::Oauth(oauth::OauthErrors::Refresh(oauth::jwt_cookies::RefreshError::ExpiredAuthorization))
    }

    pub fn internal_server_error() -> Self {
        Self::InternalServerError
    }

    pub fn bad_gateway() -> Self {
        Self::BadGateway
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        log::error!("Failed to serialize response: {}", e);
        Self::InternalServerError
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(e: ValidationErrors) -> Self {
        log::error!("Validation error: {:?}", e);
        Self::BadRequest(e.to_string())
    }
}