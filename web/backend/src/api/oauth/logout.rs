//! API endpoint to logout the user.
use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, HttpResponse, dev::Payload, error::Error, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    api::oauth::jwt_cookies::{
        eliminate_cookies, JsonAccessToken, JsonRefreshToken, REFRESH_COOKIE_NAME
    },
    errors::BackendError,
};

struct MaybeBearer {
    bearer: Option<BearerAuth>,
}

impl FromRequest for MaybeBearer {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(Ok(MaybeBearer { bearer: BearerAuth::from_request(req, payload).into_inner().ok() }))
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
/// # Implementation details
/// The logout endpoint is expected to be called with a valid access token, and
/// will remove the access token from the redis database, effectively logging
/// the user out. It will also remove the refresh token from the redis database,
/// and delete the refresh token cookie from the user's browser, along with the
/// user online cookie.
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
            Err(error) => {
                return error.into();
            }
        };

        let is_still_present = access_token.is_still_present_in_redis(&redis_client).await;

        if is_still_present.map_or(true, |present| !present) {
            return BackendError::Unauthorized.into();
        }

        if let Err(error) = access_token.delete_from_redis(&redis_client).await {
            return error.into();
        }
    }

    let refresh_cookie = match req.cookie(REFRESH_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => {
            return BackendError::Unauthorized.into();
        }
    };

    let refresh_token = match JsonRefreshToken::decode(refresh_cookie.value()) {
        Ok(token) => token,
        Err(_) => {
            return BackendError::Unauthorized.into();
        }
    };

    if refresh_token.is_still_present_in_redis(&redis_client).await.map_or(true, |present| !present)
    {
        return BackendError::Unauthorized.into();
    }

    if let Err(error) = refresh_token.delete_from_redis(&redis_client).await {
        return error.into();
    }

    eliminate_cookies(HttpResponse::Ok())
}
