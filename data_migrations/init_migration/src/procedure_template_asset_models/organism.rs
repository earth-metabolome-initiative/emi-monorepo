//! Submodule defining partial builders for procedure template asset_models
//! related to organisms.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::organisms::{organism, sample},
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a organism trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn organism_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, organism(user, conn)?.id(conn)?)
}

/// Returns a partial builder for a sample trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn sample_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, sample(user, conn)?.id(conn)?)
}
