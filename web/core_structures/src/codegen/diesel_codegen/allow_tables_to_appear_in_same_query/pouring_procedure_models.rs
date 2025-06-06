use crate::codegen::diesel_codegen::tables::{
    pouring_procedure_models::pouring_procedure_models, procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(pouring_procedure_models, procedure_models);
