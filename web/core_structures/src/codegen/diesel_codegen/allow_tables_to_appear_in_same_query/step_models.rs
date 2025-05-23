use crate::codegen::diesel_codegen::tables::{documents::documents, step_models::step_models};
diesel::allow_tables_to_appear_in_same_query!(step_models, documents);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(step_models, procedure_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(step_models, users);
