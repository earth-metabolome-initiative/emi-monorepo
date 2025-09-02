//! Submodule defining the compatibility rules for vials.

use core_structures::traits::CompatibleWith;

use crate::asset_models::containers::{
    boxes::vial_rack_1_5ml,
    vial_caps::{sealed_cap_vial_1_5ml, splitted_cap_vial_1_5ml},
    vial_inserts::vial_insert_200ul,
    vials::vial_1_5ml,
};

/// Initializes the compatibility rules for vials.
///
/// # Arguments
///
/// * `user` - The user who is creating the compatibility rules.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection fails to insert the compatibility rules.
pub(super) fn init_vial_rules(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<()> {
    let vial_1_5ml = vial_1_5ml(user, conn)?;
    let vial_1_5ml_sealed_cap = sealed_cap_vial_1_5ml(user, conn)?;
    let vial_1_5ml_splitted_cap = splitted_cap_vial_1_5ml(user, conn)?;
    let vial_insert = vial_insert_200ul(user, conn)?;
    let vial_rack = vial_rack_1_5ml(user, conn)?;

    // A vial is compatible with one sealed cap.
    vial_1_5ml.compatible_with_quantity(&vial_1_5ml_sealed_cap, 1, user, conn)?;
    // A vial is compatible with one splitted cap.
    vial_1_5ml.compatible_with_quantity(&vial_1_5ml_splitted_cap, 1, user, conn)?;
    // A vial is compatible with one insert.
    vial_1_5ml.compatible_with_quantity(&vial_insert, 1, user, conn)?;
    // A Vial rack can hold 81 vials.
    vial_rack.compatible_with_quantity(&vial_1_5ml, 81, user, conn)?;

    Ok(())
}
