mod name;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/oauth/*`
    cfg.service(
        web::scope(web_common::api::auth::users::ENDPOINT)
            .service(name::update_name),
        );
}
