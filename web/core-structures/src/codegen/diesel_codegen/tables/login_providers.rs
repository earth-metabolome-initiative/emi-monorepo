diesel::table! {
    public.login_providers(id) { id -> diesel::sql_types::SmallInt, name ->
    diesel::sql_types::Text, icon_id -> diesel::sql_types::SmallInt, color_id ->
    diesel::sql_types::SmallInt, client_id -> diesel::sql_types::Text, redirect_uri ->
    diesel::sql_types::Text, oauth_url -> diesel::sql_types::Text, scope ->
    diesel::sql_types::Text }
}
