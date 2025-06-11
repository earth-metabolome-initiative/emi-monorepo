use crate::codegen::diesel_codegen::tables::{team_members::team_members, teams::teams};
diesel::allow_tables_to_appear_in_same_query!(team_members, teams);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(team_members, users);
use crate::codegen::diesel_codegen::tables::team_projects::team_projects;
diesel::allow_tables_to_appear_in_same_query!(team_members, team_projects);
