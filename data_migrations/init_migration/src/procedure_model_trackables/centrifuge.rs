//! Submodule defining partial builders for procedure model trackables related
//! to centrifuges.

use core_structures::{User, tables::insertables::InsertableProcedureModelTrackableBuilder};
use diesel::PgConnection;

use crate::{
    procedure_model_trackables::default_pmt::default_pmt,
    trackables::instruments::centrifuge::safelock_centrifuge,
};

/// Returns a partial builder for a safelock centrifuge trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn safelock_centrifuge_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureModelTrackableBuilder> {
    default_pmt(user, safelock_centrifuge(user, conn)?.id(conn)?)
}
