diesel::table! {
    #[sql_name = "Universities"] public.universities(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, uuid_university -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, university_name -> diesel::sql_types::Text, country ->
    diesel::sql_types::Text, alpha_two -> diesel::sql_types::Text, web_pages ->
    diesel::sql_types::Text, state -> diesel::sql_types::Text, domains ->
    diesel::sql_types::Text }
}
