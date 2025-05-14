diesel::table! {
    container_models(id) { id -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float, container_category ->
    container_categories::diesel_impls::ContainerCategory, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
