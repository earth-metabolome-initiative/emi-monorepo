diesel::table! {
    container_models(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, liters ->
    diesel::sql_types::Float, container_category ->
    ::container_categories::diesel_impls::ContainerCategory }
}
