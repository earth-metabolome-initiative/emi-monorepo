diesel::table! {
    public.rooms(id) { id -> diesel::sql_types::Integer, name -> diesel::sql_types::Text,
    description -> diesel::sql_types::Text, qrcode -> rosetta_uuid::diesel_impls::Uuid,
    addresses_id -> diesel::sql_types::Integer, geolocation ->
    postgis_diesel::sql_types::Geography, created_by -> diesel::sql_types::Integer,
    created_at -> rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
