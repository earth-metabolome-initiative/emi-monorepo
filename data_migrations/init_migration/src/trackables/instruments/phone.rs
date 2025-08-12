//! Submodule creating the instrument commercial product model for the Ball Mill
//! instrument.

use core_structures::{PhoneModel, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

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
pub(crate) fn phone(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhoneModel> {
    let name = "Phone";

    if let Some(existing) = PhoneModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PhoneModel::new()
        .name(name.to_owned())?
        .description(
            "A phone (smartphone) which may be used to take pictures or as a positioning device."
                .to_owned(),
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
