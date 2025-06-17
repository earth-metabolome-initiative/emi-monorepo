use crate::codegen::diesel_codegen::tables::{
    ball_mill_container_models::ball_mill_container_models,
    ball_mill_procedure_models::ball_mill_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_models,
    ball_mill_container_models
);
use crate::codegen::diesel_codegen::tables::ball_mill_machine_models::ball_mill_machine_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedure_models, ball_mill_machine_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedure_models, procedure_models);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_models,
    volumetric_container_models
);
