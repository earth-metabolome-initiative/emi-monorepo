diesel::define_sql_function! {
    fn pg_get_function_result(
        oid: diesel::sql_types::Oid,
    ) -> diesel::sql_types::Text
}

diesel::define_sql_function! {
    fn pg_get_function_arguments(
        oid: diesel::sql_types::Oid,
    ) -> diesel::sql_types::Text
}
