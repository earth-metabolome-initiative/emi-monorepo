diesel::table! {
    public.commercial_product_lots(id) { id -> diesel::sql_types::Integer, lot ->
    diesel::sql_types::Text, product_model_id -> diesel::sql_types::Integer }
}
