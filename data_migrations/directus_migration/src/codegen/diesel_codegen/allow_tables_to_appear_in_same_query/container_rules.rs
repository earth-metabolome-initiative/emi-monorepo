use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, container_rules::container_rules,
};
diesel::allow_tables_to_appear_in_same_query!(container_rules, container_models);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(container_rules, directus_users);
