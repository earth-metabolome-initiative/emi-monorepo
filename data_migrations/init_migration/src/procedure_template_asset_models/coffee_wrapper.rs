//! Submodule defining partial builders for procedure template trackables
//! related to coffee wrappers.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    procedure_template_trackables::default_pmt::default_pmt,
    trackables::containers::wrappers::coffee_filter_wrapper,
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
    default_pmt(user, coffee_filter_wrapper(user, conn)?.id(conn)?)
}
