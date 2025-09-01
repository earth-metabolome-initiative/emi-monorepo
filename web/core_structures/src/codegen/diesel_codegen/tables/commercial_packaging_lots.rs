diesel::table! {
    commercial_packaging_lots(id) { id -> diesel::sql_types::Integer, product_model_id ->
    diesel::sql_types::Integer }
}
