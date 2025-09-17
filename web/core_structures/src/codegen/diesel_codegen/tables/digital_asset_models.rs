diesel::table! {
    digital_asset_models(id) { id -> diesel::sql_types::Integer, parent_model ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, mime_type ->
    ::media_types::diesel_impls::MediaType }
}
