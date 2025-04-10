use serde::{Deserialize, Serialize};
pub(crate) mod auth;
pub(crate) mod oauth;
pub(crate) mod utils;

use web_common::api::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub enum FrontendApiError {
    API(ApiError),
    Reqwasm(String),
}

impl From<reqwasm::Error> for FrontendApiError {
    fn from(e: reqwasm::Error) -> Self {
        Self::Reqwasm(e.to_string())
    }
}

impl From<ApiError> for FrontendApiError {
    fn from(e: ApiError) -> Self {
        Self::API(e)
    }
}

impl From<serde_json::Error> for FrontendApiError {
    fn from(e: serde_json::Error) -> Self {
        Self::API(ApiError::from(e))
    }
}
