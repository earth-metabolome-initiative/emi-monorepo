use crate::codegen::diesel_codegen::tables::{
    storage_containers::storage_containers, trackable_locations::trackable_locations,
};
diesel::allow_tables_to_appear_in_same_query!(trackable_locations, storage_containers);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(trackable_locations, users);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(trackable_locations, trackables);
