//! Submodule defining the compatibility rules for freeze dryer.

use core_structures::traits::CompatibleWith;

use crate::asset_models::{
    containers::conical_centrifugal_tubes::conical_centrifugal_tube_50ml,
    instruments::freeze_dryer::freeze_dryer,
};

/// Initializes the compatibility rules for freeze dryer.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_freeze_dryer_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let freeze_dryer_instument = freeze_dryer(user, conn)?;
    let cct = conical_centrifugal_tube_50ml(user, conn)?;

    freeze_dryer_instument.compatible_with(&cct, user, conn)?;

    Ok(())
}
