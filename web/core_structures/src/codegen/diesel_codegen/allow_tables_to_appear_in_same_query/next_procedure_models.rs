use crate::codegen::diesel_codegen::tables::{
    next_procedure_models::next_procedure_models, parent_procedure_models::parent_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(next_procedure_models, parent_procedure_models);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(next_procedure_models, procedure_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(next_procedure_models, users);
