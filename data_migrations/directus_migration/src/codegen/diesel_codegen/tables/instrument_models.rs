diesel::table! {
    #[sql_name = "Instrument_Models"] public.instrument_models(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, date_created -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, user_updated ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_updated ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    instrument_type -> diesel::sql_types::Integer, instrument_model ->
    diesel::sql_types::Text, brand -> diesel::sql_types::Integer, barcode ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
