//! Submodule to initialize the freeze dryer in the database.

use core_structures::{FreezeDrierModel, User};
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
) -> anyhow::Result<FreezeDrierModel> {
    let name = "Freeze dryer";

    if let Some(existing) = FreezeDrierModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(FreezeDrierModel::new()
        .name(name.to_owned())?
        .description(
            "A freeze dryer (or lyophilisator) used to sublimate water content of samples."
                .to_owned(),
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
