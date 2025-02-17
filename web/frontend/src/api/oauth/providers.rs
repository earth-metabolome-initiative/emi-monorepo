use reqwasm::http::Request;
use web_common::api::oauth::providers::*;

use crate::api::FrontendApiError;

pub async fn retrieve_login_providers() -> Result<Vec<OAuth2LoginProvider>, FrontendApiError> {
    Request::get(FULL_ENDPOINT)
        .send()
        .await
        .map_err(FrontendApiError::from)?
        .json()
        .await
        .map_err(FrontendApiError::from)
}
