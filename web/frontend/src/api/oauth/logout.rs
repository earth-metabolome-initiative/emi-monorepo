use crate::api::utils::add_optional_bearer;
use crate::api::{ApiError, FrontendApiError};
use reqwasm::http::Request;
use web_common::api::oauth::jwt_cookies::{AccessToken, FULL_LOGOUT_ENDPOINT};

/// Logout the user.
///
/// # Arguments
/// * `access_token` - The access token of the user to logout.
pub async fn logout(access_token: Option<&AccessToken>) -> Result<(), FrontendApiError> {
    let response = add_optional_bearer(Request::get(FULL_LOGOUT_ENDPOINT), access_token)
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
