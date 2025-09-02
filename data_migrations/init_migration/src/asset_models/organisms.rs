//! Submodule to initialize the `instruments` in the database.

use core_structures::{AssetModel, User, tables::insertables::AssetModelBuildable};
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
pub(crate) fn organism(user: &User, conn: &mut PgConnection) -> anyhow::Result<AssetModel> {
    const ORGANISM: &str = "Organism";

    if let Some(existing_organism) = AssetModel::from_name(ORGANISM, conn).optional()? {
        return Ok(existing_organism);
    }

    Ok(AssetModel::new()
        .name(ORGANISM.to_owned())?
        .description("Organisms used in laboratory procedures".to_owned())?
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
pub(crate) fn sample(user: &User, conn: &mut PgConnection) -> anyhow::Result<AssetModel> {
    const SAMPLE: &str = "sample";

    if let Some(existing_sample) = AssetModel::from_name(SAMPLE, conn).optional()? {
        return Ok(existing_sample);
    }

    Ok(AssetModel::new()
        .name(SAMPLE.to_owned())?
        .description("Samples used in laboratory procedures".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
