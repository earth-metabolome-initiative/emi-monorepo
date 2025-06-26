use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(container_models, trackables);
