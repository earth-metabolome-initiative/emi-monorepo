use crate::codegen::diesel_codegen::tables::{
    procedure_models::procedure_models, procedure_step_models::procedure_step_models,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_step_models, procedure_models);
use crate::codegen::diesel_codegen::tables::step_models::step_models;
diesel::allow_tables_to_appear_in_same_query!(procedure_step_models, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedure_step_models, users);
