diesel::table! {
    public.organizations(id) { name -> diesel::sql_types::Text, url ->
    diesel::sql_types::Text, country -> diesel::sql_types::Text, alpha_two_code ->
    iso_codes::country_codes::diesel_impls::CountryCode, state_province ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, domain ->
    diesel::sql_types::Text, id -> diesel::sql_types::SmallInt }
}
