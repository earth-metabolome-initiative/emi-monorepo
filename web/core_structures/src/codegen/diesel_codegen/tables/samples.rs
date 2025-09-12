diesel::table! {
    samples(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, model ->
    diesel::sql_types::Integer, sample_source -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, sample_source_model ->
    diesel::sql_types::Integer }
}
