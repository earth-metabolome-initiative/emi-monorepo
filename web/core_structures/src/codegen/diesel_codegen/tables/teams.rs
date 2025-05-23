diesel::table! {
    teams(id) { id -> diesel::sql_types::Integer, name -> diesel::sql_types::Text,
    description -> diesel::sql_types::Text, icon -> diesel::sql_types::Text, color_id ->
    diesel::sql_types::SmallInt, state_id -> diesel::sql_types::SmallInt, parent_team_id
    -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
