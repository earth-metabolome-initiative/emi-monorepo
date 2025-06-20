diesel::table! {
    compatibility_rules(left_trackable_id, right_trackable_id) { left_trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, right_trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, quantity -> diesel::sql_types::Nullable <
    diesel::sql_types::SmallInt >, created_by -> diesel::sql_types::Integer, created_at
    -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
