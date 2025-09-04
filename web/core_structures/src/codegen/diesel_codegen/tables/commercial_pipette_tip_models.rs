diesel::table! {
    commercial_pipette_tip_models(id) { id -> diesel::sql_types::Integer,
    pipette_tip_model -> diesel::sql_types::Integer }
}
