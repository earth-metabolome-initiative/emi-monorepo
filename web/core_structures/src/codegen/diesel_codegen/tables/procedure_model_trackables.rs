diesel::table! {
    procedure_model_trackables(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, optional -> diesel::sql_types::Bool, procedure_model_id ->
    diesel::sql_types::Integer, trackable_id -> ::rosetta_uuid::diesel_impls::Uuid,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
