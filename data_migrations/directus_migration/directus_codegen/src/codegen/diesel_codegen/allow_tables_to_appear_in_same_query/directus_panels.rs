use crate::codegen::diesel_codegen::tables::{
    directus_dashboards::directus_dashboards, directus_panels::directus_panels,
};
diesel::allow_tables_to_appear_in_same_query!(directus_panels, directus_dashboards);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(directus_panels, directus_users);
