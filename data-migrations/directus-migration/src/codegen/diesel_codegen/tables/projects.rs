diesel::table! {
    #[sql_name = "Projects"] public.projects(id) { id -> diesel::sql_types::Integer,
    status -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, uuid_project ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, project_id ->
    diesel::sql_types::Text, project_description -> diesel::sql_types::Text,
    parent_project -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, batch
    -> diesel::sql_types::Integer }
}
