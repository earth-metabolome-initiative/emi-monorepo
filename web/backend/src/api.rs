//! Root submodule for the API
use actix_web::web;

mod auth;
mod healthchecker;
pub(crate) mod oauth;
mod ws;
use healthchecker::health_checker_handler;
pub(crate) use ws::LNCommand;
pub use ws::{ListenNotifyHandle, ListenNotifyServer};

/// API Configuration
pub fn configure(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope(api_path::api::ENDPOINT)
            .service(health_checker_handler)
            .configure(auth::configure)
            .configure(ws::configure)
            .configure(oauth::configure),
    );
}
