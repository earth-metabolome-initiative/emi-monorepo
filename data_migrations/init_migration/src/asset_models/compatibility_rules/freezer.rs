//! Submodule defining the compatibility rules for freezers.

use core_structures::traits::CompatibleWith;

use crate::asset_models::{containers::boxes::polystyrene_box, instruments::freezer::freezer};

/// Initializes the compatibility rules for freezers.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_freezer_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let freezer = freezer(user, conn)?;
    let polystyrene_box = polystyrene_box(user, conn)?;

    // A freezer can accomodate a polystyrene box.
    freezer.compatible_with(&polystyrene_box, user, conn)?;

    Ok(())
}
