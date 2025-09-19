//! API endpoint to refresh the access token.
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_codegen::get;
use api_path::api::oauth::jwt_cookies::AccessToken;
use core_structures::User;
use diesel::OptionalExtension;
use web_common_traits::database::Read;

use crate::{
    KeyPair,
    api::oauth::jwt_cookies::{JsonAccessToken, JsonRefreshToken, REFRESH_COOKIE_NAME},
    errors::BackendError,
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
/// the name defined by the constant `REFRESH_COOKIE_NAME`. The refresh token is
/// then decoded, and checked whether it is still present in the redis database.
/// If it is, we check that the user associated to the token still exists in the
/// database, and if it does, we create a new access token and return it. If any
/// of the checks fail, we return an error.
pub async fn refresh_access_token_handler(
    req: HttpRequest,
    pool: web::Data<crate::DBPool>,
    redis_client: web::Data<redis::Client>,
    key_pair: web::Data<crate::KeyPair>,
) -> HttpResponse {
    match refresh_access_token(&req, &pool, &redis_client, &key_pair).await {
        Ok((_, token)) => actix_web::HttpResponse::Ok().json(token),
        Err(error) => error.into(),
    }
}

pub(crate) async fn refresh_access_token(
    req: &HttpRequest,
    pool: &crate::DBPool,
    redis_client: &redis::Client,
    key_pair: &KeyPair,
) -> Result<(User, AccessToken), BackendError> {
    let refresh_cookie = req.cookie(REFRESH_COOKIE_NAME).ok_or(BackendError::Unauthorized)?;
    let refresh_token = JsonRefreshToken::decode(refresh_cookie.value(), key_pair)?;

    // If the token is expired, we return an error.
    if refresh_token.is_expired() || !refresh_token.is_still_present_in_redis(redis_client).await? {
        return Err(BackendError::Unauthorized);
    }

    // Finally, we get the user from the database, as while the user indeed seems
    // to be authenticated and it still exists in the redis database, we still need
    // to check whether the user exists in the database, as it may have been deleted
    // in the meantime.
    let mut connection = pool.get()?;

    // If the user doesn't exist, we return an error, otherwise we return the user.
    let user = User::read(refresh_token.user_id(), &mut connection)
        .optional()?
        .ok_or(BackendError::Unauthorized)?;

    // If the user exists, we create a new access token and return it.
    let access_token = JsonAccessToken::new(refresh_token.user_id(), false);

    access_token.insert_into_redis(redis_client).await?;

    // We return the access token as part of the JSON response, and we
    // expect the frontend to store it in a variable and use it for
    // future requests as part of the Authorization header. This means
    // that upon refreshing the page the user will have to request a
    // new access token, but this is a security measure to prevent
    // unauthorized access to the API.

    Ok((user, AccessToken::new(access_token.encode(key_pair)?)))
}
