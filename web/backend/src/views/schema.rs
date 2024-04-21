diesel::table! {
    public_user (id) {
        id -> diesel::sql_types::Integer,
        first_name -> diesel::sql_types::Text,
        middle_name -> diesel::sql_types::Text,
        last_name -> diesel::sql_types::Text,
        created_at -> diesel::sql_types::Timestamp,
        updated_at -> diesel::sql_types::Timestamp,
    }
}

