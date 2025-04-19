diesel::table! {
    public.email_providers(email_id, login_provider_id) { email_id ->
    diesel::sql_types::Integer, login_provider_id -> diesel::sql_types::SmallInt }
}
