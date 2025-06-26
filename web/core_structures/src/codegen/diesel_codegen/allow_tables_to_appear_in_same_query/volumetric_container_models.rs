use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, volumetric_container_models::volumetric_container_models,
};
diesel::allow_tables_to_appear_in_same_query!(volumetric_container_models, container_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(volumetric_container_models, trackables);
