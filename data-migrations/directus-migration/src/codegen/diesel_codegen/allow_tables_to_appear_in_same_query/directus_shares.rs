use crate::codegen::diesel_codegen::tables::directus_shares::directus_shares;
use crate::codegen::diesel_codegen::tables::directus_roles::directus_roles;
diesel::allow_tables_to_appear_in_same_query!(directus_shares, directus_roles);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_shares, directus_users);
use crate::codegen::diesel_codegen::tables::directus_collections::directus_collections;
diesel::allow_tables_to_appear_in_same_query!(directus_shares, directus_collections);
