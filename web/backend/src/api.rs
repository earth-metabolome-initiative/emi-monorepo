use actix_web::web;

mod auth;
mod healthchecker;
mod oauth;
use crate::api::oauth::access_token_validator;
use actix_web_httpauth::middleware::HttpAuthentication;
use healthchecker::health_checker_handler;

pub fn configure(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/api")
            .service(health_checker_handler)
            .service(
                web::scope("/auth")
                    // We wrap the auth routes with the access token validator middleware
                    // which makes sure that the user has provided a valid access token.
                    .wrap(HttpAuthentication::bearer(access_token_validator))
                    .configure(auth::configure),
            )
            .service(web::scope("/oauth").configure(oauth::configure)),
    );
}
