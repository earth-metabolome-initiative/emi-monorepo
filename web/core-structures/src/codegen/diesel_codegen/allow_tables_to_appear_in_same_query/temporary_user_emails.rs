use crate::codegen::diesel_codegen::tables::{
    login_providers::login_providers, temporary_user_emails::temporary_user_emails,
};
diesel::allow_tables_to_appear_in_same_query!(temporary_user_emails, login_providers);
