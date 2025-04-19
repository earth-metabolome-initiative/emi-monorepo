diesel::table! {
    public.nameplate_models(id) { id -> diesel::sql_types::Integer, nameplate_category_id
    -> diesel::sql_types::SmallInt, created_by -> diesel::sql_types::Integer, created_at
    -> rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
