//! Submodule defining partial builders for procedure template asset_models
//! related to pipettes.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::pipettes::{pipette_200ul, pipette_1000ul},
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a pipettes 1000ul trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_1000ul_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, pipette_1000ul(user, conn)?, conn)
}

/// Returns a partial builder for a pipettes 200ul trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_200ul_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, pipette_200ul(user, conn)?, conn)
}
