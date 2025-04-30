diesel::table! {
    public.cities(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, iso -> iso_codes::country_codes::diesel_impls::CountryCode }
}
