use crate::codegen::diesel_codegen::tables::{
    packaging_models::packaging_models,
    packaging_procedure_templates::packaging_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_templates, packaging_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    packaging_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_templates, procedure_templates);
