use crate::codegen::diesel_codegen::tables::{
    email_providers::email_providers, user_emails::user_emails,
};
diesel::allow_tables_to_appear_in_same_query!(email_providers, user_emails);
use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
diesel::allow_tables_to_appear_in_same_query!(email_providers, login_providers);
