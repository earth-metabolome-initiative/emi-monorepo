use crate::codegen::diesel_codegen::tables::{
    trackable_locations::trackable_locations, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(trackable_locations, trackables);
use crate::codegen::diesel_codegen::tables::storage_containers::storage_containers;
diesel::allow_tables_to_appear_in_same_query!(trackable_locations, storage_containers);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(trackable_locations, users);
