//! Submodule to initialize the `soils` in the database.

use core_structures::{
    SampleModel, SoilModel, User,
    tables::insertables::{AssetModelSettable, SampleModelSettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

/// Returns the soil trackable.
///
/// # Arguments
///
/// * `user` - The user for whom the soil is being created.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn soil_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<SoilModel> {
    const SOIL: &str = "Soil";

    if let Some(existing_soil) = SoilModel::from_name(SOIL, conn).optional()? {
        return Ok(existing_soil);
    }

    Ok(SoilModel::new()
        .name(SOIL)?
        .description("soils used in laboratory procedures")?
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
pub fn soil_sample_model(user: &User, conn: &mut PgConnection) -> anyhow::Result<SampleModel> {
    const SAMPLE: &str = "Soil Sample";

    if let Some(existing_sample) = SampleModel::from_name(SAMPLE, conn).optional()? {
        return Ok(existing_sample);
    }

    Ok(SampleModel::new()
        .name(SAMPLE)?
        .description("Samples used in laboratory procedures")?
        .sample_source_model(soil_model(user, conn)?)?
        .created_by(user)?
        .insert(user.id, conn)?)
}
