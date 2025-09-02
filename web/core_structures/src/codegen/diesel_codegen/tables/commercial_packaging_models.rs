diesel::table! {
    commercial_packaging_models(id) { id -> diesel::sql_types::Integer, parent_model ->
    diesel::sql_types::Integer }
}
