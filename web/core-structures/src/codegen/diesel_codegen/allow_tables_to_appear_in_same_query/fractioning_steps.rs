use crate::codegen::diesel_codegen::tables::{fractioning_steps::fractioning_steps, users::users};
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, users);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, processables);
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, instruments);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(fractioning_steps, steps);
