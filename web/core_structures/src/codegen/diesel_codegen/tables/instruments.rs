diesel::table! {
    instruments(id) { id -> diesel::sql_types::Integer, instrument_model_id ->
    diesel::sql_types::Integer, instrument_state_id -> diesel::sql_types::SmallInt,
    qrcode -> ::rosetta_uuid::diesel_impls::Uuid, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
