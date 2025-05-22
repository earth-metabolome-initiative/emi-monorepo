diesel::table! {
    step_storage_containers(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, step_id ->
    ::rosetta_uuid::diesel_impls::Uuid, storage_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
