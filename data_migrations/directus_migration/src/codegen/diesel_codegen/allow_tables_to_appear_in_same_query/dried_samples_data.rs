use crate::codegen::diesel_codegen::tables::{
    batches::batches, dried_samples_data::dried_samples_data,
};
diesel::allow_tables_to_appear_in_same_query!(dried_samples_data, batches);
use crate::codegen::diesel_codegen::tables::containers::containers;
diesel::allow_tables_to_appear_in_same_query!(dried_samples_data, containers);
use crate::codegen::diesel_codegen::tables::field_data::field_data;
diesel::allow_tables_to_appear_in_same_query!(dried_samples_data, field_data);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(dried_samples_data, directus_users);
