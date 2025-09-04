//! Submodule defining the compatibility rules for safelock tube rules.

use core_structures::traits::CompatibleWith;

use crate::asset_models::{
    bead::bead_3mm,
    containers::safelock_tubes::safelock_tubes_2ml,
    instruments::{ball_mill_machine::ball_mill_machine, centrifuge::safelock_centrifuge},
};

/// Initializes the compatibility rules for safelock tubes.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_safelock_tubes_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let safelock_tubes = safelock_tubes_2ml(user, conn)?;
    let ball_mill_machine = ball_mill_machine(user, conn)?;
    let bead_3mm = bead_3mm(user, conn)?;
    let centrifuge = safelock_centrifuge(user, conn)?;

    ball_mill_machine.compatible_with(&safelock_tubes, user, conn)?;
    centrifuge.compatible_with(&safelock_tubes, user, conn)?;
    bead_3mm.compatible_with(&safelock_tubes, user, conn)?;

    Ok(())
}
