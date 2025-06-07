diesel::table! {
    volumetric_processables(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, liters ->
    diesel::sql_types::Float }
}
