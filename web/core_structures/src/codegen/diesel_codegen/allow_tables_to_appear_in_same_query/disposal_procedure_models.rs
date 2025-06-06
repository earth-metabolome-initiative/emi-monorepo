use crate::codegen::diesel_codegen::tables::{
    disposal_procedure_models::disposal_procedure_models, procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(disposal_procedure_models, procedure_models);
