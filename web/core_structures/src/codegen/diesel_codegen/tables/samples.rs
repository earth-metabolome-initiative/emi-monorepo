diesel::table! {
    samples(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, model ->
    diesel::sql_types::Integer, sample_source -> ::rosetta_uuid::diesel_impls::Uuid }
}
