//! Submodule implementing function to update a user's name.
use crate::models::User;
use crate::transactions::is_authorized;
use crate::DieselConn;
use diesel::prelude::*;
use validator::Validate;
use web_common::api::auth::users::name::Name;
use web_common::api::ApiError;

/// Method to update a user's name.
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `editor` - The user performing the update.
/// * `target_user_id` - The id of the user to update.
/// * `name` - The new name for the user.
///
/// # Implementation details
/// The procedure checks whether the provided name is valid using the Validate trait.
/// It then checks whether the editor is authorized to perform the update using the is_authorized
/// function. If the editor is authorized, the user's name is updated in the database.
pub fn update_user_name(
    conn: &mut DieselConn,
    editor: User,
    target_user_id: uuid::Uuid,
    name: Name,
) -> Result<(), ApiError> {
    name.validate().map_err(|e| ApiError::from(e))?;

    if !is_authorized(
        conn,
        editor.id,
        web_common::api::ws::messages::Table::Users,
        target_user_id,
        vec![web_common::api::ws::messages::Role::Admin],
    )
    .map_err(|diesel_error| {
        log::error!("Failed to check authorization: {}", diesel_error);
        ApiError::internal_server_error()
    })? {
        return Err(ApiError::unauthorized());
    }

    {
        use crate::schema::users;
        let middle_name: Option<String> = name.middle_name().map(|s| s.to_string());
        diesel::update(users::dsl::users.filter(users::dsl::id.eq(target_user_id)))
            .set((
                users::dsl::first_name.eq(name.first_name().as_ref()),
                users::dsl::middle_name.eq(middle_name),
                users::dsl::last_name.eq(name.last_name().as_ref()),
            ))
            .execute(conn)
            .map_err(|e| {
                log::error!("Failed to update user name: {}", e);
                ApiError::internal_server_error()
            })?;
    }

    Ok(())
}
