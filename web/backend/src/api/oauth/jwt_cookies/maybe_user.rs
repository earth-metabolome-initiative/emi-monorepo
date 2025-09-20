//! Submodule defining the `MaybeUser` type, which is used to represent a user
//! that may be either a regular user, a temporary user, or an anonymous user.
//! This is used in the context of JWT authentication and cookies, where a user
//! may be authenticated via a JWT token or may be anonymous.

use std::{fmt::Display, future::ready, pin::Pin};

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

use super::{JsonAccessToken, REFRESH_COOKIE_NAME};
use crate::{BackendError, KeyPair};

#[derive(Debug, Clone, PartialEq)]
pub enum MaybeUser {
    TemporaryUser(TemporaryUser),
    User(User),
    Anonymous,
}

impl Display for MaybeUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeUser::TemporaryUser(user) => write!(f, "TemporaryUser(id={})", user.id),
            MaybeUser::User(user) => write!(f, "User(id={})", user.id),
            MaybeUser::Anonymous => write!(f, "Anonymous"),
        }
    }
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
        // We try to extract the refresh cookie if it is present.
        let token = if let Some(cookie) = req.cookie(REFRESH_COOKIE_NAME) {
            cookie.value().to_owned()
        } else if let Ok(bearer) = BearerAuth::from_request(req, payload).into_inner() {
            bearer.token().to_owned()
        } else {
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
        let Some(diesel_pool) =
            req.app_data::<Data<crate::DBPool>>().map(|pool| pool.get_ref().clone())
        else {
            unreachable!("Diesel pool not present in request - server has been misconfigured");
        };

        let Some(key): Option<KeyPair> =
            req.app_data::<Data<KeyPair>>().map(|key| key.get_ref().clone())
        else {
            unreachable!("KeyPair not present in request - server has been misconfigured");
        };

        Box::pin(async move {
            let access_token = JsonAccessToken::decode(token.as_str(), &key)?;

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
