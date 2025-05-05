use actix_web::web;

mod auth;
mod healthchecker;
pub(crate) mod oauth;
mod ws;
use healthchecker::health_checker_handler;
pub use ws::{LNCommand, ListenNotifyHandle, ListenNotifyServer};

pub fn configure(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope(api_path::api::ENDPOINT)
            .service(health_checker_handler)
            .configure(auth::configure)
            .configure(ws::configure)
            .configure(oauth::configure),
    );
}
