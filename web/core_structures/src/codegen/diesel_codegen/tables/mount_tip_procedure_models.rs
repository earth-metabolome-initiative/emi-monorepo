diesel::table! {
    mount_tip_procedure_models(id) { id -> diesel::sql_types::Integer, pipette ->
    diesel::sql_types::Integer, pipette_tip -> diesel::sql_types::Integer }
}
