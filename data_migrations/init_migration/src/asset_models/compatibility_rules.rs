//! Submodule for defining the compatibility rules between different
//! `asset_models`.

use core_structures::User;

use crate::asset_models::compatibility_rules::{
    ball_mill::init_ball_mill_rules,
    conical_centrifugal_tube_rules::init_conical_centrifugal_tube_rules,
    freeze_dryer::init_freeze_dryer_rules, freezer::init_freezer_rules,
    pipette_rules::init_pipette_rules, safelock_tubes_rules::init_safelock_tubes_rules,
    samples::init_sample_rules, vial_rules::init_vial_rules,
};

mod ball_mill;
mod conical_centrifugal_tube_rules;
mod freeze_dryer;
mod freezer;
mod pipette_rules;
mod safelock_tubes_rules;
mod samples;
mod vial_rules;

/// Initializes the compatibility rules for `asset_models`.
///
/// # Arguments
///
/// * `user` - The user for whom the compatibility rules are being initialized.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn init_compatibility_rules(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    init_conical_centrifugal_tube_rules(user, conn)?;
    init_vial_rules(user, conn)?;
    init_pipette_rules(user, conn)?;
    init_safelock_tubes_rules(user, conn)?;
    init_freezer_rules(user, conn)?;
    init_freeze_dryer_rules(user, conn)?;
    init_ball_mill_rules(user, conn)?;
    init_sample_rules(user, conn)?;
    Ok(())
}
