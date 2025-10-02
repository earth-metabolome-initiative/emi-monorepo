//! Submodule to initialize the `markers` in the database.

use core_structures::{
    PhysicalAssetModel, User,
    tables::insertables::{AssetModelSettable, PhysicalAssetModelAttribute},
};
use diesel::PgConnection;
use web_common_traits::database::{DispatchableInsertableVariant, InsertError, Insertable};

/// Returns the marker model for cardboard arrows, creating it if it does not
/// exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the marker model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the insertion fails.
pub(crate) fn marker_arrow_model(
    user: &User,
    conn: &mut PgConnection,
) -> Result<PhysicalAssetModel, InsertError<PhysicalAssetModelAttribute>> {
    const MARKER_ARROW: &str = "Marker Arrow";

    if let Ok(marker_arrow) = PhysicalAssetModel::from_name(MARKER_ARROW, conn) {
        return Ok(marker_arrow);
    }

    PhysicalAssetModel::new()
        .name(MARKER_ARROW)?
        .description("Marker arrow to highlight in a photograph a subject of interest.")?
        .created_by(user)?
        .insert(user.id, conn)
}
