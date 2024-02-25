//! Functions and APIs dealing with JWT cookies necessary for OAuth2 logins.
use crate::models::User;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::dev::Payload;
use actix_web::dev::ServiceRequest;
use actix_web::error::{Error, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::get;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use base64::prelude::*;
use chrono::Duration;
use chrono::Utc;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use futures::executor::block_on;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::future::{ready, Ready};
use std::num::ParseIntError;
use uuid::Uuid;

/// Set a const with the expected cookie name.
pub(crate) const REFRESH_COOKIE_NAME: &str = "refresh_token";
pub(crate) const USER_ONLINE_COOKIE_NAME: &str = "user_online";

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

    pub fn access_token_public_key(&self) -> Result<Vec<u8>, String> {
        BASE64_STANDARD
            .decode(&self.access_token_base_64_public_key)
            .map_err(|e| e.to_string())
    }

    pub fn access_token_private_key(&self) -> Result<Vec<u8>, String> {
        BASE64_STANDARD
            .decode(&self.access_token_base_64_private_key)
            .map_err(|e| e.to_string())
    }

    pub fn access_token_minutes(&self) -> i64 {
        self.access_token_minutes
    }

    pub fn refresh_token_public_key(&self) -> Result<Vec<u8>, String> {
        BASE64_STANDARD
            .decode(&self.refresh_token_base_64_public_key)
            .map_err(|e| e.to_string())
    }

    pub fn refresh_token_private_key(&self) -> Result<Vec<u8>, String> {
        BASE64_STANDARD
            .decode(&self.refresh_token_base_64_private_key)
            .map_err(|e| e.to_string())
    }

    pub fn refresh_token_minutes(&self) -> i64 {
        self.refresh_token_minutes
    }
}

#[derive(Deserialize, Serialize)]
struct JsonWebToken {
    user_id: Uuid,
    token_id: Uuid,
    expires_in: usize,
    created_at: usize,
}

impl JsonWebToken {
    fn new(user_id: Uuid, minutes: i64) -> JsonWebToken {
        let token_id = Uuid::new_v4();
        let now = Utc::now();
        let created_at = now.timestamp() as usize;
        let expires_in = (now + Duration::minutes(minutes)).timestamp() as usize;
        JsonWebToken {
            user_id,
            token_id,
            expires_in,
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
            self.user_id.to_string(),
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

        Ok(user_id == self.user_id.to_string())
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
        self.user_id
    }

    fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp() as usize;
        now > self.expires_in
    }

