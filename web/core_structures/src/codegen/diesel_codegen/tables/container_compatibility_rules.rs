diesel::table! {
    container_compatibility_rules(container_model_id, contained_asset_model) {
    container_model_id -> diesel::sql_types::Integer, contained_asset_model ->
    diesel::sql_types::Integer, quantity -> diesel::sql_types::Nullable <
    diesel::sql_types::SmallInt >, created_by -> diesel::sql_types::Integer, created_at
    -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
