diesel::table! {
    #[sql_name = "Rooms"] public.rooms(id) { id -> diesel::sql_types::Integer, status ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, building ->
    diesel::sql_types::Integer, room_name -> diesel::sql_types::Text, comment ->
    diesel::sql_types::Text, address -> diesel::sql_types::Integer, geolocation ->
    postgis_diesel::sql_types::Geometry, qr_code -> rosetta_uuid::diesel_impls::Uuid }
}
