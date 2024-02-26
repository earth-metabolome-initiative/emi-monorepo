use crate::api::{ApiError, FrontendApiError};
use reqwasm::http::Request;
use web_common::api::oauth::jwt_cookies::FULL_LOGOUT_ENDPOINT;

pub async fn logout() -> Result<(), FrontendApiError> {
    let response = Request::get(FULL_LOGOUT_ENDPOINT)
        .send()
        .await
        .map_err(FrontendApiError::from)?;

    match response.status() {
        200 => Ok(()),
        _ => {
            let api_error: ApiError = response.json().await.map_err(FrontendApiError::from)?;
            Err(FrontendApiError::from(api_error))
        }
    }
}
