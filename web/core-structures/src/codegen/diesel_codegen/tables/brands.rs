diesel::table! {
    public.brands(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, brand_state_id ->
    diesel::sql_types::SmallInt }
}
