use crate::codegen::diesel_codegen::tables::{sampling_steps::sampling_steps, users::users};
diesel::allow_tables_to_appear_in_same_query!(sampling_steps, users);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(sampling_steps, steps);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(sampling_steps, processables);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(sampling_steps, trackables);
