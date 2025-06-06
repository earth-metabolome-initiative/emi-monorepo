use crate::codegen::diesel_codegen::tables::{
    directus_users::directus_users, injection_methods::injection_methods,
};
diesel::allow_tables_to_appear_in_same_query!(injection_methods, directus_users);
