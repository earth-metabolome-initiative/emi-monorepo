//! Functions and APIs dealing with JWT cookies necessary for OAuth2 logins.
use crate::models::User;
use crate::DieselConn;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::dev::Payload;
use actix_web::dev::ServiceRequest;
use actix_web::error::{Error, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use base64::prelude::*;
use chrono::Duration;
use chrono::Utc;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use futures::Future;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::future::ready;
use std::num::ParseIntError;
use std::pin::Pin;
use uuid::Uuid;
use web_common::api::oauth::jwt_cookies::*;
use web_common::api::ApiError;

/// Set a const with the expected cookie name.
pub(crate) const REFRESH_COOKIE_NAME: &str = "refresh_token";

struct JWTConfig {
    access_token_base_64_public_key: String,
    access_token_base_64_private_key: String,
    access_token_minutes: i64,
    refresh_token_base_64_public_key: String,
    refresh_token_base_64_private_key: String,
    refresh_token_minutes: i64,
}

impl JWTConfig {
    pub fn from_env() -> Result<JWTConfig, String> {
        dotenvy::dotenv().ok();
        Ok(JWTConfig {
            access_token_base_64_public_key: env::var("ACCESS_TOKEN_PUBLIC_KEY")
                .map_err(|e| e.to_string())?,
            access_token_base_64_private_key: env::var("ACCESS_TOKEN_PRIVATE_KEY")
                .map_err(|e| e.to_string())?,
            access_token_minutes: env::var("ACCESS_TOKEN_MINUTES")
                .map_err(|e| e.to_string())?
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
            refresh_token_base_64_public_key: env::var("REFRESH_TOKEN_PUBLIC_KEY")
                .map_err(|e| e.to_string())?,
            refresh_token_base_64_private_key: env::var("REFRESH_TOKEN_PRIVATE_KEY")
                .map_err(|e| e.to_string())?,
            refresh_token_minutes: env::var("REFRESH_TOKEN_MINUTES")
                .map_err(|e| e.to_string())?
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?,
        })
    }

    pub fn access_token_public_key(&self) -> Result<String, String> {
        String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&self.access_token_base_64_public_key)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    pub fn access_token_private_key(&self) -> Result<String, String> {
        String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&self.access_token_base_64_private_key)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    pub fn access_token_minutes(&self) -> i64 {
        self.access_token_minutes
    }

    pub fn refresh_token_public_key(&self) -> Result<String, String> {
        String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&self.refresh_token_base_64_public_key)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    pub fn refresh_token_private_key(&self) -> Result<String, String> {
        String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&self.refresh_token_base_64_private_key)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    pub fn refresh_token_minutes(&self) -> i64 {
        self.refresh_token_minutes
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct JsonWebToken {
    sub: Uuid,
    token_id: Uuid,
    exp: usize,
    created_at: usize,
}

impl JsonWebToken {
    fn new(user_id: Uuid, minutes: i64) -> JsonWebToken {
        let token_id = Uuid::new_v4();
        let now = Utc::now();
        let created_at = now.timestamp() as usize;
        let expires_in = (now + Duration::try_minutes(minutes).unwrap()).timestamp() as usize;
        JsonWebToken {
            sub: user_id,
            token_id,
            exp: expires_in,
            created_at,
        }
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    /// * `minutes` - The duration of the token in minutes.
    async fn insert_into_redis(
        &self,
        redis_client: &redis::Client,
        minutes: i64,
    ) -> Result<(), String> {
        // We insert the token: user_id pair into the redis database,
        // with as duration the same as the duration of the token.

        let mut con = redis_client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;

        con.set_ex(
            self.token_id.to_string(),
            self.user_id().to_string(),
            (minutes * 60) as u64,
        )
        .await
        .map_err(|e| e.to_string())
    }

    /// Checks whether the token is still present in the redis database and has the same user_id.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, String> {
        let mut con = redis_client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;

        // While extremely unlikely, it is possible that the token is still present in the
        // redis database with a different user_id. This may happen due to a collision, or
        // more likely, if some potentially malicious action is taking place.
        let user_id: String = con
            .get(self.token_id.to_string())
            .await
            .map_err(|e| e.to_string())?;

        Ok(user_id == self.user_id().to_string())
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn delete_from_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        let mut con = redis_client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;

        con.del(self.token_id.to_string())
            .await
            .map_err(|e| e.to_string())
    }

    fn user_id(&self) -> Uuid {
        self.sub
    }

    fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp() as usize;
        now > self.exp
    }

    fn encode(&self, private_key: String) -> Result<String, String> {
        log::info!("Encoding token");
        encode(
            &Header::new(Algorithm::RS256),
            &self,
            &EncodingKey::from_rsa_pem(private_key.as_bytes()).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    fn decode(token: &str, public_key: String) -> Result<JsonWebToken, String> {
        log::info!("Decoding token");
        let decode = decode::<JsonWebToken>(
            token,
            &DecodingKey::from_rsa_pem(public_key.as_bytes()).map_err(|e| e.to_string())?,
            &Validation::new(Algorithm::RS256),
        );

        match decode {
            Ok(token) => Ok(token.claims),
            Err(_) => Err("Invalid token".to_string()),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(crate) struct JsonRefreshToken {
    web_token: JsonWebToken,
}

impl JsonRefreshToken {
    pub fn new(user_id: Uuid) -> Result<JsonRefreshToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonRefreshToken {
            web_token: JsonWebToken::new(user_id, config.refresh_token_minutes()),
        })
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn insert_into_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        let config = JWTConfig::from_env()?;
        self.web_token
            .insert_into_redis(redis_client, config.refresh_token_minutes())
            .await
    }

    /// Checks whether the token is still present in the redis database and has the same user_id.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, String> {
        self.web_token.is_still_present_in_redis(redis_client).await
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn delete_from_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        self.web_token.delete_from_redis(redis_client).await
    }

    pub fn user_id(&self) -> Uuid {
        self.web_token.user_id()
    }

    pub fn is_expired(&self) -> bool {
        self.web_token.is_expired()
    }

    pub fn encode(&self) -> Result<String, String> {
        let config = JWTConfig::from_env()?;
        self.web_token.encode(config.refresh_token_private_key()?)
    }

    pub fn decode(token: &str) -> Result<JsonRefreshToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonRefreshToken {
            web_token: JsonWebToken::decode(token, config.refresh_token_public_key()?)?,
        })
    }
}

/// We do a small unit test to ensure that the encode and decode process works as expected.
/// for the JsonRefreshToken struct.
#[cfg(test)]
mod refresh_token_tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        dotenvy::dotenv().ok();
        let user_id = Uuid::new_v4();
        let token = JsonRefreshToken::new(user_id).unwrap();
        let encoded = token.encode().unwrap();
        let decoded = JsonRefreshToken::decode(&encoded).unwrap();
        assert_eq!(token, decoded);
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub(crate) struct JsonAccessToken {
    web_token: JsonWebToken,
}

impl JsonAccessToken {
    pub fn new(user_id: Uuid) -> Result<JsonAccessToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonAccessToken {
            web_token: JsonWebToken::new(user_id, config.access_token_minutes()),
        })
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn insert_into_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        let config = JWTConfig::from_env()?;
        self.web_token
            .insert_into_redis(redis_client, config.access_token_minutes())
            .await
    }

    /// Checks whether the token is still present in the redis database and has the same user_id.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, String> {
        self.web_token.is_still_present_in_redis(redis_client).await
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    pub async fn delete_from_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        self.web_token.delete_from_redis(redis_client).await
    }

    fn user_id(&self) -> Uuid {
        self.web_token.user_id()
    }

    fn is_expired(&self) -> bool {
        self.web_token.is_expired()
    }

    pub fn encode(&self) -> Result<String, String> {
        let config = JWTConfig::from_env()?;
        self.web_token.encode(config.access_token_private_key()?)
    }

    pub fn decode(token: &str) -> Result<JsonAccessToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonAccessToken {
            web_token: JsonWebToken::decode(token, config.access_token_public_key()?)?,
        })
    }
}

