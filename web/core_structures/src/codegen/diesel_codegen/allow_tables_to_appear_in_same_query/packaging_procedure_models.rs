use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, packaging_procedure_models::packaging_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_models, container_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    packaging_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_models, procedure_models);
