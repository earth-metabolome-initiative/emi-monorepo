use serde::{Deserialize, Serialize};
pub(crate) mod oauth;
pub(crate) use oauth::*;

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