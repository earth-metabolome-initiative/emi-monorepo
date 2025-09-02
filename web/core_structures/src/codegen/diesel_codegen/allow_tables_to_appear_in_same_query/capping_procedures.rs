use crate::codegen::diesel_codegen::tables::{
    capping_procedure_templates::capping_procedure_templates,
    capping_procedures::capping_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(capping_procedures, capping_procedure_templates);
use crate::codegen::diesel_codegen::tables::caps_models::caps_models;
diesel::allow_tables_to_appear_in_same_query!(capping_procedures, caps_models);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(capping_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(capping_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(capping_procedures, procedures);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(capping_procedures, volumetric_containers);
