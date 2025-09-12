use crate::codegen::diesel_codegen::tables::{
    directus_collections::directus_collections, directus_shares::directus_shares,
};
diesel::allow_tables_to_appear_in_same_query!(directus_shares, directus_collections);
use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
diesel::allow_tables_to_appear_in_same_query!(directus_shares, directus_roles);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_shares, directus_users);
