diesel::table! {
    public.users(id) { id -> diesel::sql_types::Integer, first_name ->
    diesel::sql_types::Text, last_name -> diesel::sql_types::Text, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
