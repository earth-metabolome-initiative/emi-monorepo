diesel::table! {
    capping_rules(container_id, cap_id) { container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, cap_id -> ::rosetta_uuid::diesel_impls::Uuid }
}
