use crate::codegen::diesel_codegen::tables::{
    ball_mill_procedure_templates::ball_mill_procedure_templates,
    ball_mill_procedures::ball_mill_procedures,
};
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, ball_mill_procedure_templates);
use crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, procedure_templates);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, procedures);
use crate::codegen::diesel_codegen::tables::assets::assets;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, assets);
use crate::codegen::diesel_codegen::tables::ball_mill_machine_models::ball_mill_machine_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, ball_mill_machine_models);
use crate::codegen::diesel_codegen::tables::ball_mill_machines::ball_mill_machines;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, ball_mill_machines);
use crate::codegen::diesel_codegen::tables::beads_models::beads_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, beads_models);
use crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, procedure_assets);
use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedures, volumetric_containers);
