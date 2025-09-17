//! Submodule defining the compatibility rules for samples.

use core_structures::traits::CompatibleWith;

use crate::asset_models::{
    containers::wrappers::coffee_filter_wrapper, organisms::organism_sample_model,
};

/// Initializes the compatibility rules for samples.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_sample_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let sample = organism_sample_model(user, conn)?;
    let coffee_wrapper = coffee_filter_wrapper(user, conn)?;
    // A vial is compatible with one insert.
    coffee_wrapper.compatible_with(&sample, user, conn)?;

    Ok(())
}
