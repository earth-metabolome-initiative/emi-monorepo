//! Submodule defining the `MaybeUser` type, which is used to represent a user
//! that may be either a regular user, a temporary user, or an anonymous user.
//! This is used in the context of JWT authentication and cookies, where a user
//! may be authenticated via a JWT token or may be anonymous.

use std::{future::ready, pin::Pin};

use actix_http::Payload;
use actix_web::{
    Error as ActixError, FromRequest, HttpRequest,
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web::Data,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use core_structures::{TemporaryUser, User};
use diesel::OptionalExtension;
use web_common_traits::database::Read;

use super::JsonAccessToken;
use crate::BackendError;

#[derive(Debug, Clone, PartialEq)]
pub enum MaybeUser {
    TemporaryUser(TemporaryUser),
    User(User),
    Anonymous,
}

impl From<User> for MaybeUser {
    fn from(user: User) -> Self {
        MaybeUser::User(user)
    }
}

impl From<TemporaryUser> for MaybeUser {
    fn from(user: TemporaryUser) -> Self {
        MaybeUser::TemporaryUser(user)
    }
}

impl TryFrom<MaybeUser> for User {
    type Error = BackendError;

    fn try_from(value: MaybeUser) -> Result<Self, Self::Error> {
        match value {
            MaybeUser::User(user) => Ok(user),
            MaybeUser::TemporaryUser(_) | MaybeUser::Anonymous => Err(BackendError::Unauthorized),
        }
    }
}

impl TryFrom<MaybeUser> for TemporaryUser {
    type Error = BackendError;

    fn try_from(value: MaybeUser) -> Result<Self, Self::Error> {
        match value {
            MaybeUser::TemporaryUser(user) => Ok(user),
            MaybeUser::User(_) | MaybeUser::Anonymous => Err(BackendError::Unauthorized),
        }
    }
}

impl FromRequest for MaybeUser {
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // First, we extract the token from the request.
        // An authentication header is expected to be present in the request, and is
        // of the form:
        // Authorization: Bearer <token here>
        //

        // TODO! INCLUDE THE HANDLING OF COOKIES HERE AS WELL.

        let Ok(bearer) = BearerAuth::from_request(req, payload).into_inner() else {
            return Box::pin(ready(Ok(MaybeUser::Anonymous)));
        };

        // Next up, we check whether the token is still present in the redis database.
        let redis_client = if let Some(client) = req.app_data::<Data<redis::Client>>() {
            client.get_ref().clone()
        } else {
            log::error!("Redis client not present in request");
            return Box::pin(ready(Err(ErrorInternalServerError(
                serde_json::json!({"status": "fail", "message": "Internal server error"}),
            ))));
        };

        // Finally, we get the user from the database, as while the user indeed seems
        // to be authenticated and it still exists in the redis database, we still need
        // to check whether the user exists in the database, as it may have been deleted
        // in the meantime.
        let diesel_pool = if let Some(pool) = req.app_data::<Data<crate::DBPool>>() {
            pool.get_ref().clone()
        } else {
            log::error!("Database pool not present in request");
            return Box::pin(ready(Err(ErrorInternalServerError(
                serde_json::json!({"status": "fail", "message": "Internal server error"}),
            ))));
        };

        Box::pin(async move {
            let token = bearer.token();
            let access_token = JsonAccessToken::decode(token)?;

            let mut conn = diesel_pool.get().map_err(BackendError::from)?;

            // If the token is expired, we return an error.
            if access_token.is_expired() {
                log::debug!("Token has expired");
                return Err(ErrorUnauthorized(
                    serde_json::json!({"status": "fail", "message": "Token has expired"}),
                ));
            }

            if access_token
                .is_still_present_in_redis(&redis_client)
                .await
                .map_or(true, |present| !present)
            {
                log::error!("Token not present in redis");
                return Err(ErrorUnauthorized(
                    serde_json::json!({"status": "fail", "message": "Invalid token"}),
                ));
            }

            let user_wrapper: MaybeUser = if access_token.is_temporary() {
                match TemporaryUser::read(access_token.user_id(), &mut conn)
                    .optional()
                    .map_err(BackendError::from)?
                {
                    None => {
                        return Err(BackendError::Unauthorized.into());
                    }
                    Some(user) => user.into(),
                }
            } else {
                match User::read(access_token.user_id(), &mut conn)
                    .optional()
                    .map_err(BackendError::from)?
                {
                    None => {
                        return Err(BackendError::Unauthorized.into());
                    }
                    Some(user) => user.into(),
                }
            };

            Ok(user_wrapper)
        })
    }
}
