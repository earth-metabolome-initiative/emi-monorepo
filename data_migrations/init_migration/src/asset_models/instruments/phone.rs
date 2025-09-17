//! Submodule creating the instrument commercial product model for the Ball Mill
//! instrument.

use core_structures::{PhoneModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

/// Returns the smartphone device.
///
/// # Implementation Details
///
/// This function either instantiate a new smartphone from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the smartphone is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn phone_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhoneModel> {
    let name = "Phone";

    if let Some(existing) = PhoneModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PhoneModel::new()
        .name(name)?
        .description(
            "A phone (smartphone) which may be used to take pictures or as a positioning device.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?)
}
