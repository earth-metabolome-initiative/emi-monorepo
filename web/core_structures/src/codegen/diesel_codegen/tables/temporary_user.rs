diesel::table! {
    temporary_user(id) { id -> diesel::sql_types::Integer, email ->
    diesel::sql_types::Text, login_provider_id -> diesel::sql_types::SmallInt }
}
