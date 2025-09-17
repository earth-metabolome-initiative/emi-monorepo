diesel::table! {
    #[sql_name = "Projects"] projects(id) { id -> diesel::sql_types::Integer, status ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, uuid_project ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, project_id ->
    diesel::sql_types::Text, project_description -> diesel::sql_types::Text,
    parent_project -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, batch
    -> diesel::sql_types::Integer }
}
