use reqwasm::http::Request;
use web_common::api::oauth::jwt_cookies::FULL_LOGOUT_ENDPOINT;

use crate::api::{ApiError, FrontendApiError};

/// Logout the user.
///
/// # Arguments
/// * `access_token` - The access token of the user to logout.
pub async fn logout() -> Result<(), FrontendApiError> {
    let response =
        Request::get(FULL_LOGOUT_ENDPOINT).send().await.map_err(FrontendApiError::from)?;

    match response.status() {
        200 => Ok(()),
        _ => {
            let api_error: ApiError = response.json().await.map_err(FrontendApiError::from)?;
            Err(FrontendApiError::from(api_error))
        }
    }
}
