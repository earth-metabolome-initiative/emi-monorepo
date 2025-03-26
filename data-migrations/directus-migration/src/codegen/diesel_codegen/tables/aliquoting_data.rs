diesel::table! {
    #[sql_name = "Aliquoting_Data"] public.aliquoting_data(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, sample_container ->
    diesel::sql_types::Integer, uuid_aliquot -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, aliquot_volume -> diesel::sql_types::Float,
    aliquot_volume_unit -> diesel::sql_types::Integer, parent_container ->
    diesel::sql_types::Integer, parent_sample_container -> diesel::sql_types::Integer }
}
