use crate::codegen::diesel_codegen::tables::{colors::colors, trackable_states::trackable_states};
diesel::allow_tables_to_appear_in_same_query!(trackable_states, colors);
