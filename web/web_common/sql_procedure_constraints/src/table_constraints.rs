//! Submodule providing constraint structs that can be applied to procedure and
//! procedure template tables.

mod procedure_descendant_naming;
pub use procedure_descendant_naming::{PROCEDURES_TABLE_NAME, ProcedureDescendantNaming};
mod procedure_template_descendant_naming;
pub use procedure_template_descendant_naming::{
    PROCEDURE_TEMPLATES_TABLE_NAME, ProcedureTemplateDescendantNaming,
};
mod matching_procedure_and_template_tables;
pub use matching_procedure_and_template_tables::MatchingProcedureAndTemplateTables;
mod procedure_template_asset_model_unique_index;
pub use procedure_template_asset_model_unique_index::ProcedureTemplateAssetModelUniqueIndex;
