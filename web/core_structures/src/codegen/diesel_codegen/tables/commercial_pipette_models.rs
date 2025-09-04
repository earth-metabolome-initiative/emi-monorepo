diesel::table! {
    commercial_pipette_models(id) { id -> diesel::sql_types::Integer, pipette_model ->
    diesel::sql_types::Integer }
}
