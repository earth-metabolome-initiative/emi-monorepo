diesel::table! {
    temporary_user_emails(id) { id -> rosetta_uuid::diesel_impls::Uuid, email ->
    diesel::sql_types::Text, login_provider_id -> diesel::sql_types::SmallInt, validated
    -> diesel::sql_types::Bool }
}
