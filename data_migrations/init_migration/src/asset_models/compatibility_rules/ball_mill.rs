//! Submodule defining the compatibility rules relative to the ball mill
//! machine.

use core_structures::traits::CompatibleWith;

use crate::asset_models::{bead::bead_3mm, instruments::ball_mill_machine::ball_mill_machine};

/// Initializes the compatibility rules for the ball mill machine.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_ball_mill_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let ball_mill_machine = ball_mill_machine(user, conn)?;
    let bead_3mm = bead_3mm(user, conn)?;

    ball_mill_machine.compatible_with(&bead_3mm, user, conn)?;

    Ok(())
}
