//! APIs handling the user name.

use crate::models::User as DBUser;
use actix_web::put;
use actix_web::{web, HttpResponse, Responder};
use validator::Validate;
use web_common::api::auth::users::name::Name;

/// Updates the currently logged in user's name.
///
/// # Arguments
/// * `user` - The currently logged in user.
/// * `new_name` - The new name to set for the user.
/// * `pool` - The database connection pool, from the Actix web app data.
#[put("/name")]
pub async fn update_name(
    user: DBUser,
    new_name: web::Json<Name>,
    pool: web::Data<crate::DBPool>,
) -> impl Responder {
    log::info!("Updating user name.");

    // We make sure that the provided name is valid by using the validator
    // functions defined in the `Name` struct from the `web_common` crate.
    if let Err(e) = new_name.validate() {
        log::warn!("Invalid name: {:?}", e);
        // If the name is invalid, we return a 400 Bad Request response,
        // which will be handled by the frontend. We also include the
        // validation errors in the response.
        return HttpResponse::BadRequest().json(web_common::api::ApiError::from(e));
    }

    // We get the database connection from the pool.
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get a database connection: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // We now update the user's name in the database.
    match user.update_name(new_name.into_inner(), &mut conn).await {
        Ok(_) => {
            log::info!("User name updated successfully.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to update user name: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
