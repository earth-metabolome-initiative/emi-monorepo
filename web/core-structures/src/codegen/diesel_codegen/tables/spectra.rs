diesel::table! {
    public.spectra(id) { id -> diesel::sql_types::Integer, spectra_collection_id ->
    diesel::sql_types::Integer }
}
