//! Submodule defining partial builders for procedure template asset_models
//! related to freezers.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::freezer::freezer,
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a freezer trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn freezer_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, freezer(user, conn)?, conn)
}
