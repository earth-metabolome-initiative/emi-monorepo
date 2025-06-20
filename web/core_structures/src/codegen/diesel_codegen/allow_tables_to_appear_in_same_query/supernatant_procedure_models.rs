use crate::codegen::diesel_codegen::tables::{
    procedure_model_trackables::procedure_model_trackables,
    supernatant_procedure_models::supernatant_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    supernatant_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(supernatant_procedure_models, procedure_models);
