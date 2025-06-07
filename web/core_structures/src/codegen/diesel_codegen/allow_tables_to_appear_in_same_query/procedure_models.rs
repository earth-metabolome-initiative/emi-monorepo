use crate::codegen::diesel_codegen::tables::{
    documents::documents, procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_models, documents);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedure_models, users);
