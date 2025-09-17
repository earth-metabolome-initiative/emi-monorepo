use crate::codegen::diesel_codegen::tables::{
    container_compatibility_rules::container_compatibility_rules,
    storage_procedures::storage_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, container_compatibility_rules);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, container_models);
use crate::codegen::diesel_codegen::tables::containers::containers;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, containers);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, physical_asset_models);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, procedure_template_asset_models);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, procedures);
use crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, storage_procedure_templates);
