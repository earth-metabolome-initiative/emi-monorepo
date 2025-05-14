diesel::table! {
    photographs(id) { id -> rosetta_uuid::diesel_impls::Uuid, path ->
    diesel::sql_types::Text, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
