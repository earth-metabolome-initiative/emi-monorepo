//! Submodule defining partial builders for procedure template trackables
//! related to vial caps.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    procedure_template_trackables::default_pmt::default_pmt,
    trackables::containers::vial_caps::{sealed_cap_vial_1_5ml, splitted_cap_vial_1_5ml},
};

/// Returns a partial builder for a splitted vial cap 1.5ml trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn splitted_cap_vial_1_5ml_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, splitted_cap_vial_1_5ml(user, conn)?.id(conn)?)
}

/// Returns a partial builder for a sealed vial cap 1.5ml trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn sealed_cap_vial_1_5ml_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, sealed_cap_vial_1_5ml(user, conn)?.id(conn)?)
}
