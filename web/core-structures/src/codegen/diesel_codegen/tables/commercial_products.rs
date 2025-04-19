diesel::table! {
    public.commercial_products(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, photograph_id ->
    rosetta_uuid::diesel_impls::Uuid, deprecation_date -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, brand_id ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
