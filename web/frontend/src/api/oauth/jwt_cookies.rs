//! Submodule for APIs involving JWT cookies.
use crate::api::FrontendApiError;
use reqwasm::http::Request;
use web_common::api::{oauth::jwt_cookies::*, ApiError};

/// Refresh the JWT cookie.
pub async fn refresh_jwt_cookie() -> Result<AccessToken, FrontendApiError> {
    let response = Request::get(FULL_REFRESH_ENDPOINT)
        .send()
        .await
        .map_err(FrontendApiError::from)?;

    match response.status() {
        200 => {
            let access_token: AccessToken =
                response.json().await.map_err(FrontendApiError::from)?;
            Ok(access_token)
        }
        _ => {
            let api_error: ApiError = response.json().await.map_err(FrontendApiError::from)?;
            Err(FrontendApiError::from(api_error))
        }
    }
}
