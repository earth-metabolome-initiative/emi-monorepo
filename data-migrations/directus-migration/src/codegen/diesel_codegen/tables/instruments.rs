diesel::table! {
    #[sql_name = "Instruments"] public.instruments(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, uuid_instrument ->
    rosetta_uuid::diesel_impls::Uuid, instrument_id -> diesel::sql_types::Text,
    instrument_model -> diesel::sql_types::Integer, instrument_location ->
    diesel::sql_types::Integer, grams -> diesel::sql_types::Nullable <
    diesel::sql_types::Float > }
}
