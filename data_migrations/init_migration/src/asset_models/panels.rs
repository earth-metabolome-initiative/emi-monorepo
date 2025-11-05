//! Submodule to initialize the `panels` in the database.

use core_structures::{PhysicalAssetModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

/// Returns the panel asset model.
///
/// # Arguments
///
/// * `user` - The user for whom the panel is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn panel_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhysicalAssetModel> {
    const PANELS: &str = "Panel";

    if let Some(existing_panel) = PhysicalAssetModel::from_name(PANELS, conn).optional()? {
        return Ok(existing_panel);
    }

    Ok(PhysicalAssetModel::new()
        .name(PANELS)?
        .description("Panel for documenting organisms – typically used in Botanical Gardens")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
