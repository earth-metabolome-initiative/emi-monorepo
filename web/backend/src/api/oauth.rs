//! Submodule for OAuth2 authentication.

use actix_web::{post, web, HttpResponse, Responder};
pub mod github;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

pub(crate) struct OauthConfig {
    client_origin: String,
    jwt_secret: String,
    jwt_expires_in: i64,
    jwt_max_age: i64,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct JsonWebTokenClaims {
    id: i32,
    exp: usize,
    iat: usize,
}

impl JsonWebTokenClaims {
    pub fn new(id: i32, exp: usize, iat: usize) -> JsonWebTokenClaims {
        JsonWebTokenClaims { id, exp, iat }
    }
}

impl OauthConfig {
    pub fn from_env() -> Result<OauthConfig, String> {
        dotenvy::dotenv().ok();
        let client_origin = env::var("CLIENT_ORIGIN");
        let jwt_secret = env::var("JWT_SECRET");
        let jwt_expires_in = env::var("JWT_EXPIRES_IN");
        let jwt_max_age = env::var("JWT_MAX_AGE");

        if client_origin.is_err() {
            return Err("CLIENT_ORIGIN not set".to_string());
        }

        if jwt_secret.is_err() {
            return Err("JWT_SECRET not set".to_string());
        }

        if jwt_expires_in.is_err() {
            return Err("JWT_EXPIRES_IN not set".to_string());
        }

        if jwt_max_age.is_err() {
            return Err("JWT_MAX_AGE not set".to_string());
        }

        Ok(OauthConfig {
            client_origin: client_origin.unwrap(),
            jwt_secret: jwt_secret.unwrap(),
            jwt_expires_in: jwt_expires_in
                .unwrap()
                .parse::<i64>()
                .map_err(|_| "JWT_EXPIRES_IN must be a valid integer".to_string())?,
            jwt_max_age: jwt_max_age
                .unwrap()
                .parse::<i64>()
                .map_err(|_| "JWT_MAX_AGE must be a valid integer".to_string())?,
        })
    }
}

/// Enum describing the available OAuth providers.
///
/// Ensures that all OAuth providers are accounted for.
pub enum OauthProviders {
    GitHub,
}

impl TryFrom<String> for OauthProviders {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "github" => Ok(OauthProviders::GitHub),
            _ => Err("Invalid OAuth provider".to_string()),
        }
    }
}

#[derive(Deserialize)]
/// Struct representing the query parameters for the OAuth2 login request.
///
/// This struct is NEVER instantiated within the application, it is solely
/// used to define the parameters received from the OAuth2 provider.
///
/// # Fields
/// * `code` - The authorization code returned by the OAuth2 provider.
/// * `state` - The state parameter returned by the OAuth2 provider.
pub(crate) struct QueryCode {
    pub code: String,
    pub state: String,
}

/// Function to create a JWT cookie for a given user.
///
/// # Arguments
/// * `user_id` - The ID of the user to create the JWT for.
/// * `config` - The OAuth2 configuration to use to create the JWT.
pub(crate) fn create_jwt_cookie(user_id: i32, config: &OauthConfig) -> Cookie {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(config.jwt_expires_in)).timestamp() as usize;
    let claims: JsonWebTokenClaims = JsonWebTokenClaims::new(user_id, iat, exp);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * config.jwt_max_age, 0))
        .http_only(true)
        .finish();

    cookie
}
