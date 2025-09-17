//! Submodule to initialize the

use core_structures::{
    CommercialProduct, User, VolumetricContainerModel,
    tables::insertables::{
        AssetModelSettable, CommercialProductSettable, VolumetricContainerModelSettable,
    },
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::brands::vici_schweiz;

/// Returns the vial insert of 200μl.
///
/// # Implementation Details
///
/// This function either instantiate a new vial insert of 200μl from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the vial that is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn vial_insert_200ul(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<VolumetricContainerModel> {
    let name = "Vial Insert 200μl";

    if let Some(existing_tube) = VolumetricContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(VolumetricContainerModel::new()
        .name(name)?
        .description("Vial insert of 200μl, used to hold samples in vials.")?
        .created_by(user)?
        .liters(0.0002)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a vial insert
pub(crate) fn init_vici_schweiz_insert(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let insert = "VICI Schweiz Insert 200μl";
    if let Some(insert) = CommercialProduct::from_name(insert, conn).optional()? {
        return Ok(insert);
    }

    let insert_trackable = vial_insert_200ul(user, conn)?;
    let vici_schweiz = vici_schweiz(user, conn)?;
    Ok(CommercialProduct::new()
        .name(insert)?
        .description("VICI Schweiz insert, used to decrease needed amount of extract for mass spectrometry analysis.")?
        .created_by(user)?
        .brand(vici_schweiz)?
        .parent_model(insert_trackable)?
        .insert(user.id, conn)?)
}
