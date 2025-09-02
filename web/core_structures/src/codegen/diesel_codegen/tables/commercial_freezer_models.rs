diesel::table! {
    commercial_freezer_models(id) { id -> diesel::sql_types::Integer, parent_model ->
    diesel::sql_types::Integer }
}
