//! Submodule for OAuth2 authentication.

mod github;
pub(crate) mod jwt_cookies;
mod logout;
mod providers;
pub(crate) mod refresh;
use actix_web::web;
pub(crate) use jwt_cookies::access_token_validator;

#[derive(serde::Deserialize)]
/// Struct representing the query parameters for the OAuth2 login request.
///
/// This struct is NEVER instantiated within the application, it is solely
/// used to define the parameters received from the OAuth2 provider.
pub(crate) struct QueryCode {
    /// The authorization code returned by the OAuth2 provider.
    pub code: String,
    /// The original location from where the request started, such as the Login
    /// page.
    pub state: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    // All these endpoints will show up under `/api/oauth/*`
    cfg.service(
        web::scope(web_common::api::oauth::ENDPOINT)
            .service(github::github_oauth_handler)
            .service(providers::get_providers)
            .service(refresh::refresh_access_token_handler)
            .service(logout::logout),
    );
}
