//! Submodule to initialize the freeze dryer in the database.

use core_structures::{FreezeDryerModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Returns the freeze dryer.
///
/// # Implementation Details
///
/// This function either instantiate a new freeze dryer from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the freeze dryer is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn freeze_dryer(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<FreezeDryerModel> {
    let name = "Freeze dryer";

    if let Some(existing) = FreezeDryerModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(FreezeDryerModel::new()
        .name(name)?
        .description(
            "A freeze dryer (or lyophilisator) used to sublimate water content of samples.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?)
}
