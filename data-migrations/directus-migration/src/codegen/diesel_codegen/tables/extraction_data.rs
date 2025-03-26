#[cfg(feature = "32-column-tables")]
diesel::table! {
    #[sql_name = "Extraction_Data"] public.extraction_data(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, dried_weight -> diesel::sql_types::Float,
    dried_weight_unit -> diesel::sql_types::Integer, extraction_method ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, batch ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, solvent_volume ->
    diesel::sql_types::Nullable < diesel::sql_types::Float >, solvent_volume_unit ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, uuid_extraction ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, sample_container ->
    diesel::sql_types::Integer, parent_container -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, parent_sample_container -> diesel::sql_types::Integer,
    extraction_container -> diesel::sql_types::Nullable < diesel::sql_types::Integer > }
}
