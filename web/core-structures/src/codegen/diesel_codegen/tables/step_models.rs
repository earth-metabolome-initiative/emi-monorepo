diesel::table! {
    public.step_models(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, snoozable ->
    diesel::sql_types::Bool, copiable -> diesel::sql_types::Nullable <
    diesel::sql_types::Bool >, photograph_id -> rosetta_uuid::diesel_impls::Uuid,
    step_model_category_id -> diesel::sql_types::SmallInt, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
