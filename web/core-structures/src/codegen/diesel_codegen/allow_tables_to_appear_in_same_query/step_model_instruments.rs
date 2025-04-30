use crate::codegen::diesel_codegen::tables::{
    step_model_instruments::step_model_instruments, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(step_model_instruments, users);
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(step_model_instruments, instruments);
use crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models;
diesel::allow_tables_to_appear_in_same_query!(step_model_instruments, step_model_instrument_models);
