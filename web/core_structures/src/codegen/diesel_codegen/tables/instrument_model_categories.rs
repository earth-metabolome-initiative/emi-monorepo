diesel::table! {
    instrument_model_categories(id) { id -> diesel::sql_types::Integer,
    instrument_model_id -> diesel::sql_types::Integer, instrument_category ->
    instrument_categories::diesel_impls::InstrumentCategory, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
