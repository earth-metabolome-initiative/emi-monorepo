//! Functions and APIs dealing with JWT cookies necessary for `OAuth2` logins.
use actix_web::{
    HttpResponse, HttpResponseBuilder,
    cookie::{Cookie, time::Duration as ActixWebDuration},
    dev::ServiceRequest,
    http::header::LOCATION,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use api_path::api::oauth::jwt_cookies::USER_ONLINE_COOKIE_NAME;
use base64::prelude::*;
use chrono::{Duration, Utc};
use core_structures::LoginProvider;
use diesel::PgConnection;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use redis::AsyncCommands;
use rosetta_uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::errors::BackendError;

mod known_user;
use known_user::handle_known_user;
mod unknown_user;
use unknown_user::handle_unknown_user;
mod maybe_user;
pub use maybe_user::MaybeUser;

/// Set a const with the expected cookie name.
pub(crate) const REFRESH_COOKIE_NAME: &str = "refresh_token";

pub(crate) fn eliminate_cookies(mut builder: HttpResponseBuilder) -> HttpResponse {
    let refresh_cookie = Cookie::build(REFRESH_COOKIE_NAME, "")
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .http_only(true)
        .finish();

    let user_online_cookie = Cookie::build(USER_ONLINE_COOKIE_NAME, "")
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .http_only(false)
        .finish();

    builder.cookie(refresh_cookie).cookie(user_online_cookie);
    builder.finish()
}

#[derive(Debug)]
struct JWTConfig {
    access_token_public_key: String,
    access_token_private_key: String,
    access_token_minutes: i64,
    refresh_token_minutes: i64,
}

impl JWTConfig {
    fn from_env() -> Result<JWTConfig, BackendError> {
        Ok(JWTConfig {
            access_token_public_key: String::from_utf8(
                base64::engine::general_purpose::STANDARD
                    .decode(&std::env::var("ACCESS_TOKEN_PUBLIC_KEY")?)?,
            )?,
            access_token_private_key: String::from_utf8(
                base64::engine::general_purpose::STANDARD
                    .decode(&std::env::var("ACCESS_TOKEN_PRIVATE_KEY")?)?,
            )?,
            access_token_minutes: std::env::var("ACCESS_TOKEN_MINUTES")?.parse()?,
            refresh_token_minutes: std::env::var("REFRESH_TOKEN_MINUTES")?.parse()?,
        })
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct JsonWebToken {
    user_id: i32,
    token_id: Uuid,
    exp: i64,
    created_at: i64,
    temporary: bool,
}

impl JsonWebToken {
    fn new(user_id: i32, temporary: bool, minutes: i64) -> JsonWebToken {
        let token_id = Uuid::new_v4();
        let now = Utc::now();
        let created_at = now.timestamp();
        let expires_in = (now + Duration::try_minutes(minutes).unwrap()).timestamp();
        JsonWebToken { user_id, token_id, exp: expires_in, created_at, temporary }
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    /// * `minutes` - The duration of the token in minutes.
    async fn insert_into_redis(
        &self,
        redis_client: &redis::Client,
        minutes: u64,
    ) -> Result<(), BackendError> {
        // We insert the token: `user_id` pair into the redis database,
        // with as duration the same as the duration of the token.

        let mut con = redis_client.get_multiplexed_async_connection().await?;

        Ok(con.set_ex(self.token_id, (self.user_id, self.temporary), minutes * 60).await?)
    }

    /// Checks whether the token is still present in the redis database and has
    /// the same `user_id`.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, BackendError> {
        let mut con = redis_client.get_multiplexed_async_connection().await?;

        // While extremely unlikely, it is possible that the token is still present in
        // the redis database with a different `user_id`. This may happen due to a
        // collision, or more likely, if some potentially malicious action is
        // taking place.
        let (user_id, temporary): (i32, bool) = con.get(self.token_id).await?;

        Ok(user_id == self.user_id && temporary == self.temporary)
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn delete_from_redis(&self, redis_client: &redis::Client) -> Result<(), BackendError> {
        let mut con = redis_client.get_multiplexed_async_connection().await?;

        Ok(con.del(self.token_id).await?)
    }

    fn user_id(&self) -> i32 {
        self.user_id
    }

    fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    fn encode(&self, private_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::new(Algorithm::RS256),
            &self,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())?,
        )
    }

    fn decode(
        token: &str,
        public_key: &str,
    ) -> Result<TokenData<JsonWebToken>, jsonwebtoken::errors::Error> {
        decode::<JsonWebToken>(
            token,
            &DecodingKey::from_rsa_pem(public_key.as_bytes())?,
            &Validation::new(Algorithm::RS256),
        )
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(crate) struct JsonRefreshToken {
    web_token: JsonWebToken,
}

impl JsonRefreshToken {
    pub(crate) fn new(user_id: i32, temporary: bool) -> Result<JsonRefreshToken, BackendError> {
        Ok(JsonRefreshToken {
            web_token: JsonWebToken::new(
                user_id,
                temporary,
                JWTConfig::from_env()?.refresh_token_minutes,
            ),
        })
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn insert_into_redis(&self, redis_client: &redis::Client) -> Result<(), BackendError> {
        let config = JWTConfig::from_env()?;
        self.web_token
            .insert_into_redis(redis_client, u64::try_from(config.refresh_token_minutes).unwrap())
            .await
    }

    /// Checks whether the token is still present in the redis database and has
    /// the same `user_id`.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub(crate) async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, BackendError> {
        self.web_token.is_still_present_in_redis(redis_client).await
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub(crate) async fn delete_from_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<(), BackendError> {
        self.web_token.delete_from_redis(redis_client).await
    }

    pub(crate) fn user_id(&self) -> i32 {
        self.web_token.user_id()
    }

    pub(crate) fn is_expired(&self) -> bool {
        self.web_token.is_expired()
    }

    pub(crate) fn encode(&self) -> Result<String, BackendError> {
        Ok(self.web_token.encode(&JWTConfig::from_env()?.access_token_private_key)?)
    }

    pub(crate) fn decode(token: &str) -> Result<JsonRefreshToken, BackendError> {
        Ok(JsonRefreshToken {
            web_token: JsonWebToken::decode(
                token,
                &JWTConfig::from_env()?.access_token_public_key,
            )?
            .claims,
        })
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(crate) struct JsonAccessToken {
    web_token: JsonWebToken,
}

impl JsonAccessToken {
    pub fn new(user_id: i32, temporary: bool) -> Result<JsonAccessToken, BackendError> {
        Ok(JsonAccessToken {
            web_token: JsonWebToken::new(
                user_id,
                temporary,
                JWTConfig::from_env()?.access_token_minutes,
            ),
        })
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn insert_into_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<(), BackendError> {
        let config = JWTConfig::from_env()?;
        self.web_token
            .insert_into_redis(redis_client, u64::try_from(config.access_token_minutes).unwrap())
            .await
    }

    /// Checks whether the token is still present in the redis database and has
    /// the same `user_id`.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, BackendError> {
        self.web_token.is_still_present_in_redis(redis_client).await
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn delete_from_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<(), BackendError> {
        self.web_token.delete_from_redis(redis_client).await
    }

    fn user_id(&self) -> i32 {
        self.web_token.user_id()
    }

    fn is_expired(&self) -> bool {
        self.web_token.is_expired()
    }

    fn is_temporary(&self) -> bool {
        self.web_token.temporary
    }

    pub fn encode(&self) -> Result<String, BackendError> {
        let config = JWTConfig::from_env()?;
        Ok(self.web_token.encode(&config.access_token_private_key)?)
    }

    pub fn decode(token: &str) -> Result<JsonAccessToken, BackendError> {
        let config = JWTConfig::from_env()?;
        Ok(JsonAccessToken {
            web_token: JsonWebToken::decode(token, &config.access_token_public_key)?.claims,
        })
    }
}

/// Function to create a JWT cookie for a given user.
///
/// # Arguments
/// * `user_id` - The ID of the user to create the JWT for.
/// * `redis_client` - The redis client to use for the login.
/// * `temporary` - Whether the cookie is temporary or not.
async fn encode_jwt_refresh_cookie<'a>(
    user_id: i32,
    redis_client: &redis::Client,
    temporary: bool,
) -> Result<Cookie<'a>, BackendError> {
    log::info!("Creating refresh token");
    let config = JWTConfig::from_env()?;

    let token = JsonRefreshToken::new(user_id, temporary)?;

    token.insert_into_redis(redis_client).await?;

    let cookie = Cookie::build(REFRESH_COOKIE_NAME, token.encode()?)
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(ActixWebDuration::minutes(config.refresh_token_minutes))
        // The HTTP_ONLY flag is set to true to prevent the cookie from being accessed by
        // JavaScript. This is a security measure to prevent XSS attacks.
        .http_only(true)
        .finish();

    Ok(cookie)
}

/// Function to create the user online cookie
fn encode_user_online_cookie<'a>() -> Result<Cookie<'a>, BackendError> {
    let config = JWTConfig::from_env()?;
    Ok(Cookie::build(USER_ONLINE_COOKIE_NAME, "true")
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(ActixWebDuration::minutes(config.refresh_token_minutes))
        // We want to be able to check the existence of this cookie from the frontend
        // by using javascript (or in our case Yew) so we set http_only to false.
        .http_only(false)
        .finish())
}

/// Function to build the overall response for a successful login.
///
/// # Arguments
/// * `emails` - The emails of the user.
/// * `primary_email` - The primary email of the user.
/// * `provider` - The provider to use for the login.
/// * `state` - The state to redirect to after the login.
/// * `redis_client` - The redis client to use for the login.
/// * `conn` - The database connection to use for the login.
pub(crate) async fn build_login_response(
    emails: &[&str],
    primary_email: &str,
    provider: &LoginProvider,
    _state: &str,
    redis_client: &redis::Client,
    conn: &mut PgConnection,
) -> Result<HttpResponse, BackendError> {
    let refresh_cookie = if let Some(user) = handle_known_user(emails, provider, conn)? {
        encode_jwt_refresh_cookie(user.id, redis_client, false).await?
    } else {
        let temporary_user = handle_unknown_user(primary_email, provider, conn)?;
        encode_jwt_refresh_cookie(temporary_user.id, redis_client, true).await?
    };

    let login_cookie = encode_user_online_cookie()?;

    let response = HttpResponse::Found()
        .append_header((LOCATION, "/"))
        .cookie(refresh_cookie)
        .cookie(login_cookie)
        .finish();

    Ok(response)
}

pub(crate) async fn access_token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    match JsonAccessToken::decode(credentials.token()) {
        Ok(token) => {
            if token.is_expired() {
                return Err((BackendError::Unauthorized.into(), req));
            }
            Ok(req)
        }
        Err(_) => Err((BackendError::Unauthorized.into(), req)),
    }
}
