diesel::table! {
    #[sql_name = "Aliquoting_Data"] public.aliquoting_data(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, sample_container ->
    diesel::sql_types::Integer, uuid_aliquot -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, aliquot_volume -> diesel::sql_types::Float,
    aliquot_volume_unit -> diesel::sql_types::Integer, parent_container ->
    diesel::sql_types::Integer, parent_sample_container -> diesel::sql_types::Integer }
}
