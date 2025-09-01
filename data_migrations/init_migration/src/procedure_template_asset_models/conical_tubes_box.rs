//! Submodule defining partial builders for procedure template trackables
//! related to conical tubes boxes.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    procedure_template_trackables::default_pmt::default_pmt,
    trackables::containers::{boxes::polystyrene_box, racks::conical_centrifugal_tube_50ml_rack},
};

/// Returns a partial builder for a conical tubes box trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn cct_box_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, polystyrene_box(user, conn)?.id(conn)?)
}

/// Returns a partial builder for a conical tubes rack trackable.
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn cct_rack_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, conical_centrifugal_tube_50ml_rack(user, conn)?.id(conn)?)
}
