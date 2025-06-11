use crate::codegen::diesel_codegen::tables::{brands::brands, users::users};
diesel::allow_tables_to_appear_in_same_query!(brands, users);
