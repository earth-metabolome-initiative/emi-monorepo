use crate::codegen::diesel_codegen::tables::{
    packaging_models::packaging_models, packaging_procedure_models::packaging_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_models, packaging_models);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_procedure_models, procedure_models);
