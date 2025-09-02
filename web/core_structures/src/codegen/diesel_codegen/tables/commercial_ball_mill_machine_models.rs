diesel::table! {
    commercial_ball_mill_machine_models(id) { id -> diesel::sql_types::Integer,
    parent_model -> diesel::sql_types::Integer }
}
