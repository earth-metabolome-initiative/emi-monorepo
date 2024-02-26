use actix_web::web;

mod auth;
mod healthchecker;
mod oauth;

use healthchecker::health_checker_handler;

pub fn configure(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope(web_common::api::ENDPOINT)
            .service(health_checker_handler)
            .configure(auth::configure)
            .configure(oauth::configure),
    );
}
