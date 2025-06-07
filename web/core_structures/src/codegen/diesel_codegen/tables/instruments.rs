diesel::table! {
    instruments(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, instrument_model_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
