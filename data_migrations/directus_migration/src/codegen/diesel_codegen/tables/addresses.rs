diesel::table! {
    #[sql_name = "Addresses"] addresses(id) { id -> diesel::sql_types::Integer, country
    -> diesel::sql_types::Text, city -> diesel::sql_types::Text, street ->
    diesel::sql_types::Text, street_number -> diesel::sql_types::Text, postal_code ->
    diesel::sql_types::Text, geolocation -> ::postgis_diesel::sql_types::Geometry,
    city_code -> diesel::sql_types::Text }
}
