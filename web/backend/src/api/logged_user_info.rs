//! Returns a user object for the currently logged in user.
//!

use crate::api::oauth::AuthenticationGuard;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use log::{debug, error};

use crate::models::User as DBUser;
use web_common::user::User as CommonUser;

#[get("/logged_user_info")]
pub async fn logged_user_info(
    auth_guard: AuthenticationGuard,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    debug!("Getting logged user info for user {}", auth_guard.user_id());
    let user = DBUser::get(auth_guard.user_id(), &pool);

    if user.is_err() {
        error!("Failed to get user info: {:?}", user.err());
        return HttpResponse::InternalServerError().finish();
    }

    let user = user.unwrap();
    HttpResponse::Ok().json(CommonUser::new(
        user.first_name,
        user.middle_name,
        user.last_name,
        user.id,
    ))
}
