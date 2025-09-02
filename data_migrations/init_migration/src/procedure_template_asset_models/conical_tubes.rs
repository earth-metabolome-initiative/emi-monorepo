//! Submodule defining partial builders for procedure template asset_models
//! related to conical tubes.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::containers::conical_centrifugal_tubes::conical_centrifugal_tube_50ml,
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a conical tubes trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn cct_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, conical_centrifugal_tube_50ml(user, conn)?.id(conn)?.id(conn)?.id(conn)?)
}
