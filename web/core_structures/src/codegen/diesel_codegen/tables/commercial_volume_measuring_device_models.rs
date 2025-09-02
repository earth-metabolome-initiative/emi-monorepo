diesel::table! {
    commercial_volume_measuring_device_models(id) { id -> diesel::sql_types::Integer,
    parent_model -> diesel::sql_types::Integer }
}
