//! Set of APIs solely accessible once the user is logged in.
use actix_web::web;
mod users;
use crate::api::oauth::access_token_validator;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/auth/*`
    cfg.service(
        web::scope(web_common::api::auth::ENDPOINT)
            // We wrap the auth routes with the access token validator middleware
            // which makes sure that the user has provided a valid access token.
            .wrap(HttpAuthentication::bearer(access_token_validator))
            .configure(users::configure)
    );
}
