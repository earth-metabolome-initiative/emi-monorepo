use crate::codegen::diesel_codegen::tables::{
    camera_models::camera_models, instrument_models::instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(camera_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(camera_models, trackables);
