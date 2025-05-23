diesel::table! {
    step_models(id) { id -> diesel::sql_types::Integer, procedure_model_id ->
    diesel::sql_types::Integer, name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, snoozable -> diesel::sql_types::Bool, copiable ->
    diesel::sql_types::Bool, photograph_id -> ::rosetta_uuid::diesel_impls::Uuid, icon ->
    diesel::sql_types::Text, step_model_category ->
    ::step_model_categories::diesel_impls::StepModelCategory, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
