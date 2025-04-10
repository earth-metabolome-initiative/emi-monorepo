use crate::codegen::diesel_codegen::tables::directus_dashboards::directus_dashboards;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_dashboards, directus_users);
