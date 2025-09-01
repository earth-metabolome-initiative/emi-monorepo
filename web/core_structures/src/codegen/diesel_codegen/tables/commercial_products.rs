diesel::table! {
    commercial_products(id) { id -> diesel::sql_types::Integer, deprecation_date ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    brand_id -> diesel::sql_types::Integer }
}
