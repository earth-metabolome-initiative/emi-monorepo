diesel::table! {
    documents(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, mime_type ->
    ::media_types::diesel_impls::MediaType, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
