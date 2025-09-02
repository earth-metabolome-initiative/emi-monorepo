diesel::table! {
    commercial_product_lots(id) { id -> diesel::sql_types::Integer, lot ->
    diesel::sql_types::Text, product_model -> diesel::sql_types::Integer }
}
