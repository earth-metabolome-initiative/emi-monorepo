//! Submodule defining partial builders for procedure model trackables related
//! to pipette tips.

use core_structures::{User, tables::insertables::InsertableProcedureModelTrackableBuilder};
use diesel::PgConnection;

use crate::{
    procedure_model_trackables::default_pmt::default_pmt,
    trackables::instruments::pipette_tips::{pipette_tip_200ul, pipette_tip_1000ul},
};

/// Returns a partial builder for a pipette tips 1000ul trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_tips_1000ul_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureModelTrackableBuilder> {
    default_pmt(user, pipette_tip_1000ul(user, conn)?.id(conn)?)
}

/// Returns a partial builder for a pipette tips 200ul trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_tips_200ul_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureModelTrackableBuilder> {
    default_pmt(user, pipette_tip_200ul(user, conn)?.id(conn)?)
}
