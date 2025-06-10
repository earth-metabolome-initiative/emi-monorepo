use crate::codegen::diesel_codegen::tables::projects::projects;
use crate::codegen::diesel_codegen::tables::team_projects::team_projects;
diesel::allow_tables_to_appear_in_same_query!(team_projects, projects);
use crate::codegen::diesel_codegen::tables::teams::teams;
diesel::allow_tables_to_appear_in_same_query!(team_projects, teams);
