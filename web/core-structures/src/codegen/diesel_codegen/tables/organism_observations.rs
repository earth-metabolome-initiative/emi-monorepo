diesel::table! {
    public.organism_observations(id) { id -> rosetta_uuid::diesel_impls::Uuid, wild ->
    diesel::sql_types::Bool, project_id -> diesel::sql_types::Integer, organism_id ->
    rosetta_uuid::diesel_impls::Uuid, subject_id -> diesel::sql_types::SmallInt, picture
    -> diesel::sql_types::Binary, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
