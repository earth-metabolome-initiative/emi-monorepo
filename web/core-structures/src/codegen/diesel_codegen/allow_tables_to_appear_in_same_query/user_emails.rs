use crate::codegen::diesel_codegen::tables::{user_emails::user_emails, users::users};
diesel::allow_tables_to_appear_in_same_query!(user_emails, users);
