use crate::api::FrontendApiError;
use reqwasm::http::Request;
use web_common::api::oauth::providers::*;

pub async fn retrieve_login_providers() -> Result<Vec<OAuth2LoginProvider>, FrontendApiError> {
    Request::get(FULL_ENDPOINT)
        .send()
        .await
        .map_err(FrontendApiError::from)?
        .json()
        .await
        .map_err(FrontendApiError::from)
}
