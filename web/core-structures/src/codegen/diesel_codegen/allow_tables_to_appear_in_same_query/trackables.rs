use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(trackables, container_models);
use crate::codegen::diesel_codegen::tables::trackable_states::trackable_states;
diesel::allow_tables_to_appear_in_same_query!(trackables, trackable_states);
use crate::codegen::diesel_codegen::tables::projects::projects;
diesel::allow_tables_to_appear_in_same_query!(trackables, projects);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(trackables, users);
