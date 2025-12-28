//! Submodule providing constraint structs that can be applied to columns in
//! procedure and procedure template tables.

mod asset_model_column_naming;
pub use asset_model_column_naming::AssetModelColumnNaming;
mod asset_column_naming;
pub use asset_column_naming::AssetColumnNaming;
mod procedure_template_asset_model_column_naming;
pub use procedure_template_asset_model_column_naming::{
    PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME, ProcedureTemplateAssetModelColumnNaming,
};
mod matching_asset_model_columns;
pub use matching_asset_model_columns::MatchingAssetModelColumns;
mod horizontal_asset_model_foreign_key;
pub use horizontal_asset_model_foreign_key::HorizontalAssetModelForeignKey;
