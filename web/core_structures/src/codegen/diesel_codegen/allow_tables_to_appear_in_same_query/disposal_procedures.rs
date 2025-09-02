use crate::codegen::diesel_codegen::tables::{
    disposal_procedure_templates::disposal_procedure_templates,
    disposal_procedures::disposal_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(disposal_procedures, disposal_procedure_templates);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(disposal_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(disposal_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(disposal_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(disposal_procedures, procedures);
