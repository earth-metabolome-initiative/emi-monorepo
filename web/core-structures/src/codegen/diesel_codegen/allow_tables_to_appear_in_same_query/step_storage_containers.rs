use crate::codegen::diesel_codegen::tables::{
    step_storage_containers::step_storage_containers, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(step_storage_containers, users);
use crate::codegen::diesel_codegen::tables::storage_containers::storage_containers;
diesel::allow_tables_to_appear_in_same_query!(step_storage_containers, storage_containers);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(step_storage_containers, steps);
