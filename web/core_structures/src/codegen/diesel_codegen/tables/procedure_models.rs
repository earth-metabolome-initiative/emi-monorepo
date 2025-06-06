diesel::table! {
    procedure_models(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, deprecated ->
    diesel::sql_types::Bool, photograph_id -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, icon -> diesel::sql_types::Text, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
