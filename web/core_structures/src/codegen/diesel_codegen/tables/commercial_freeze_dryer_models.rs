diesel::table! {
    commercial_freeze_dryer_models(id) { id -> diesel::sql_types::Integer, parent_model
    -> diesel::sql_types::Integer }
}
