//! Returns a user object for the currently logged in user.
//!

use actix_web::{get, HttpResponse, Responder};

use crate::models::User as DBUser;
use web_common::api::auth::users::name::Name;
use web_common::api::auth::users::User as CommonUser;

#[get("/me")]
pub async fn logged_user_info(user: DBUser) -> impl Responder {
    log::info!("Retrieving logged user info.");

    let name = Name::new(
        user.first_name.unwrap_or_default(),
        user.middle_name,
        user.last_name.unwrap_or_default(),
    );

    HttpResponse::Ok().json(CommonUser::new(name, user.id))
}
