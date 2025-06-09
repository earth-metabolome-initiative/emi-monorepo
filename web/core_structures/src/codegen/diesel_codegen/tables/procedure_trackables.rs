diesel::table! {
    procedure_trackables(procedure_id, trackable_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_model_id -> diesel::sql_types::Integer,
    trackable_id -> ::rosetta_uuid::diesel_impls::Uuid, procedure_model_trackable_id ->
    diesel::sql_types::Integer, ancestor_trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, parent_trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
