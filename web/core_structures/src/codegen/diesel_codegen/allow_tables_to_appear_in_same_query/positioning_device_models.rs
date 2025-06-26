use crate::codegen::diesel_codegen::tables::{
    instrument_models::instrument_models, positioning_device_models::positioning_device_models,
};
diesel::allow_tables_to_appear_in_same_query!(positioning_device_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(positioning_device_models, trackables);
