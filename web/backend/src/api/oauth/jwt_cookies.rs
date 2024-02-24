//! Functions and APIs dealing with JWT cookies necessary for OAuth2 logins.
use crate::models::User;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::dev::Payload;
use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use chrono::Duration;
use chrono::Utc;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::future::{ready, Ready};

/// Set a const with the expected cookie name.
pub(crate) const JWT_COOKIE_NAME: &str = "access_token";

#[derive(Deserialize, Serialize)]
pub(crate) struct JsonWebTokenClaims {
    user_id: i32,
    exp: usize,
    iat: usize,
}

impl JsonWebTokenClaims {
    pub fn new(user_id: i32, exp: usize, iat: usize) -> JsonWebTokenClaims {
        JsonWebTokenClaims { user_id, exp, iat }
    }
}

struct JWTConfig {
    // This is currently not used for logins such as GitHub, but it is commonly
    // needed by othe OAuth2 providers.
    jwt_secret: String,
    jwt_max_days: i64,
}

impl JWTConfig {
    pub fn from_env() -> Result<JWTConfig, String> {
        dotenvy::dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET");
        let jwt_max_days = env::var("JWT_EXPIRATION_DAYS");

        if jwt_secret.is_err() {
            return Err("JWT_SECRET not set".to_string());
        }

        if jwt_max_days.is_err() {
            return Err("JWT_EXPIRATION_DAYS not set".to_string());
        }

        Ok(JWTConfig {
            jwt_secret: jwt_secret.unwrap(),
            jwt_max_days: jwt_max_days
                .unwrap()
                .parse::<i64>()
                .map_err(|_| "JWT_EXPIRATION_DAYS must be a valid integer".to_string())?,
        })
    }
}

/// Function to create a JWT cookie for a given user.
///
/// # Arguments
/// * `user_id` - The ID of the user to create the JWT for.
/// * `config` - The OAuth2 configuration to use to create the JWT.
pub(crate) fn encode_jwt_cookie(user_id: &i32) -> Result<Cookie, Error> {
    let config = JWTConfig::from_env().expect("Failed to load JWT config");
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::days(config.jwt_max_days)).timestamp() as usize;
    let claims = JsonWebTokenClaims::new(*user_id, iat, exp);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|e| ErrorUnauthorized(e.to_string()))?;

    Ok(Cookie::build(JWT_COOKIE_NAME, token)
        .path("/")
        .max_age(ActixWebDuration::days(config.jwt_max_days))
        .http_only(true)
        .finish())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub(crate) enum Permissions {
    // If is the admin of at least one project
    ProjectAdmin,
    // If is the admin of at least one organization
    OrganizationAdmin,
    // If is the admin of the website
    WebsiteAdmin,
    // If is a logged user
    LoggedUser,
}

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req.cookie(JWT_COOKIE_NAME).map(|c| c.value().to_string());

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "You are not logged in, please provide token"}),
            )));
        }

        let pool = req
            .app_data::<web::Data<Pool<ConnectionManager<PgConnection>>>>()
            .unwrap();

        let jwt_secret = JWTConfig::from_env().unwrap().jwt_secret;
        let decode = decode::<JsonWebTokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => match User::get(token.claims.user_id, &mut pool.get().unwrap()) {
                Ok(user) => ready(Ok(user)),
                Err(_) => ready(Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
                ))),
            },
            Err(_) => ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
            ))),
        }
    }
}
