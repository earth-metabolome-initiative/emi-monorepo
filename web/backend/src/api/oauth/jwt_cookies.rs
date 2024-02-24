//! Functions and APIs dealing with JWT cookies necessary for OAuth2 logins.
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::dev::ServiceRequest;
use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::web;
use actix_web_grants::authorities::AttachAuthorities;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use chrono::Duration;
use chrono::Utc;
use crate::models::User;
use diesel::r2d2::Pool;
use jsonwebtoken::{
    decode, encode, Algorithm::HS256, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

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

    Ok(Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::days(config.jwt_max_days))
        .http_only(true)
        .finish())
}

/// Function to decode a JWT cookie and return the user ID.
///
/// # Arguments
/// * `token` - The JWT token to decode.
/// * `pool` - The database connection pool to use to decode the token.
///
pub(crate) fn decode_jwt_cookie(
    token: &str,
    pool: &web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<User, Error> {
    let config = JWTConfig::from_env().expect("Failed to load JWT config");
    decode::<JsonWebTokenClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(HS256),
    )
    .map(|token| {
        let user_id = token.claims.user_id;
        let mut conn = pool.get().unwrap();
        let user = User::get(user_id, &mut conn).unwrap();
        user
    })
    .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub(crate) enum Permissions {
    ProjectAdmin,
    OrganizationAdmin,
    User,
}

/// Function to guard a route with JWT authentication.
pub(crate) async fn authentication_guard(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We get the Connection Pool from the application data
    // stored in the provided request.
    let pool = req
        .app_data::<web::Data<Pool<ConnectionManager<PgConnection>>>>()
        .unwrap();

    // We just get permissions from JWT
    let result = decode_jwt_cookie(credentials.token(), pool);

    match result {
        Ok(user) => {
            let mut permissions = vec![Permissions::User];

            let mut conn = pool.get().unwrap();

            if user.is_admin(&mut conn) {
                permissions.push(Permissions::ProjectAdmin);
            }

            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}
