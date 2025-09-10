//! Submodule to initialize the bottles in the database.

use core_structures::{
    User, VolumetricContainerModel,
    tables::insertables::{AssetModelSettable, VolumetricContainerModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Returns the 1L bottle container model, creating it if it does not exist.
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
pub(crate) fn bottle_1l(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<VolumetricContainerModel> {
    let name = "Bottle (1L)";

    if let Some(existing_bottle) = VolumetricContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_bottle);
    }

    Ok(VolumetricContainerModel::new()
        .name(name)?
        .description("Standard 1L bottle, used to store solvents and reagents.")?
        .liters(1.0)?
        .created_by(user)?
        .insert(user.id, conn)?)
}
