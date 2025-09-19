//! Submodule defining partial builders for procedure template `asset_models`
//! related to pipette tips.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::pipette_tips::{pipette_tip_200ul, pipette_tip_1000ul},
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a pipette tips 1000ul trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_tips_1000ul_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(&pipette_tip_1000ul(user, conn)?, conn)
}

/// Returns a partial builder for a pipette tips 200ul trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn pipette_tips_200ul_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(&pipette_tip_200ul(user, conn)?, conn)
}
