use crate::codegen::diesel_codegen::tables::{
    fractioning_procedure_models::fractioning_procedure_models,
    procedure_model_trackables::procedure_model_trackables,
};
diesel::allow_tables_to_appear_in_same_query!(
    fractioning_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(fractioning_procedure_models, procedure_models);
use crate::codegen::diesel_codegen::tables::weighing_instrument_models::weighing_instrument_models;
diesel::allow_tables_to_appear_in_same_query!(
    fractioning_procedure_models,
    weighing_instrument_models
);
