//! Submodule defining partial builders for procedure template asset_models
//! related to ball mill.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::ball_mill_machine::ball_mill_machine,
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a safelock ball mill trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn safelock_ball_mill_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, ball_mill_machine(user, conn)?.id(conn)?.id(conn)?)
}
