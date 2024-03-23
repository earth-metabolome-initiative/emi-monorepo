mod profile;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/oauth/*`
    cfg.service(web::scope(web_common::api::documents::ENDPOINT).configure(profile::configure));
}
