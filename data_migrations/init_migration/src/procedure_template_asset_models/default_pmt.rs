//! Submodule defining default partial builder initialization for
//! a standard procedure template asset_model.

use core_structures::{
    AssetModel, ProcedureTemplateAssetModel, User,
    tables::insertables::{
        InsertableProcedureTemplateAssetModelBuilder, ProcedureTemplateAssetModelSettable,
    },
};
use web_common_traits::database::Insertable;

/// Returns a partial builder for a default procedure template asset_model.
///
/// # Arguments
///
/// * `user` - The user who is creating the asset_model.
/// * `asset_model` - The asset_model to associate with the procedure template
///   asset_model.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(super) fn default_pmt<T>(
    user: &User,
    asset_model: T,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder>
where
    T: AsRef<AssetModel>,
{
    let asset_model = asset_model.as_ref();
    Ok(ProcedureTemplateAssetModel::new()
        .name(&asset_model.name)?
        .asset_model(asset_model.id)?
        .created_by(user.id)?)
}
