diesel::table! {
    shared_procedure_model_trackables(parent_id, child_id) { parent_id ->
    diesel::sql_types::Integer, parent_trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, parent_procedure_model_id ->
    diesel::sql_types::Integer, child_id -> diesel::sql_types::Integer,
    child_trackable_id -> ::rosetta_uuid::diesel_impls::Uuid, child_procedure_model_id ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
