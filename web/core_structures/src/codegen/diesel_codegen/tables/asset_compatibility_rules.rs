diesel::table! {
    asset_compatibility_rules(left_asset_model_id, right_asset_model_id) { left_asset_model_id ->
    diesel::sql_types::Integer, right_asset_model_id -> diesel::sql_types::Integer,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
