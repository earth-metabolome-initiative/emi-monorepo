diesel::table! {
    public.step_model_nameplate_categories(id) { id -> diesel::sql_types::Integer,
    step_model_id -> diesel::sql_types::Integer, nameplate_category_id ->
    diesel::sql_types::SmallInt, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
