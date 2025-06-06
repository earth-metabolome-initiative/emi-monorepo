use crate::codegen::diesel_codegen::tables::directus_flows::directus_flows;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_flows, directus_users);
