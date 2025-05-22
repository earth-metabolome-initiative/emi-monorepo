diesel::table! {
    ranks(id) { name -> diesel::sql_types::Text, description -> diesel::sql_types::Text,
    id -> diesel::sql_types::SmallInt }
}
