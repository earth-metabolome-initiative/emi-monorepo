use crate::codegen::diesel_codegen::tables::{
    instrument_models::instrument_models, instruments::instruments,
};
diesel::allow_tables_to_appear_in_same_query!(instruments, instrument_models);
use crate::codegen::diesel_codegen::tables::instrument_states::instrument_states;
diesel::allow_tables_to_appear_in_same_query!(instruments, instrument_states);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(instruments, users);
