use crate::codegen::diesel_codegen::tables::directus_versions::directus_versions;
use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
diesel::allow_tables_to_appear_in_same_query!(directus_versions, directus_collections);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_versions, directus_users);
