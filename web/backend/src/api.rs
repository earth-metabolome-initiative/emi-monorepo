use actix_web::web;

mod auth;
mod healthchecker;
mod oauth;

use healthchecker::health_checker_handler;

pub fn configure(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/api")
            .service(health_checker_handler)
            .service(web::scope("/auth").configure(auth::configure))
            .service(web::scope("/oauth").configure(oauth::configure)),
    );
}
