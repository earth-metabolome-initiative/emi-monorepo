diesel::table! {
    public.packaging_step_models(id) { id -> diesel::sql_types::Integer,
    packaging_model_id -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, created_by ->
    diesel::sql_types::Integer, updated_by -> diesel::sql_types::Integer }
}
