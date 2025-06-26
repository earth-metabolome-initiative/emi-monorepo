use crate::codegen::diesel_codegen::tables::{
    camera_models::camera_models, photograph_procedure_models::photograph_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(photograph_procedure_models, camera_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    photograph_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(photograph_procedure_models, procedure_models);
