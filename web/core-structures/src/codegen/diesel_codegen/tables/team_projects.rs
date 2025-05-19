diesel::table! {
    team_projects(team_id, project_id) { team_id -> diesel::sql_types::Integer,
    project_id -> diesel::sql_types::Integer }
}
