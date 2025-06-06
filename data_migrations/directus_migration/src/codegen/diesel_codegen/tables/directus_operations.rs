diesel::table! {
    directus_operations(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, key ->
    diesel::sql_types::Text, #[sql_name = "type"] r#type -> diesel::sql_types::Text,
    position_x -> diesel::sql_types::Integer, position_y -> diesel::sql_types::Integer,
    options -> diesel::sql_types::Nullable < diesel::sql_types::Json >, resolve ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, reject ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, flow ->
    ::rosetta_uuid::diesel_impls::Uuid, date_created -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid > }
}
