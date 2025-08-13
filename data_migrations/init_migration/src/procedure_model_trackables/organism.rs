//! Submodule defining partial builders for procedure model trackables related
//! to organisms.

use core_structures::{User, tables::insertables::InsertableProcedureModelTrackableBuilder};
use diesel::PgConnection;

use crate::{
    procedure_model_trackables::default_pmt::default_pmt,
    trackables::organisms::{organism, sample},
};

/// Returns a partial builder for a organism trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn organism_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureModelTrackableBuilder> {
    default_pmt(user, organism(user, conn)?)
}

/// Returns a partial builder for a sample trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn sample_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureModelTrackableBuilder> {
    default_pmt(user, sample(user, conn)?)
}
