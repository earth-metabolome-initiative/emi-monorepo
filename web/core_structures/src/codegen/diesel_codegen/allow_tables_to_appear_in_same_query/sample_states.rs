use crate::codegen::diesel_codegen::tables::{colors::colors, sample_states::sample_states};
diesel::allow_tables_to_appear_in_same_query!(sample_states, colors);
