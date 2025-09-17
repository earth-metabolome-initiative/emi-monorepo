//! Submodule to initialize the `organisms` in the database.

use core_structures::{
    OrganismModel, SampleModel, User,
    tables::insertables::{AssetModelSettable, SampleModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

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
pub fn organism_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<OrganismModel> {
    const ORGANISM: &str = "Organism";

    if let Some(existing_organism) = OrganismModel::from_name(ORGANISM, conn).optional()? {
        return Ok(existing_organism);
    }

    Ok(OrganismModel::new()
        .name(ORGANISM)?
        .description("Organisms used in laboratory procedures")?
        .created_by(user)?
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
pub fn organism_sample_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<SampleModel> {
    const SAMPLE: &str = "Organism Sample";

    if let Some(existing_sample) = SampleModel::from_name(SAMPLE, conn).optional()? {
        return Ok(existing_sample);
    }

    Ok(SampleModel::new()
        .name(SAMPLE)?
        .description("Samples used in laboratory procedures")?
        .sample_source_model(organism_model(user, conn)?)?
        .created_by(user)?
        .insert(user.id, conn)?)
}
