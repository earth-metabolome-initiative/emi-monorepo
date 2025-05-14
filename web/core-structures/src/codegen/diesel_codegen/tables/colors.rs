diesel::table! {
    colors(id) { name -> diesel::sql_types::Text, hexadecimal_value ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, id ->
    diesel::sql_types::SmallInt }
}
