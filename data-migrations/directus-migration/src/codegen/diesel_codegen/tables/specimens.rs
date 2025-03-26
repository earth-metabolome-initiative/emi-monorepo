diesel::table! {
    #[sql_name = "Specimens"] public.specimens(id) { id -> diesel::sql_types::Integer,
    status -> diesel::sql_types::Text, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz > }
}
