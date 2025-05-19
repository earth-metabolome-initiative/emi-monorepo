diesel::table! {
    steps(id) { id -> rosetta_uuid::diesel_impls::Uuid, procedure_id ->
    diesel::sql_types::Integer, step_model_id -> diesel::sql_types::Integer, begun_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, finished_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
