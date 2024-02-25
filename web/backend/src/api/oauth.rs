//! Submodule for OAuth2 authentication.

mod github;
pub(crate) mod jwt_cookies;
mod providers;
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
/// * `state` - The state parameter returned by the OAuth2 provider.
pub(crate) struct QueryCode {
    pub code: String,
    pub state: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/oauth/*`
    cfg.service(github::github_oauth_handler);
    cfg.service(providers::get_providers);
    cfg.service(jwt_cookies::refresh_access_token);
    cfg.service(jwt_cookies::logout);
}