/// We do a small unit test to ensure that the encode and decode process works as expected
/// for the JsonAccessToken struct.
#[cfg(test)]
mod access_token_tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        dotenvy::dotenv().ok();
        let user_id = Uuid::new_v4();
        let token = JsonAccessToken::new(user_id).unwrap();
        let encoded = token.encode().unwrap();
        let decoded = JsonAccessToken::decode(&encoded).unwrap();
        assert_eq!(token, decoded);
    }
}

/// Function to create a JWT cookie for a given user.
///
/// # Arguments
/// * `user_id` - The ID of the user to create the JWT for.
/// * `redis_client` - The redis client to use for the login.
async fn encode_jwt_refresh_cookie<'a>(
    user_id: Uuid,
    redis_client: &redis::Client,
) -> Result<Cookie<'a>, String> {
    log::info!("Creating refresh token");
    let config = JWTConfig::from_env()?;

    let token = JsonRefreshToken::new(user_id)?;

    token.insert_into_redis(redis_client).await?;

    let cookie = Cookie::build(REFRESH_COOKIE_NAME, token.encode()?)
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(ActixWebDuration::minutes(config.refresh_token_minutes()))
        // The HTTP_ONLY flag is set to true to prevent the cookie from being accessed by
        // JavaScript. This is a security measure to prevent XSS attacks.
        .http_only(true)
        .finish();

    log::info!("Created refresh cookie: {:?}", cookie);

    Ok(cookie)
}

/// Function to create the user online cookie
fn encode_user_online_cookie<'a>() -> Result<Cookie<'a>, String> {
    let config = JWTConfig::from_env()?;
    Ok(Cookie::build(USER_ONLINE_COOKIE_NAME, "true")
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(ActixWebDuration::minutes(config.refresh_token_minutes()))
        // We want to be able to check the existance of this cookie from the frontend
        // by using javascript (or in our case Yew) so we set http_only to false.
        .http_only(false)
        .finish())
}

