use crate::codegen::diesel_codegen::tables::{
    trackables::trackables, weighing_device_models::weighing_device_models,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_device_models, trackables);
