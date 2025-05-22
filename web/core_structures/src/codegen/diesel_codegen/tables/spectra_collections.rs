diesel::table! {
    spectra_collections(id) { id -> diesel::sql_types::Integer, notes ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
