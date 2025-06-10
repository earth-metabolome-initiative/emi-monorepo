use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
use crate::codegen::diesel_codegen::tables::shaking_procedure_models::shaking_procedure_models;
diesel::allow_tables_to_appear_in_same_query!(shaking_procedure_models, procedure_models);
