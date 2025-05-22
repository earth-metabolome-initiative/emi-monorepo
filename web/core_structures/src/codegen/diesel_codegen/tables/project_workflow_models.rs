diesel::table! {
    project_workflow_models(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, created_by ->
    diesel::sql_types::Integer, updated_by -> diesel::sql_types::Integer }
}
