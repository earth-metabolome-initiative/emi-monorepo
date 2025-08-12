//! Submodule defining partial builders for procedure model trackables related
//! to safelock tubes.

use core_structures::{User, tables::insertables::InsertableProcedureModelTrackableBuilder};
use diesel::PgConnection;

use crate::{
    procedure_model_trackables::default_pmt::default_pmt,
    trackables::containers::safelock_tubes::safelock_tubes_2ml,
};

/// Returns a partial builder for a safelock tube trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn safelock_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureModelTrackableBuilder> {
    default_pmt(user, safelock_tubes_2ml(user, conn)?.id(conn)?.id(conn)?)
}
