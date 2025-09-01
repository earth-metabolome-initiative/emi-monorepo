diesel::table! {
    asset_model_ancestors(descendant_model, ancestor_model) { descendant_model ->
    diesel::sql_types::Integer, ancestor_model -> diesel::sql_types::Integer }
}
