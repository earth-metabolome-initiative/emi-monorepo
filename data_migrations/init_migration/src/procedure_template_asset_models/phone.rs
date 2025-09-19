//! Submodule defining partial builders for procedure template `asset_models`
//! related to phones.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::phone::phone_model,
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a phone trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn phone_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(&phone_model(user, conn)?, conn)
}
