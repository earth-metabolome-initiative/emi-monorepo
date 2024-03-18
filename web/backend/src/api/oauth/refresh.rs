//! API endpoint to refresh the access token.
use actix_web::{get, web, HttpRequest, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use crate::models::User;
use web_common::api::ApiError;
use web_common::api::oauth::jwt_cookies::AccessToken;
use crate::api::oauth::jwt_cookies::{eliminate_cookies, REFRESH_COOKIE_NAME, JsonRefreshToken, JsonAccessToken};

#[get("/refresh")]
/// Endpoint to refresh the access token, given a valid refresh token.
///
/// # Arguments
/// * `req` - The request to refresh the token.
/// * `pool` - The database pool to use for the login.
/// * `redis_client` - The redis client to use for the login.
///
/// # Implementative details
/// The refresh token is expected to be present in the request as a cookie, with the name
/// defined by the constant REFRESH_COOKIE_NAME. The refresh token is then decoded, and
/// checked whether it is still present in the redis database. If it is, we check that the
/// user associated to the token still exists in the database, and if it does, we create a
/// new access token and return it. If any of the checks fail, we return an error.
pub async fn refresh_access_token(
    req: HttpRequest,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    redis_client: web::Data<redis::Client>,
) -> HttpResponse {
    log::info!("Refreshing access token");
    let refresh_cookie = match req.cookie(REFRESH_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => {
            log::debug!("Refresh token not present in request");
            return eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::unauthorized());
        }
    };

    let refresh_token = match JsonRefreshToken::decode(refresh_cookie.value()) {
        Ok(token) => token,
        Err(_) => {
            log::debug!("Unable to decode refresh token");
            return eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::unauthorized());
        }
    };

    // If the token is expired, we return an error.
    if refresh_token.is_expired() {
        return eliminate_cookies(HttpResponse::Unauthorized())
            .json(ApiError::ExpiredAuthorization);
    }

    // Next up, we check whether the token is still present in the redis database.
    let is_still_present = refresh_token.is_still_present_in_redis(&redis_client).await;

    if is_still_present.map_or(true, |present| !present) {
        log::debug!("Refresh token not present in redis");
        return eliminate_cookies(HttpResponse::Unauthorized())
            .json(ApiError::ExpiredAuthorization);
    }

    // Finally, we get the user from the database, as while the user indeed seems
    // to be authenticated and it still exists in the redis database, we still need
    // to check whether the user exists in the database, as it may have been deleted
    // in the meantime.
    let mut connection = match pool.get() {
        Ok(pool) => pool,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error())
        }
    };

    // If the user doesn't exist, we return an error, otherwise we return the user.
    let user = User::get(refresh_token.user_id(), &mut connection);

    if user.is_err() {
        log::debug!("User associated to token doesn't exist");
        return HttpResponse::Unauthorized().json(ApiError::unauthorized());
    }

    // If the user exists, we create a new access token and return it.
    let access_token = match JsonAccessToken::new(refresh_token.user_id()) {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error())
        }
    };

    match access_token.insert_into_redis(&redis_client).await {
        Ok(_) => (),
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error())
        }
    }

    // We return the access token as part of the JSON response, and we
    // expect the frontend to store it in a variable and use it for
    // future requests as part of the Authorization header. This means
    // that upon refreshing the page the user will have to request a
    // new access token, but this is a security measure to prevent
    // unauthorized access to the API.

    match access_token.encode() {
        Ok(encoded_token) => HttpResponse::Ok().json(AccessToken::new(encoded_token)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::internal_server_error()),
    }
}