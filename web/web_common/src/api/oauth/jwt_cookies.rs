use serde::{Deserialize, Serialize};

use crate::combine_path;

pub const REFRESH_ENDPOINT: &str = "/refresh";
pub const FULL_REFRESH_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, REFRESH_ENDPOINT);

pub const LOGOUT_ENDPOINT: &str = "/logout";
pub const FULL_LOGOUT_ENDPOINT: &str = combine_path!(super::FULL_ENDPOINT, LOGOUT_ENDPOINT);

pub const USER_ONLINE_COOKIE_NAME: &str = "user_online";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AccessToken {
    token: String,
}

impl AccessToken {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn header(&self) -> (&str, String) {
        ("Authorization", format!("Bearer {}", self.token))
    }
}

#[cfg(feature = "backend")]
impl From<AccessToken> for actix_web::HttpResponse {
    fn from(token: AccessToken) -> Self {
        actix_web::HttpResponse::Ok().json(token)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RefreshError {
    Unauthorized,
}
