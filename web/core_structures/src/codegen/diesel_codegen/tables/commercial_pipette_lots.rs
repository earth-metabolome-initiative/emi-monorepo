diesel::table! {
    commercial_pipette_lots(id) { id -> diesel::sql_types::Integer, product_model ->
    diesel::sql_types::Integer }
}
