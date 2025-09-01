diesel::table! {
    physical_asset_models(id) { id -> diesel::sql_types::Integer, parent_model_id ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer > }
}
