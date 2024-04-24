diesel::table! {
    public_users (id) {
        id -> diesel::sql_types::Integer,
        first_name -> diesel::sql_types::Text,
        middle_name -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        last_name -> diesel::sql_types::Text,
        created_at -> diesel::sql_types::Timestamp,
        updated_at -> diesel::sql_types::Timestamp,
        thumbnail_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        picture_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
    }
}

