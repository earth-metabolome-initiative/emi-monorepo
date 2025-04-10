use actix_web::web;

mod auth;
mod healthchecker;
mod oauth;
mod ws;

use healthchecker::health_checker_handler;

pub fn configure(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope(api_path::api::ENDPOINT)
            .service(health_checker_handler)
            .configure(auth::configure)
            .configure(ws::configure)
            .configure(oauth::configure),
    );
}
