use crate::codegen::diesel_codegen::tables::{
    ball_mill_container_models::ball_mill_container_models,
    ball_mill_machine_models::ball_mill_machine_models,
};
diesel::allow_tables_to_appear_in_same_query!(ball_mill_container_models, ball_mill_machine_models);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(ball_mill_container_models, container_models);
