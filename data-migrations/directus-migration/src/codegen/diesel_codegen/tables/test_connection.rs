diesel::table! {
    #[sql_name = "Test_Connection"] public.test_connection(id) { id ->
    diesel::sql_types::Integer }
}
