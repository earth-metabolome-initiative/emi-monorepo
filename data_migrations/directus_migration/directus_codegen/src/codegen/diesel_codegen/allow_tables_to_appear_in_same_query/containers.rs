use crate::codegen::diesel_codegen::tables::containers::containers;
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(containers, container_models);
use crate::codegen::diesel_codegen::tables::universities::universities;
diesel::allow_tables_to_appear_in_same_query!(containers, universities);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(containers, directus_users);
