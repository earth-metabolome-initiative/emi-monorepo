diesel::table! {
    commercial_weighing_device_lots(id) { id -> diesel::sql_types::Integer, product_model
    -> diesel::sql_types::Integer }
}
