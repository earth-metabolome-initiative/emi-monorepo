//! Submodule creating the instrument commercial product model for the Pipette
//! 200 instrument.

use core_structures::{User, VolumeMeasuringDeviceModel, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};
/// Returns the volume measuring device model instance.
///
/// # Implementation Details
///
/// This function either instantiate a new volume measuring device model in
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
pub(crate) fn volume_measuring_device(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<VolumeMeasuringDeviceModel> {
    let name = "Volume Measuring Device";

    if let Some(existing) = VolumeMeasuringDeviceModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(VolumeMeasuringDeviceModel::new()
        .name(name)?
        .description("A generic volume measuring device")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
