use crate::codegen::diesel_codegen::tables::{icons::icons, project_states::project_states};
diesel::allow_tables_to_appear_in_same_query!(project_states, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(project_states, colors);
