use crate::codegen::diesel_codegen::tables::{procedure_models::procedure_models, users::users};
diesel::allow_tables_to_appear_in_same_query!(procedure_models, users);
