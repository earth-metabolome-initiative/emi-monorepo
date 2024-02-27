use crate::api::FrontendApiError;
use reqwasm::http::Request;
use web_common::api::oauth::jwt_cookies::*;
use web_common::api::{auth::users::me::*, ApiError};
use web_common::user::User;
use crate::api::utils::add_bearer;

/// Returns the informations regarding the currently logged user.
///
/// # Arguments
/// * `access_token` - The access token of the user to get the informations from.
pub async fn me(access_token: &AccessToken) -> Result<User, FrontendApiError> {
    let response = add_bearer(
        Request::get(FULL_ENDPOINT),
        &access_token
    )
        .send()
        .await
        .map_err(FrontendApiError::from)?;

    match response.status() {
        200 => {
            let user: User = response.json().await.map_err(FrontendApiError::from)?;
            Ok(user)
        }
        _ => {
            let api_error: ApiError = response.json().await.map_err(FrontendApiError::from)?;
            Err(FrontendApiError::from(api_error))
        }
    }
}
