use crate::codegen::diesel_codegen::tables::{brands::brands, container_models::container_models};
diesel::allow_tables_to_appear_in_same_query!(container_models, brands);
use crate::codegen::diesel_codegen::tables::container_types::container_types;
diesel::allow_tables_to_appear_in_same_query!(container_models, container_types);
use crate::codegen::diesel_codegen::tables::si_units::si_units;
diesel::allow_tables_to_appear_in_same_query!(container_models, si_units);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(container_models, directus_users);
