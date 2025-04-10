//! API endpoint to refresh the access token.
use actix_web::{get, web, HttpRequest, HttpResponse};
use backend_request_errors::BackendRequestError;
use core_structures::User;
use web_common::api::{oauth::jwt_cookies::AccessToken, ApiError};
use web_common_traits::database::Loadable;

use crate::api::oauth::jwt_cookies::{
    eliminate_cookies, JsonAccessToken, JsonRefreshToken, REFRESH_COOKIE_NAME,
};

#[get("/refresh")]
/// Endpoint to refresh the access token, given a valid refresh token.
///
/// # Arguments
/// * `req` - The request to refresh the token.
/// * `pool` - The database pool to use for the login.
/// * `redis_client` - The redis client to use for the login.
///
/// # Implementation details
/// The refresh token is expected to be present in the request as a cookie, with
/// the name defined by the constant REFRESH_COOKIE_NAME. The refresh token is
/// then decoded, and checked whether it is still present in the redis database.
/// If it is, we check that the user associated to the token still exists in the
/// database, and if it does, we create a new access token and return it. If any
/// of the checks fail, we return an error.
pub async fn refresh_access_token_handler(
    req: HttpRequest,
    pool: web::Data<crate::DBPool>,
    redis_client: web::Data<redis::Client>,
) -> HttpResponse {
    match refresh_access_token(&req, &pool, &redis_client).await {
        Ok(response) => response.1.into(),
        Err(error) => error,
    }
}

pub async fn refresh_access_token(
    req: &HttpRequest,
    pool: &web::Data<crate::DBPool>,
    redis_client: &web::Data<redis::Client>,
) -> Result<(User, AccessToken), HttpResponse> {
    let Some(refresh_cookie) = req.cookie(REFRESH_COOKIE_NAME) else {
        log::debug!("Refresh token not present in request");
        return Err(eliminate_cookies(HttpResponse::Unauthorized()).json(Error::Unauthorized));
    };

    let Ok(refresh_token) = JsonRefreshToken::decode(refresh_cookie.value()) else {
        log::debug!("Unable to decode refresh token");
        return Err(eliminate_cookies(HttpResponse::Unauthorized()).json(Error::Unauthorized));
    };

    // If the token is expired, we return an error.
    if refresh_token.is_expired() {
        return Err(eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::Unauthorized));
    }

    // Next up, we check whether the token is still present in the redis database.
    let is_still_present = refresh_token.is_still_present_in_redis(&redis_client).await;

    if is_still_present.map_or(true, |present| !present) {
        log::debug!("Refresh token not present in redis");
        return Err(eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::Unauthorized));
    }

    // Finally, we get the user from the database, as while the user indeed seems
    // to be authenticated and it still exists in the redis database, we still need
    // to check whether the user exists in the database, as it may have been deleted
    // in the meantime.
    let mut connection = pool.get().await.map_err(ApiError::from)?;

    // If the user doesn't exist, we return an error, otherwise we return the user.
    let Some(user) = User::load(&refresh_token.user_id(), &mut connection).await.ok().flatten()
    else {
        return Err(eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::Unauthorized));
    };

    // If the user exists, we create a new access token and return it.
    let Ok(access_token) = JsonAccessToken::new(refresh_token.user_id()) else {
        return Err(HttpResponse::InternalServerError().json(ApiError::internal_server_error()));
    };

    if let Err(_err) = access_token.insert_into_redis(&redis_client).await {
        return Err(HttpResponse::InternalServerError().json(ApiError::internal_server_error()));
    }

    // We return the access token as part of the JSON response, and we
    // expect the frontend to store it in a variable and use it for
    // future requests as part of the Authorization header. This means
    // that upon refreshing the page the user will have to request a
    // new access token, but this is a security measure to prevent
    // unauthorized access to the API.

    match access_token.encode() {
        Ok(encoded_token) => Ok((user, AccessToken::new(encoded_token))),
        Err(_) => Err(HttpResponse::InternalServerError().json(ApiError::internal_server_error())),
    }
}
