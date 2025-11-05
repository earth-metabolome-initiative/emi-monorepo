//! Submodule defining the compatibility rules for conical centrifugal tubes.

use core_structures::traits::CanContain;

use crate::asset_models::containers::{
    boxes::polystyrene_box, conical_centrifugal_tubes::conical_centrifugal_tube_50ml,
    racks::conical_centrifugal_tube_50ml_rack, wrappers::coffee_filter_wrapper,
};
use crate::asset_models::organisms::organism_sample_model;
/// Initializes the compatibility rules for conical centrifugal tubes.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_conical_centrifugal_tube_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let cct_50ml = conical_centrifugal_tube_50ml(user, conn)?;
    let cct_50ml_rack = conical_centrifugal_tube_50ml_rack(user, conn)?;

    // We can fit 24 conical centrifugal tubes of 50ml in a rack.
    cct_50ml_rack.can_contain(&cct_50ml, 24, user, conn)?;

    let polystyrene_box = polystyrene_box(user, conn)?;
    polystyrene_box.can_contain(&cct_50ml, 50, user, conn)?;

    let coffee_filter = coffee_filter_wrapper(user, conn)?;
    cct_50ml.can_contain(&coffee_filter, 1, user, conn)?;

    let sample = organism_sample_model(user, conn)?;
    cct_50ml.can_contain(&sample, 1, user, conn)?;

    Ok(())
}
