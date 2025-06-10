use crate::codegen::diesel_codegen::tables::login_providers::login_providers;
use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
diesel::allow_tables_to_appear_in_same_query!(temporary_user, login_providers);
