//! Submodule defining partial builders for procedure template `asset_models`
//! related to volume measuring devices.

use core_structures::{User, tables::insertables::InsertableProcedureTemplateAssetModelBuilder};
use diesel::PgConnection;

use crate::{
    asset_models::instruments::volume_measuring_device::volume_measuring_device,
    procedure_template_asset_models::default_pmt::default_pmt,
};

/// Returns a partial builder for a volume measuring device procedure template
///
/// # Arguments
///
/// * `user` - The user who is creating the trackable.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(crate) fn volume_measuring_device_builder(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder> {
    default_pmt(user, volume_measuring_device(user, conn)?, conn)
}
