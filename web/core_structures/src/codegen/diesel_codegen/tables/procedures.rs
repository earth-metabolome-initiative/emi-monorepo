diesel::table! {
    procedures(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, procedure_model_id ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
