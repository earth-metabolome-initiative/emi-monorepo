use crate::codegen::diesel_codegen::tables::{
    directus_sessions::directus_sessions, directus_users::directus_users,
};
diesel::allow_tables_to_appear_in_same_query!(directus_sessions, directus_users);
use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
diesel::allow_tables_to_appear_in_same_query!(directus_sessions, directus_shares);
