diesel::table! {
    asset_compatibility_rules(left_asset_model, right_asset_model) { left_asset_model ->
    diesel::sql_types::Integer, right_asset_model -> diesel::sql_types::Integer,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
