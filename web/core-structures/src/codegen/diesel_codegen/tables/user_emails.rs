diesel::table! {
    user_emails(id) { id -> diesel::sql_types::Integer, email -> diesel::sql_types::Text,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, primary_email ->
    diesel::sql_types::Bool }
}
