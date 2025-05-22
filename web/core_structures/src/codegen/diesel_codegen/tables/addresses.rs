diesel::table! {
    addresses(id) { id -> diesel::sql_types::Integer, city_id ->
    diesel::sql_types::Integer, street_name -> diesel::sql_types::Text, street_number ->
    diesel::sql_types::Text, postal_code -> diesel::sql_types::Text, geolocation ->
    ::postgis_diesel::sql_types::Geography }
}
