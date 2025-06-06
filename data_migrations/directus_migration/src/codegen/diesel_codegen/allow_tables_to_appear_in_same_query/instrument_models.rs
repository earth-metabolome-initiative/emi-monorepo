use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
use crate::codegen::diesel_codegen::tables::brands::brands;
diesel::allow_tables_to_appear_in_same_query!(instrument_models, brands);
use crate::codegen::diesel_codegen::tables::instrument_types::instrument_types;
diesel::allow_tables_to_appear_in_same_query!(instrument_models, instrument_types);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(instrument_models, directus_users);
