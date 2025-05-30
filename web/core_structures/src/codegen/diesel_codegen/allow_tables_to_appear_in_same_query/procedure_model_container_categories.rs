use crate::codegen::diesel_codegen::tables::{
    procedure_model_container_categories::procedure_model_container_categories,
    procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    procedure_model_container_categories,
    procedure_models
);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedure_model_container_categories, users);
