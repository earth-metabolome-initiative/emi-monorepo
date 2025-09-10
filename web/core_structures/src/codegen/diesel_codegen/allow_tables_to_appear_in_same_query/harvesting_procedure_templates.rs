use crate::codegen::diesel_codegen::tables::{
    harvesting_procedure_templates::harvesting_procedure_templates,
    procedure_template_asset_models::procedure_template_asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    harvesting_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedure_templates, procedure_templates);
use crate::codegen::diesel_codegen::tables::sample_models::sample_models;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedure_templates, sample_models);
use crate::codegen::diesel_codegen::tables::sample_source_models::sample_source_models;
diesel::allow_tables_to_appear_in_same_query!(harvesting_procedure_templates, sample_source_models);
