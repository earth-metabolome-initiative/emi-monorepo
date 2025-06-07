diesel::table! {
    #[sql_name = "Container_Models"] container_models(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, container_type ->
    diesel::sql_types::Integer, volume -> diesel::sql_types::Float, volume_unit ->
    diesel::sql_types::Integer, brand -> diesel::sql_types::Integer, is_sample_container
    -> diesel::sql_types::Bool, #[sql_name = "columns"] __columns ->
    diesel::sql_types::Integer, columns_numeric -> diesel::sql_types::Bool, rows ->
    diesel::sql_types::Integer, rows_numeric -> diesel::sql_types::Bool }
}
