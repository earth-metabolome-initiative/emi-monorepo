//! Submodule defining the compatibility rules for freezers.

use core_structures::traits::CompatibleWith;

use crate::asset_models::{
    containers::conical_centrifugal_tubes::conical_centrifugal_tube_50ml,
    instruments::freezer::freezer,
};

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
    let cct = conical_centrifugal_tube_50ml(user, conn)?;

    // A freezer can accomodate a conical centrifugal tube.
    freezer.compatible_with(&cct, user, conn)?;

    Ok(())
}
