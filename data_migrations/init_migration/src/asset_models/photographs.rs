//! Submodule to initialize the `instruments` in the database.

use core_structures::{
    DigitalAssetModel, User,
    tables::insertables::{AssetModelSettable, DigitalAssetModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Returns the photograph asset model.
///
/// # Arguments
///
/// * `user` - The user for whom the photograph is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn photograph(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<DigitalAssetModel> {
    const PHOTOGRAPHS: &str = "Photograph";

    if let Some(existing_photograph) = DigitalAssetModel::from_name(PHOTOGRAPHS, conn).optional()? {
        return Ok(existing_photograph);
    }

    Ok(DigitalAssetModel::new()
        .name(PHOTOGRAPHS)?
        .description("Photograph for documenting organisms and their habitats")?
        .mime_type("image/jpeg")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
