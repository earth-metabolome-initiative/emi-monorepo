diesel::table! {
    packaging_models(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, kilograms ->
    diesel::sql_types::Float }
}
