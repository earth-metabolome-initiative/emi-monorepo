use crate::api::FrontendApiError;
use crate::stores::user_state::UserState;
use reqwasm::http::Request;
use web_common::api::oauth::jwt_cookies::*;
use web_common::api::{auth::users::me::*, ApiError};
use yewdux::prelude::*;
use web_common::user::User;
use std::rc::Rc;

/// Returns the informations regarding the currently logged user.
pub async fn me(access_token: &AccessToken) -> Result<User, FrontendApiError> {
    let header = access_token.header();

    let response = Request::get(FULL_ENDPOINT)
        .header(header.0, &header.1)
        .send()
        .await
        .map_err(FrontendApiError::from)?;

    match response.status() {
        200 => {
            let user: User = response.json().await.map_err(FrontendApiError::from)?;
            Ok(user)
        }
        _ => {
            let api_error: ApiError =
                response.json().await.map_err(FrontendApiError::from)?;
            Err(FrontendApiError::from(api_error))
        }
    }
}
