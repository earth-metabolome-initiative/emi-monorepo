use crate::codegen::diesel_codegen::tables::{shaking_steps::shaking_steps, users::users};
diesel::allow_tables_to_appear_in_same_query!(shaking_steps, users);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(shaking_steps, steps);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(shaking_steps, processables);
