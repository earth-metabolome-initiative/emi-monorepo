diesel::table! {
    public.addresses(id) { id -> diesel::sql_types::Integer, iso ->
    diesel::sql_types::Text, city_code -> diesel::sql_types::Text, street_name ->
    diesel::sql_types::Text, street_number -> diesel::sql_types::Text, postal_code ->
    diesel::sql_types::Text, geolocation -> postgis_diesel::sql_types::Geography }
}
