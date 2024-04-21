diesel::table! {
    formats_view (id) {
        id -> diesel::sql_types::Uuid,
        mime_type -> diesel::sql_types::Text,
        extension -> diesel::sql_types::Text,
        format_description -> diesel::sql_types::Text,
    }
}

diesel::table! {
    public_user (id) {
        id -> diesel::sql_types::Uuid,
        first_name -> diesel::sql_types::Text,
        middle_name -> diesel::sql_types::Text,
        last_name -> diesel::sql_types::Text,
        created_at -> diesel::sql_types::Timestamp,
        updated_at -> diesel::sql_types::Timestamp,
    }
}

