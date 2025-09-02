//! Submodule defining partial builders for procedure template asset_models
//! related to coffee wrappers.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::containers::wrappers::coffee_filter_wrapper,
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a coffee wrapper trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn coffee_wrapper_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, coffee_filter_wrapper(user, conn)?.id(conn)?.id(conn)?)
}
