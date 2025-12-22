//! Submodule defining default partial builder initialization for
//! a standard procedure template `asset_model`.

use core_structures::{
    AssetModel, ProcedureTemplateAssetModel,
    tables::insertables::{
        InsertableProcedureTemplateAssetModelBuilder, ProcedureTemplateAssetModelSettable,
    },
};
use diesel::PgConnection;
use web_common_traits::{
    database::{Insertable, Read},
    prelude::ExtensionTable,
};

/// Returns a partial builder for a default procedure template `asset_model`.
///
/// # Arguments
///
/// * `asset_model` - The `asset_model` to associate with the procedure template
///   `asset_model`.
/// * `conn` - The database connection to use.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(super) fn default_pmt<AssetModelLike>(
    asset_model_like: &AssetModelLike,
    conn: &mut PgConnection,
) -> anyhow::Result<InsertableProcedureTemplateAssetModelBuilder>
where
    AssetModelLike: ExtensionTable<AssetModel>,
    for<'a> &'a AssetModelLike:
        diesel::Identifiable<Id = <&'a AssetModel as diesel::Identifiable>::Id>,
{
    use diesel::Identifiable;
    let asset_model_id = AssetModel::read(*asset_model_like.id(), conn)?;
    Ok(ProcedureTemplateAssetModel::new().name(&asset_model.name)?.asset_model(asset_model_id)?)
}
