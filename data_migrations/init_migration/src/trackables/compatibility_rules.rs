//! Submodule for defining the compatibility rules between different trackables.

use core_structures::User;

use crate::trackables::compatibility_rules::conical_centrifugal_tube_rules::init_conical_centrifugal_tube_rules;

mod conical_centrifugal_tube_rules;
mod pipette_rules;
mod safelock_tubes_rules;

/// Initializes the compatibility rules for trackables.
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
    Ok(())
}
