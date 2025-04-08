diesel::table! {
    #[sql_name = "MS_Data"] public.ms_data(id) { id -> diesel::sql_types::Integer, status
    -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, uuid_ms_file ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, status_comment ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, filename ->
    diesel::sql_types::Text, injection_volume -> diesel::sql_types::Integer,
    injection_volume_unit -> diesel::sql_types::Integer, injection_method ->
    diesel::sql_types::Integer, instrument_used -> diesel::sql_types::Integer, batch ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, parent_sample_container
    -> diesel::sql_types::Integer, converted -> diesel::sql_types::Nullable <
    diesel::sql_types::Bool >, processed -> diesel::sql_types::Nullable <
    diesel::sql_types::Bool > }
}
