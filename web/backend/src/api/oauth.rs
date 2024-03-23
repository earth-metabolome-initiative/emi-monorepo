//! Submodule for OAuth2 authentication.

mod github;
pub(crate) mod jwt_cookies;
mod logout;
mod providers;
mod refresh;
use actix_web::web;
pub(crate) use jwt_cookies::access_token_validator;

use serde::Deserialize;

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
/// * `state` - The original location from where the request started, such as the Login page.
pub(crate) struct QueryCode {
    pub code: String,
    pub state: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/oauth/*`
    cfg.service(
        web::scope(web_common::api::oauth::ENDPOINT)
            .service(github::github_oauth_handler)
            .service(providers::get_providers)
            .service(refresh::refresh_access_token)
            .service(logout::logout),
    );
}
