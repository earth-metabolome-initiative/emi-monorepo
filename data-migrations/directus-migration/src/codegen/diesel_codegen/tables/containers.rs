#[cfg(feature = "32-column-tables")]
diesel::table! {
    #[sql_name = "Containers"] public.containers(id) { id -> diesel::sql_types::Integer,
    status -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, used ->
    diesel::sql_types::Bool, reserved -> diesel::sql_types::Bool, uuid_container ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, container_id ->
    diesel::sql_types::Text, container_model -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, is_finite -> diesel::sql_types::Nullable <
    diesel::sql_types::Bool >, #[sql_name = "columns"] __columns ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, rows ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, rows_numeric ->
    diesel::sql_types::Nullable < diesel::sql_types::Bool >, columns_numeric ->
    diesel::sql_types::Nullable < diesel::sql_types::Bool >, location ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, old_id ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, parent_container ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer > }
}
