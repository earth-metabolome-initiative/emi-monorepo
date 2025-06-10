use crate::codegen::diesel_codegen::tables::colors::colors;
use crate::codegen::diesel_codegen::tables::instrument_states::instrument_states;
diesel::allow_tables_to_appear_in_same_query!(instrument_states, colors);