    fn encode(&self, private_key: Vec<u8>) -> Result<String, String> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(private_key.as_ref()),
        )
        .map_err(|e| e.to_string())
    }

    fn decode(token: &str, public_key: Vec<u8>) -> Result<JsonWebToken, String> {
        let decode = decode::<JsonWebToken>(
            token,
            &DecodingKey::from_rsa_pem(public_key.as_ref()).map_err(|e| e.to_string())?,
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => Ok(token.claims),
            Err(_) => Err("Invalid token".to_string()),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct JsonRefreshToken {
    web_token: JsonWebToken,
}

impl JsonRefreshToken {
    fn new(user_id: Uuid) -> Result<JsonRefreshToken, String> {
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
    async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, String> {
        self.web_token.is_still_present_in_redis(redis_client).await
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn delete_from_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        self.web_token.delete_from_redis(redis_client).await
    }

    fn user_id(&self) -> Uuid {
        self.web_token.user_id()
    }

    fn is_expired(&self) -> bool {
        self.web_token.is_expired()
    }

    fn encode(&self) -> Result<String, String> {
        let config = JWTConfig::from_env()?;
        self.web_token.encode(config.refresh_token_private_key()?)
    }

    fn decode(token: &str) -> Result<JsonRefreshToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonRefreshToken {
            web_token: JsonWebToken::decode(token, config.refresh_token_public_key()?)?,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct JsonAccessToken {
    web_token: JsonWebToken,
}

impl JsonAccessToken {
    fn new(user_id: Uuid) -> Result<JsonAccessToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonAccessToken {
            web_token: JsonWebToken::new(user_id, config.access_token_minutes()),
        })
    }

    /// Function to insert the token into the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn insert_into_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        let config = JWTConfig::from_env()?;
        self.web_token
            .insert_into_redis(redis_client, config.access_token_minutes())
            .await
    }

    /// Checks whether the token is still present in the redis database and has the same user_id.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn is_still_present_in_redis(
        &self,
        redis_client: &redis::Client,
    ) -> Result<bool, String> {
        self.web_token.is_still_present_in_redis(redis_client).await
    }

    /// Delete the token from the redis database.
    ///
    /// # Arguments
    /// * `redis_client` - The redis client to use for the login.
    async fn delete_from_redis(&self, redis_client: &redis::Client) -> Result<(), String> {
        self.web_token.delete_from_redis(redis_client).await
    }

    fn user_id(&self) -> Uuid {
        self.web_token.user_id()
    }

    fn is_expired(&self) -> bool {
        self.web_token.is_expired()
    }

    fn encode(&self) -> Result<String, String> {
        let config = JWTConfig::from_env()?;
        self.web_token.encode(config.access_token_private_key()?)
    }

    fn decode(token: &str) -> Result<JsonAccessToken, String> {
        let config = JWTConfig::from_env()?;
        Ok(JsonAccessToken {
            web_token: JsonWebToken::decode(token, config.access_token_public_key()?)?,
        })
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
    let config = JWTConfig::from_env()?;

    let token = JsonRefreshToken::new(user_id)?;

    token.insert_into_redis(redis_client).await?;

    Ok(Cookie::build(REFRESH_COOKIE_NAME, token.encode()?)
        .path("/")
        .max_age(ActixWebDuration::minutes(config.refresh_token_minutes()))
        // The HTTP_ONLY flag is set to true to prevent the cookie from being accessed by
        // JavaScript. This is a security measure to prevent XSS attacks.
        .http_only(true)
        .finish())
}

/// Function to create the user online cookie
fn encode_user_online_cookie<'a>() -> Result<Cookie<'a>, String> {
    let config = JWTConfig::from_env()?;
    Ok(Cookie::build(USER_ONLINE_COOKIE_NAME, "true")
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
    let refresh_cookie = encode_jwt_refresh_cookie(user_id, redis_client).await;

    if refresh_cookie.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let login_cookie = encode_user_online_cookie();

    if login_cookie.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Found()
        .append_header((LOCATION, state.to_string()))
        .cookie(refresh_cookie.unwrap())
        .cookie(login_cookie.unwrap())
        .finish()
}

pub(crate) async fn access_token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
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

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // First, we extract the token from the request.
        // An authentication header is expected to be present in the request, and is
        // of the form:
        // Authorization: Bearer <token here>
        //
        let bearer = BearerAuth::from_request(req, payload).into_inner();

        // If the token is not present, we return an error.
        if bearer.is_err() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token"}),
            )));
        }

        // Next, we decode the token to get the user_id.
        let decode = JsonAccessToken::decode(bearer.unwrap().token());

        // If the token is invalid, we return an error.
        if decode.is_err() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token"}),
            )));
        }

        let access_token = decode.unwrap();

        // If the token is expired, we return an error.
        if access_token.is_expired() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Token has expired"}),
            )));
        }

        // Next up, we check whether the token is still present in the redis database.
        let redis_client = match req.app_data::<web::Data<redis::Client>>() {
            Some(client) => client,
            None => {
                return ready(Err(ErrorInternalServerError(
                    json!({"status": "fail", "message": "Internal server error"}),
                )))
            }
        };

        block_on(async move {
            let is_still_present = access_token.is_still_present_in_redis(&redis_client).await;

            if is_still_present.is_err() || !is_still_present.unwrap() {
                return ready(Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token"}),
                )));
            }

            // Finally, we get the user from the database, as while the user indeed seems
            // to be authenticated and it still exists in the redis database, we still need
            // to check whether the user exists in the database, as it may have been deleted
            // in the meantime.
            let pool = req
                .app_data::<web::Data<Pool<ConnectionManager<PgConnection>>>>()
                .unwrap();

            // If the user doesn't exist, we return an error, otherwise we return the user.
            match User::get(access_token.user_id(), &mut pool.get().unwrap()) {
                Ok(user) => ready(Ok(user)),
                Err(_) => ready(Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
                ))),
            }
        })
    }
}

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
    // First, we extract the refresh token from the request.
    let refresh_cookie = req.cookie(REFRESH_COOKIE_NAME);

    // If the refresh token is not present, we return an error.
    if refresh_cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    // Next, we decode the token to get the user_id.
    let decode = JsonRefreshToken::decode(refresh_cookie.unwrap().value());

    // If the token is invalid, we return an error.
    if decode.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    let refresh_token = decode.unwrap();

    // If the token is expired, we return an error.
    if refresh_token.is_expired() {
        return HttpResponse::Unauthorized().finish();
    }

    // Next up, we check whether the token is still present in the redis database.
    let is_still_present = refresh_token.is_still_present_in_redis(&redis_client).await;

    if is_still_present.is_err() || !is_still_present.unwrap() {
        return HttpResponse::Unauthorized().finish();
    }

    // Finally, we get the user from the database, as while the user indeed seems
    // to be authenticated and it still exists in the redis database, we still need
    // to check whether the user exists in the database, as it may have been deleted
    // in the meantime.
    let mut connection = match pool.get() {
        Ok(pool) => pool,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // If the user doesn't exist, we return an error, otherwise we return the user.
    let user = User::get(refresh_token.user_id(), &mut connection);

    if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    // If the user exists, we create a new access token and return it.
    let access_token = JsonAccessToken::new(refresh_token.user_id());

    if access_token.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let access_token = access_token.unwrap();

    match access_token.insert_into_redis(&redis_client).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    // We return the access token as part of the JSON response, and we
    // expect the frontend to store it in a variable and use it for
    // future requests as part of the Authorization header. This means
    // that upon refreshing the page the user will have to request a
    // new access token, but this is a security measure to prevent
    // unauthorized access to the API.

    HttpResponse::Ok().json(json!({"access_token": access_token.encode().unwrap()}))
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
async fn logout(
    req: HttpRequest,
    bearer: BearerAuth,
    redis_client: web::Data<redis::Client>,
) -> impl Responder {
    let access_token = JsonAccessToken::decode(bearer.token());

    if access_token.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    let access_token = access_token.unwrap();

    let is_still_present = access_token.is_still_present_in_redis(&redis_client).await;

    if is_still_present.is_err() || !is_still_present.unwrap() {
        return HttpResponse::Unauthorized().finish();
    }

    let refresh_cookie = req.cookie(REFRESH_COOKIE_NAME);

    if refresh_cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let refresh_token = JsonRefreshToken::decode(refresh_cookie.unwrap().value());

    if refresh_token.is_err() {
        return HttpResponse::Unauthorized().finish();
    }

    let refresh_token = refresh_token.unwrap();

    let is_still_present = refresh_token.is_still_present_in_redis(&redis_client).await;

    if is_still_present.is_err() || !is_still_present.unwrap() {
        return HttpResponse::Unauthorized().finish();
    }

    match access_token.delete_from_redis(&redis_client).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    match refresh_token.delete_from_redis(&redis_client).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    // We delete the refresh token cookie and the user online cookie from the user's browser.
    let refresh_cookie = Cookie::build(REFRESH_COOKIE_NAME, "")
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(true)
        .finish();

    let user_online_cookie = Cookie::build(USER_ONLINE_COOKIE_NAME, "")
        .path("/")
        .max_age(ActixWebDuration::ZERO)
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(refresh_cookie)
        .cookie(user_online_cookie)
        .json(serde_json::json!({"status": "success"}))
}
