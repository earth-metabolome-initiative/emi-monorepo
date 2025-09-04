//! Submodule to initialize the sample containers in the database.

use core_structures::{
    CommercialProduct, ContainerModel, User,
    tables::insertables::{AssetModelSettable, CommercialProductSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    asset_models::containers::conical_centrifugal_tubes::conical_centrifugal_tube_50ml, fisherbrand,
};

/// Returns the standard rack.
///
/// # Implementation Details
///
/// This function either instantiate a new standard rack from the
/// database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the standard rack is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
fn standard_rack(user: &User, conn: &mut PgConnection) -> anyhow::Result<ContainerModel> {
    let name = "Rack";

    if let Some(existing_tube) = ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(ContainerModel::new()
        .name(name)?
        .description("Rack, a common container for organizing samples")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns the rack for storing conical centrifugal tubes of 50ml.
///
/// # Implementation Details
///
/// This function either instantiate a new rack for conical centrifugal tubes of
/// 50ml from the database or inserts it if it does not exist before returning
/// it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn conical_centrifugal_tube_50ml_rack(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<ContainerModel> {
    let name = "Conical Centrifugal Tube 50ml Rack";

    if let Some(existing_rack) = ContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_rack);
    }

    let standard_rack = standard_rack(user, conn)?;

    Ok(ContainerModel::new()
        .name(name)?
        .description("Rack for storing conical centrifugal tubes of 50ml")?
        .parent_model(Some(standard_rack.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Fisherbrand Delrin Conical Centrifugal Tube
/// Rack 50ml product.
pub(crate) fn init_fisherbrand_cct_rack(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let conical_tube = "Fisherbrand Rack for Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, conn).optional()? {
        return Ok(conical_tube);
    }

    let conical_tube_rack_50ml = conical_centrifugal_tube_50ml(user, conn)?;
    let fisherbrand = fisherbrand(user, conn)?;
    Ok(CommercialProduct::new()
        .name(conical_tube)?
        .description("Rack for Conical sample tube of 50ml, used to store dried samples library.")?
        .created_by(user.id)?
        .brand(fisherbrand.id)?
        .parent_model(Some(conical_tube_rack_50ml.id))?
        .insert(user.id, conn)?)
}
