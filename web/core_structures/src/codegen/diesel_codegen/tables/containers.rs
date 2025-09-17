diesel::table! {
    containers(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, container_model ->
    diesel::sql_types::Integer }
}
