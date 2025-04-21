use crate::codegen::diesel_codegen::tables::{
    weighing_step_models::weighing_step_models, weighing_steps::weighing_steps,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_steps, weighing_step_models);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(weighing_steps, steps);
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(weighing_steps, instruments);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(weighing_steps, users);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(weighing_steps, processables);
