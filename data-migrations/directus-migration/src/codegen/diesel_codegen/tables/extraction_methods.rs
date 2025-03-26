diesel::table! {
    #[sql_name = "Extraction_Methods"] public.extraction_methods(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, method_name ->
    diesel::sql_types::Text, method_description -> diesel::sql_types::Text }
}
