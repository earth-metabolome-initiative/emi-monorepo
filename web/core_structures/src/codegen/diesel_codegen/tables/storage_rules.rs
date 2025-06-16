diesel::table! {
    storage_rules(parent_container_id, child_container_id) { parent_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, child_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, quantity -> diesel::sql_types::SmallInt }
}
