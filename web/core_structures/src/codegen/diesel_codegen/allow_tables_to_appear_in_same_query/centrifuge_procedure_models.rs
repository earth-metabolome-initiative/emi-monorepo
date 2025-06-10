use crate::codegen::diesel_codegen::tables::centrifuge_procedure_models::centrifuge_procedure_models;
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedure_models, procedure_models);
