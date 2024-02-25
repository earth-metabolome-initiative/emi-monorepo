use serde::{Deserialize, Serialize};
pub mod oauth;
pub mod auth;


pub const ENDPOINT: &str = "/api";
pub const FULL_ENDPOINT: &str = ENDPOINT;

#[derive(Debug, Deserialize, Serialize)]
pub enum ApiError {
    Oauth(oauth::OauthErrors),
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
}
