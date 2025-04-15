use crate::codegen::diesel_codegen::tables::{
    directus_users::directus_users, directus_versions::directus_versions,
};
diesel::allow_tables_to_appear_in_same_query!(directus_versions, directus_users);
use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
diesel::allow_tables_to_appear_in_same_query!(directus_versions, directus_collections);
