diesel::table! {
    spectra(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, spectra_collection_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
