mod picture;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/oauth/*`
    cfg.service(
        web::scope(web_common::api::documents::profile::ENDPOINT)
            .service(picture::user_picture_handler)
    );
}
