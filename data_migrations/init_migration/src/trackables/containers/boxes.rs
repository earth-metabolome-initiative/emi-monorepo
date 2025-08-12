//! Submodule defining standard boxes.

use core_structures::{CommercialProduct, ContainerModel, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::fisherbrand;

/// Returns the polystyrene box.
///
/// # Implementation Details
///
/// This function either instantiate a new polystyrene box from the
/// database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn polystyrene_box(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<core_structures::ContainerModel> {
    let name = "Polystyrene Box";

    if let Some(existing_box) = core_structures::ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_box);
    }

    Ok(ContainerModel::new()
        .name(name.to_owned())?
        .description("Polystyrene box, a container typically used for liquid nitrogen".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns the vial box.
///
/// # Implementation Details
///
/// This function either instantiate a new vial box from the
/// database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn vial_box(
    user: &core_structures::User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<core_structures::ContainerModel> {
    let name = "Vial Box";

    if let Some(existing_box) = core_structures::ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_box);
    }

    Ok(ContainerModel::new()
        .name(name.to_owned())?
        .description("Vial box, a container typically used for storing vials".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Fisherbrand Kryobox Vial Box product.
///
/// # Arguments
///
/// * `user` - The user for whom the product is being created.
/// * `conn` - The database connection.
pub(crate) fn init_fisherbrand_kryobox_vial_box(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let name = "Fisherbrand Kryobox Vial Box";
    if let Some(existing) = CommercialProduct::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let vial_box_trackable = vial_box(user, conn)?;
    let fisherbrand = fisherbrand(user, conn)?;
    Ok(CommercialProduct::new()
        .name(name.to_owned())?
        .description("Fisherbrand Kryobox Vial Box, used to store vials.".to_owned())?
        .created_by(user.id)?
        .brand(fisherbrand.id)?
        .parent(Some(vial_box_trackable.id))?
        .insert(user.id, conn)?)
}
