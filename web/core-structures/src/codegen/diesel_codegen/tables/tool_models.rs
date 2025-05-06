diesel::table! {
    public.tool_models(id) { id -> diesel::sql_types::Integer, tool_category ->
    tool_categories::diesel_impls::ToolCategory, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
