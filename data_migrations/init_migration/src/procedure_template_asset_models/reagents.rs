//! Submodule defining partial builders for procedure template `asset_models`
//! related to reagents.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::reagent_models::{absolute_ethanol, distilled_water},
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a the ethanol reagent.
///
/// # Arguments
///
/// * `user` - The user who is creating the reagent.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn absolute_ethanol_model_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(&absolute_ethanol(user, conn)?, conn)
}

/// Returns a partial builder for distilled water reagent.
///
/// # Arguments
///
/// * `user` - The user who is creating the reagent.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn distilled_water_model_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(&distilled_water(user, conn)?, conn)
}
