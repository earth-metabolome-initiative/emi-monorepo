//! Set of APIs solely accessible once the user is logged in.
mod logged_user_info;
mod logout;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/auth/*`
    cfg.service(logged_user_info::logged_user_info);
}