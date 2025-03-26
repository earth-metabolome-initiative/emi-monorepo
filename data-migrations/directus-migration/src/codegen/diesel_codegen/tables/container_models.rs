diesel::table! {
    #[sql_name = "Container_Models"] public.container_models(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, container_type ->
    diesel::sql_types::Integer, volume -> diesel::sql_types::Float, volume_unit ->
    diesel::sql_types::Integer, brand -> diesel::sql_types::Integer, is_sample_container
    -> diesel::sql_types::Bool, #[sql_name = "columns"] __columns ->
    diesel::sql_types::Integer, columns_numeric -> diesel::sql_types::Bool, rows ->
    diesel::sql_types::Integer, rows_numeric -> diesel::sql_types::Bool }
}
