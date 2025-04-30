use crate::codegen::diesel_codegen::tables::{
    extraction_data::extraction_data, extraction_methods::extraction_methods,
};
diesel::allow_tables_to_appear_in_same_query!(extraction_data, extraction_methods);
use crate::codegen::diesel_codegen::tables::batches::batches;
diesel::allow_tables_to_appear_in_same_query!(extraction_data, batches);
use crate::codegen::diesel_codegen::tables::si_units::si_units;
diesel::allow_tables_to_appear_in_same_query!(extraction_data, si_units);
use crate::codegen::diesel_codegen::tables::containers::containers;
diesel::allow_tables_to_appear_in_same_query!(extraction_data, containers);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(extraction_data, container_models);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(extraction_data, directus_users);
