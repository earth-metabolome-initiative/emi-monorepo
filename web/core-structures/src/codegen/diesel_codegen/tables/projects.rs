diesel::table! {
    public.projects(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, state_id ->
    diesel::sql_types::SmallInt, icon_id -> diesel::sql_types::SmallInt, color_id ->
    diesel::sql_types::SmallInt, parent_project_id -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, budget -> diesel::sql_types::Nullable <
    diesel::sql_types::Double >, expenses -> diesel::sql_types::Nullable <
    diesel::sql_types::Double >, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, expected_end_date ->
    rosetta_timestamp::diesel_impls::TimestampUTC, end_date ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
