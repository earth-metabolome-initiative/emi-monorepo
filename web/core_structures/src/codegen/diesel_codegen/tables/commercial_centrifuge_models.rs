diesel::table! {
    commercial_centrifuge_models(id) { id -> diesel::sql_types::Integer, parent_model ->
    diesel::sql_types::Integer }
}
