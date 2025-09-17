use crate::codegen::diesel_codegen::tables::{
    directus_users::directus_users, specimens::specimens,
};
diesel::allow_tables_to_appear_in_same_query!(specimens, directus_users);
