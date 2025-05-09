use crate::codegen::diesel_codegen::tables::{
    step_container_models::step_container_models, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(step_container_models, users);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(step_container_models, steps);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(step_container_models, container_models);
