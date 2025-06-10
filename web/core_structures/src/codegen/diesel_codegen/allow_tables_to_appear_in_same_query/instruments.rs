use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(instruments, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(instruments, trackables);
