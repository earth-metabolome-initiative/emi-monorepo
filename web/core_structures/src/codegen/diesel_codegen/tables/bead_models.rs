diesel::table! {
    bead_models(id) { id -> diesel::sql_types::Integer, diameter_millimeters ->
    diesel::sql_types::Float }
}
