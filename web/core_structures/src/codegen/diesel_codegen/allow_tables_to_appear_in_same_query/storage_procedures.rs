use crate::codegen::diesel_codegen::tables::{
    procedure_templates::procedure_templates, storage_procedures::storage_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, storage_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, procedures);
use crate::codegen::diesel_codegen::tables::containers::containers;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, containers);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(storage_procedures, procedure_assets);
