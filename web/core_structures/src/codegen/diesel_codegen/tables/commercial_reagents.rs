diesel::table! {
    commercial_reagents(id) { id -> ::rosetta_uuid::diesel_impls::Uuid,
    commercial_product_lot_id -> diesel::sql_types::Integer }
}
