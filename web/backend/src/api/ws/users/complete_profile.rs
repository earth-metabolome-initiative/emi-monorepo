//! Submodule implementing function to update a user's name.
use crate::diesel::connection::SimpleConnection;
use crate::models::User;
use crate::DieselConn;
use diesel::prelude::*;
use validator::Validate;
use web_common::api::auth::users::CompleteProfile;
use web_common::api::ApiError;

/// Method to update a user's name.
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `editor` - The user performing the update.
/// * `target_user_id` - The id of the user to update.
/// * `profile` - The data to use to complete the profile.
///
/// # Implementation details
/// The procedure checks whether the provided name is valid using the Validate trait.
/// It then checks whether the editor is authorized to perform the update using the is_authorized
/// function. If the editor is authorized, the user's name is updated in the database.
pub fn complete_profile(
    conn: &mut DieselConn,
    user: &User,
    profile: CompleteProfile,
) -> Result<(), ApiError> {
    profile.validate()?;

    let (name, profile_picture) = profile.scompose();
    let squared_profile_picture = profile_picture.to_square().map_err(|errors| {
        log::error!("Failed to square profile picture: {}", errors.join(", "));
        ApiError::internal_server_error()
    })?;

    conn.batch_execute("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;")
        .expect("Failed to set transaction isolation level");

    // We need to execute multiple queries in a single transaction so to
    // avoid that the user is left with a profile picture but no name or vice versa.
    conn.transaction::<_, ApiError, _>(|conn| {
        user.update_name(name, conn)?;
        user.insert_profile_pictures(squared_profile_picture, conn)?;
        Ok(())
    })
}
