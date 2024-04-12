diesel::table! {
    edits_view (id) {
        id -> diesel::sql_types::Uuid,
        editable_id -> diesel::sql_types::Uuid,
        edited_at -> diesel::sql_types::Timestamp,
        editor_id -> diesel::sql_types::Uuid,
        editor_first_name -> diesel::sql_types::Text,
        editor_middle_name -> diesel::sql_types::Text,
        editor_last_name -> diesel::sql_types::Text,
        edit_title -> diesel::sql_types::Text,
        edit_extended_reason -> diesel::sql_types::Text,
    }
}

diesel::table! {
    last_edits_view (id) {
        id -> diesel::sql_types::Uuid,
        editable_id -> diesel::sql_types::Uuid,
        edited_at -> diesel::sql_types::Timestamp,
        editor_id -> diesel::sql_types::Uuid,
        editor_first_name -> diesel::sql_types::Text,
        editor_middle_name -> diesel::sql_types::Text,
        editor_last_name -> diesel::sql_types::Text,
        edit_title -> diesel::sql_types::Text,
        edit_extended_reason -> diesel::sql_types::Text,
    }
}

diesel::table! {
    formats_view (id) {
        id -> diesel::sql_types::Uuid,
        mime_type -> diesel::sql_types::Text,
        extension -> diesel::sql_types::Text,
        format_description -> diesel::sql_types::Text,
    }
}

diesel::table! {
    documents_view (id) {
        id -> diesel::sql_types::Uuid,
        format_id -> diesel::sql_types::Uuid,
        document_path -> diesel::sql_types::Text,
        bytes -> diesel::sql_types::Integer,
        document_name -> diesel::sql_types::Text,
        description -> diesel::sql_types::Text,
        creator_id -> diesel::sql_types::Uuid,
        creator_first_name -> diesel::sql_types::Text,
        creator_middle_name -> diesel::sql_types::Text,
        creator_last_name -> diesel::sql_types::Text,
        last_edit_id -> diesel::sql_types::Uuid,
        last_edit_at -> diesel::sql_types::Timestamp,
        last_editor_id -> diesel::sql_types::Uuid,
        last_editor_first_name -> diesel::sql_types::Text,
        last_editor_middle_name -> diesel::sql_types::Text,
        last_editor_last_name -> diesel::sql_types::Text,
        last_edit_title -> diesel::sql_types::Text,
        last_edit_extended_reason -> diesel::sql_types::Text,
        extension -> diesel::sql_types::Text,
        mime_type -> diesel::sql_types::Text,
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

