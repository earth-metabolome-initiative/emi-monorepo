diesel::table! {
    procedure_trackables(procedure_id, trackable_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
