diesel::table! {
    public.directus_operations(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, key ->
    diesel::sql_types::Text, #[sql_name = "type"] r#type -> diesel::sql_types::Text,
    position_x -> diesel::sql_types::Integer, position_y -> diesel::sql_types::Integer,
    options -> diesel::sql_types::Nullable < diesel::sql_types::Json >, resolve ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, reject ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, flow ->
    diesel::sql_types::Uuid, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid > }
}