/// Function to build the overall response for a successful login.
///
/// # Arguments
/// * `user_id` - The ID of the user that has logged in.
/// * `state` - The state to redirect to after the login.
/// * `redis_client` - The redis client to use for the login.
pub(crate) async fn build_login_response<'a>(
    user_id: Uuid,
    state: &str,
    redis_client: &redis::Client,
) -> HttpResponse {
    let refresh_cookie = match encode_jwt_refresh_cookie(user_id, redis_client).await {
        Ok(cookie) => cookie,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
        }
    };

    log::info!("Created refresh token");

    log::info!("Creating login token");

    let login_cookie = match encode_user_online_cookie() {
        Ok(cookie) => cookie,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
        }
    };

    log::info!("Created login token");

    let response = HttpResponse::Found()
        .append_header((LOCATION, state.to_string()))
        .cookie(refresh_cookie)
        .cookie(login_cookie)
        .finish();

    log::info!("Returning login response");

    response
}

pub(crate) async fn access_token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    log::info!("Validating access token");
    match JsonAccessToken::decode(credentials.token()) {
        Ok(token) => {
            if token.is_expired() {
                return Err((
                    ErrorUnauthorized(json!({"status": "fail", "message": "Token has expired"})),
                    req,
                ));
            }
            Ok(req)
        }
        Err(_) => Err((
            ErrorUnauthorized(json!({"status": "fail", "message": "Invalid token"})),
            req,
        )),
    }
}

pub(crate) fn eliminate_cookies(mut builder: HttpResponseBuilder) -> HttpResponseBuilder {
    log::info!("Eliminating cookies");
    let refresh_cookie = Cookie::build(REFRESH_COOKIE_NAME, "")
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(true)
        .finish();

    let user_online_cookie = Cookie::build(USER_ONLINE_COOKIE_NAME, "")
        .same_site(actix_web::cookie::SameSite::Strict)
        .secure(true)
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(false)
        .finish();

    builder.cookie(refresh_cookie).cookie(user_online_cookie);
    builder
}

impl User {
    pub(crate) fn is_token_expired<S>(token: S) -> Result<bool, String>
    where
        S: AsRef<str>,
    {
        JsonAccessToken::decode(token.as_ref()).map(|token| token.is_expired())
    }

    pub(crate) async fn from_bearer_token<S>(
        redis_client: redis::Client,
        diesel_pool: Pool<ConnectionManager<PgConnection>>,
        token: S,
    ) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        let access_token = match JsonAccessToken::decode(token.as_ref()) {
            Ok(token) => token,
            Err(_) => {
                log::debug!("Unable to decode access token");
                return Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token"}),
                ));
            }
        };

        let mut conn: DieselConn = match diesel_pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                log::error!("Unable to get connection from pool.");
                return Err(ErrorInternalServerError(
                    json!({"status": "fail", "message": "Internal server error"}),
                ));
            }
        };

        // If the token is expired, we return an error.
        if access_token.is_expired() {
            log::debug!("Token has expired");
            return Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Token has expired"}),
            ));
        }

        let is_still_present = access_token.is_still_present_in_redis(&redis_client).await;

        if is_still_present.is_err() || !is_still_present.unwrap() {
            log::error!("Token not present in redis");
            return Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token"}),
            ));
        }

        // If the user doesn't exist, we return an error, otherwise we return the user.
        match User::get(access_token.user_id(), &mut conn) {
            Ok(user) => Ok(user),
            Err(_) => {
                log::debug!("Did not find user in database");
                Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
                ))
            }
        }
    }
}

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // First, we extract the token from the request.
        // An authentication header is expected to be present in the request, and is
        // of the form:
        // Authorization: Bearer <token here>
        //
        let bearer = match BearerAuth::from_request(req, payload).into_inner() {
            Ok(bearer) => bearer,
            Err(_) => {
                log::debug!("Bearer token not present in request");
                return Box::pin(ready(Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token"}),
                ))));
            }
        };

        // Next up, we check whether the token is still present in the redis database.
        let redis_client = match req.app_data::<web::Data<redis::Client>>() {
            Some(client) => client.clone(),
            None => {
                log::error!("Redis client not present in request");
                return Box::pin(ready(Err(ErrorInternalServerError(
                    json!({"status": "fail", "message": "Internal server error"}),
                ))));
            }
        }
        .get_ref()
        .clone();

        // Finally, we get the user from the database, as while the user indeed seems
        // to be authenticated and it still exists in the redis database, we still need
        // to check whether the user exists in the database, as it may have been deleted
        // in the meantime.
        let diesel_pool =
            match req.app_data::<web::Data<Pool<ConnectionManager<PgConnection>>>>() {
                Some(pool) => pool.clone(),
                None => {
                    log::error!("Database pool not present in request");
                    return Box::pin(ready(Err(ErrorInternalServerError(
                        json!({"status": "fail", "message": "Internal server error"}),
                    ))));
                }
            }
            .get_ref()
            .clone();

        Box::pin(async move {
            User::from_bearer_token(redis_client, diesel_pool, &bearer.token()).await
        })
    }
}
