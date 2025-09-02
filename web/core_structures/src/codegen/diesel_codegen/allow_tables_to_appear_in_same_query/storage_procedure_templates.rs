use crate::codegen::diesel_codegen::tables::{
    container_compatibility_rules::container_compatibility_rules,
    storage_procedure_templates::storage_procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(
    storage_procedure_templates,
    container_compatibility_rules
);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_templates, container_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_templates, physical_asset_models);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(
    storage_procedure_templates,
    procedure_template_asset_models
);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_templates, procedure_templates);
