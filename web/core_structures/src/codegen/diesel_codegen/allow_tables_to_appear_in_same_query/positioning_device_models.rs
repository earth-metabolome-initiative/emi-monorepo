use crate::codegen::diesel_codegen::tables::{
    positioning_device_models::positioning_device_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(positioning_device_models, trackables);
