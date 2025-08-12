use crate::codegen::diesel_codegen::tables::{
    ball_mill_machine_models::ball_mill_machine_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(ball_mill_machine_models, trackables);
