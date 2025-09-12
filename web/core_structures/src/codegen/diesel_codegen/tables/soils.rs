diesel::table! {
    soils(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, model ->
    diesel::sql_types::Integer }
}
