use crate::codegen::diesel_codegen::tables::container_rules::container_rules;
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(container_rules, directus_users);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(container_rules, container_models);
