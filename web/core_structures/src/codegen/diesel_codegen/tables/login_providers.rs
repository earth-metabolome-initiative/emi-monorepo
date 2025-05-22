diesel::table! {
    login_providers(id) { id -> diesel::sql_types::SmallInt, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, client_id ->
    diesel::sql_types::Text, redirect_uri -> diesel::sql_types::Text, oauth_url ->
    diesel::sql_types::Text, scope -> diesel::sql_types::Text }
}
