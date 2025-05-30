use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, storage_containers::storage_containers,
};
diesel::allow_tables_to_appear_in_same_query!(storage_containers, container_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(storage_containers, users);
