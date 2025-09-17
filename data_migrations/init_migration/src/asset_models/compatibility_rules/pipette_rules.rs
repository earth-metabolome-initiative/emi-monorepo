//! Submodule defining the compatibility rules for pipettes.

use core_structures::traits::CompatibleWith;

use crate::asset_models::instruments::{
    pipette_tips::{pipette_tip_200ul, pipette_tip_1000ul},
    pipettes::{pipette_200ul, pipette_1000ul},
};

/// Initializes the compatibility rules for pipettes.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_pipette_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let pipette_1000ul = pipette_1000ul(user, conn)?;
    let pipette_tip_1000ul = pipette_tip_1000ul(user, conn)?;

    // A 1000ul pipette is compatible with a 1000ul pipette tip.
    pipette_1000ul.compatible_with(&pipette_tip_1000ul, user, conn)?;

    let pipette_200ul = pipette_200ul(user, conn)?;
    let pipette_tip_200ul = pipette_tip_200ul(user, conn)?;

    // A 200ul pipette is compatible with a 200ul pipette tip.
    pipette_200ul.compatible_with(&pipette_tip_200ul, user, conn)?;

    Ok(())
}
