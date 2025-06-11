use crate::codegen::diesel_codegen::tables::{
    ball_mill_procedure_models::ball_mill_procedure_models,
    procedure_model_trackables::procedure_model_trackables,
};
diesel::allow_tables_to_appear_in_same_query!(
    ball_mill_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_procedure_models, procedure_models);
