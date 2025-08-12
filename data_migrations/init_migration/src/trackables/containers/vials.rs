//! Submodule to initialize the

use core_structures::{CommercialProduct, User, VolumetricContainerModel};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::brands::macherey_nagel;

/// Returns the vial of 1.5 ml.
///
/// # Implementation Details
///
/// This function either instantiate a new vial of 1.5 ml from
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
pub(crate) fn vial_1_5ml(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<VolumetricContainerModel> {
    let name = "Vial 1.5ml";

    if let Some(existing_tube) = VolumetricContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(VolumetricContainerModel::new()
        .name(name.to_owned())?
        .description("Vial of 1.5 ml used for extracts storage".to_owned())?
        .created_by(user.id)?
        .liters(0.0015)?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Machinery Nagel Vial 1.5ml product.
pub(crate) fn init_macherey_nagel_vial(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let vial = "Macherey Nagel Vial 1.5ml";
    if let Some(vial) = CommercialProduct::from_name(vial, conn).optional()? {
        return Ok(vial);
    }

    let vial_1_5ml = vial_1_5ml(user, conn)?;
    let macherey_nagel = macherey_nagel(user, conn)?;
    Ok(CommercialProduct::new()
        .name(vial.to_owned())?
        .description(
            "Macherey Nagel Vial 1.5ml, used for extract library and mass spectrometry analysis."
                .to_owned(),
        )?
        .created_by(user.id)?
        .brand(macherey_nagel.id)?
        .parent(Some(vial_1_5ml.id))?
        .insert(user.id, conn)?)
}
