use crate::codegen::diesel_codegen::tables::{procedures::procedures, users::users};
diesel::allow_tables_to_appear_in_same_query!(procedures, users);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(procedures, procedure_models);
