diesel::table! {
    public.document_formats(id) { extension -> diesel::sql_types::Text, mime_type ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text, icon ->
    diesel::sql_types::Text, color -> diesel::sql_types::Text, id ->
    diesel::sql_types::SmallInt }
}
