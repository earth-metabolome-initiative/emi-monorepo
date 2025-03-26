diesel::table! {
    #[sql_name = "Instrument_Models"] public.instrument_models(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, instrument_type -> diesel::sql_types::Integer,
    instrument_model -> diesel::sql_types::Text, brand -> diesel::sql_types::Integer }
}
