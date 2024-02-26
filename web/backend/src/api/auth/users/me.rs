//! Returns a user object for the currently logged in user.
//!

use actix_web::{get, HttpResponse, Responder};

use crate::models::User;
use web_common::user::User as CommonUser;

#[get("/me")]
pub async fn logged_user_info(user: User) -> impl Responder {
    HttpResponse::Ok().json(CommonUser::new(
        user.first_name,
        user.middle_name,
        user.last_name,
        user.id,
    ))
}
