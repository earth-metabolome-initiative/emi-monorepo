//! Submodule to initialize the `instruments` in the database.

use core_structures::{PhysicalAssetModel, User, tables::insertables::AssetModelSettable};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

/// Returns the organism trackable.
///
/// # Arguments
///
/// * `user` - The user for whom the organism is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn organism(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhysicalAssetModel> {
    const ORGANISM: &str = "Organism";

    if let Some(existing_organism) = PhysicalAssetModel::from_name(ORGANISM, conn).optional()? {
        return Ok(existing_organism);
    }

    Ok(PhysicalAssetModel::new()
        .name(ORGANISM)?
        .description("Organisms used in laboratory procedures")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}

/// Returns the sample trackable.
///
/// # Arguments
///
/// * `user` - The user for whom the sample is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn sample(user: &User, conn: &mut PgConnection) -> anyhow::Result<PhysicalAssetModel> {
    const SAMPLE: &str = "sample";

    if let Some(existing_sample) = PhysicalAssetModel::from_name(SAMPLE, conn).optional()? {
        return Ok(existing_sample);
    }

    Ok(PhysicalAssetModel::new()
        .name(SAMPLE)?
        .description("Samples used in laboratory procedures")?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
