diesel::table! {
    volumetric_container_models(id) { id -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float }
}
