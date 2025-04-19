diesel::table! {
    public.user_organizations(user_id, organization_id) { user_id ->
    diesel::sql_types::Integer, organization_id -> diesel::sql_types::SmallInt }
}
