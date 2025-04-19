diesel::table! {
    public.procedure_step_models(id) { id -> diesel::sql_types::Integer,
    procedure_model_id -> diesel::sql_types::Integer, step_model_id ->
    diesel::sql_types::Integer, next_procedure_step_model_id ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >,
    prev_procedure_step_model_id -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
