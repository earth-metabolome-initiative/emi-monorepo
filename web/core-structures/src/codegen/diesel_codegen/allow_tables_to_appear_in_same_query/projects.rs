use crate::codegen::diesel_codegen::tables::{colors::colors, projects::projects};
diesel::allow_tables_to_appear_in_same_query!(projects, colors);
use crate::codegen::diesel_codegen::tables::project_states::project_states;
diesel::allow_tables_to_appear_in_same_query!(projects, project_states);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(projects, users);
