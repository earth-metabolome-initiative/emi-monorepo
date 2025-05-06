use crate::codegen::diesel_codegen::tables::{teams::teams, users::users};
diesel::allow_tables_to_appear_in_same_query!(teams, users);
use crate::codegen::diesel_codegen::tables::team_states::team_states;
diesel::allow_tables_to_appear_in_same_query!(teams, team_states);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(teams, colors);
