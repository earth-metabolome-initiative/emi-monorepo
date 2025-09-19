//! Submodule defining partial builders for procedure template `asset_models`
//! related to pipettes.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::bead::bead_3mm, procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a bead 3mm trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn bead_3mm_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, bead_3mm(user, conn)?, conn)
}
