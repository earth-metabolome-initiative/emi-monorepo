use crate::codegen::diesel_codegen::tables::colors::colors;
use crate::codegen::diesel_codegen::tables::team_states::team_states;
diesel::allow_tables_to_appear_in_same_query!(team_states, colors);
