diesel::table! {
    commercial_product_lots(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, lot ->
    diesel::sql_types::Text, product_model_id -> ::rosetta_uuid::diesel_impls::Uuid }
}
