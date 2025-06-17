use crate::codegen::diesel_codegen::tables::{
    centrifugable_container_models::centrifugable_container_models,
    centrifuge_models::centrifuge_models,
};
diesel::allow_tables_to_appear_in_same_query!(centrifugable_container_models, centrifuge_models);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(centrifugable_container_models, container_models);
