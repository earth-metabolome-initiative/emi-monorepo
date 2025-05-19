diesel::table! {
    instrument_locations(id) { id -> diesel::sql_types::Integer, instrument_id ->
    diesel::sql_types::Integer, room_id -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, created_by ->
    diesel::sql_types::Integer }
}
