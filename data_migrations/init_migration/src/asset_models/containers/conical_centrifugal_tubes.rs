//! Submodule to initialize the sample containers in the database.

use core_structures::{
    CommercialProduct, User, VolumetricContainerModel,
    tables::insertables::{
        AssetModelBuildable, CommercialProductBuildable, VolumetricContainerModelBuildable,
    },
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::greiner_bio_one;

/// Returns the conical centrifugal tube of 50ml.
///
/// # Implementation Details
///
/// This function either instantiate a new conical centrifugal tube of 50ml from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the conical tube is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn conical_centrifugal_tube_50ml(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<VolumetricContainerModel> {
    let name = "Conical Centrifugal Tube 50ml";

    if let Some(existing_tube) = VolumetricContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(VolumetricContainerModel::new()
        .name(name.to_owned())?
        .description("Conical tube of 50ml, used for sample collection.".to_owned())?
        .created_by(user.id)?
        .liters(0.05)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Greiner Conical Centrifugal Tube 50ml
/// product.
pub(crate) fn init_greiner_cct(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let conical_tube = "Greiner Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, conn).optional()? {
        return Ok(conical_tube);
    }

    let conical_tube_50ml = conical_centrifugal_tube_50ml(user, conn)?;
    let greiner = greiner_bio_one(user, conn)?;
    Ok(CommercialProduct::new()
        .name(conical_tube.to_owned())?
        .description("Conical sample tube of 50ml, used for sample collection.".to_owned())?
        .created_by(user.id)?
        .brand(greiner.id)?
        .parent_model(Some(conical_tube_50ml.id))?
        .insert(user.id, conn)?)
}
