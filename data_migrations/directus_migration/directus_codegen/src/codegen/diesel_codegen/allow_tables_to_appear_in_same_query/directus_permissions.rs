use crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions;
use crate::codegen::diesel_codegen::tables::directus_policies::directus_policies;
diesel::allow_tables_to_appear_in_same_query!(directus_permissions, directus_policies);
