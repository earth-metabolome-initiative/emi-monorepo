diesel::table! {
    sample_models(id) { id -> diesel::sql_types::Integer, sample_source_model ->
    diesel::sql_types::Integer }
}
