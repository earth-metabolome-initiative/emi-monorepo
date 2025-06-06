use crate::codegen::diesel_codegen::tables::{
    container_types::container_types, directus_users::directus_users,
};
diesel::allow_tables_to_appear_in_same_query!(container_types, directus_users);
