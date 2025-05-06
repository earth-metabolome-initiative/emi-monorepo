use crate::codegen::diesel_codegen::tables::{
    instruments::instruments, processing_steps::processing_steps,
};
diesel::allow_tables_to_appear_in_same_query!(processing_steps, instruments);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(processing_steps, users);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(processing_steps, processables);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(processing_steps, steps);
