use crate::codegen::diesel_codegen::tables::{documents::documents, trackables::trackables};
diesel::allow_tables_to_appear_in_same_query!(trackables, documents);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(trackables, users);
