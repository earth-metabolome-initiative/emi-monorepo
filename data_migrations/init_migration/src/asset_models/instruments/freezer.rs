//! Submodule to initialize the freezer in the database.

use core_structures::{FreezerModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Returns the freezer.
///
/// # Implementation Details
///
/// This function either instantiate a new freezer from
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
pub(crate) fn freezer(user: &User, conn: &mut PgConnection) -> anyhow::Result<FreezerModel> {
    let name = "Freezer -80°C";

    if let Some(existing) = FreezerModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(FreezerModel::new()
        .name(name)?
        .description("A Freezer -80°C used for long-term storage of samples or freezing of samples prior to freeze-drying steps")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
