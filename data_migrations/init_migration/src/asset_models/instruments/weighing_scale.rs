//! Submodule creating the instrument commercial product model for the Pipette
//! 200 instrument.

use core_structures::{User, WeighingDeviceModel, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};
/// Returns the weighing scale.
///
/// # Implementation Details
///
/// This function either instantiate a new weighing scale from
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
pub(crate) fn weighing_scale(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<WeighingDeviceModel> {
    let name = "Weighing Scale";

    if let Some(existing) = WeighingDeviceModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(WeighingDeviceModel::new()
        .name(name)?
        .description("A weighing scale used to measure the amount of samples.")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
