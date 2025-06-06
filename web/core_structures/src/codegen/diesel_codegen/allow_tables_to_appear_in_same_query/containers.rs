use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, containers::containers,
};
diesel::allow_tables_to_appear_in_same_query!(containers, container_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(containers, trackables);
