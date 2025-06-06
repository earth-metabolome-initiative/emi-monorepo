use crate::codegen::diesel_codegen::tables::injection_methods::injection_methods;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(injection_methods, directus_users);
