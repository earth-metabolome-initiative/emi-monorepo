use crate::codegen::diesel_codegen::tables::{documents::documents, users::users};
diesel::allow_tables_to_appear_in_same_query!(documents, users);
