diesel::table! {
    team_members(team_id, member_id) { team_id -> diesel::sql_types::Integer, member_id
    -> diesel::sql_types::Integer }
}
