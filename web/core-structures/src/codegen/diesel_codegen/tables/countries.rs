diesel::table! {
    countries(iso) { iso -> iso_codes::country_codes::diesel_impls::CountryCode, name ->
    diesel::sql_types::Text }
}
