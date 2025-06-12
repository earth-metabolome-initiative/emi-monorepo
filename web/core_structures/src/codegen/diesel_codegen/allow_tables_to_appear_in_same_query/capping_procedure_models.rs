use crate::codegen::diesel_codegen::tables::{
    capping_procedure_models::capping_procedure_models,
    procedure_model_trackables::procedure_model_trackables,
};
diesel::allow_tables_to_appear_in_same_query!(capping_procedure_models, procedure_model_trackables);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(capping_procedure_models, procedure_models);
