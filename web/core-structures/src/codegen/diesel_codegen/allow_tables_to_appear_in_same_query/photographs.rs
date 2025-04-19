use crate::codegen::diesel_codegen::tables::{photographs::photographs, users::users};
diesel::allow_tables_to_appear_in_same_query!(photographs, users);
