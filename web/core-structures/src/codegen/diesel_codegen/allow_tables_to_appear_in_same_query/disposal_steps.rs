use crate::codegen::diesel_codegen::tables::{
    disposal_steps::disposal_steps, processables::processables,
};
diesel::allow_tables_to_appear_in_same_query!(disposal_steps, processables);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(disposal_steps, steps);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(disposal_steps, users);
