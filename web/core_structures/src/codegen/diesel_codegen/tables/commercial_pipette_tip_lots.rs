diesel::table! {
    commercial_pipette_tip_lots(id) { id -> diesel::sql_types::Integer, product_model_id
    -> diesel::sql_types::Integer }
}
