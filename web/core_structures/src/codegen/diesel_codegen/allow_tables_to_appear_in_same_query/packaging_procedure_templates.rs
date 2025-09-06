use crate::codegen::diesel_codegen::tables::{
    asset_compatibility_rules::asset_compatibility_rules,
    packaging_procedure_templates::packaging_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(
    packaging_procedure_templates,
    asset_compatibility_rules
);
use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_templates, packaging_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_templates, physical_asset_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    packaging_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_templates, procedure_templates);
