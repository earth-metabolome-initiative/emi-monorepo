diesel::table! {
    #[sql_name = "Test_Connection"] test_connection(id) { id ->
    diesel::sql_types::Integer }
}
