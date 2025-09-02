use crate::codegen::diesel_codegen::tables::{
    disposal_procedure_templates::disposal_procedure_templates,
    physical_asset_models::physical_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(disposal_procedure_templates, physical_asset_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    disposal_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(disposal_procedure_templates, procedure_templates);
