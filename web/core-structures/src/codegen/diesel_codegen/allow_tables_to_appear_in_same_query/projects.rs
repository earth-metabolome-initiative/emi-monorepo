use crate::codegen::diesel_codegen::tables::{projects::projects, users::users};
diesel::allow_tables_to_appear_in_same_query!(projects, users);
use crate::codegen::diesel_codegen::tables::project_states::project_states;
diesel::allow_tables_to_appear_in_same_query!(projects, project_states);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(projects, colors);
