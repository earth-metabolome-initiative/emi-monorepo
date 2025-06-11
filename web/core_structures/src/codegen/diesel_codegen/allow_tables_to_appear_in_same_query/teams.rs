use crate::codegen::diesel_codegen::tables::{colors::colors, teams::teams};
diesel::allow_tables_to_appear_in_same_query!(teams, colors);
use crate::codegen::diesel_codegen::tables::team_states::team_states;
diesel::allow_tables_to_appear_in_same_query!(teams, team_states);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(teams, users);
