use crate::codegen::diesel_codegen::tables::{
    fractioning_steps::fractioning_steps, processables::processables,
};
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, processables);
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, instruments);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, steps);
use crate::codegen::diesel_codegen::tables::fractioning_step_models::fractioning_step_models;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, fractioning_step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, users);
