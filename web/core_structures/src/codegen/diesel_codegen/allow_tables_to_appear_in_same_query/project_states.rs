use crate::codegen::diesel_codegen::tables::{colors::colors, project_states::project_states};
diesel::allow_tables_to_appear_in_same_query!(project_states, colors);
