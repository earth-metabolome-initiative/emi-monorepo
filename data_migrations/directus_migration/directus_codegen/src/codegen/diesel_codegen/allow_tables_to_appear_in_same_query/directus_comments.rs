use crate::codegen::diesel_codegen::tables::{
    directus_comments::directus_comments, directus_users::directus_users,
};
diesel::allow_tables_to_appear_in_same_query!(directus_comments, directus_users);
