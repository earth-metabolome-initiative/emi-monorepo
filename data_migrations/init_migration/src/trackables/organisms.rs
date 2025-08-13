//! Submodule to initialize the `instruments` in the database.

use core_structures::{Trackable, User};
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
pub(crate) fn organism(user: &User, conn: &mut PgConnection) -> anyhow::Result<Trackable> {
    const ORGANISM: &str = "Organism";

    if let Some(existing_organism) = Trackable::from_name(ORGANISM, conn).optional()? {
        return Ok(existing_organism);
    }

    Ok(Trackable::new()
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
pub(crate) fn sample(user: &User, conn: &mut PgConnection) -> anyhow::Result<Trackable> {
    const SAMPLE: &str = "sample";

    if let Some(existing_sample) = Trackable::from_name(SAMPLE, conn).optional()? {
        return Ok(existing_sample);
    }

    Ok(Trackable::new()
        .name(SAMPLE.to_owned())?
        .description("Samples used in laboratory procedures".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
