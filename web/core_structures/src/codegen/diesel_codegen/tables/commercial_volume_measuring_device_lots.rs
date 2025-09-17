diesel::table! {
    commercial_volume_measuring_device_lots(id) { id -> diesel::sql_types::Integer,
    product_model -> diesel::sql_types::Integer }
}
