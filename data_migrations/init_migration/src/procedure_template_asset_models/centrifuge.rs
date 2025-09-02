//! Submodule defining partial builders for procedure template asset_models
//! related to centrifuges.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::centrifuge::safelock_centrifuge,
    procedure_template_asset_models::default_pmt::default_pmt,
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
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, safelock_centrifuge(user, conn)?.id(conn)?.id(conn)?)
}
