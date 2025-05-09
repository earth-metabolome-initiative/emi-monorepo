use crate::codegen::diesel_codegen::tables::{reagents::reagents, users::users};
diesel::allow_tables_to_appear_in_same_query!(reagents, users);
