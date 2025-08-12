use crate::codegen::diesel_codegen::tables::{
    camera_models::camera_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(camera_models, trackables);
