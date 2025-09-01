diesel::table! {
    next_procedure_templates(parent, current, successor_id) { parent ->
    diesel::sql_types::Integer, current -> diesel::sql_types::Integer, successor_id ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
