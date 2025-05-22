diesel::table! {
    shaking_steps(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, processable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC }
}
