diesel::table! {
    documents_view (id) {
        id -> Uuid,
        format_id -> Uuid,
        document_path -> Text,
        bytes -> Int4,
        document_name -> Text,
        description -> Nullable<Text>,
        creator_id -> Uuid,
        creator_first_name -> Nullable<Text>,
        creator_middle_name -> Nullable<Text>,
        creator_last_name -> Nullable<Text>,
        last_edit_id -> Nullable<Uuid>,
        last_edit_at -> Nullable<Text>,
        last_editor_id -> Nullable<Uuid>,
        last_editor_first_name -> Nullable<Text>,
        last_editor_middle_name -> Nullable<Text>,
        last_editor_last_name -> Nullable<Text>,
        last_edit_title -> Nullable<Text>,
        last_edit_extended_reason -> Nullable<Text>,
        extension -> Text,
        mime_type -> Text,
    }
}
