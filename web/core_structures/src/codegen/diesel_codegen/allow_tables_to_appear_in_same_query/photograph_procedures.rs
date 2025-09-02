use crate::codegen::diesel_codegen::tables::{
    assets::assets, photograph_procedures::photograph_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, assets);
use crate::codegen::diesel_codegen::tables::camera_models::camera_models;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, camera_models);
use crate::codegen::diesel_codegen::tables::cameras::cameras;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, cameras);
use crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(
    photograph_procedures,
    photograph_procedure_templates
);
use crate::codegen::diesel_codegen::tables::physical_assets::physical_assets;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, physical_assets);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedures, procedures);
