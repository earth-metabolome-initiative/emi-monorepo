diesel::table! {
    parent_procedure_models(parent_procedure_model_id, child_procedure_model_id) {
    parent_procedure_model_id -> diesel::sql_types::Integer, child_procedure_model_id ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
