use crate::codegen::diesel_codegen::tables::{
    login_providers::login_providers, temporary_user::temporary_user,
};
diesel::allow_tables_to_appear_in_same_query!(temporary_user, login_providers);
