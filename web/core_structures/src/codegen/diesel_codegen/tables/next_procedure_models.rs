diesel::table! {
    next_procedure_models(parent_id, current_id, successor_id) { parent_id ->
    diesel::sql_types::Integer, current_id -> diesel::sql_types::Integer, successor_id ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
