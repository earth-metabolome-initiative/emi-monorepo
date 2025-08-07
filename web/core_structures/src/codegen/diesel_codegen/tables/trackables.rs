diesel::table! {
    trackables(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, description ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, photograph_id ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, parent_id ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
