use serde::{Deserialize, Serialize};
pub mod auth;
pub mod oauth;
pub mod ws;
use validator::ValidationErrors;
use validator::ValidationError;

use crate::custom_validators::validation_errors::ValidationErrorToString;

pub const ENDPOINT: &str = "/api";
pub const FULL_ENDPOINT: &str = ENDPOINT;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApiError {
    Oauth(oauth::OauthErrors),
    BadGateway,
    BadRequest(Vec<String>),
    InternalServerError,
}

impl ApiError {
    pub fn unauthorized() -> Self {
        Self::Oauth(oauth::OauthErrors::Refresh(
            oauth::jwt_cookies::RefreshError::Unauthorized,
        ))
    }

    pub fn is_unauthorized(&self) -> bool {
        match self {
            ApiError::Oauth(oauth::OauthErrors::Refresh(
                oauth::jwt_cookies::RefreshError::Unauthorized,
            )) => true,
            _ => false,
        }
    }

    pub fn expired_authorization() -> Self {
        Self::Oauth(oauth::OauthErrors::Refresh(
            oauth::jwt_cookies::RefreshError::ExpiredAuthorization,
        ))
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
        Self::BadRequest(e.convert_to_string())
    }
}

impl From<ValidationError> for ApiError {
    fn from(e: ValidationError) -> Self {
        log::error!("Validation error: {:?}", e);
        Self::BadRequest(e.convert_to_string())
    }
}

impl From<bincode::ErrorKind> for ApiError {
    fn from(e: bincode::ErrorKind) -> Self {
        Self::BadRequest(vec![format!("Serialization failure: {}", e)])
    }
}

impl From<Box<bincode::ErrorKind>> for ApiError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Self::BadRequest(vec![format!("Serialization failure: {}", e)])
    }
}

impl Into<Vec<String>> for ApiError {
    fn into(self) -> Vec<String> {
        match self {
            ApiError::BadRequest(errors) => errors,
            ApiError::Oauth(oauth_error) => vec![format!("Oauth error: {:?}", oauth_error)],
            ApiError::BadGateway => vec!["Bad Gateway".to_string()],
            ApiError::InternalServerError => vec!["Internal Server Error".to_string()],
        }
    }
}