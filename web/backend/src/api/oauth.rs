//! Submodule for OAuth2 authentication.

pub mod github;
pub mod logout;
use actix_web::cookie::time::Duration as ActixWebDuration;
use actix_web::cookie::Cookie;
use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;
use chrono::{prelude::*, Duration};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use web_common::login_provider::OAuth2LoginProvider;

use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::json;

use std::env;

use crate::models::{LoginProvider, User};

pub(crate) struct OauthConfig {
    client_origin: String,
    jwt_secret: String,
    jwt_max_age: i64,
}

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

impl OauthConfig {
    pub fn from_env() -> Result<OauthConfig, String> {
        dotenvy::dotenv().ok();
        let client_origin = env::var("CLIENT_ORIGIN");
        let jwt_secret = env::var("JWT_SECRET");
        let jwt_max_age = env::var("JWT_MAX_AGE");

        if client_origin.is_err() {
            return Err("CLIENT_ORIGIN not set".to_string());
        }

        if jwt_secret.is_err() {
            return Err("JWT_SECRET not set".to_string());
        }

        if jwt_max_age.is_err() {
            return Err("JWT_MAX_AGE not set".to_string());
        }

        Ok(OauthConfig {
            client_origin: client_origin.unwrap(),
            jwt_secret: jwt_secret.unwrap(),
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
pub(crate) fn create_jwt_cookie(user_id: i32, config: &OauthConfig) -> Result<Cookie, String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(config.jwt_max_age)).timestamp() as usize;
    let claims: JsonWebTokenClaims = JsonWebTokenClaims::new(user_id, iat, exp);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );

    if token.is_err() {
        return Err("Failed to create JWT".to_string());
    }

    let cookie = Cookie::build("token", token.unwrap())
        .path("/")
        .max_age(ActixWebDuration::new(60 * config.jwt_max_age, 0))
        .http_only(true)
        .finish();

    Ok(cookie)
}

pub struct AuthenticationGuard {
    pub user_id: i32,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "You are not logged in, please provide token"}),
            )));
        }

        let pool = req
            .app_data::<web::Data<Pool<ConnectionManager<PgConnection>>>>()
            .unwrap();

        let jwt_secret = OauthConfig::from_env().unwrap().jwt_secret;
        let decode = decode::<JsonWebTokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => {
                if User::exists(token.claims.user_id, pool) {
                    return ready(Err(ErrorUnauthorized(
                        json!({"status": "fail", "message": "User belonging to this token no logger exists"}),
                    )));
                }

                ready(Ok(AuthenticationGuard {
                    user_id: token.claims.user_id,
                }))
            }
            Err(_) => ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token or usre doesn't exists"}),
            ))),
        }
    }
}

#[get("/oauth/providers")]
/// Returns a list of available OAuth2 providers.
async fn get_providers(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    // We retrieve the system variables using dotenvy.
    dotenvy::dotenv().ok();

    let providers = LoginProvider::get_all(&pool);

    if providers.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let providers = providers.unwrap();
    let mut oauth_providers: Vec<OAuth2LoginProvider> = Vec::new();

    for provider in providers {
        let client_id = env::var(provider.client_id_var_name);
        let redirect_uri = env::var(provider.redirect_uri_var_name);

        if client_id.is_err() || redirect_uri.is_err() {
            return HttpResponse::InternalServerError().finish();
        }

        oauth_providers.push(OAuth2LoginProvider {
            id: provider.id,
            name: provider.name,
            font_awesome_icon: provider.font_awesome_icon,
            client_id: client_id.unwrap(),
            redirect_uri: redirect_uri.unwrap(),
            oauth_url: provider.oauth_url,
            scope: provider.scope,
        });
    }

    HttpResponse::Ok().json(oauth_providers)
}
