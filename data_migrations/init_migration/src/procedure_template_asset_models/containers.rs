//! Submodule defining partial builders for procedure template `asset_models`
//! related to containers

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::containers::bottle_1l, procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a 1L bottle trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn bottle_1l_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(&bottle_1l(user, conn)?, conn)
}
