use crate::codegen::diesel_codegen::tables::{step_instruments::step_instruments, users::users};
diesel::allow_tables_to_appear_in_same_query!(step_instruments, users);
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(step_instruments, instruments);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(step_instruments, steps);
