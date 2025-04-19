use crate::codegen::diesel_codegen::tables::{
    storage_containers::storage_containers, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(storage_containers, users);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(storage_containers, container_models);
