//! Submodule defining partial builders for procedure template trackables
//! related to freezedryers.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    procedure_template_trackables::default_pmt::default_pmt,
    trackables::instruments::freeze_dryer::freeze_dryer,
};

/// Returns a partial builder for a freeze_dryer trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn freeze_dryer_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, freeze_dryer(user, conn)?.id(conn)?)
}
