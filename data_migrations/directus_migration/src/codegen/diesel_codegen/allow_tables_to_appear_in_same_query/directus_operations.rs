use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
use crate::codegen::diesel_codegen::tables::directus_flows::directus_flows;
diesel::allow_tables_to_appear_in_same_query!(directus_operations, directus_flows);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_operations, directus_users);
