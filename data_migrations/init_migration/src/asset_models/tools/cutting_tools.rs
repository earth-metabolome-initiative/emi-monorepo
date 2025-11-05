//! Submodule defining the functions to initialize `cutting_tools` asset models.
use core_structures::{PhysicalAssetModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

/// Returns a scalpel.
///
/// # Implementation Details
///
/// This function either instantiate a new scalpel from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the scalpel is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn scalpel_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhysicalAssetModel> {
    let name = "Scalpel";

    if let Some(existing) = PhysicalAssetModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PhysicalAssetModel::new()
        .name(name)?
        .description("A scalpel used to cut samples.")?
        .created_by(user)?
        .insert(user.id, conn)?)
}

/// Returns a pair of scissors.
///
/// # Implementation Details
///
/// This function either instantiate a new pair of scissors from
/// the database or inserts it if it does not exist before returning it.
///
/// # Arguments
///
/// * `user` - The user for whom the pair of scissors is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn scissor_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhysicalAssetModel> {
    let name = "Scissors";

    if let Some(existing) = PhysicalAssetModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    Ok(PhysicalAssetModel::new()
        .name(name)?
        .description("A pair of scissors.")?
        .created_by(user)?
        .insert(user.id, conn)?)
}
