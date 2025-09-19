#![doc = include_str!("../README.md")]

pub mod errors;
pub mod procedure_codegen;
pub(crate) mod structs;
pub use procedure_codegen::ProcedureCodegen;
pub(crate) use structs::{Procedure, ProcedureTemplate, procedure_templates};
pub mod constraints;
mod utils;
pub(crate) use utils::{
    PROCEDURE_ASSETS_TABLE_NAME, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME, is_asset_foreign_key,
    is_asset_model_foreign_key, is_procedure_assets_foreign_key,
    is_procedure_template_asset_model_foreign_key,
};
