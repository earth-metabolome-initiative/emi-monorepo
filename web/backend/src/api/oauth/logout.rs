//! API endpoint to logout the user.
use crate::api::oauth::jwt_cookies::{
    eliminate_cookies, JsonAccessToken, JsonRefreshToken, REFRESH_COOKIE_NAME,
};
use actix_web::error::Error;
use actix_web::dev::Payload;
use actix_web::FromRequest;
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use web_common::api::ApiError;
use std::future::ready;
use std::future::Ready;

struct MaybeBearer {
    bearer: Option<BearerAuth>,
}

impl FromRequest for MaybeBearer {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(Ok(MaybeBearer {
            bearer: BearerAuth::from_request(req, payload).into_inner().ok(),
        }))
    }
}

#[get("/logout")]
/// Endpoint to logout the user.
///
/// # Arguments
/// * `req` - The request to logout the user.
/// * `bearer` - The bearer token of the user to logout.
/// * `redis_client` - The redis client to use for the login.
///
/// # Implementative details
/// The logout endpoint is expected to be called with a valid access token, and will
/// remove the access token from the redis database, effectively logging the user out.
/// It will also remove the refresh token from the redis database, and delete the refresh
/// token cookie from the user's browser, along with the user online cookie.
pub async fn logout(
    req: HttpRequest,
    maybe_bearer: MaybeBearer,
    redis_client: web::Data<redis::Client>,
) -> HttpResponse {
    // We try to extract the bearer token from the request. If we find it,
    // we handle the removal of the access token from the redis database,
    // otherwise we solely remove the refresh token from the redis database.

    if let Some(bearer) = maybe_bearer.bearer {
        let access_token = match JsonAccessToken::decode(bearer.token()) {
            Ok(token) => token,
            Err(_) => {
                log::debug!("Unable to decode access token");
                return eliminate_cookies(HttpResponse::Unauthorized())
                    .json(ApiError::unauthorized());
            }
        };

        let is_still_present = access_token.is_still_present_in_redis(&redis_client).await;

        if is_still_present.map_or(true, |present| !present) {
            log::debug!("Access token not present in redis");
            return eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::unauthorized());
        }

        match access_token.delete_from_redis(&redis_client).await {
            Ok(_) => (),
            Err(_) => {
                log::error!("Unable to delete access token from redis");
                return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
            }
        }
    }

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

    let is_still_present = refresh_token.is_still_present_in_redis(&redis_client).await;

    if is_still_present.map_or(true, |present| !present) {
        log::debug!("Refresh token not present in redis");
        return eliminate_cookies(HttpResponse::Unauthorized()).json(ApiError::unauthorized());
    }

    match refresh_token.delete_from_redis(&redis_client).await {
        Ok(_) => (),
        Err(_) => {
            log::error!("Unable to delete refresh token from redis");
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
        }
    }

    log::debug!("Logging out user");
    // We delete the refresh token cookie and the user online cookie from the user's browser.
    eliminate_cookies(HttpResponse::Ok()).finish()
}
