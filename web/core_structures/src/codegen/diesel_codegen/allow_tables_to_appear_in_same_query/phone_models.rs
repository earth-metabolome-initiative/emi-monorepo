use crate::codegen::diesel_codegen::tables::{
    camera_models::camera_models, phone_models::phone_models,
};
diesel::allow_tables_to_appear_in_same_query!(phone_models, camera_models);
use crate::codegen::diesel_codegen::tables::positioning_device_models::positioning_device_models;
diesel::allow_tables_to_appear_in_same_query!(phone_models, positioning_device_models);
use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
diesel::allow_tables_to_appear_in_same_query!(phone_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(phone_models, trackables);
