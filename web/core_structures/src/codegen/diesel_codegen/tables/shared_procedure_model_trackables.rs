diesel::table! {
    shared_procedure_model_trackables(parent_id, child_id) { parent_id ->
    diesel::sql_types::Integer, child_id -> diesel::sql_types::Integer, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
